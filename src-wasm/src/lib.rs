use ffxiv_crafting::{Actions, Attributes, Recipe, RecipeLevel, Status};

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
    let status: Status = from_value(status)?;
    let action: Actions = from_value(action)?;
    let force_success: bool = from_value(force_success)?;
    let result =
        app_libs::simulate_one_step(status, action, force_success).map_err(err_to_string)?;
    Ok(to_value(&result)?)
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
