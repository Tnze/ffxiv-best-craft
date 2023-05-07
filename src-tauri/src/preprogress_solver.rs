use std::sync::Arc;

use ffxiv_crafting::{Actions, Status};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use super::{ProgressSolver, QualitySolver, Solver};

pub struct PreprogressSolver<const MN: usize, const WN: usize>
where
    [[(); WN ]; MN ]:,
{
    progress_index: Vec<usize>,
    quality_solvers: Vec<QualitySolver<MN, WN>>,
}

impl<const MN: usize, const WN: usize> PreprogressSolver<MN, WN>
where
    [[(); WN ]; MN ]:,
{
    pub fn new(
        init_status: Status,
        progress_list: Vec<u16>,
        progress_solver: Arc<ProgressSolver<MN, WN>>,
    ) -> Self {
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
                QualitySolver::<MN, WN>::new(s, progress_solver.clone())
            })
            .collect();
        Self {
            progress_index,
            quality_solvers,
        }
    }
}

impl<const MN: usize, const WN: usize> Solver for PreprogressSolver<MN, WN>
where
    [[(); WN ]; MN ]:,
{
    fn init(&mut self) {
        self.quality_solvers.par_iter_mut().for_each(|qs| qs.init());
    }

    fn read(&self, s: &Status) -> Option<(Actions, u32)> {
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
