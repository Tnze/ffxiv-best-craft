use crate::solver::Solver;
use ffxiv_crafting::{Actions, Attributes, Recipe, Status};
use micro_ndarray::Array;
use std::{cell::Cell, sync::Arc};

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

// pub struct QualitySolver<const MN: usize, const WN: usize>
// where
//     [[(); WN]; MN]:,
// {
//     init_status: Status,
//     progress_solver: Arc<ProgressSolver<MN, WN>>,
//     // results [d][cp][iq][iv][gs][mn][wn][touch]
//     // results: Vec<Vec<[[[[[[SolverSlot<u32>; 3]; WN]; MN]; 4]; 5]; 11]>>,
//     results: Array<SolverSlot<u32>, 8>,
// }

// impl<const MN: usize, const WN: usize> QualitySolver<MN, WN>
// where
//     [[(); WN]; MN]:,
// {
//     pub fn new(init_status: Status, progress_solver: Arc<ProgressSolver<MN, WN>>) -> Self {
//         let cp = init_status.attributes.craft_points as usize;
//         let du = init_status.recipe.durability as usize;
//         Self {
//             init_status,
//             progress_solver,
//             // results: vec![vec![Self::DEFAULT_ARY; cp + 1]; du / 5 + 1],
//             results: Array::new([du / 5 + 1, cp + 1, 11, 5, 4, MN, WN, 3]),
//         }
//     }

//     pub(crate) fn get(&self, s: &Status) -> Option<&SolverSlot<u32>> {
//         self.results
//             .get(s.durability as usize / 5)?
//             .get(s.craft_points as usize)?
//             .get(s.buffs.inner_quiet as usize)?
//             .get(s.buffs.innovation as usize)?
//             .get(s.buffs.great_strides as usize)?
//             .get(s.buffs.manipulation as usize)?
//             .get(s.buffs.wast_not as usize)?
//             .get(s.buffs.touch_combo_stage as usize)
//     }

//     pub(crate) unsafe fn get_unchecked(&self, s: &Status) -> &SolverSlot<u32> {
//         &self
//             .results
//             .get_unchecked(s.durability as usize / 5)
//             .get_unchecked(s.craft_points as usize)
//             .get_unchecked(s.buffs.inner_quiet as usize)
//             .get_unchecked(s.buffs.innovation as usize)
//             .get_unchecked(s.buffs.great_strides as usize)
//             .get_unchecked(s.buffs.manipulation as usize)
//             .get_unchecked(s.buffs.wast_not as usize)
//             .get_unchecked(s.buffs.touch_combo_stage as usize)
//     }
// }

// impl<const MN: usize, const WN: usize> Solver for QualitySolver<MN, WN>
// where
//     [[(); WN]; MN]:,
// {
//     fn init(&mut self) {
//         let mut s = self.init_status.clone();
//         let progress_solver = &*self.progress_solver;
//         let difficulty = s.recipe.difficulty;
//         for cp in 0..=s.attributes.craft_points {
//             s.craft_points = cp;
//             for du in (1..=s.recipe.durability).filter(|v| v % 5 == 0) {
//                 s.durability = du;
//                 for iq in 0..=MAX_INNER_QUIET {
//                     s.buffs.inner_quiet = iq;
//                     for iv in 0..=MAX_INNOVATION {
//                         s.buffs.innovation = iv;
//                         for gs in 0..=MAX_GREAT_STRIDES {
//                             s.buffs.great_strides = gs;
//                             for mn in 0..MN {
//                                 s.buffs.manipulation = mn as u8;
//                                 for wn in 0..WN {
//                                     s.buffs.wast_not = wn as u8;
//                                     for touch in 0..3 {
//                                         s.buffs.touch_combo_stage = touch as u8;
//                                         for sk in &TOUCH_SKILLS {
//                                             match sk {
//                                                 &Actions::Manipulation if MN == 0 => continue,
//                                                 &Actions::WasteNot | &Actions::WasteNotII
//                                                     if WN == 0 =>
//                                                 {
//                                                     continue
//                                                 }

//                                                 _ => {}
//                                             }
//                                             if s.is_action_allowed(*sk).is_err() {
//                                                 continue;
//                                             }
//                                             let mut new_s = s.clone();
//                                             new_s.cast_action(*sk);
//                                             unsafe {
//                                                 let progress =
//                                                     progress_solver.get_unchecked(&new_s).value;
//                                                 if new_s.progress + progress >= difficulty {
//                                                     let mut quality = new_s.quality;
//                                                     let mut step = 1;
//                                                     {
//                                                         let next = self.get_unchecked(&new_s);
//                                                         quality += next.value;
//                                                         step += next.step;
//                                                     }
//                                                     let slot = self
//                                                         .results
//                                                         .get_unchecked_mut(du as usize / 5)
//                                                         .get_unchecked_mut(cp as usize)
//                                                         .get_unchecked_mut(iq as usize)
//                                                         .get_unchecked_mut(iv as usize)
//                                                         .get_unchecked_mut(gs as usize)
//                                                         .get_unchecked_mut(mn as usize)
//                                                         .get_unchecked_mut(wn as usize)
//                                                         .get_unchecked_mut(touch);
//                                                     if (quality == slot.value && step < slot.step)
//                                                         || quality > slot.value
//                                                     {
//                                                         *slot = SolverSlot {
//                                                             value: quality,
//                                                             step,
//                                                             action: Some(*sk),
//                                                         }
//                                                     }
//                                                 }
//                                             }
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     fn read(&self, s: &Status) -> Option<(Actions, u32)> {
//         let max_quality = s.recipe.quality;
//         let mut new_s = s.clone();
//         let max_addon = max_quality - s.quality;
//         let mut best = {
//             let Some(&SolverSlot {
//                 value: quality,
//                 step,
//                 action,
//             }) = self.get(&s) else {
//                 return None;
//             };
//             let quality = quality.min(max_addon);
//             ((quality, step), (s.craft_points, s.durability), action)
//         };
//         for cp in 0..=s.craft_points {
//             new_s.craft_points = cp;
//             for du in 1..=s.durability {
//                 new_s.durability = du;
//                 let Some(&SolverSlot {
//                     value: quality,
//                     step,
//                     action: skill,
//                 }) = self.get(&new_s)else {
//                     return None;
//                 };
//                 let quality = quality.min(max_addon);
//                 if quality >= best.0 .0 && step < best.0 .1 {
//                     best = ((quality, step), (cp, du), skill);
//                 }
//             }
//         }
//         match best.2 {
//             Some(sk) => Some((sk, best.0 .0)),
//             None => self.progress_solver.read(&s),
//         }
//     }

//     fn read_all(&self, s: &Status) -> Vec<Actions> {
//         let max_quality = s.recipe.quality;
//         let mut new_s = s.clone();
//         let mut list = Vec::new();
//         loop {
//             let max_addon = max_quality - new_s.quality;
//             let mut new_s2 = new_s.clone();
//             let mut best = {
//                 let Some(&SolverSlot {
//                     value: quality,
//                     step,
//                     action: skill,
//                 }) = self.get(&new_s)else {
//                     return vec![];
//                 };
//                 let quality = quality.min(max_addon);
//                 (
//                     (quality, step),
//                     (new_s.craft_points, new_s.durability),
//                     skill,
//                 )
//             };
//             for cp in 0..=new_s.craft_points {
//                 new_s2.craft_points = cp;
//                 for du in 1..=new_s.durability {
//                     new_s2.durability = du;
//                     let Some(&SolverSlot {
//                         value: quality,
//                         step,
//                         action: skill,
//                     }) = self.get(&new_s2) else {
//                         return vec![];
//                     };
//                     let quality = quality.min(max_addon);
//                     if quality >= best.0 .0 && step < best.0 .1 {
//                         best = ((quality, step), (cp, du), skill);
//                     }
//                 }
//             }
//             match best.2 {
//                 Some(sk) => {
//                     new_s.cast_action(sk);
//                     list.push(sk);
//                 }
//                 None => {
//                     list.append(&mut self.progress_solver.read_all(&new_s));
//                     break list;
//                 }
//             }
//         }
//     }
// }

/// ProgressSolver 是一种专注于推动进展的求解器，给定玩家属性和配方并经过初始化后，
/// 对于任意的当前状态，可以以O(1)时间复杂度算出剩余资源最多可推多少进展。
pub struct ProgressSolver<const MN: usize, const WN: usize>
where
    [[(); WN]; MN]:,
{
    init_status: Status,
    // results[ve][mm][mn][wn][d][cp]
    results: Array<Cell<Option<SolverSlot<u16>>>, 6>,
}

impl<const MN: usize, const WN: usize> ProgressSolver<MN, WN>
where
    [[(); WN]; MN]:,
{
    pub fn new(init_status: Status) -> Self {
        let cp = init_status.attributes.craft_points as usize;
        let du = init_status.recipe.durability as usize;
        Self {
            init_status,
            results: Array::new([5, 6, MN, WN, du / 5 + 1, cp + 1]),
        }
    }

    unsafe fn get_unchecked(&self, s: &Status) -> &Cell<Option<SolverSlot<u16>>> {
        self.results.get_unchecked([
            s.buffs.veneration as usize,
            s.buffs.muscle_memory as usize,
            s.buffs.manipulation as usize,
            s.buffs.wast_not as usize,
            s.durability as usize / 5,
            s.craft_points as usize,
        ])
    }

    fn get(&self, s: &Status) -> &Cell<Option<SolverSlot<u16>>> {
        &self.results[[
            s.buffs.veneration as usize,
            s.buffs.muscle_memory as usize,
            s.buffs.manipulation as usize,
            s.buffs.wast_not as usize,
            s.durability as usize / 5,
            s.craft_points as usize,
        ]]
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
            match sk {
                &Actions::Manipulation if MN < 9 => continue,
                &Actions::WasteNot if WN < 5 => continue,
                &Actions::WasteNotII if WN < 9 => continue,
                _ => {}
            }
            if s.is_action_allowed(*sk).is_err() {
                continue;
            }
            let mut new_s = s.clone();
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

impl<const MN: usize, const WN: usize> Solver for ProgressSolver<MN, WN>
where
    [[(); WN]; MN]:,
{
    fn init(&mut self) {}

    fn read(&self, s: &Status) -> Option<Actions> {
        let difficulty = s.recipe.difficulty;
        let max_addon = difficulty - s.progress;
        let mut new_s2 = s.clone();
        let mut best = {
            let SolverSlot {
                value: progress,
                step,
                action: skill,
            } = self.inner_read(&s);
            let progress = progress.min(max_addon);
            ((progress, step), (s.craft_points, s.durability), skill)
        };
        for cp in 0..=s.craft_points {
            new_s2.craft_points = cp;
            for du in 1..=s.durability {
                new_s2.durability = du;
                let SolverSlot {
                    value: progress,
                    step,
                    action: skill,
                } = self.inner_read(&new_s2);
                let progress = progress.min(max_addon);
                if progress >= best.0 .0 && step < best.0 .1 {
                    best = ((progress, step), (cp, du), skill);
                }
            }
        }
        match best.2 {
            Some(sk) => Some(sk),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use ffxiv_crafting::{Attributes, Recipe, Status};

    use super::ProgressSolver;
    use super::Solver;

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
        let mut solver = ProgressSolver::<8, 8>::new(init_status.clone());
        solver.init();
        let actions = solver.read_all(&init_status);
        println!("{actions:?}");
    }
}