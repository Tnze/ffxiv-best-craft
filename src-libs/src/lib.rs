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

pub mod analyzer;
pub mod solver;

pub use ffxiv_crafting;
use ffxiv_crafting::{
    Actions, Attributes, CastActionError, Condition, ConditionIterator, Recipe, RecipeLevel, Status,
};
use rand::{seq::SliceRandom, Rng};
use serde::Serialize;

#[derive(Serialize)]
pub struct CastErrorPos {
    pub pos: usize,
    pub err: CastActionError,
}

#[derive(Serialize)]
pub struct SimulateResult {
    pub status: Status,
    pub errors: Vec<CastErrorPos>,
}

/// 初始化一个表示一次制作的初始状态的Status对象，包含玩家属性、配方信息和初期品质
pub fn new_status(
    attrs: Attributes,
    recipe: Recipe,
    recipe_level: RecipeLevel,
) -> Result<Status, String> {
    if recipe.job_level > attrs.level + 5 {
        Err("player-level-lower-than-recipe-requirement".to_string())
    } else {
        Ok(Status::new(attrs, recipe, recipe_level))
    }
}

/// 模拟以指定初始状态按顺序执行一个技能序列后的结果，
/// 返回值`SimulateResult`包含了最终状态以及模拟过程中每个技能失败的位置及原因
pub fn simulate(status: Status, actions: Vec<Actions>) -> SimulateResult {
    let mut result = SimulateResult {
        status,
        errors: Vec::new(),
    };
    for (pos, sk) in actions.iter().enumerate() {
        match result.status.is_action_allowed(*sk) {
            Ok(_) => result.status.cast_action(*sk),
            Err(err) => result.errors.push(CastErrorPos { pos, err }),
        }
    }
    result
}

#[derive(Serialize)]
pub struct SimulateOneStepResult {
    pub status: Status,
    pub is_success: bool,
}

/// 只模拟一步制作，计算技能概率和制作状态，更新制作状态
pub fn simulate_one_step(
    status: &mut Status,
    action: Actions,
    force_success: bool,
    rng: &mut impl Rng,
) -> Result<bool, CastActionError> {
    status.is_action_allowed(action)?;
    let is_success = force_success || status.success_rate(action) as f32 / 100.0 > rng.gen();
    if is_success {
        status.cast_action(action);
    } else {
        status.cast_action(match action {
            Actions::RapidSynthesis => Actions::RapidSynthesisFail,
            Actions::HastyTouch => Actions::HastyTouchFail,
            Actions::DaringTouch => Actions::DaringTouchFail,
            _ => unreachable!(),
        });
    }
    if !matches!(action, Actions::FinalAppraisal | Actions::HeartAndSoul) {
        status.condition = match status.condition {
            Condition::Good if !force_success => Condition::Normal,
            Condition::Excellent if !force_success => Condition::Poor,
            Condition::GoodOmen => Condition::Good,
            _ => {
                ConditionIterator::new(
                    status.recipe.conditions_flag as i32,
                    status.attributes.level as i32,
                )
                .collect::<Vec<_>>()
                .choose_weighted(rng, |c| c.1)
                .unwrap()
                .0
            }
        };
    }
    Ok(is_success)
}

/// 计算当前状态下可以释放技能的集合，用于模拟界面将不可释放技能置灰
pub fn allowed_list(status: Status, skills: Vec<Actions>) -> Vec<String> {
    skills
        .iter()
        .map(|&sk| match status.is_action_allowed(sk) {
            Ok(_) => "ok".to_string(),
            Err(err) => err.to_string(),
        })
        .collect()
}

/// 计算当前状态下，传入的技能列表中每个技能需要消耗的制作力，
/// 技能消耗的制作力会受到技能连击和当前制作状态（球色—）的影响
pub fn craftpoints_list(status: Status, skills: Vec<Actions>) -> Vec<i32> {
    skills.iter().map(|&sk| status.craft_point(sk)).collect()
}

pub fn high_quality_probability(status: Status) -> Option<i32> {
    status.high_quality_probability()
}
