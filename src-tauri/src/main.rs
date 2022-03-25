#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ffxiv_crafting::{CastActionError, Recipe, Skills, Status};

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
fn simulate(skills: Vec<Skills>) -> Skills {
  return Skills::AdvancedTouch
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![new_recipe, simulate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
