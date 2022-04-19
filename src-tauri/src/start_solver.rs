use super::{OrdinarySolver, Solver};

pub struct StartSolver<const MN: usize, const WN: usize, const PG: usize>
where
    [[(); WN + 1]; MN + 1]:,
    [(); PG + 1]:,
{
    pub driver: OrdinarySolver<MN, WN, PG>,
}

impl<const MN: usize, const WN: usize, const PG: usize> Solver for StartSolver<MN, WN, PG>
where
    [[(); WN + 1]; MN + 1]:,
    [(); PG + 1]:,
{
    fn init(&mut self) {
        todo!()
    }

    fn read(&self, s: &ffxiv_crafting::Status) -> Option<ffxiv_crafting::Skills> {
        todo!()
    }

    fn read_all(&self, s: &ffxiv_crafting::Status) -> Vec<ffxiv_crafting::Skills> {
        todo!()
    }
}
