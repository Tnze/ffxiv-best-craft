use super::solver::Solver;
use ffxiv_crafting::{Attributes, Recipe, Skills, Status};

const MAX_GREAT_STRIDES: u8 = 3;
const MAX_VENERATION: u8 = 4;
const MAX_INNOVATION: u8 = 4;
const MAX_MUSCLE_MEMORY: u8 = 5;
const MAX_INNER_QUIET: u8 = 10;

#[derive(Copy, Clone)]
struct SolverSlot<V> {
    value: V,
    step: u8,
    skill: Option<Skills>,
}

pub struct OrdinarySolver<const MN: usize, const WN: usize>
where
    [(); MN + 1]:,
    [(); WN + 1]:,
{
    pub driver: ProgressSolver<MN, WN>,
    allowed_list: Vec<Skills>,
    // results [d][cp][iq][iv][gs][mn][wn]
    results: Vec<Vec<[[[[[SolverSlot<u32>; WN + 1]; MN + 1]; 4]; 5]; 11]>>,
}

impl<const MN: usize, const WN: usize> OrdinarySolver<MN, WN>
where
    [(); MN + 1]:,
    [(); WN + 1]:,
{
    const DEFAULT_SLOT: SolverSlot<u32> = SolverSlot {
        value: 0,
        step: 0,
        skill: None,
    };
    const DEFAULT_ARY: [[[[[SolverSlot<u32>; WN + 1]; MN + 1]; 4]; 5]; 11] =
        [[[[[Self::DEFAULT_SLOT; WN + 1]; MN + 1]; 4]; 5]; 11];
    pub fn new(driver: ProgressSolver<MN, WN>, allowed_list: Vec<Skills>) -> Self {
        let cp = driver.init_status.attributes.craft_points as usize;
        let du = driver.init_status.recipe.durability as usize;
        Self {
            driver,
            allowed_list,
            results: vec![vec![Self::DEFAULT_ARY; cp + 1]; du + 1],
        }
    }

    fn get(&self, s: &Status) -> Option<&SolverSlot<u32>> {
        self.results
            .get(s.durability as usize)?
            .get(s.craft_points as usize)?
            .get(s.buffs.inner_quiet as usize)?
            .get(s.buffs.innovation as usize)?
            .get(s.buffs.great_strides as usize)?
            .get(s.buffs.manipulation as usize)?
            .get(s.buffs.wast_not as usize)
    }

    unsafe fn get_unchecked(&self, s: &Status) -> &SolverSlot<u32> {
        &self
            .results
            .get_unchecked(s.durability as usize)
            .get_unchecked(s.craft_points as usize)
            .get_unchecked(s.buffs.inner_quiet as usize)
            .get_unchecked(s.buffs.innovation as usize)
            .get_unchecked(s.buffs.great_strides as usize)
            .get_unchecked(s.buffs.manipulation as usize)
            .get_unchecked(s.buffs.wast_not as usize)
    }
}

impl<const MN: usize, const WN: usize> Solver for OrdinarySolver<MN, WN>
where
    [(); MN + 1]:,
    [(); WN + 1]:,
{
    fn init(&mut self) {
        let mut s = self.driver.init_status.clone();
        let difficulty = s.recipe.difficulty;
        for cp in 0..=s.attributes.craft_points {
            s.craft_points = cp;
            for du in 1..=s.recipe.durability {
                s.durability = du;
                for iq in 0..=MAX_INNER_QUIET {
                    s.buffs.inner_quiet = iq;
                    for iv in 0..=MAX_INNOVATION {
                        s.buffs.innovation = iv;
                        for gs in 0..=MAX_GREAT_STRIDES {
                            s.buffs.great_strides = gs;
                            for mn in 0..=MN {
                                s.buffs.manipulation = mn as u8;
                                for wn in 0..=WN {
                                    s.buffs.wast_not = wn as u8;
                                    for sk in &self.allowed_list {
                                        if s.is_action_allowed(*sk).is_ok() {
                                            let mut new_s = s.clone();
                                            new_s.cast_action(*sk);
                                            unsafe {
                                                let progress =
                                                    self.driver.get_unchecked(&new_s).value;
                                                if progress == difficulty {
                                                    let mut quality = new_s.quality;
                                                    let mut step = 1;
                                                    {
                                                        let next = self.get_unchecked(&new_s);
                                                        quality += next.value;
                                                        step += next.step;
                                                    }
                                                    let slot = self
                                                        .results
                                                        .get_unchecked_mut(du as usize)
                                                        .get_unchecked_mut(cp as usize)
                                                        .get_unchecked_mut(iq as usize)
                                                        .get_unchecked_mut(iv as usize)
                                                        .get_unchecked_mut(gs as usize)
                                                        .get_unchecked_mut(mn as usize)
                                                        .get_unchecked_mut(wn as usize);
                                                    if quality > slot.value
                                                        || (quality == slot.value
                                                            && step < slot.step)
                                                    {
                                                        *slot = SolverSlot {
                                                            value: quality,
                                                            step,
                                                            skill: Some(*sk),
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
                }
            }
        }
    }

    fn read(&self, s: &Status) -> Option<Skills> {
        self.get(s)?.skill
    }

    fn read_all(&self, s: &Status) -> Vec<Skills> {
        let max_quality = s.recipe.quality;
        let mut new_s = s.clone();
        let mut list = Vec::new();
        loop {
            let max_addon = max_quality - new_s.quality;
            let mut new_s2 = new_s.clone();
            let mut best = {
                if let Some(&SolverSlot {
                    value: quality,
                    step,
                    skill,
                }) = self.get(&new_s)
                {
                    let quality = quality.min(max_addon);
                    (
                        (quality, step),
                        (new_s.craft_points, new_s.durability),
                        skill,
                    )
                } else {
                    return vec![];
                }
            };
            for cp in 0..=new_s.craft_points {
                new_s2.craft_points = cp;
                for du in 1..=new_s.durability {
                    new_s2.durability = du;
                    if let Some(&SolverSlot {
                        value: quality,
                        step,
                        skill,
                    }) = self.get(&new_s2)
                    {
                        let quality = quality.min(max_addon);
                        if quality >= best.0 .0 && step < best.0 .1 {
                            best = ((quality, step), (cp, du), skill);
                        }
                    } else {
                        return vec![];
                    }
                }
            }
            match best.2 {
                Some(sk) => {
                    new_s.cast_action(sk);
                    list.push(sk);
                }
                None => {
                    list.append(&mut self.driver.read_all(&new_s));
                    break list;
                }
            }
        }
    }
}

/// ProgressSolver 是一种专注于推动进展的求解器，给定玩家属性和配方并经过初始化后，
/// 对于任意的当前状态，可以以O(1)时间复杂度算出剩余资源最多可推多少进展。
pub struct ProgressSolver<const MN: usize, const WN: usize>
where
    [(); MN + 1]:,
    [(); WN + 1]:,
{
    init_status: Status,
    allowed_list: Vec<Skills>,
    // results[d][cp][ve][mm][mn][wn]
    results: Vec<Vec<[[[[SolverSlot<u16>; WN + 1]; MN + 1]; 6]; 5]>>,
}

impl<const MN: usize, const WN: usize> ProgressSolver<MN, WN>
where
    [(); MN + 1]:,
    [(); WN + 1]:,
{
    const DEFAULT_SLOT: SolverSlot<u16> = SolverSlot {
        value: 0,
        step: 0,
        skill: None,
    };
    const DEFAULT_ARY: [[[[SolverSlot<u16>; WN + 1]; MN + 1]; 6]; 5] =
        [[[[Self::DEFAULT_SLOT; WN + 1]; MN + 1]; 6]; 5];
    pub fn new(s: &Status, allowed_list: Vec<Skills>) -> Self {
        let cp = s.attributes.craft_points as usize;
        let du = s.recipe.durability as usize;
        Self {
            init_status: s.clone(),
            allowed_list,
            results: vec![vec![Self::DEFAULT_ARY; cp + 1]; du + 1],
        }
    }
}

impl<const MN: usize, const WN: usize> ProgressSolver<MN, WN>
where
    [(); MN + 1]:,
    [(); WN + 1]:,
{
    unsafe fn get_unchecked(&self, s: &Status) -> &SolverSlot<u16> {
        self.results
            .get_unchecked(s.durability as usize)
            .get_unchecked(s.craft_points as usize)
            .get_unchecked(s.buffs.veneration as usize)
            .get_unchecked(s.buffs.muscle_memory as usize)
            .get_unchecked(s.buffs.manipulation as usize)
            .get_unchecked(s.buffs.wast_not as usize)
    }

    fn get(&self, s: &Status) -> &SolverSlot<u16> {
        &self.results[s.durability as usize][s.craft_points as usize][s.buffs.veneration as usize]
            [s.buffs.muscle_memory as usize][s.buffs.manipulation as usize]
            [s.buffs.wast_not as usize]
    }
}

impl<const MN: usize, const WN: usize> Solver for ProgressSolver<MN, WN>
where
    [(); MN + 1]:,
    [(); WN + 1]:,
{
    fn init(&mut self) {
        let mut s = self.init_status.clone();
        for cp in 0..=self.init_status.attributes.craft_points {
            s.craft_points = cp;
            for du in 1..=self.init_status.recipe.durability {
                s.durability = du;
                for ve in 0..=MAX_VENERATION {
                    s.buffs.veneration = ve;
                    for mm in 0..=MAX_MUSCLE_MEMORY {
                        s.buffs.muscle_memory = mm;
                        for mn in 0..=MN {
                            s.buffs.manipulation = mn as u8;
                            for wn in 0..=WN {
                                s.buffs.wast_not = wn as u8;
                                for sk in &self.allowed_list {
                                    if s.is_action_allowed(*sk).is_ok() {
                                        let mut new_s = s.clone();
                                        new_s.cast_action(*sk);
                                        let mut progress = new_s.progress;
                                        let mut step = 1;
                                        if new_s.durability > 0 {
                                            let next = &self.results[new_s.durability as usize]
                                                [new_s.craft_points as usize]
                                                [new_s.buffs.veneration as usize]
                                                [new_s.buffs.muscle_memory as usize]
                                                [new_s.buffs.manipulation as usize]
                                                [new_s.buffs.wast_not as usize];
                                            progress += next.value;
                                            progress = progress.min(s.recipe.difficulty);
                                            step += next.step;
                                        }
                                        let slot = &mut self.results[du as usize][cp as usize]
                                            [ve as usize][mm as usize]
                                            [mn as usize][wn as usize];
                                        if progress > slot.value
                                            || (progress == slot.value && step < slot.step)
                                        {
                                            *slot = SolverSlot {
                                                value: progress,
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
    fn read(&self, s: &Status) -> Option<Skills> {
        self.results[s.durability as usize][s.craft_points as usize][s.buffs.veneration as usize]
            [s.buffs.muscle_memory as usize][s.buffs.manipulation as usize]
            [s.buffs.wast_not as usize]
            .skill
    }
    fn read_all(&self, s: &Status) -> Vec<Skills> {
        let difficulty = s.recipe.difficulty;
        let mut new_s = s.clone();
        let mut list = Vec::new();
        loop {
            let max_addon = difficulty - new_s.progress;
            let mut new_s2 = new_s.clone();
            let mut best = {
                let &SolverSlot {
                    value: progress,
                    step,
                    skill,
                } = self.get(&new_s);
                let progress = progress.min(max_addon);
                (
                    (progress, step),
                    (new_s.craft_points, new_s.durability),
                    skill,
                )
            };
            for cp in 0..=new_s.craft_points {
                new_s2.craft_points = cp;
                for du in 1..=new_s.durability {
                    new_s2.durability = du;
                    let &SolverSlot {
                        value: progress,
                        step,
                        skill,
                    } = self.get(&new_s2);
                    let progress = progress.min(max_addon);
                    if progress >= best.0 .0 && step < best.0 .1 {
                        best = ((progress, step), (cp, du), skill);
                    }
                }
            }
            match best.2 {
                Some(sk) => {
                    new_s.cast_action(sk);
                    list.push(sk);
                    if new_s.is_finished() {
                        break list;
                    }
                }
                None => break list,
            }
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct SolverHash {
    pub attributes: Attributes,
    pub recipe: Recipe,
}
