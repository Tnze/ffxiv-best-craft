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
use serde::{Deserialize, Serialize};

fn simulation(
    rng: &mut impl Rng,
    s: &mut Status,
    actions: &[Actions],
    ignore_errors: bool,
) -> Result<Vec<SimulateOneStepResult>, CastActionError> {
    let mut history = Vec::new();
    for action in actions {
        let result = simulate_one_step(s, *action, false, rng);
        let is_success = if ignore_errors {
            result.unwrap_or(false)
        } else {
            result?
        };

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

pub fn stat(status: Status, actions: &[Actions], n: usize, ignore_errors: bool) -> Statistics {
    let mut rng = thread_rng();
    let results = std::iter::from_fn({
        let mut rng = rng.clone();
        move || {
            let mut s = status.clone();
            Some(simulation(&mut rng, &mut s, actions, ignore_errors).map(|res| (res, s)))
        }
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
                Some(p) if p > rng.gen_range(0..100) => statistics.highqual += 1,
                _ => statistics.normal += 1,
            },
        }
    }
    statistics
}

#[derive(Default, Serialize)]
pub struct CollectableStatistics {
    // 发生技能错误的模拟频数
    pub errors: i32,
    // 技能模拟完成后仍处于制作状态的模拟频数
    pub unfinished: i32,
    // 进展未推满的模拟频数
    pub fails: i32,
    // 无收藏价值
    pub no_collectability: i32,
    // 收藏价值第一档
    pub low_collectability: i32,
    // 收藏价值第二档
    pub middle_collectability: i32,
    // 收藏价值第三档
    pub high_collectability: i32,
}

#[derive(Default, Deserialize)]
pub struct CollectablesShopRefine {
    pub low_collectability: u32,
    pub mid_collectability: u32,
    pub high_collectability: u32,
}

pub fn stat_collectables(
    status: Status,
    actions: &[Actions],
    n: usize,
    ignore_errors: bool,
    collectables_shop_refine: CollectablesShopRefine,
) -> CollectableStatistics {
    let mut rng = thread_rng();
    let results = std::iter::from_fn(|| {
        let mut s = status.clone();
        Some(simulation(&mut rng, &mut s, actions, ignore_errors).map(|res| (res, s)))
    });
    let mut statistics = CollectableStatistics::default();
    for result in results.take(n) {
        match result {
            Err(_cast_err) => {
                statistics.errors += 1;
            }
            Ok((_history, status)) => {
                if !status.is_finished() {
                    statistics.unfinished += 1;
                } else if status.progress < status.recipe.difficulty {
                    statistics.fails += 1;
                } else {
                    let collectability = status.quality / 10;
                    if collectables_shop_refine.high_collectability > 0
                        && collectability >= collectables_shop_refine.high_collectability
                    {
                        statistics.high_collectability += 1;
                    } else if collectables_shop_refine.mid_collectability > 0
                        && collectability >= collectables_shop_refine.mid_collectability
                    {
                        statistics.middle_collectability += 1;
                    } else if collectables_shop_refine.low_collectability > 0
                        && collectability >= collectables_shop_refine.low_collectability
                    {
                        statistics.low_collectability += 1;
                    } else {
                        statistics.no_collectability += 1;
                    }
                }
            }
        }
    }
    statistics
}
