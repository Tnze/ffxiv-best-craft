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

use crate::{simulate_one_step, SimulateOneStepResult};
use ffxiv_crafting::{Actions, CastActionError, Status};
use rand::{thread_rng, Rng};
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct Statistics {
    // 发生技能错误的模拟频数
    pub errors: i32,
    // 技能模拟完成后仍处于制作状态的模拟频数
    pub unfinished: i32,
    // 进展未推满的模拟频数
    pub fails: i32,
    // 进展推满，HQ率未达到100%的模拟频数
    pub normal: i32,
    // 进展推满，品质也推满的模拟频数
    pub highqual: i32,
}

pub fn stat(status: Status, actions: &[Actions], n: usize) -> Statistics {
    let mut rng = thread_rng();
    let mut rng2 = rng.clone();
    let results = std::iter::from_fn(|| {
        let mut s = status.clone();
        Some(simulation(&mut rng, &mut s, actions).map(|res| (res, s)))
    });
    let mut statistics = Statistics::default();
    for result in results.take(n) {
        match result {
            Err(_cast_err) => {
                statistics.errors += 1;
            }
            Ok((_history, status)) if !status.is_finished() => {
                statistics.unfinished += 1;
            }
            Ok((_history, status)) if status.progress < status.recipe.difficulty => {
                statistics.fails += 1;
            }
            Ok((_history, status)) => match status.high_quality_probability() {
                None => statistics.errors += 1,
                Some(p) if p > rng2.gen_range(0..100) => statistics.highqual += 1,
                _ => statistics.normal += 1,
            },
        }
    }
    statistics
}

fn simulation(
    rng: &mut impl Rng,
    s: &mut Status,
    actions: &[Actions],
) -> Result<Vec<SimulateOneStepResult>, CastActionError> {
    let mut history = Vec::new();
    for action in actions {
        let is_success = simulate_one_step(s, *action, false, rng)?;
        history.push(SimulateOneStepResult {
            status: s.clone(),
            is_success,
        });
        if s.is_finished() {
            break;
        }
    }
    Ok(history)
}
