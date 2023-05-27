use ffxiv_crafting::{Actions, Status};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::dynamic_programing_solver::QualitySolver;

pub struct PreprogressSolver {
    progress_index: Vec<usize>,
    quality_solvers: Vec<QualitySolver>,
}

impl PreprogressSolver {
    pub fn new(init_status: Status, progress_list: Vec<u16>, mn: usize, wn: usize) -> Self {
        let progress_index = progress_list
            .iter()
            .scan(0, |prev, &x| {
                let v = x - *prev;
                *prev = x;
                Some(v)
            })
            .enumerate()
            .map(|(i, v)| std::iter::repeat(i).take(v as usize))
            .flatten()
            .chain(std::iter::once(progress_list.len()))
            .collect::<Vec<usize>>();
        assert_eq!(
            progress_index.len(),
            *progress_list.last().unwrap() as usize + 1
        );
        let quality_solvers = progress_list
            .iter()
            .map(|v| {
                let mut s = init_status.clone();
                s.progress = s.recipe.difficulty - *v;
                QualitySolver::new(s, mn, wn)
            })
            .collect();
        Self {
            progress_index,
            quality_solvers,
        }
    }
}

impl crate::solver::Solver for PreprogressSolver {
    fn init(&mut self) {
        self.quality_solvers.par_iter_mut().for_each(|qs| qs.init());
    }

    fn read(&self, s: &Status) -> Option<Actions> {
        let left_progress = s.recipe.difficulty - s.progress;
        let i = *self.progress_index.get(left_progress as usize)?;
        self.quality_solvers.get(i)?.read(s)
    }

    fn read_all(&self, s: &Status) -> Vec<Actions> {
        let left_progress = s.recipe.difficulty - s.progress;
        self.progress_index
            .get(left_progress as usize)
            .and_then(|&i| {
                let qs = self.quality_solvers.get(i);
                Some(qs?.read_all(s))
            })
            .unwrap_or(vec![])
    }
}

pub fn preprogress_list(status: &Status) -> Vec<u16> {
    macro_rules! level_based {
        ($level:literal, $e1:literal, $e2:literal) => {
            if status.attributes.level < $level {
                $e1
            } else {
                $e2
            }
        };
    }
    vec![
        status.calc_synthesis(level_based!(31, 1.0, 1.2)), // basic synth
        status.calc_synthesis(level_based!(82, 1.5, 1.8)), // careful synth
    ]
}

#[cfg(test)]
mod test {
    use ffxiv_crafting::{Attributes, Recipe, Status};

    use crate::preprogress_solver::{PreprogressSolver, preprogress_list};
    use crate::solver::Solver;

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
        use ffxiv_crafting::Actions;
        let mut init_status = init();
        init_status.cast_action(Actions::MuscleMemory);
        init_status.cast_action(Actions::Manipulation);
        init_status.cast_action(Actions::Veneration);
        init_status.cast_action(Actions::WasteNotII);
        init_status.cast_action(Actions::Groundwork);
        init_status.cast_action(Actions::Groundwork);
        init_status.cast_action(Actions::FinalAppraisal);
        init_status.cast_action(Actions::Groundwork);
        let solver = PreprogressSolver::new(init_status.clone(), preprogress_list(&init_status), 8, 8);
        let actions = solver.read_all(&init_status);
        println!("{actions:?}");
    }
}
