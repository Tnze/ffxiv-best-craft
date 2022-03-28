#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ffxiv_crafting::{Attributes, CastActionError, Recipe, Skills, Status};
use serde::Serialize;

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
    let mut s = Status::new(attrs, recipe);
    s.quality = init_quality;
    s
}

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

#[tauri::command]
fn simulate(status: Status, skills: Vec<Skills>) -> SimulateResult {
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
    result
}

#[derive(Serialize)]
struct RecipeRow {
    id: usize,
    rlv: i32,
    name: String,
    job: String,

    difficulty_factor: u16,
    quality_factor: u16,
    durability_factor: u16,
}

#[tauri::command(async)]
fn recipe_table() -> Vec<RecipeRow> {
    csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .from_reader(include_bytes!("../assets/Recipe.csv").as_slice())
        .records()
        .map(|row| {
            let row = row?;
            Ok(RecipeRow {
                id: row.get(1).unwrap().parse().unwrap(),
                rlv: row
                    .get(3)
                    .unwrap()
                    .strip_prefix("RecipeLevelTable#")
                    .unwrap()
                    .parse()
                    .unwrap(),
                name: row.get(4).unwrap().into(),
                job: row.get(2).unwrap().into(),
                difficulty_factor: row.get(29).unwrap().parse().unwrap(),
                quality_factor: row.get(30).unwrap().parse().unwrap(),
                durability_factor: row.get(31).unwrap().parse().unwrap(),
            })
        })
        .collect::<Result<Vec<_>, csv::Error>>()
        .unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            new_recipe,
            new_status,
            simulate,
            recipe_table
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
