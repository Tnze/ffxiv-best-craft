// This file is part of BestCraft.
// Copyright (C) 2026 Tnze
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

use crate::{
    simulate,
    solver::{Score, Solver},
    SimulateResult,
};
use ffxiv_crafting::{Actions, Buffs, Status};
use micro_ndarray::Array;
use std::cell::Cell;

pub fn solve(
    status: Status,
    use_manipulation: bool,
    use_waste_not: usize,
    use_observe: bool,
) -> Vec<Actions> {
    let solver = QualitySolver::new(
        status.clone(),
        use_manipulation || status.buffs.manipulation > 0,
        (status.buffs.wast_not.max(status.buffs.wast_not_ii) as usize + 1).max(use_waste_not + 1),
        use_observe,
    );
    let result1 = solver.read_all(&status);
    let SimulateResult { status: s1, .. } = simulate(status.clone(), result1.clone());
    // Try reflect
    let Ok(()) = status.is_action_allowed(Actions::Reflect) else {
        return result1;
    };
    let mut s = status.clone();
    s.cast_action(Actions::Reflect);
    let mut result2 = solver.read_all(&s);
    if result2.len() != 0 {
        result2.insert(0, Actions::Reflect);
    }
    let SimulateResult { status: s2, .. } = simulate(s, result2.clone());
    if Score::from((&s1, result1.len())) > Score::from((&s2, result2.len())) {
        result1
    } else {
        result2
    }
}

#[derive(Clone, Copy, Default)]
struct SolverSlot<T> {
    value: T,
    step: u16,
    action: Option<Actions>,
    is_some: bool,
}

const SYNTH_SKILLS: [Actions; 12] = [
    Actions::BasicSynthesis,
    Actions::WasteNot,
    Actions::Veneration,
    Actions::WasteNotII,
    Actions::CarefulSynthesis,
    Actions::Groundwork,
    Actions::DelicateSynthesis,
    Actions::IntensiveSynthesis,
    Actions::PrudentSynthesis,
    Actions::Observe,
    Actions::TrainedPerfection,
    Actions::ImmaculateMend,
];

const TOUCH_SKILLS: [Actions; 17] = [
    Actions::BasicTouch,
    Actions::RefinedTouch,
    Actions::MastersMend,
    Actions::WasteNot,
    Actions::StandardTouch,
    Actions::GreatStrides,
    Actions::Innovation,
    // Actions::QuickInnovation,
    Actions::WasteNotII,
    Actions::ByregotsBlessing,
    Actions::PrudentTouch,
    Actions::PreparatoryTouch,
    Actions::AdvancedTouch,
    Actions::TrainedFinesse,
    Actions::Manipulation,
    Actions::Observe,
    Actions::TrainedPerfection,
    Actions::ImmaculateMend,
];

pub struct QualitySolver {
    progress_solver: ProgressSolver,
    mn: bool,
    wn: usize,
    obz: bool,
    // results [obz][iq][iv][gs][mn][wn][touch][tp][qi][d][cp]
    results: Array<Cell<SolverSlot<u32>>, 10>,
}

impl QualitySolver {
    pub fn new(mut init_status: Status, mn: bool, wn: usize, obz: bool) -> Self {
        init_status.progress = 0;
        let cp = init_status.attributes.craft_points as usize;
        let du = init_status.recipe.durability as usize;
        let progress_solver = ProgressSolver::new(init_status, mn, wn, obz);
        let size = [
            obz as usize + 1,
            11,
            5,
            4,
            mn as usize * 8 + 1,
            wn + 1,
            3,
            3,
            du / 5 + 1,
            cp + 1,
        ];
        // let results = Array::new(size);
        let results = unsafe {
            use std::alloc::{alloc_zeroed, Layout};

            let length = size.iter().product();
            let layout = Layout::array::<Cell<SolverSlot<u32>>>(length).unwrap();
            let ptr = alloc_zeroed(layout).cast();
            let data = Vec::from_raw_parts(ptr, length, length);
            Array::from_flat(data, size).unwrap()
        };
        Self {
            progress_solver,
            wn,
            mn,
            obz,
            results,
        }
    }

    fn get(&self, s: &Status) -> &Cell<SolverSlot<u32>> {
        let i = [
            s.buffs.observed as usize,
            s.buffs.inner_quiet as usize,
            s.buffs.innovation as usize,
            s.buffs.great_strides as usize,
            s.buffs.manipulation as usize,
            s.buffs.wast_not.max(s.buffs.wast_not_ii) as usize,
            s.buffs.touch_combo_stage as usize,
            s.buffs.trained_perfection as usize,
            s.durability as usize / 5,
            s.craft_points as usize,
        ];
        // #[cfg(not(debug_assertions))]
        // unsafe {
        //     self.results.get_unchecked(i)
        // }
        // #[cfg(debug_assertions)]
        &self.results[i]
    }

    fn inner_read(&self, s: &Status) -> SolverSlot<u32> {
        let slot = self.get(s);
        {
            let result = slot.get();
            if result.is_some {
                return result;
            }
        }
        if s.durability == 0 {
            let result = SolverSlot {
                value: 0,
                step: 0,
                action: None,
                is_some: true,
            };
            slot.set(result);
            return result;
        }
        let mut best = SolverSlot {
            value: 0,
            step: 0,
            action: None,
            is_some: true,
        };
        for sk in TOUCH_SKILLS {
            if (matches!(sk, Actions::Manipulation) && !self.mn)
                || (matches!(sk, Actions::WasteNotII) && self.wn < 8)
                || (matches!(sk, Actions::WasteNot) && self.wn < 4)
                || (matches!(sk, Actions::Observe) && !self.obz)
                || (matches!(sk, Actions::AdvancedTouch)
                    && s.buffs.observed == 0
                    && s.buffs.touch_combo_stage != 2)
            {
                continue;
            }
            if s.is_action_allowed(sk).is_err() {
                continue;
            }

            let mut new_s = s.clone();
            new_s.quality = 0;
            new_s.cast_action(sk);

            let progress = self.progress_solver.inner_read(&new_s).value;
            if progress + new_s.progress >= new_s.recipe.difficulty {
                let next = self.inner_read(&new_s);
                let quality = new_s.quality + next.value;
                let step = 1 + next.step;

                if (quality == best.value && step < best.step) || quality > best.value {
                    best = SolverSlot {
                        value: quality,
                        step,
                        action: Some(sk),
                        is_some: true,
                    }
                }
            }
        }
        slot.set(best);
        best
    }
}

impl Solver for QualitySolver {
    fn init(&mut self) {}

    fn read(&self, s: &Status) -> Option<Actions> {
        if s.is_finished() {
            return None;
        }
        let max_quality = s.recipe.quality;
        let mut new_s = s.clone();
        new_s.buffs = Buffs {
            great_strides: s.buffs.great_strides,
            innovation: s.buffs.innovation,
            inner_quiet: s.buffs.inner_quiet,
            manipulation: s.buffs.manipulation,
            wast_not: s.buffs.wast_not,
            wast_not_ii: s.buffs.wast_not_ii,
            touch_combo_stage: s.buffs.touch_combo_stage,
            observed: s.buffs.observed,
            trained_perfection: s.buffs.trained_perfection,
            ..Buffs::default()
        };
        let max_addon = max_quality - s.quality;
        let mut best = {
            let SolverSlot {
                value: quality,
                step,
                action,
                ..
            } = self.inner_read(s);
            let quality = quality.min(max_addon);
            (quality, step, action)
        };
        for cp in 0..=s.craft_points {
            new_s.craft_points = cp;
            for du in 1..=s.durability {
                new_s.durability = du;
                let SolverSlot {
                    value: quality,
                    step,
                    action: skill,
                    ..
                } = self.inner_read(&new_s);
                let quality = quality.min(max_addon);
                if quality >= best.0 && step < best.1 {
                    best = (quality, step, skill);
                }
            }
        }
        best.2.or_else(|| self.progress_solver.read(s))
    }
}

/// ProgressSolver 是一种专注于推动进展的求解器，给定玩家属性和配方并经过初始化后，
/// 对于任意的当前状态，可以以O(1)时间复杂度算出剩余资源最多可推多少进展。
pub struct ProgressSolver {
    mn: bool,
    wn: usize,
    obz: bool,
    // [obz][ve][mn][wn][tp][d][cp]
    results: Array<Cell<SolverSlot<u16>>, 7>,
}

impl ProgressSolver {
    pub fn new(init_status: Status, mn: bool, wn: usize, obz: bool) -> Self {
        let cp = init_status.attributes.craft_points as usize;
        let du = init_status.recipe.durability as usize;
        let size = [
            obz as usize + 1,
            5,
            mn as usize * 8 + 1,
            wn + 1,
            3,
            du / 5 + 1,
            cp + 1,
        ];
        let results = unsafe {
            use std::alloc::{alloc_zeroed, Layout};

            let length = size.iter().product();
            let layout = Layout::array::<Cell<SolverSlot<u16>>>(length).unwrap();
            let ptr = alloc_zeroed(layout).cast();
            let data = Vec::from_raw_parts(ptr, length, length);
            Array::from_flat(data, size).unwrap()
        };
        Self {
            mn,
            wn,
            obz,
            results,
        }
    }

    fn get(&self, s: &Status) -> &Cell<SolverSlot<u16>> {
        let i = [
            s.buffs.observed as usize,
            s.buffs.veneration as usize,
            s.buffs.manipulation as usize,
            s.buffs.wast_not.max(s.buffs.wast_not_ii) as usize,
            s.buffs.trained_perfection as usize,
            (s.durability as usize).div_ceil(5),
            s.craft_points as usize,
        ];
        // #[cfg(not(debug_assertions))]
        // unsafe {
        //     self.results.get_unchecked(i)
        // }
        // #[cfg(debug_assertions)]
        &self.results[i]
    }

    fn inner_read(&self, s: &Status) -> SolverSlot<u16> {
        let slot = self.get(s);
        {
            let result = slot.get();
            if result.is_some {
                return result;
            }
        }
        if s.durability == 0 {
            let result = SolverSlot {
                value: 0,
                step: 0,
                action: None,
                is_some: true,
            };
            slot.set(result);
            return result;
        }
        let mut best = SolverSlot {
            value: 0,
            step: 0,
            action: None,
            is_some: true,
        };
        for sk in SYNTH_SKILLS {
            if (matches!(sk, Actions::Manipulation) && !self.mn)
                || (matches!(sk, Actions::WasteNotII) && self.wn < 8)
                || (matches!(sk, Actions::WasteNot) && self.wn < 4)
                || (matches!(sk, Actions::Observe) && !self.obz)
            {
                continue;
            }
            if s.is_action_allowed(sk).is_err() {
                continue;
            }
            let mut new_s = s.clone();
            new_s.progress = 0;
            new_s.cast_action(sk);
            let mut progress = new_s.progress;
            let mut step = 1;
            if new_s.durability > 0 {
                let next = self.inner_read(&new_s);
                progress += next.value;
                step += next.step;
            }
            if progress.min(s.recipe.difficulty) > best.value
                || (progress == best.value && step < best.step)
            {
                best = SolverSlot {
                    value: progress,
                    step,
                    action: Some(sk),
                    is_some: true,
                }
            }
        }
        slot.set(best);
        best
    }
}

impl Solver for ProgressSolver {
    fn init(&mut self) {}

    fn read(&self, s: &Status) -> Option<Actions> {
        if s.is_finished() {
            return None;
        }
        let difficulty = s.recipe.difficulty;
        let max_addon = difficulty - s.progress;
        let mut best = {
            let SolverSlot {
                value: progress,
                step,
                action,
                ..
            } = self.inner_read(s);
            let progress = progress.min(max_addon);
            (progress, step, action)
        };
        let mut new_s2 = s.clone();
        new_s2.buffs = Buffs {
            muscle_memory: s.buffs.muscle_memory,
            veneration: s.buffs.veneration,
            manipulation: s.buffs.manipulation,
            wast_not: s.buffs.wast_not,
            wast_not_ii: s.buffs.wast_not_ii,
            observed: s.buffs.observed,
            trained_perfection: s.buffs.trained_perfection,
            ..Buffs::default()
        };
        for cp in 0..=s.craft_points {
            new_s2.craft_points = cp;
            for du in 1..=s.durability {
                new_s2.durability = du;
                let SolverSlot {
                    value: progress,
                    step,
                    action,
                    ..
                } = self.inner_read(&new_s2);
                let progress = progress.min(max_addon);
                if progress >= best.0 && step < best.1 {
                    best = (progress, step, action);
                }
            }
        }
        best.2
    }
}
