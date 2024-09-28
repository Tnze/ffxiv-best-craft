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

use app_libs::{
    ffxiv_crafting::{Actions, Attributes, Recipe, RecipeLevel, Status},
    SimulateOneStepResult,
};

use rand::thread_rng;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

fn err_to_string<T: ToString>(v: T) -> String {
    v.to_string()
}

#[wasm_bindgen]
pub fn new_status(
    attrs: JsValue,
    recipe: JsValue,
    recipe_level: JsValue,
) -> Result<JsValue, JsValue> {
    let attrs: Attributes = from_value(attrs)?;
    let recipe: Recipe = from_value(recipe)?;
    let recipe_level: RecipeLevel = from_value(recipe_level)?;
    let result = app_libs::new_status(attrs, recipe, recipe_level).map_err(err_to_string)?;
    Ok(to_value(&result)?)
}

#[wasm_bindgen]
pub fn simulate(status: JsValue, actions: JsValue) -> Result<JsValue, JsValue> {
    let status: Status = from_value(status)?;
    let actions: Vec<Actions> = from_value(actions)?;
    Ok(to_value(&app_libs::simulate(status, actions))?)
}

#[wasm_bindgen]
pub fn simulate_one_step(
    status: JsValue,
    action: JsValue,
    force_success: JsValue,
) -> Result<JsValue, JsValue> {
    let mut status: Status = from_value(status)?;
    let action: Actions = from_value(action)?;
    let force_success: bool = from_value(force_success)?;
    let is_success =
        app_libs::simulate_one_step(&mut status, action, force_success, &mut thread_rng())
            .map_err(err_to_string)?;
    Ok(to_value(&SimulateOneStepResult { status, is_success })?)
}

#[wasm_bindgen]
pub fn allowed_list(status: JsValue, skills: JsValue) -> Result<JsValue, JsValue> {
    let status: Status = from_value(status)?;
    let skills: Vec<Actions> = from_value(skills)?;
    Ok(to_value(&app_libs::allowed_list(status, skills))?)
}

#[wasm_bindgen]
pub fn craftpoints_list(status: JsValue, skills: JsValue) -> Result<JsValue, JsValue> {
    let status: Status = from_value(status)?;
    let skills: Vec<Actions> = from_value(skills)?;
    Ok(to_value(&app_libs::craftpoints_list(status, skills))?)
}

#[wasm_bindgen]
pub fn high_quality_probability(status: JsValue) -> Result<JsValue, JsValue> {
    let status: Status = from_value(status)?;
    Ok(to_value(&app_libs::high_quality_probability(status))?)
}

#[wasm_bindgen]
pub fn rika_solve(status: JsValue) -> Result<JsValue, JsValue> {
    use app_libs::solver::rika_solver::solve;
    let status: Status = from_value(status)?;
    let result: Vec<Actions> = solve(status);
    Ok(to_value(&result)?)
}

#[wasm_bindgen]
pub fn dfs_solve(status: JsValue, depth: usize, specialist: bool) -> Result<JsValue, JsValue> {
    use app_libs::solver::depth_first_search_solver::solve;
    let status: Status = from_value(status)?;
    let result: Vec<Actions> = solve(status, depth, specialist);
    Ok(to_value(&result)?)
}

#[wasm_bindgen]
pub fn nq_solve(status: JsValue, depth: usize, specialist: bool) -> Result<JsValue, JsValue> {
    use app_libs::solver::normal_progress_solver::solve;
    let status: Status = from_value(status)?;
    let result: Vec<Actions> = solve(status, depth, specialist);
    Ok(to_value(&result)?)
}

#[wasm_bindgen]
pub fn reflect_solve(status: JsValue, use_observe: bool) -> Result<JsValue, JsValue> {
    use app_libs::solver::reflect_solver::solve;
    let status: Status = from_value(status)?;
    let result: Vec<Actions> = solve(status.clone(), false, 0, use_observe);
    Ok(to_value(&result)?)
}

#[wasm_bindgen]
pub fn raphael_solve(
    status: JsValue,
    use_manipultaion: bool,
    use_heart_and_soul: bool,
    use_quick_innovation: bool,
    backload_progress: bool,
    adversarial: bool,
) -> Result<JsValue, JsValue> {
    use app_libs::solver::raphael::solve;
    let status: Status = from_value(status)?;
    let result: Vec<Actions> = solve(
        status.clone(),
        use_manipultaion,
        use_heart_and_soul,
        use_quick_innovation,
        backload_progress,
        adversarial,
    );
    Ok(to_value(&result)?)
}

#[wasm_bindgen]
pub fn rand_simulation(
    status: JsValue,
    actions: JsValue,
    n: usize,
    ignore_errors: bool,
) -> Result<JsValue, JsValue> {
    let status: Status = from_value(status)?;
    let actions: Vec<Actions> = from_value(actions)?;
    let result = app_libs::analyzer::rand_simulations::stat(status, &actions, n, ignore_errors);
    Ok(to_value(&result)?)
}

#[wasm_bindgen]
pub fn calc_attributes_scope(status: JsValue, actions: JsValue) -> Result<JsValue, JsValue> {
    let status: Status = from_value(status)?;
    let actions: Vec<Actions> = from_value(actions)?;
    let result = app_libs::analyzer::scope_of_application::calc_scope(status, &actions);
    Ok(to_value(&result)?)
}
