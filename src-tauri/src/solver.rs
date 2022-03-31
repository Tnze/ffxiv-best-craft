use ffxiv_crafting::{Attributes, Buffs, Recipe, Skills, Status};

const MAX_GREAT_STRIDES: u8 = 3;
const MAX_VENERATION: u8 = 4;
const MAX_INNOVATION: u8 = 4;
const MAX_INNER_QUIET: u8 = 10;
const MAX_MANIPULATION: u8 = 8;
const MAX_WAST_NOT: u8 = 8;

// struct Solver {}

// impl Solver {
//     fn new(driver: Driver) -> Self {
//         Self {}
//     }

//     fn init(&mut self, report: Option<&mut impl FnMut(f32)>) {}
// }

#[derive(Copy, Clone)]
struct DriverSlot {
    progress: u16,
    step: u8,
    skill: Option<Skills>,
}

pub struct Driver {
    init_status: Status,
    // results[d][cp][obs][ve][mn][wn]
    results: Vec<Vec<[[[[DriverSlot; 9]; 9]; 5]; 2]>>,
}

impl Driver {
    pub fn new(s: &Status) -> Self {
        let cp = s.attributes.craft_points as usize;
        let du = s.recipe.durability as usize;
        let default = DriverSlot {
            progress: 0,
            step: 0,
            skill: None,
        };
        Self {
            init_status: s.clone(),
            results: vec![vec![[[[[default; 9]; 9]; 5]; 2]; cp + 1]; du + 1],
        }
    }

    pub fn init(&mut self, allowd_list: Vec<Skills>) {
        let mut s = self.init_status.clone();
        for cp in 0..=self.init_status.attributes.craft_points {
            s.craft_points = cp;
            for du in 0..=self.init_status.recipe.durability {
                s.durability = du;
                for obs in 0..=1 {
                    s.buffs.observed = obs;
                    for ve in 0..=MAX_VENERATION {
                        s.buffs.veneration = ve;
                        for mn in 0..=MAX_MANIPULATION {
                            s.buffs.manipulation = mn;
                            for wn in 0..=MAX_WAST_NOT {
                                s.buffs.wast_not = wn;
                                for sk in &allowd_list {
                                    let mut new_s = s.clone();
                                    if new_s.is_action_allowed(*sk).is_ok()
                                        && (!matches!(sk, &Skills::FocusedSynthesis)
                                            || new_s.buffs.observed > 0)
                                    {
                                        new_s.cast_action(*sk);
                                        let mut progress = new_s.progress;
                                        let mut step = 1;
                                        if new_s.durability > 0 {
                                            let next = &self.results[new_s.durability as usize]
                                                [new_s.craft_points as usize]
                                                [new_s.buffs.observed as usize]
                                                [new_s.buffs.veneration as usize]
                                                [new_s.buffs.manipulation as usize]
                                                [new_s.buffs.wast_not as usize];
                                            progress += next.progress;
                                            progress = progress.min(s.recipe.difficulty);
                                            step += next.step;
                                        }
                                        let slot = &mut self.results[du as usize][cp as usize]
                                            [obs as usize]
                                            [ve as usize][mn as usize]
                                            [wn as usize];
                                        if progress > slot.progress
                                            || (progress == slot.progress && step < slot.step)
                                        {
                                            *slot = DriverSlot {
                                                progress,
                                                step,
                                                skill: Some(*sk),
                                            };
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn read(&self, s: &Status) -> (u16, u8, Option<Skills>) {
        let DriverSlot {
            progress,
            step,
            skill,
        } = self.results[s.durability as usize][s.craft_points as usize][s.buffs.observed as usize]
            [s.buffs.veneration as usize][s.buffs.manipulation as usize][s.buffs.wast_not as usize];
        return (progress, step, skill);
    }

    pub fn read_all(&self, s: &Status) -> (u16, Vec<Skills>) {
        // Only the step 2 is required for solve a list how can we make progress
        // The purpose of step 1 is only to find out at least how many resources we can use.
        // Hopefully this will helps us avoid lengthy steps

        let max_progress_addon = s.recipe.difficulty - s.progress;
        let mut best = {
            let (progress, step, _skill) = self.read(s);
            let progress = progress.min(max_progress_addon);
            ((progress, step), (s.craft_points, s.durability))
        };
        // Step 1
        let mut new_s = s.clone();
        for cp in 0..=s.craft_points {
            new_s.craft_points = cp;
            for du in 1..=s.durability {
                new_s.durability = du;
                let (progress, step, _skill) = self.read(&new_s);
                let progress = progress.min(max_progress_addon);
                if progress >= best.0 .0 && step < best.0 .1 {
                    best = ((progress, step), (cp, du));
                }
            }
        }
        // Step 2
        new_s.craft_points = best.1 .0;
        new_s.durability = best.1 .1;
        let mut list = Vec::new();
        loop {
            if let (_progress, _step, Some(skill)) = self.read(&new_s) {
                new_s.cast_action(skill);
                list.push(skill);
            } else {
                break (new_s.progress, list);
            }
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct SolverHash {
    pub attributes: Attributes,
    pub recipe: Recipe,
}
