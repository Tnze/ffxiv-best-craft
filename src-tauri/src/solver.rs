use ffxiv_crafting::{Actions, Status};

pub(crate) const MAX_GREAT_STRIDES: u8 = 3;
pub(crate) const MAX_VENERATION: u8 = 4;
pub(crate) const MAX_INNOVATION: u8 = 4;
pub(crate) const MAX_MUSCLE_MEMORY: u8 = 5;
pub(crate) const MAX_INNER_QUIET: u8 = 10;

#[derive(Copy, Clone)]
pub(crate) struct SolverSlot<V> {
    pub(crate) value: V,
    pub(crate) step: u8,
    pub(crate) action: Option<Actions>,
}

pub trait Solver {
    fn init(&mut self);
    fn read(&self, s: &Status) -> Option<(Actions, u32)>;
    fn read_all(&self, s: &Status) -> Vec<Actions>;
}
