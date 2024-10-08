// This file is part of BestCraft.
// Copyright (C) 2024 Tnze
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

use app_libs::{
    ffxiv_crafting::{Actions, Status},
    solver::{Score, Solver},
};

use crate::memoization_solver::Solver as MemorizationSolver;

pub struct PreprogressSolver {
    quality_solver: MemorizationSolver,
}

impl PreprogressSolver {
    pub fn new(init_status: Status, mn: bool, wn: usize, obz: bool) -> Self {
        Self {
            quality_solver: MemorizationSolver::new(init_status, mn, wn, obz),
        }
    }
}

impl Solver for PreprogressSolver {
    fn init(&mut self) {}

    fn read(&self, s: &Status) -> Option<Actions> {
        let prog_120 = s.calc_synthesis(1.2);
        let prog_180 = s.calc_synthesis(1.8);

        let (final_actions, final_cp) = match s.recipe.difficulty - s.progress {
            x if x <= prog_120 => (Actions::BasicSynthesis, 0),
            x if x <= prog_180 => (Actions::CarefulSynthesis, 7),

            _ => return None,
        };
        let final_du = 1;
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
