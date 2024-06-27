// This file is part of BestCraft.
// Copyright (C) 2024 Tnze
//
// BestCraft is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// BestCraft is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use app_libs::ffxiv_crafting::{Actions, Buffs, Status};
use micro_ndarray::Array;
use std::cell::Cell;

#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct Slot {
    pub(crate) score: u32,
    pub(crate) steps: u16,
    pub(crate) action: Option<Actions>,
    is_some: bool,
}

pub struct Solver {
    init_status: Status,
    allow_mn: bool,
    allow_wn: usize,
    allow_obz: bool,
    touch_caches: Array<Cell<Slot>, 9>,
}

impl Solver {
    const MAX_INNER_QUIET: usize = 10;
    const MAX_INNOVATION: usize = 4;
    const MAX_MANIPULATION: usize = 8;
    const MAX_GREAT_STRIDES: usize = 3;
    const MAX_TOUCH_COMBO: usize = 2;
    // const MAX_VENERATION: usize = 4;
    const MAX_OBSERVE: usize = 1;
    const TOUCH_SKILLS: [(Actions, u16); 17] = [
        (Actions::BasicTouch, 10),
        (Actions::RefinedTouch, 10)
        (Actions::StandardTouch, 10),
        (Actions::AdvancedTouch, 10),
        (Actions::PrudentTouch, 5),
        (Actions::PreparatoryTouch, 20),
        (Actions::TrainedFinesse, 0),
        (Actions::GreatStrides, 0),
        (Actions::ByregotsBlessing, 10),
        (Actions::Observe, 0),
        (Actions::Manipulation, 0),
        (Actions::Innovation, 0),
        (Actions::QuickInnovation, 0)
        (Actions::WasteNot, 0),
        (Actions::WasteNotII, 0),
        (Actions::MastersMend, 0),
        (Actions::TrainedPerfection, 0),
        (Actions::ImmaculateMend, 0)
    ];

    pub(crate) fn new(init_status: Status, mn: bool, wn: usize, obz: bool) -> Self {
        let size = [
            obz as usize * Self::MAX_OBSERVE + 1,
            Self::MAX_INNER_QUIET + 1,
            Self::MAX_INNOVATION + 1,
            Self::MAX_GREAT_STRIDES + 1,
            mn as usize * Self::MAX_MANIPULATION + 1,
            wn + 1,
            Self::MAX_TOUCH_COMBO + 1,
            init_status.recipe.durability as usize / 5 + 1,
            init_status.attributes.craft_points as usize + 1,
        ];
        // let touch_caches = Array::new(size);
        let touch_caches = unsafe {
            use std::alloc::{alloc_zeroed, Layout};

            let length = size.iter().product();
            let layout = Layout::array::<Cell<Slot>>(length).unwrap();
            let ptr = alloc_zeroed(layout).cast();
            let data = Vec::from_raw_parts(ptr, length, length);
            Array::from_flat(data, size).unwrap()
        };
        Self {
            allow_mn: mn,
            allow_wn: wn,
            allow_obz: obz,
            touch_caches,
            init_status,
        }
    }

    pub(crate) fn next_touch(&self, craft_points: i32, durability: u16, buffs: Buffs) -> Slot {
        let this_cell = unsafe {
            self.touch_caches.get_unchecked([
                buffs.observed as usize,
                buffs.inner_quiet as usize,
                buffs.innovation as usize,
                buffs.great_strides as usize,
                buffs.manipulation as usize,
                buffs.wast_not as usize,
                buffs.touch_combo_stage as usize,
                durability as usize / 5,
                craft_points as usize,
            ])
        };
        {
            let slot = this_cell.get();
            if slot.is_some {
                return slot;
            }
        }
        let mut best = Slot {
            score: 0,
            steps: 0,
            action: None,
            is_some: true,
        };
        let mut curr_status = self.init_status.clone();
        curr_status.craft_points = craft_points;
        curr_status.durability = durability;
        curr_status.buffs = buffs;

        let filter = |(action, consumed_du)| {
            if durability < curr_status.calc_durability(consumed_du) {
                return None;
            }
            match action {
                x if curr_status.is_action_allowed(x).is_err() => None,
                x if curr_status.success_rate(x) < 100 => None,
                Actions::Manipulation if !self.allow_mn => None,
                Actions::WasteNotII if self.allow_wn < 8 || curr_status.buffs.wast_not >= 8 => None,
                Actions::WasteNot if self.allow_wn < 4 || curr_status.buffs.wast_not >= 4 => None,
                Actions::Observe if !self.allow_obz => None,
                x => Some(x),
            }
        };
        for action in Self::TOUCH_SKILLS.into_iter().filter_map(filter) {
            let mut s = curr_status.clone();
            let quality_before = s.quality;
            s.cast_action(action);
            let mut score = s.quality - quality_before;
            let mut steps = 1;
            if let Slot {
                score: next_score,
                steps: next_steps,
                action: Some(_),
                ..
            } = self.next_touch(s.craft_points, s.durability, s.buffs)
            {
                score += next_score;
                steps += next_steps;
            }
            if score
                .cmp(&best.score)
                .then_with(|| steps.cmp(&best.steps).reverse())
                .is_gt()
            {
                best = Slot {
                    score,
                    steps,
                    action: Some(action),
                    is_some: true,
                };
            }
        }
        this_cell.set(best);
        best
    }
}
