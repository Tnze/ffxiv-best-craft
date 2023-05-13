use std::cell::Cell;

use ffxiv_crafting::{Actions, Buffs, Status};
use micro_ndarray::Array;

pub struct Solver {
    init_status: Status,
    // touch_caches: Vec<
    //     [[[[[[Option<Actions>; Self::MAX_INNER_QUIET + 1]; Self::MAX_INNOVATION + 1];
    //         Self::MAX_MANIPULATION + 1]; Self::MAX_GREAT_STRIDES + 1];
    //         Self::MAX_TOUCH_COMBO + 1]; Self::MAX_OBSERVE + 1],
    // >,
    touch_caches: Array<Cell<Option<Actions>>, 8>,
    // synth_caches: Vec<
    //     [[[Option<Actions>; Self::MAX_VENERATION + 1]; Self::MAX_MANIPULATION + 1];
    //         Self::MAX_OBSERVE + 1],
    // >,
    synth_caches: Array<Cell<Option<Actions>>, 5>,
}

impl Solver {
    const MAX_INNER_QUIET: usize = 10;
    const MAX_INNOVATION: usize = 4;
    const MAX_MANIPULATION: usize = 8;
    const MAX_GREAT_STRIDES: usize = 3;
    const MAX_TOUCH_COMBO: usize = 2;
    const MAX_VENERATION: usize = 4;
    const MAX_OBSERVE: usize = 1;
    const TOUCH_SKILLS: [(Actions, u16); 13] = [
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
    ];
    const SYNTH_SKILLS: [(Actions, u16); 13] = [
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
    ];

    pub fn new(init_status: Status) -> Self {
        Self {
            touch_caches: Array::new([
                init_status.attributes.craft_points as usize + 1,
                init_status.recipe.durability as usize + 1,
                Self::MAX_INNER_QUIET + 1,
                Self::MAX_INNOVATION + 1,
                Self::MAX_MANIPULATION + 1,
                Self::MAX_GREAT_STRIDES + 1,
                Self::MAX_TOUCH_COMBO + 1,
                Self::MAX_OBSERVE + 1,
            ]),
            synth_caches: Array::new([
                init_status.attributes.craft_points as usize + 1,
                init_status.recipe.durability as usize + 1,
                Self::MAX_VENERATION + 1,
                Self::MAX_MANIPULATION + 1,
                Self::MAX_OBSERVE + 1,
            ]),
            init_status,
        }
    }

    fn touch_index(
        &self,
        craft_points: i32,
        durability: u16,
        buffs: Buffs,
    ) -> &Cell<Option<Actions>> {
        &self.touch_caches[[
            craft_points as usize,
            durability as usize,
            buffs.inner_quiet as usize,
            buffs.innovation as usize,
            buffs.manipulation as usize,
            buffs.great_strides as usize,
            buffs.touch_combo_stage as usize,
            buffs.observed as usize,
        ]]
    }

    fn synth_index(
        &self,
        craft_points: i32,
        durability: u16,
        buffs: Buffs,
    ) -> &Cell<Option<Actions>> {
        &self.synth_caches[[
            craft_points as usize,
            durability as usize,
            buffs.veneration as usize,
            buffs.manipulation as usize,
            buffs.observed as usize,
        ]]
    }

    pub fn next_touch(&self, craft_points: i32, durability: u16, buffs: Buffs) -> Option<Actions> {
        if craft_points == 0 || durability == 0 {
            return None;
        }
        let this_cell = self.touch_index(craft_points, durability, buffs);
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

    fn next_synth(&self, craft_points: i32, durability: u16, buffs: Buffs) -> Option<Actions> {
        if craft_points == 0 || durability == 0 {
            return None;
        }
        let this_cell = self.synth_index(craft_points, durability, buffs);
        if let Some(action) = this_cell.get() {
            return Some(action);
        }
        let mut init_status = self.init_status.clone();
        init_status.craft_points = craft_points;
        init_status.durability = durability;
        init_status.buffs = buffs;
        let mut best_action = None;
        let mut best_quality = None;
        for (action, consumed_du) in Self::SYNTH_SKILLS {
            if init_status.is_action_allowed(action).is_err()
                || durability < init_status.calc_durability(consumed_du)
                || init_status.success_rate(action) < 100
            {
                continue;
            }
            let mut s = init_status.clone();
            s.cast_action(action);
            while let Some(action) = self.next_synth(s.craft_points, s.durability, s.buffs) {
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
}

impl crate::solver::Solver for Solver {
    fn init(&mut self) {}

    fn read(&self, s: &Status) -> Option<Actions> {
        self.next_touch(s.craft_points, s.durability, s.buffs)
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
        let mut solver = super::Solver::new(status.clone());
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
