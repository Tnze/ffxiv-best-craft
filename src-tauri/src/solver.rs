use ffxiv_crafting::{Skills, Status};

pub trait Solver {
    fn init(&mut self);
    fn read(&self, s: &Status) -> Option<Skills>;
    fn read_all(&self, s: &Status) -> Vec<Skills>;
}
