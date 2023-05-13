use std::cell::{Cell, UnsafeCell};

use ffxiv_crafting::{Actions, Buffs, Status};
use micro_ndarray::Array;

pub struct Solver {
    init_status: Status,
    touch_caches: Array<Cell<Option<Actions>>, 9>,
    synth_caches: Vec<UnsafeCell<Option<Array<Cell<Option<Actions>>, 6>>>>,
}

impl Solver {
    const MAX_INNER_QUIET: usize = 10;
    const MAX_INNOVATION: usize = 4;
    const MAX_MANIPULATION: usize = 8;
    const MAX_WASTE_NOT: usize = 8;
    const MAX_GREAT_STRIDES: usize = 3;
    const MAX_TOUCH_COMBO: usize = 2;
    const MAX_VENERATION: usize = 4;
    const MAX_OBSERVE: usize = 1;
    const TOUCH_SKILLS: [(Actions, u16); 15] = [
        (Actions::BasicTouch, 10),
        (Actions::MastersMend, 0),
        (Actions::StandardTouch, 10),
        (Actions::GreatStrides, 0),
        (Actions::Innovation, 0),
        (Actions::ByregotsBlessing, 10),
        (Actions::PrudentTouch, 5),
        (Actions::PreparatoryTouch, 20),
        (Actions::AdvancedTouch, 10),
        (Actions::TrainedFinesse, 0),
        (Actions::Manipulation, 0),
        (Actions::Observe, 0),
        (Actions::FocusedTouch, 10),
        (Actions::WasteNot, 0),
        (Actions::WasteNotII, 0),
    ];
    const SYNTH_SKILLS: [(Actions, u16); 10] = [
        (Actions::PrudentSynthesis, 5),
        (Actions::BasicSynthesis, 10),
        (Actions::CarefulSynthesis, 10),
        (Actions::Groundwork, 20),
        (Actions::Veneration, 0),
        (Actions::Manipulation, 0),
        (Actions::Observe, 0),
        (Actions::FocusedSynthesis, 10),
        (Actions::WasteNot, 0),
        (Actions::WasteNotII, 0),
    ];

    pub fn new(init_status: Status) -> Self {
        Self {
            touch_caches: Array::new([
                init_status.attributes.craft_points as usize + 1,
                init_status.recipe.durability as usize + 1,
                Self::MAX_INNER_QUIET + 1,
                Self::MAX_INNOVATION + 1,
                Self::MAX_MANIPULATION + 1,
                Self::MAX_WASTE_NOT + 1,
                Self::MAX_GREAT_STRIDES + 1,
                Self::MAX_TOUCH_COMBO + 1,
                Self::MAX_OBSERVE + 1,
            ]),
            synth_caches: std::iter::from_fn(|| Some(UnsafeCell::new(None)))
                .take(init_status.recipe.difficulty as usize + 1)
                .collect(),
            init_status,
        }
    }

    fn new_synth_table(&self) -> Array<Cell<Option<Actions>>, 6> {
        Array::new([
            self.init_status.attributes.craft_points as usize + 1,
            self.init_status.recipe.durability as usize + 1,
            Self::MAX_VENERATION + 1,
            Self::MAX_MANIPULATION + 1,
            Self::MAX_WASTE_NOT + 1,
            Self::MAX_OBSERVE + 1,
        ])
    }

    pub fn next_touch(&self, craft_points: i32, durability: u16, buffs: Buffs) -> Option<Actions> {
        if craft_points == 0 || durability == 0 {
            return None;
        }
        let this_cell = &self.touch_caches[[
            craft_points as usize,
            durability as usize,
            buffs.inner_quiet as usize,
            buffs.innovation as usize,
            buffs.manipulation as usize,
            buffs.wast_not as usize,
            buffs.great_strides as usize,
            buffs.touch_combo_stage as usize,
            buffs.observed as usize,
        ]];
        if let Some(action) = this_cell.get() {
            return Some(action);
        }
        let mut init_status = self.init_status.clone();
        init_status.craft_points = craft_points;
        init_status.durability = durability;
        init_status.buffs = buffs;
        let mut best_action = None;
        let mut best_quality = None;
        for (action, consumed_du) in Self::TOUCH_SKILLS {
            if init_status.is_action_allowed(action).is_err()
                || durability < init_status.calc_durability(consumed_du)
                || init_status.success_rate(action) < 100
            {
                continue;
            }
            let mut s = init_status.clone();
            s.cast_action(action);
            while let Some(action) = self.next_touch(s.craft_points, s.durability, s.buffs) {
                if let Err(e) = s.is_action_allowed(action) {
                    panic!("not allowed {:?} on {:?}: {:?}", action, s, e);
                }
                s.cast_action(action);
            }
            if s.quality > best_quality.unwrap_or_default() {
                best_action = Some(action);
                best_quality = Some(s.quality);
            }
        }
        this_cell.set(best_action);
        best_action
    }

    fn next_synth(
        &self,
        progress: u16,
        craft_points: i32,
        durability: u16,
        buffs: Buffs,
    ) -> Option<Actions> {
        if craft_points == 0 || durability == 0 {
            return None;
        }
        let this_cell = unsafe {
            (*self.synth_caches[progress as usize].get())
                .get_or_insert_with(|| self.new_synth_table())
                .get_unchecked_mut([
                    craft_points as usize,
                    durability as usize,
                    buffs.veneration as usize,
                    buffs.manipulation as usize,
                    buffs.wast_not as usize,
                    buffs.observed as usize,
                ])
        };
        if let Some(action) = this_cell.get() {
            return Some(action);
        }
        let mut init_status = self.init_status.clone();
        init_status.craft_points = craft_points;
        init_status.durability = durability;
        init_status.buffs = buffs;
        let mut best_action = None;
        let mut best_progress = None;
        for (action, consumed_du) in Self::SYNTH_SKILLS {
            if init_status.is_action_allowed(action).is_err()
                || durability < init_status.calc_durability(consumed_du)
                || init_status.success_rate(action) < 100
            {
                continue;
            }
            let mut s = init_status.clone();
            s.cast_action(action);
            while !s.is_finished() && let Some(action) =
                self.next_synth(s.progress, s.craft_points, s.durability, s.buffs)
            {
                if let Err(e) = s.is_action_allowed(action) {
                    panic!("not allowed {:?} on {:?}: {:?}", action, s, e);
                }
                s.cast_action(action);
            }
            if s.progress > best_progress.unwrap_or_default() {
                best_action = Some(action);
                best_progress = Some(s.progress);
            }
        }
        this_cell.set(best_action);
        best_action
    }
}

impl crate::solver::Solver for Solver {
    fn init(&mut self) {}

    fn read(&self, _s: &Status) -> Option<Actions> {
        None
    }

    fn read_all(&self, s: &Status) -> Vec<Actions> {
        let mut best = Vec::new();
        let mut best_quality = 0;
        let mut buffer = Vec::new();
        // 尝试所有资源分配组合
        for synth_cp in 0..s.craft_points {
            for synth_du in 1..s.durability {
                // 测试此方案是否能推满进展
                let mut tmp_s = s.clone();
                while !tmp_s.is_finished() && let Some(action) = self.next_synth(s.progress, synth_cp, synth_du, self.init_status.buffs) {
                    tmp_s.cast_action(action)
                }
                if tmp_s.progress < tmp_s.recipe.difficulty {
                    continue; // 推不满，方案否决
                }

                // 测试该方案能推多少品质
                buffer.clear();
                let mut tmp_s = s.clone();
                while let Some(action) = self.next_touch(
                    tmp_s.craft_points - synth_cp,
                    tmp_s.durability - synth_du,
                    s.buffs,
                ) {
                    tmp_s.cast_action(action);
                    buffer.push(action);
                }
                if best_quality < tmp_s.quality {
                    best.clone_from(&buffer);
                    best_quality = tmp_s.quality;
                }
            }
        }
        best
    }
}

#[cfg(test)]
mod test {
    use ffxiv_crafting::{Attributes, Recipe, Status};

    fn init() -> Status {
        let r = Recipe {
            rlv: 620,
            job_level: 90,
            difficulty: 5720,
            quality: 12900,
            durability: 70,
            conditions_flag: 15,
        };
        let a = Attributes {
            level: 90,
            craftsmanship: 4214,
            control: 3528,
            craft_points: 691,
        };
        Status::new(a, r)
    }

    #[test]
    fn test() {
        let mut status = init();
        let solver = super::Solver::new(status.clone());
        while let Some(action) =
            solver.next_touch(status.craft_points, status.durability, status.buffs)
        {
            println!("{:?}", action);
            if let Err(err) = status.is_action_allowed(action) {
                panic!("not allowed on {:?}: {:?}", status, err);
            }
            status.cast_action(action);
            if status.is_finished() {
                break;
            }
        }
    }
    #[test]
    fn test2() {
        let mut status = init();
        let solver = super::Solver::new(status.clone());
        while let Some(action) =
            solver.next_touch(status.craft_points, status.durability, status.buffs)
        {
            println!("{:?}", action);
            if let Err(err) = status.is_action_allowed(action) {
                panic!("not allowed on {:?}: {:?}", status, err);
            }
            status.cast_action(action);
            if status.is_finished() {
                break;
            }
        }
    }
}
