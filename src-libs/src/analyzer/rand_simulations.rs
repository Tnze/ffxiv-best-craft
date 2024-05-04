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

use ffxiv_crafting::{Actions, CastActionError, Status};
use rand::{seq::SliceRandom, thread_rng, Rng};

fn stat(status: Status, actions: &[Actions]) {
    let results = run(status, actions, 10000);
    let mut errors = 0;
    let mut unfinished = 0;
    let mut fails = 0;
    let mut normal = 0;
    let mut high = 0;
    for result in &results {
        match result {
            Err(cast_err) => {
                errors += 1;
            }
            Ok(status) if !status.is_finished() => {
                unfinished += 1;
            }
            Ok(status) if status.progress < status.recipe.difficulty => {
                fails += 1;
            }
            Ok(status) if !matches!(status.high_quality_probability(), Some(100)) => {
                normal += 1;
            }
            Ok(_) => {
                high += 1;
            }
        }
    }
}

fn run(status: Status, actions: &[Actions], n: usize) -> Vec<Result<Status, CastActionError>> {
    std::iter::from_fn(|| {
        let mut s = status.clone();
        Some(simulation(&mut s, actions).map(|()| s))
    })
    .take(n)
    .collect()
}

fn simulation(s: &mut Status, actions: &[Actions]) -> Result<(), CastActionError> {
    let mut rng = thread_rng();
    for action in actions {
        let _succ = simulate_one_step(s, action, &mut rng);
        if s.is_finished() {
            break;
        }
    }
    Ok(())
}

/// 只模拟一步制作，计算技能概率和制作状态，更新制作状态
pub fn simulate_one_step(
    status: &mut Status,
    action: Actions,
    rng: &mut impl Rng,
) -> Result<bool, CastActionError> {
    status.is_action_allowed(action)?;
    let is_success = status.success_rate(action) as f32 / 100.0 > random();
    if is_success {
        status.cast_action(action);
    } else {
        status.cast_action(match action {
            Actions::RapidSynthesis => Actions::RapidSynthesisFail,
            Actions::HastyTouch => Actions::HastyTouchFail,
            Actions::FocusedSynthesis => Actions::FocusedSynthesisFail,
            Actions::FocusedTouch => Actions::FocusedTouchFail,
            _ => unreachable!(),
        });
    }
    if !matches!(action, Actions::FinalAppraisal | Actions::HeartAndSoul) {
        status.condition = match status.condition {
            Condition::Good => Condition::Normal,
            Condition::Poor => Condition::Excellent,
            Condition::GoodOmen => Condition::Good,
            _ => {
                ConditionIterator::new(
                    status.recipe.conditions_flag as i32,
                    status.attributes.level as i32,
                )
                .collect::<Vec<_>>()
                .choose_weighted(&mut rng, |c| c.1)
                .unwrap()
                .0
            }
        };
    }
    Ok(is_success)
}
