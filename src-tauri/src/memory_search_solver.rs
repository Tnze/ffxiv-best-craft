use ffxiv_crafting::{Actions, Buffs, Status};
use micro_ndarray::Array;
use std::cell::Cell;

pub struct Solver {
    init_status: Status,
    // start_caches: Array<Cell<Option<Actions>>, 7>,
    touch_caches: Array<Cell<Option<Actions>>, 9>,
    synth_caches: Array<Cell<Option<Actions>>, 6>,
}

impl Solver {
    // const MAX_MUSCLE_MEMORY: usize = 6;
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
            // start_caches: Array::new([
            //     init_status.attributes.craft_points as usize + 1,
            //     init_status.recipe.durability as usize + 1,
            //     Self::MAX_MUSCLE_MEMORY + 1,
            //     Self::MAX_VENERATION + 1,
            //     Self::MAX_MANIPULATION + 1,
            //     Self::MAX_WASTE_NOT + 1,
            //     Self::MAX_OBSERVE + 1,
            // ]),
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
            synth_caches: Array::new([
                init_status.attributes.craft_points as usize + 1,
                init_status.recipe.durability as usize + 1,
                Self::MAX_VENERATION + 1,
                Self::MAX_MANIPULATION + 1,
                Self::MAX_WASTE_NOT + 1,
                Self::MAX_OBSERVE + 1,
            ]),
            init_status,
        }
    }

    pub fn next_touch(&self, craft_points: i32, durability: u16, buffs: Buffs) -> Option<Actions> {
        if craft_points == 0 || durability == 0 {
            return None;
        }
        let this_cell = unsafe {
            self.touch_caches.get_unchecked([
                craft_points as usize,
                durability as usize,
                buffs.inner_quiet as usize,
                buffs.innovation as usize,
                buffs.manipulation as usize,
                buffs.wast_not as usize,
                buffs.great_strides as usize,
                buffs.touch_combo_stage as usize,
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
                #[cfg(debug_assertions)]
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

    fn next_synth(&self, craft_points: i32, durability: u16, buffs: Buffs) -> Option<Actions> {
        if craft_points == 0 || durability == 0 {
            return None;
        }
        let this_cell = unsafe {
            self.synth_caches.get_unchecked([
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
        let mut best_progress = 0;
        for (action, consumed_du) in Self::SYNTH_SKILLS {
            if init_status.is_action_allowed(action).is_err()
                || durability < init_status.calc_durability(consumed_du)
                || init_status.success_rate(action) < 100
            {
                continue;
            }
            let mut s = init_status.clone();
            s.cast_action(action);
            if !s.is_finished() {
                while let Some(action) = self.next_synth(s.craft_points, s.durability, s.buffs) {
                    #[cfg(debug_assertions)]
                    if let Err(e) = s.is_action_allowed(action) {
                        panic!("not allowed {:?} on {:?}: {}", action, s, e);
                    }
                    s.cast_action(action);
                    if s.is_finished() {
                        break;
                    }
                }
            }
            if s.progress > best_progress {
                this_cell.set(Some(action));
                best_progress = s.progress;
            }
        }
        this_cell.get()
    }

    fn solve(&self, tmp_s: &Status) -> Vec<Actions> {
        let mut actions = Vec::new();
        let mut best_actions = Vec::new();
        let mut best_score = 0;

        for (du, cp, mut synth) in SynthIterator::new(self, tmp_s)
            .filter(|&(du, cp, _)| du <= tmp_s.durability && cp <= tmp_s.craft_points)
        {
            let mut tmp_s = tmp_s.clone();
            let prev_history_len = actions.len();
            loop {
                let rem_cp = tmp_s.craft_points - cp;
                let rem_du = tmp_s.durability - du;
                let Some(action) = self.next_touch(rem_cp, rem_du, tmp_s.buffs) else {
                        break;
                    };
                #[cfg(debug_assertions)]
                if let Err(e) = tmp_s.is_action_allowed(action) {
                    panic!("not allowed {:?} on {:?}: {}", action, tmp_s, e);
                }
                tmp_s.cast_action(action);
                actions.push(action);
            }
            actions.append(&mut synth);
            println!("actions: {actions:?}");
            if tmp_s.quality > best_score {
                best_actions = actions.clone();
                best_score = tmp_s.quality;
            }
            actions.truncate(prev_history_len);
        }
        best_actions
    }
}

impl crate::solver::Solver for Solver {
    fn init(&mut self) {}

    fn read(&self, _s: &Status) -> Option<Actions> {
        None
    }

    fn read_all(&self, s: &Status) -> Vec<Actions> {
        if s.is_action_allowed(Actions::Reflect).is_ok() {
            let mut tmp_s = s.clone();
            tmp_s.cast_action(Actions::Reflect);

            let mut actions2 = self.solve(&tmp_s);
            let mut actions1 = vec![Actions::Reflect];
            actions1.append(&mut actions2);
            actions1
        } else {
            self.solve(s)
        }
    }
}

struct SynthIterator<'a> {
    solver: &'a Solver,
    status: &'a Status,

    prev: i32,
    du: u16,

    // inner
    left: i32,
    right: i32,
    size: i32,
}

impl<'a> SynthIterator<'a> {
    fn new(solver: &'a Solver, status: &'a Status) -> Self {
        Self {
            solver,
            status,

            prev: status.attributes.craft_points,
            du: 0,

            left: 0,
            right: 0,
            size: 0,
        }
    }

    fn inner_next(&mut self) -> Option<i32> {
        if !self.judge(self.du, self.right, None) {
            return None;
        }
        while self.left < self.right {
            let mid = self.left + self.size / 2;
            let ok = self.judge(self.du, mid, None);
            if ok {
                self.right = mid;
            } else {
                self.left = mid + 1;
            }
            self.size = self.right - self.left;
        }
        Some(self.right)
    }

    fn judge(&self, du: u16, cp: i32, mut solution: Option<&mut Vec<Actions>>) -> bool {
        let mut s = self.status.clone();
        s.durability = du;
        s.craft_points = cp;

        while let Some(action) = self
            .solver
            .next_synth(s.craft_points, s.durability, s.buffs)
        {
            if let Err(err) = s.is_action_allowed(action) {
                panic!("not allowed on {:?}: {:?}", s, err);
            }
            s.cast_action(action);
            if let Some(solution) = &mut solution {
                solution.push(action);
            }
            if s.is_finished() {
                break;
            }
        }
        s.progress >= s.recipe.difficulty
    }

    fn solution(&self, du: u16, cp: i32) -> Vec<Actions> {
        let mut solution = Vec::new();
        let mut s = self.status.clone();
        s.durability = du;
        s.craft_points = cp;

        while let Some(action) = self
            .solver
            .next_synth(s.craft_points, s.durability, s.buffs)
        {
            if let Err(err) = s.is_action_allowed(action) {
                panic!("not allowed on {:?}: {:?}", s, err);
            }
            s.cast_action(action);
            solution.push(action);
            if s.is_finished() {
                break;
            }
        }
        solution
    }
}

impl Iterator for SynthIterator<'_> {
    type Item = (u16, i32, Vec<Actions>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.du += 5;
            self.left = 0;
            self.right = self.prev;
            self.size = self.prev;
            if self.du > self.status.recipe.durability {
                return None;
            }
            if let Some(min_cp) = self.inner_next() {
                let solution = self.solution(self.du, min_cp);
                return Some((self.du, min_cp, solution));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use ffxiv_crafting::{Attributes, Recipe, Status};

    use crate::Solver;

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
        let status = init();
        let solver = super::Solver::new(status.clone());

        let actions = solver.read_all(&status);
        println!("{actions:?}");
        let num_of_table = solver
            .synth_caches
            .iter()
            .filter(|(_, v)| v.get().is_some())
            .count();
        let total_tables = solver.synth_caches.iter().count();
        println!(
            "number of tables: {num_of_table}/{total_tables}, {:.2}%",
            num_of_table as f32 / total_tables as f32 * 1e2
        );
    }
}
