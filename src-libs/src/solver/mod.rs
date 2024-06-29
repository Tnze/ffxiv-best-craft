// This file is part of BestCraft.
// Copyright (C) 2023 Tnze
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

pub mod depth_first_search_solver;
pub mod normal_progress_solver;
pub mod reflect_solver;
pub mod rika_solver;

use std::cmp::Ordering;

use ffxiv_crafting::{Actions, Attributes, Recipe, Status};

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct SolverHash {
    pub attributes: Attributes,
    pub recipe: Recipe,
}

pub trait Solver {
    fn init(&mut self);
    fn read(&self, s: &Status) -> Option<Actions>;
    fn read_all(&self, s: &Status) -> Vec<Actions> {
        let mut result = Vec::new();
        let mut status = s.clone();
        while let Some(action) = self.read(&status) {
            if status.is_action_allowed(action).is_err() {
                break;
            }
            status.cast_action(action);
            result.push(action);
            if status.is_finished() {
                break;
            }
        }
        result
    }
}

#[derive(PartialEq, Eq)]
pub struct Score {
    pub quality: u32,
    pub prgress: u16,
    pub steps: u16,
}

impl From<&Status> for Score {
    fn from(s: &Status) -> Self {
        Self {
            quality: s.quality,
            prgress: s.progress,
            steps: s.step as u16,
        }
    }
}

impl From<(&Status, usize)> for Score {
    fn from((s, steps): (&Status, usize)) -> Self {
        Self {
            quality: s.quality,
            prgress: s.progress,
            steps: steps as u16,
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        self.prgress
            .cmp(&other.prgress)
            .then_with(|| self.quality.cmp(&other.quality))
            .then_with(|| self.steps.cmp(&other.steps).reverse())
    }
}
