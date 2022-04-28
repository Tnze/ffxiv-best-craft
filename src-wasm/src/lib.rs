mod utils;

use ffxiv_crafting::{Attributes, CastActionError, Recipe, Skills, Status};
use serde::Serialize;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize)]
struct CastErrorPos {
    pos: usize,
    err: CastActionError,
}

#[derive(Serialize)]
struct SimulateResult {
    status: Status,
    errors: Vec<CastErrorPos>,
}

#[wasm_bindgen]
pub fn new_recipe(
    rlv: i32,
    difficulty_factor: u16,
    quality_factor: u16,
    durability_factor: u16,
) -> JsValue {
    let r = Recipe::new(rlv, difficulty_factor, quality_factor, durability_factor);
    JsValue::from_serde(&r).unwrap()
}

#[wasm_bindgen]
pub fn new_status(attrs: JsValue, recipe: JsValue, init_quality: u32) -> Result<JsValue, String> {
    let attrs: Attributes = attrs.into_serde().unwrap();
    let recipe: Recipe = recipe.into_serde().unwrap();
    if recipe.job_level > attrs.level + 5 {
        Err("Player level lower than recipe's require".to_string())
    } else {
        let mut s = Status::new(attrs, recipe);
        s.quality = init_quality;
        Ok(JsValue::from_serde(&s).unwrap())
    }
}

#[wasm_bindgen]
pub fn simulate(status: JsValue, skills: JsValue) -> JsValue {
    let status: Status = status.into_serde().unwrap();
    let skills: Vec<Skills> = skills.into_serde().unwrap();
    let mut result = SimulateResult {
        status,
        errors: Vec::new(),
    };
    for (pos, sk) in skills.iter().enumerate() {
        match result.status.is_action_allowed(*sk) {
            Ok(_) => result.status.cast_action(*sk),
            Err(err) => result.errors.push(CastErrorPos { pos, err }),
        }
    }
    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn allowed_list(status: JsValue, skills: JsValue) -> JsValue {
    let status: Status = status.into_serde().unwrap();
    let skills: Vec<Skills> = skills.into_serde().unwrap();
    let list = skills
        .iter()
        .map(|&sk| match status.is_action_allowed(sk) {
            Ok(_) => "ok".to_string(),
            Err(err) => err.to_string(),
        })
        .collect::<Vec<_>>();
    JsValue::from_serde(&list).unwrap()
}

#[wasm_bindgen]
pub fn craftpoints_list(status: JsValue, skills: JsValue) -> JsValue {
    let status: Status = status.into_serde().unwrap();
    let skills: Vec<Skills> = skills.into_serde().unwrap();
    let list = skills
        .iter()
        .map(|&sk| status.craft_point(sk))
        .collect::<Vec<_>>();
    JsValue::from_serde(&list).unwrap()
}
