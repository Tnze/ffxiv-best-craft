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

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::{hash_map::Entry, BTreeMap, HashMap};
use std::sync::Arc;

use app_libs::analyzer::scope_of_application::Scope;
use app_libs::{
    analyzer::rand_simulations::Statistics,
    ffxiv_crafting::{Actions, Attributes, Recipe, RecipeLevel, Status},
    solver::{
        depth_first_search_solver, normal_progress_solver, raphael, reflect_solver, rika_solver,
        Solver, SolverHash,
    },
    SimulateOneStepResult, SimulateResult,
};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use rand::thread_rng;
use sea_orm::{entity::*, query::*, Database, DatabaseConnection, FromQueryResult};
use serde::Serialize;
use tauri::Manager;
use tokio::sync::{Mutex, OnceCell};

mod db;
mod memoization_solver;
mod muscle_memory_solver;
mod rika_tnze_solver;

use db::{craft_types, item_with_amount, items, prelude::*, recipes};

/// 创建新的Recipe对象，蕴含了模拟一次制作过程所必要的全部配方信息
#[tauri::command(async)]
async fn recipe_level_table(
    rlv: i32,
    app_state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<db::recipe_level_tables::Model, String> {
    let db = app_state.get_db(app_handle).await.map_err(err_to_string)?;
    let Some(rt) = RecipeLevelTables::find_by_id(rlv)
        .one(db)
        .await
        .map_err(err_to_string)?
    else {
        return Err(String::from("unknown-recipe-level"));
    };
    Ok(rt)
}

#[tauri::command(async)]
fn new_status(
    attrs: Attributes,
    recipe: Recipe,
    recipe_level: RecipeLevel,
) -> Result<Status, String> {
    app_libs::new_status(attrs, recipe, recipe_level)
}

#[tauri::command(async)]
fn simulate(status: Status, actions: Vec<Actions>) -> SimulateResult {
    app_libs::simulate(status, actions)
}

#[tauri::command(async)]
fn simulate_one_step(
    mut status: Status,
    action: Actions,
    force_success: bool,
) -> Result<SimulateOneStepResult, String> {
    let mut rng = thread_rng();
    app_libs::simulate_one_step(&mut status, action, force_success, &mut rng)
        .map(|is_success| SimulateOneStepResult { status, is_success })
        .map_err(err_to_string)
}

#[tauri::command(async)]
fn allowed_list(status: Status, skills: Vec<Actions>) -> Vec<String> {
    app_libs::allowed_list(status, skills)
}

#[tauri::command(async)]
fn craftpoints_list(status: Status, skills: Vec<Actions>) -> Vec<i32> {
    app_libs::craftpoints_list(status, skills)
}

#[tauri::command(async)]
fn high_quality_probability(status: Status) -> Option<i32> {
    app_libs::high_quality_probability(status)
}

fn err_to_string<T: ToString>(v: T) -> String {
    v.to_string()
}

#[derive(FromQueryResult, Serialize)]
struct RecipeInfo {
    id: i32,
    rlv: i32,
    item_id: i32,
    item_name: String,
    job: String,

    difficulty_factor: u16,
    quality_factor: u16,
    durability_factor: u16,
    material_quality_factor: u8,

    required_craftsmanship: u16,
    required_control: u16,

    can_hq: bool,
}

#[tauri::command(async)]
async fn recipe_table(
    page_id: u64,
    search_name: String,
    app_state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(Vec<RecipeInfo>, u64), String> {
    let db = app_state.get_db(app_handle).await.map_err(err_to_string)?;
    let paginate = Recipes::find()
        .join(JoinType::InnerJoin, recipes::Relation::CraftTypes.def())
        .join(JoinType::InnerJoin, recipes::Relation::ItemWithAmount.def())
        .join(
            JoinType::InnerJoin,
            recipes::Relation::RecipeLevelTables.def(),
        )
        .join(JoinType::InnerJoin, item_with_amount::Relation::Items.def())
        .filter(items::Column::Name.like(&search_name))
        .column_as(recipes::Column::Id, "id")
        .column_as(recipes::Column::RecipeLevelId, "rlv")
        .column_as(items::Column::Id, "item_id")
        .column_as(items::Column::Name, "item_name")
        .column_as(craft_types::Column::Name, "job")
        .column_as(recipes::Column::DifficultyFactor, "difficulty_factor")
        .column_as(recipes::Column::QualityFactor, "quality_factor")
        .column_as(recipes::Column::DurabilityFactor, "durability_factor")
        .column_as(
            recipes::Column::MaterialQualityFactor,
            "material_quality_factor",
        )
        .column_as(
            recipes::Column::RequiredCraftsmanship,
            "required_craftsmanship",
        )
        .column_as(recipes::Column::RequiredControl, "required_control")
        .column_as(recipes::Column::CanHq, "can_hq")
        .into_model::<RecipeInfo>()
        .paginate(db, 200);
    let p = paginate.num_pages().await.map_err(err_to_string)?;
    let data = paginate.fetch_page(page_id).await.map_err(err_to_string)?;
    Ok((data, p))
}

#[tauri::command(async)]
async fn recipes_ingredientions(
    recipe_id: u32,
    app_state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<Vec<(i32, i32)>, String> {
    let db = app_state.get_db(app_handle).await?;
    let mut needs = BTreeMap::new();
    let ing = ItemWithAmount::find()
        .filter(item_with_amount::Column::RecipeId.eq(recipe_id))
        .all(db)
        .await
        .map_err(err_to_string)?;
    for v in &ing {
        needs
            .entry(v.ingredient_id)
            .and_modify(|e| *e += v.amount)
            .or_insert(v.amount);
    }
    Ok(needs.into_iter().collect())
}

#[tauri::command(async)]
async fn item_info(
    item_id: i32,
    app_state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<items::Model, String> {
    let db = app_state.get_db(app_handle).await?;
    Items::find_by_id(item_id)
        .one(db)
        .await
        .map_err(err_to_string)?
        .ok_or("Item not found".to_string())
}

type SolverInstance = Arc<Mutex<Option<Box<dyn Solver + Send>>>>;
struct AppState {
    solver_list: Mutex<HashMap<SolverHash, SolverInstance>>,
    db: OnceCell<DatabaseConnection>,
}

impl AppState {
    fn new() -> Self {
        Self {
            solver_list: Mutex::new(HashMap::new()),
            db: OnceCell::new(),
        }
    }

    async fn get_db(&self, app_handle: tauri::AppHandle) -> Result<&DatabaseConnection, String> {
        const ESCAPE_SET: &AsciiSet = &CONTROLS.add(b'?').add(b'#');
        let path = app_handle
            .path_resolver()
            .resolve_resource("assets/xiv.db")
            .ok_or("database file not found")?;
        let path = utf8_percent_encode(&path.to_string_lossy(), ESCAPE_SET).to_string();
        self.db
            .get_or_try_init(|| Database::connect(format!("sqlite:{}?mode=ro", path)))
            .await
            .map_err(err_to_string)
    }
}

#[tauri::command(async)]
async fn create_solver(
    status: Status,
    use_muscle_memory: bool,
    use_manipulation: bool,
    use_observe: bool,
    app_state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let key = SolverHash {
        attributes: status.attributes,
        recipe: status.recipe,
    };
    let solver_slot = {
        let mut list = app_state.solver_list.lock().await;
        match list.entry(key.clone()) {
            Entry::Vacant(o) => o.insert(Arc::new(Mutex::new(None))).clone(),
            Entry::Occupied(ref o) => {
                return match o.get().try_lock() {
                    Ok(_) => Err("solver-already-exist".into()),
                    Err(_) => Err("solver-is-creating".into()),
                }
            }
        }
    };
    // let solver: Box<dyn Solver + Send> = Box::new(memory_search_solver::Solver::new(status));
    let solver: Box<dyn Solver + Send> = if use_muscle_memory {
        Box::new(muscle_memory_solver::PreprogressSolver::new(
            status,
            use_manipulation,
            8,
            use_observe,
        ))
    } else {
        Box::new(reflect_solver::QualitySolver::new(
            status,
            use_manipulation,
            8 + 1,
            use_observe,
        ))
    };
    *solver_slot.lock().await = Some(solver);
    Ok(())
}

/// 调用求解器
#[tauri::command(async)]
async fn read_solver(
    status: Status,
    app_state: tauri::State<'_, AppState>,
) -> Result<Vec<Actions>, String> {
    let key = SolverHash {
        attributes: status.attributes,
        recipe: status.recipe,
    };
    let result = app_state
        .solver_list
        .lock()
        .await
        .get(&key)
        .ok_or_else(|| "solver-doesn-t-exist".to_string())?
        .lock()
        .await
        .as_ref()
        .ok_or_else(|| "solver-isn-t-prepared".to_string())?
        .read_all(&status);
    Ok(result)
}

#[tauri::command(async)]
fn rika_solve(status: Status) -> Vec<Actions> {
    rika_solver::solve(status)
}

#[tauri::command(async)]
fn rika_solve_tnzever(
    status: Status,
    use_manipulation: bool,
    use_wast_not: usize,
    use_observe: bool,
    reduce_steps: bool,
) -> Vec<Actions> {
    rika_tnze_solver::solve(
        status,
        use_manipulation,
        use_wast_not,
        use_observe,
        reduce_steps,
    )
}

#[tauri::command(async)]
fn dfs_solve(status: Status, depth: usize, specialist: bool) -> Vec<Actions> {
    depth_first_search_solver::solve(status, depth, specialist)
}

#[tauri::command(async)]
fn nq_solve(status: Status, depth: usize, specialist: bool) -> Vec<Actions> {
    normal_progress_solver::solve(status, depth, specialist)
}

#[tauri::command(async)]
fn reflect_solve(
    status: Status,
    use_manipulation: bool,
    use_waste_not: usize,
    use_observe: bool,
) -> Vec<Actions> {
    reflect_solver::solve(status.clone(), use_manipulation, use_waste_not, use_observe)
}

#[tauri::command(async)]
fn raphael_solve(status: Status, backload_progress: bool, minimize_steps: bool) -> Vec<Actions> {
    raphael::solve(status.clone(), backload_progress, minimize_steps)
}

/// 释放求解器
#[tauri::command(async)]
async fn destroy_solver(
    status: Status,
    app_state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let key = SolverHash {
        attributes: status.attributes,
        recipe: status.recipe,
    };
    app_state
        .solver_list
        .lock()
        .await
        .remove(&key)
        .ok_or_else(|| "solver not exists".to_string())?;
    Ok(())
}

/// Set the window to Dark mode or Light mode by passing the is_dark argument.
/// if the is_dark is None, means use the system's default value.
///
/// This function also try its best to set the window to be vibrancy. And return true if successed,
/// which means the front-end should make its background transparent to show the window effect below.
#[tauri::command]
fn set_theme(app_handle: tauri::AppHandle, is_dark: Option<bool>) -> bool {
    let window = app_handle.get_window("main").unwrap();
    #[cfg(target_os = "macos")]
    {
        use window_vibrancy::NSVisualEffectMaterial::HudWindow;
        window_vibrancy::apply_vibrancy(&window, HudWindow, None, None).is_ok()
    }
    #[cfg(target_os = "windows")]
    {
        window_vibrancy::apply_mica(&window, is_dark).is_ok()
    }
    #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
    {
        false // Linux doesn't support vibrancy window
    }
}

#[tauri::command(async)]
fn rand_simulation(
    status: Status,
    actions: Vec<Actions>,
    n: usize,
    ignore_errors: bool,
) -> Statistics {
    app_libs::analyzer::rand_simulations::stat(status, &actions, n, ignore_errors)
}

#[tauri::command(async)]
fn calc_attributes_scope(status: Status, actions: Vec<Actions>) -> Scope {
    app_libs::analyzer::scope_of_application::calc_scope(status, &actions)
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            recipe_level_table,
            new_status,
            simulate,
            simulate_one_step,
            high_quality_probability,
            allowed_list,
            craftpoints_list,
            recipe_table,
            recipes_ingredientions,
            item_info,
            create_solver,
            read_solver,
            destroy_solver,
            rika_solve,
            rika_solve_tnzever,
            dfs_solve,
            nq_solve,
            reflect_solve,
            raphael_solve,
            set_theme,
            rand_simulation,
            calc_attributes_scope,
        ])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.set_decorations(true)?;

            let packaeg_info = app.package_info();
            window.set_title(
                format!("{} v{}", window.title()?, packaeg_info.version.to_string()).as_str(),
            )?;

            #[cfg(target_os = "macos")]
            {
                use window_vibrancy::NSVisualEffectMaterial;
                let _ = window_vibrancy::apply_vibrancy(
                    &window,
                    NSVisualEffectMaterial::HudWindow,
                    None,
                    None,
                );
            }

            #[cfg(target_os = "windows")]
            let _ = window_vibrancy::apply_mica(&window, None);

            Ok(())
        })
        .run(tauri::generate_context!())
        .map_err(|err| msgbox::create("Error", err.to_string().as_str(), msgbox::IconType::Error))
        .unwrap();
}
