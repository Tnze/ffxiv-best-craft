use ffxiv_crafting::{Actions, Buffs, Status};

pub struct Solver {
    init_status: Status,
    results: Vec<
        [[[[[[Option<Actions>; Self::MAX_INNER_QUIET + 1]; Self::MAX_INNOVATION + 1];
            Self::MAX_MANIPULATION + 1]; Self::MAX_GREAT_STRIDES + 1];
            Self::MAX_TOUCH_COMBO + 1]; Self::MAX_OBSERVE + 1],
    >,
}

impl Solver {
    const MAX_INNER_QUIET: usize = 10;
    const MAX_INNOVATION: usize = 4 + 2;
    const MAX_MANIPULATION: usize = 8 + 2;
    const MAX_GREAT_STRIDES: usize = 3 + 2;
    const MAX_TOUCH_COMBO: usize = 2;
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

    pub fn new(init_status: Status) -> Self {
        let len = (init_status.attributes.craft_points as usize + 1)
            * (init_status.recipe.durability as usize + 1);
        Self {
            init_status,
            results: vec![Default::default(); len],
        }
    }

    #[rustfmt::skip]
    pub fn read(&mut self, craft_points: i32, durability: u16, buffs: Buffs) -> Option<Actions> {
        if craft_points == 0 || durability == 0 {
            return None;
        }
        if let Some(action) = self.results
            [craft_points as usize * (self.init_status.recipe.durability as usize + 1) + durability as usize]
            [buffs.observed as usize]
            [buffs.touch_combo_stage as usize]
            [buffs.great_strides as usize]
            [buffs.manipulation as usize]
            [buffs.innovation as usize]
            [buffs.inner_quiet as usize] {
            return Some(action);
        }
        let mut init_status = self.init_status.clone();
        init_status.craft_points = craft_points;
        init_status.durability = durability;
        init_status.buffs = buffs;
        let mut best_action = None;
        let mut best_quality = None;
        for (action, consumed_du) in Self::TOUCH_SKILLS {
            if init_status.is_action_allowed(action).is_err() ||
                durability < init_status.calc_durability(consumed_du) ||
                init_status.success_rate(action) < 100 {
                continue;
            }
            let mut s = init_status.clone();
            s.cast_action(action);
            while let Some(action) = self.read(s.craft_points, s.durability, s.buffs) {
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
        self.results
            [craft_points as usize * (self.init_status.recipe.durability as usize + 1) + durability as usize]
            [buffs.observed as usize]
            [buffs.touch_combo_stage as usize]
            [buffs.great_strides as usize]
            [buffs.manipulation as usize]
            [buffs.innovation as usize]
            [buffs.inner_quiet as usize] = best_action;
        best_action
    }
}

#[cfg(test)]
mod test {
    use ffxiv_crafting::{Attributes, ConditionIterator, Recipe, Status};
    #[test]
    fn test() {
        let r = Recipe {
            rlv: 611,
            job_level: 90,
            difficulty: 7480,
            quality: 13620,
            durability: 60,
            conditions_flag: 435,
        };
        let a = Attributes {
            level: 90,
            craftsmanship: 4214,
            control: 3528,
            craft_points: 691,
        };
        let mut s = Status::new(a, r);
        let mut solver = super::Solver::new(s.clone());

        println!("{:?}", solver.read(19, 10, s.buffs));

        while let Some(action) = solver.read(s.craft_points, s.durability, s.buffs) {
            println!("{:?}", action);
            if let Err(e) = s.is_action_allowed(action) {
                panic!("not allowed on {:?}: {:?}", s, e);
            }
            s.cast_action(action);
            if s.is_finished() {
                break;
            }
        }
    }
}
