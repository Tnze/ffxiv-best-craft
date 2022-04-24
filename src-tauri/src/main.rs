#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(generic_const_exprs)]

use ffxiv_crafting::{Attributes, CastActionError, Recipe, Skills, Status};
use serde::Serialize;

fn new_recipe(
    rlv: i32,
    difficulty_factor: u16,
    quality_factor: u16,
    durability_factor: u16,
) -> Recipe {
    Recipe::new(rlv, difficulty_factor, quality_factor, durability_factor)
}

fn new_status(attrs: Attributes, recipe: Recipe, init_quality: u32) -> Result<Status, String> {
    if recipe.job_level > attrs.level + 5 {
        Err("Player level lower than recipe's require".to_string())
    } else {
        let mut s = Status::new(attrs, recipe);
        s.quality = init_quality;
        Ok(s)
    }
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

fn allowed_list(status: Status, skills: Vec<Skills>) -> Vec<String> {
    skills
        .iter()
        .map(|&sk| match status.is_action_allowed(sk) {
            Ok(_) => "ok".to_string(),
            Err(err) => err.to_string(),
        })
        .collect()
}

fn craftpoints_list(status: Status, skills: Vec<Skills>) -> Vec<i32> {
    skills.iter().map(|&sk| status.craft_point(sk)).collect()
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


fn main() {}
