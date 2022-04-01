use ffxiv_crafting::{Attributes, Recipe, Skills, Status};

const MAX_GREAT_STRIDES: u8 = 3;
const MAX_VENERATION: u8 = 4;
const MAX_INNOVATION: u8 = 4;
const MAX_MUSCLE_MEMORY: u8 = 5;
const MAX_INNER_QUIET: u8 = 10;
const MAX_MANIPULATION: u8 = 0;
const MAX_WAST_NOT: u8 = 8;

#[derive(Copy, Clone)]
struct SolverSlot {
    quality: u32,
    step: u8,
    skill: Option<Skills>,
}

pub struct Solver {
    pub driver: Driver,
    // results [d][cp][iq][iv][gs][mn][wn]
    results: Vec<Vec<[[[[[SolverSlot; 9]; 1]; 4]; 5]; 11]>>,
}

impl Solver {
    pub fn new(driver: Driver, allowd_list: &[Skills]) -> Self {
        let cp = driver.init_status.attributes.craft_points as usize;
        let du = driver.init_status.recipe.durability as usize;
        const DEFAULT_SLOT: SolverSlot = SolverSlot {
            quality: 0,
            step: 0,
            skill: None,
        };
        const DEFAULT_ARY: [[[[[SolverSlot; 9]; 1]; 4]; 5]; 11] =
            [[[[[DEFAULT_SLOT; 9]; 1]; 4]; 5]; 11];
        Self {
            driver,
            results: vec![vec![DEFAULT_ARY; cp + 1]; du + 1],
        }
    }

    pub fn init(&mut self, allowd_list: &[Skills]) {
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
                            for mn in 0..=MAX_MANIPULATION {
                                s.buffs.manipulation = mn;
                                for wn in 0..=MAX_WAST_NOT {
                                    s.buffs.wast_not = wn;
                                    for sk in allowd_list {
                                        if s.is_action_allowed(*sk).is_ok() {
                                            let mut new_s = s.clone();
                                            new_s.cast_action(*sk);
                                            let (progress, _step, _sk) = self.driver.read(&new_s);
                                            if progress == difficulty {
                                                let mut quality = new_s.quality;
                                                let mut step = 1;
                                                {
                                                    let next = &self.results
                                                        [new_s.durability as usize]
                                                        [new_s.craft_points as usize]
                                                        [new_s.buffs.inner_quiet as usize]
                                                        [new_s.buffs.innovation as usize]
                                                        [new_s.buffs.great_strides as usize]
                                                        [new_s.buffs.manipulation as usize]
                                                        [new_s.buffs.wast_not as usize];
                                                    quality += next.quality;
                                                    step += next.step;
                                                }
                                                let slot = &mut self.results[du as usize]
                                                    [cp as usize]
                                                    [iq as usize]
                                                    [iv as usize]
                                                    [gs as usize]
                                                    [mn as usize]
                                                    [wn as usize];
                                                if quality > slot.quality
                                                    || (quality == slot.quality && step < slot.step)
                                                {
                                                    *slot = SolverSlot {
                                                        quality,
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

    pub fn read(&self, s: &Status) -> (u32, u8, Option<Skills>) {
        let SolverSlot {
            quality,
            step,
            skill,
        } = self.results[s.durability as usize][s.craft_points as usize]
            [s.buffs.inner_quiet as usize][s.buffs.innovation as usize]
            [s.buffs.great_strides as usize][s.buffs.manipulation as usize]
            [s.buffs.wast_not as usize];
        (quality, step, skill)
    }

    pub fn read_all(&self, s: &Status) -> (u32, Vec<Skills>) {
        let max_quality = s.recipe.quality;
        let mut new_s = s.clone();
        let mut list = Vec::new();
        loop {
            let max_addon = max_quality - new_s.quality;
            let mut new_s2 = new_s.clone();
            let mut best = {
                let (quality, step, skill) = self.read(&new_s);
                let quality = quality.min(max_addon);
                (
                    (quality, step),
                    (new_s.craft_points, new_s.durability),
                    skill,
                )
            };
            for cp in 0..=new_s.craft_points {
                new_s2.craft_points = cp;
                for du in 1..=new_s.durability {
                    new_s2.durability = du;
                    let (quality, step, skill) = self.read(&new_s2);
                    let quality = quality.min(max_addon);
                    if quality >= best.0 .0 && step < best.0 .1 {
                        best = ((quality, step), (cp, du), skill);
                    }
                }
            }
            match best.2 {
                Some(sk) => {
                    new_s.cast_action(sk);
                    list.push(sk);
                }
                None => {
                    list.append(&mut self.driver.read_all(&new_s).1);
                    break (new_s.quality, list);
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
struct DriverSlot {
    progress: u16,
    step: u8,
    skill: Option<Skills>,
}

pub struct Driver {
    init_status: Status,
    // results[d][cp][ve][mm][mn][wn]
    results: Vec<Vec<[[[[DriverSlot; 9]; 1]; 6]; 5]>>,
}

impl Driver {
    pub fn new(s: &Status) -> Self {
        let cp = s.attributes.craft_points as usize;
        let du = s.recipe.durability as usize;
        const DEFAULT_SLOT: DriverSlot = DriverSlot {
            progress: 0,
            step: 0,
            skill: None,
        };
        const DEFAULT_ARY: [[[[DriverSlot; 9]; 1]; 6]; 5] = [[[[DEFAULT_SLOT; 9]; 1]; 6]; 5];
        Self {
            init_status: s.clone(),
            results: vec![vec![DEFAULT_ARY; cp + 1]; du + 1],
        }
    }

    pub fn init(&mut self, allowd_list: &[Skills]) {
        let mut s = self.init_status.clone();
        for cp in 0..=self.init_status.attributes.craft_points {
            s.craft_points = cp;
            for du in 1..=self.init_status.recipe.durability {
                s.durability = du;
                for ve in 0..=MAX_VENERATION {
                    s.buffs.veneration = ve;
                    for mm in 0..=MAX_MUSCLE_MEMORY {
                        s.buffs.muscle_memory = mm;
                        for mn in 0..=MAX_MANIPULATION {
                            s.buffs.manipulation = mn;
                            for wn in 0..=MAX_WAST_NOT {
                                s.buffs.wast_not = wn;
                                for sk in allowd_list {
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
                                            progress += next.progress;
                                            progress = progress.min(s.recipe.difficulty);
                                            step += next.step;
                                        }
                                        let slot = &mut self.results[du as usize][cp as usize]
                                            [ve as usize][mm as usize]
                                            [mn as usize][wn as usize];
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
        } = self.results[s.durability as usize][s.craft_points as usize]
            [s.buffs.veneration as usize][s.buffs.muscle_memory as usize]
            [s.buffs.manipulation as usize][s.buffs.wast_not as usize];
        return (progress, step, skill);
    }

    pub fn read_all(&self, s: &Status) -> (u16, Vec<Skills>) {
        let difficulty = s.recipe.difficulty;
        let mut new_s = s.clone();
        let mut list = Vec::new();
        loop {
            let max_addon = difficulty - new_s.progress;
            let mut new_s2 = new_s.clone();
            let mut best = {
                let (progress, step, skill) = self.read(&new_s);
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
                    let (progress, step, skill) = self.read(&new_s2);
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
                        break (new_s.progress, list);
                    }
                }
                None => break (new_s.progress, list),
            }
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct SolverHash {
    pub attributes: Attributes,
    pub recipe: Recipe,
}
