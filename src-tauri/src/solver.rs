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
pub(crate) struct Score {
    pub(crate) quality: u32,
    pub(crate) prgress: u16,
    pub(crate) steps: u16,
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
