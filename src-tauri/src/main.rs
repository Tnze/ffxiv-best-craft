#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ffxiv_crafting::{Attributes, CastActionError, Recipe, Skills, Status};

#[tauri::command]
fn new_recipe(
    rlv: i32,
    difficulty_factor: u16,
    quality_factor: u16,
    durability_factor: u16,
) -> Recipe {
    Recipe::new(rlv, difficulty_factor, quality_factor, durability_factor)
}

#[tauri::command]
fn new_status(attrs: Attributes, recipe: Recipe, init_quality: i32) -> Status {
    Status {
        quality: init_quality,
        ..(Status::new(attrs, recipe))
    }
}

#[tauri::command]
fn simulate(status: Status, skills: Vec<Skills>) -> Result<Status, CastActionError> {
    skills.iter().fold(Ok(status), |s, sk| s?.cast_action(*sk))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![new_recipe, new_status, simulate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
