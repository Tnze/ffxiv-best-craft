use ffxiv_crafting::{Actions, Status, Buffs};
use micro_ndarray::Array;
use std::cell::Cell;

#[derive(Clone, Copy, Default)]
struct SolverSlot<T> {
    value: T,
    step: i32,
    action: Option<Actions>,
}

const SYNTH_SKILLS: [Actions; 9] = [
    Actions::BasicSynthesis,
    Actions::WasteNot,
    Actions::Veneration,
    Actions::WasteNotII,
    Actions::CarefulSynthesis,
    Actions::Groundwork,
    Actions::DelicateSynthesis,
    Actions::IntensiveSynthesis,
    Actions::PrudentSynthesis,
];

const TOUCH_SKILLS: [Actions; 13] = [
    Actions::BasicTouch,
    Actions::MastersMend,
    Actions::WasteNot,
    Actions::StandardTouch,
    Actions::GreatStrides,
    Actions::Innovation,
    Actions::WasteNotII,
    Actions::ByregotsBlessing,
    Actions::PrudentTouch,
    Actions::PreparatoryTouch,
    Actions::AdvancedTouch,
    Actions::TrainedFinesse,
    Actions::Manipulation,
];

pub struct QualitySolver {
    progress_solver: ProgressSolver,
    mn: usize,
    wn: usize,
    // results [iq][iv][gs][mn][wn][touch][d][cp]
    results: Array<Cell<Option<SolverSlot<u32>>>, 8>,
}

impl QualitySolver {
    pub fn new(init_status: Status, mn: usize, wn: usize) -> Self {
        let cp = init_status.attributes.craft_points as usize;
        let du = init_status.recipe.durability as usize;
        let progress_solver = ProgressSolver::new(init_status, mn, wn);
        Self {
            progress_solver,
            wn,
            mn,
            results: Array::new([11, 5, 4, mn + 1, wn + 1, 3, du / 5 + 1, cp + 1]),
        }
    }

    fn get(&self, s: &Status) -> &Cell<Option<SolverSlot<u32>>> {
        let i = [
            s.buffs.inner_quiet as usize,
            s.buffs.innovation as usize,
            s.buffs.great_strides as usize,
            s.buffs.manipulation as usize,
            s.buffs.wast_not as usize,
            s.buffs.touch_combo_stage as usize,
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
        if let Some(result) = slot.get() {
            return result;
        }
        if s.durability == 0 {
            let result = SolverSlot {
                value: 0,
                step: 0,
                action: None,
            };
            slot.set(Some(result));
            return result;
        }
        let mut best = SolverSlot {
            value: 0,
            step: 0,
            action: None,
        };
        for sk in &TOUCH_SKILLS {
            match sk {
                &Actions::Manipulation if self.mn == 0 => continue,
                &Actions::WasteNot | &Actions::WasteNotII if self.wn == 0 => continue,
                _ => {}
            }
            if s.is_action_allowed(*sk).is_err() {
                continue;
            }

            let mut new_s = s.clone();
            new_s.buffs = Buffs {
                great_strides: s.buffs.great_strides,
                innovation: s.buffs.innovation,
                inner_quiet: s.buffs.inner_quiet,
                manipulation: s.buffs.manipulation,
                wast_not: s.buffs.wast_not,
                touch_combo_stage: s.buffs.touch_combo_stage,
                ..Buffs::default()
            };
            new_s.quality = 0;
            new_s.cast_action(*sk);

            let progress = self.progress_solver.inner_read(&new_s).value;
            if new_s.progress + progress >= new_s.recipe.difficulty {
                let mut quality = new_s.quality;
                let mut step = 1;
                {
                    let next = self.inner_read(&new_s);
                    quality += next.value;
                    step += next.step;
                }
                if (quality == best.value && step < best.step) || quality > best.value {
                    best = SolverSlot {
                        value: quality,
                        step,
                        action: Some(*sk),
                    }
                }
            }
        }
        slot.set(Some(best));
        best
    }
}

impl crate::solver::Solver for QualitySolver {
    fn init(&mut self) {}

    fn read(&self, s: &Status) -> Option<Actions> {
        if s.is_finished() {
            return None
        }
        let max_quality = s.recipe.quality;
        let mut new_s = s.clone();
        let max_addon = max_quality - s.quality;
        let mut best = {
            let SolverSlot {
                value: quality,
                step,
                action,
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
    mn: usize,
    wn: usize,
    // results[ve][mm][mn][wn][d][cp]
    results: Array<Cell<Option<SolverSlot<u16>>>, 6>,
}

impl ProgressSolver {
    pub fn new(init_status: Status, mn: usize, wn: usize) -> Self {
        let cp = init_status.attributes.craft_points as usize;
        let du = init_status.recipe.durability as usize;
        Self {
            mn,
            wn,
            results: Array::new([5, 6, mn + 1, mn + 1, du / 5 + 1, cp + 1]),
        }
    }

    fn get(&self, s: &Status) -> &Cell<Option<SolverSlot<u16>>> {
        let i = [
            s.buffs.veneration as usize,
            s.buffs.muscle_memory as usize,
            s.buffs.manipulation as usize,
            s.buffs.wast_not as usize,
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

    fn inner_read(&self, s: &Status) -> SolverSlot<u16> {
        let slot = self.get(s);
        if let Some(result) = slot.get() {
            return result;
        }
        if s.durability == 0 {
            let result = SolverSlot {
                value: 0,
                step: 0,
                action: None,
            };
            slot.set(Some(result));
            return result;
        }
        let mut best = SolverSlot {
            value: 0,
            step: 0,
            action: None,
        };
        for sk in &SYNTH_SKILLS {
            match *sk {
                Actions::Manipulation if self.mn < 9 => continue,
                Actions::WasteNot if self.wn < 5 => continue,
                Actions::WasteNotII if self.wn < 9 => continue,
                _ => {}
            }
            if s.is_action_allowed(*sk).is_err() {
                continue;
            }
            let mut new_s = s.clone();
            new_s.buffs = Buffs{
                muscle_memory: s.buffs.muscle_memory,
                veneration: s.buffs.veneration,
                manipulation: s.buffs.manipulation,
                wast_not: s.buffs.wast_not,
                ..Buffs::default()
            };
            new_s.progress = 0;
            new_s.cast_action(*sk);
            let mut progress = new_s.progress;
            let mut step = 1;
            if new_s.durability > 0 {
                let next = self.inner_read(&new_s);
                progress += next.value;
                step += next.step;
            }
            if progress > best.value || (progress == best.value && step < best.step) {
                best = SolverSlot {
                    value: progress,
                    step,
                    action: Some(*sk),
                }
            }
        }
        slot.set(Some(best));
        best
    }
}

impl crate::solver::Solver for ProgressSolver {
    fn init(&mut self) {}

    fn read(&self, s: &Status) -> Option<Actions> {
        if s.is_finished() {
            return None
        }
        let difficulty = s.recipe.difficulty;
        let max_addon = difficulty - s.progress;
        let mut best = {
            let SolverSlot {
                value: progress,
                step,
                action,
            } = self.inner_read(s);
            let progress = progress.min(max_addon);
            (progress, step, action)
        };
        let mut new_s2 = s.clone();
        for cp in 0..=s.craft_points {
            new_s2.craft_points = cp;
            for du in 1..=s.durability {
                new_s2.durability = du;
                let SolverSlot {
                    value: progress,
                    step,
                    action,
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

#[cfg(test)]
mod test {
    use ffxiv_crafting::{Attributes, Recipe, Status};

    use super::ProgressSolver;
    use super::QualitySolver;
    use crate::solver::Solver;

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
        let init_status = init();
        let solver = ProgressSolver::new(init_status.clone(), 8, 8);
        let actions = solver.read_all(&init_status);
        println!("{actions:?}");
    }

    #[test]
    fn test2() {
        let mut init_status = init();
        init_status.cast_action(ffxiv_crafting::Actions::Reflect);
        let solver = QualitySolver::new(init_status.clone(), 8, 8);
        let actions = solver.read_all(&init_status);
        println!("{actions:?}");
    }
}
