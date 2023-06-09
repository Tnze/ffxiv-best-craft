use ffxiv_crafting::{Actions, Status};

use crate::{memoization_solver::Solver, solver::Score};

pub struct PreprogressSolver {
    quality_solver: Solver,
}

impl PreprogressSolver {
    pub fn new(init_status: Status, mn: bool, wn: usize, obz: bool) -> Self {
        Self {
            quality_solver: Solver::new(init_status, mn, wn, obz),
        }
    }
}

impl crate::solver::Solver for PreprogressSolver {
    fn init(&mut self) {}

    fn read(&self, s: &Status) -> Option<Actions> {
        let prog_120 = s.calc_synthesis(1.2);
        let prog_180 = s.calc_synthesis(1.8);
        let prog_200 = s.calc_synthesis(2.0);

        let (final_actions, final_cp, final_du) = match s.recipe.difficulty - s.progress {
            x if x <= prog_120 => (Actions::BasicSynthesis, 0, 1),
            x if x <= prog_180 => (Actions::CarefulSynthesis, 7, 1),
            x if x <= prog_200 => (
                match s.buffs.observed {
                    0 => Actions::Observe,
                    _ => Actions::FocusedSynthesis,
                },
                12,
                1,
            ),
            _ => return None,
        };
        let craft_points = s.craft_points - final_cp;
        let durability = s.durability.saturating_sub(final_du);
        self.quality_solver
            .next_touch(craft_points, durability, s.buffs)
            .action
            .or(Some(final_actions))
    }

    fn read_all(&self, s: &Status) -> Vec<Actions> {
        let mut best_score = Score::from(s);
        let mut best_actions = Vec::new();

        let mut actions = Vec::new();
        'rs: for cp in (0..=s.craft_points).rev() {
            for du in (1..=s.durability).filter(|x| x % 5 == 0).rev() {
                let mut s = s.clone();
                s.craft_points = cp;
                s.durability = du;
                actions.clear();
                while let Some(next_action) = self.read(&s) {
                    if s.is_action_allowed(next_action).is_err() {
                        break;
                    }
                    s.cast_action(next_action);
                    actions.push(next_action);
                }

                let score = Score::from(&s);
                if score > best_score {
                    best_score = score;
                    best_actions = actions.clone();
                }

                if s.quality < s.recipe.quality {
                    break 'rs;
                }
            }
        }
        best_actions
    }
}
