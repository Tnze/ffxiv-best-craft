use ffxiv_crafting::{Actions, Status};

pub trait Solver {
    fn run(s: &Status) -> (String, Option<Actions>);
}

pub struct RedstoneSuan;

impl Solver for RedstoneSuan {
    fn run(s: &Status) -> (String, Option<Actions>) {
        let mut sb = String::new();
        (sb, None)
    }
}

#[cfg(test)]
mod test {
    use super::{RedstoneSuan, Solver};
    use ffxiv_crafting::{Attributes, Recipe, Status};

    #[test]
    fn simulate() {
        let r = Recipe::new(611, 7480, 13620, 60);
        let a = Attributes {
            level: 90,
            craftsmanship: 4214,
            control: 3528,
            craft_points: 691,
        };
        for i in 0..3 {
            println!("running simulation {i}");
            let mut status = Status::new(a, r);
            RedstoneSuan::run(&status);
        }
    }
}
