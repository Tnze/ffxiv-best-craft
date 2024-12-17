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

use std::{
    collections::{hash_map::Entry, BTreeMap, HashMap},
    sync::Arc,
};

use app_libs::{
    analyzer::{rand_simulations, scope_of_application::Scope},
    ffxiv_crafting::{Actions, Attributes, Recipe, Status},
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
use tauri::{path::BaseDirectory, Manager};
use tokio::sync::{Mutex, OnceCell};

mod db;
mod memoization_solver;
mod muscle_memory_solver;
mod rika_tnze_solver;

use db::{
    collectables_shop_refine, craft_types, item_action, item_food, item_food_effect,
    item_with_amount, items, prelude::*, recipe_level_tables, recipes,
};

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
fn new_status(attrs: Attributes, recipe: Recipe) -> Result<Status, String> {
    app_libs::new_status(attrs, recipe)
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
    craft_type_id: Option<i32>,
    recipe_level: Option<i32>,
    job_level_min: Option<i32>,
    job_level_max: Option<i32>,
    app_state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(Vec<RecipeInfo>, u64), String> {
    let db = app_state.get_db(app_handle).await.map_err(err_to_string)?;
    let mut query = Recipes::find()
        .join(JoinType::InnerJoin, recipes::Relation::CraftTypes.def())
        .join(JoinType::InnerJoin, recipes::Relation::ItemWithAmount.def())
        .join(
            JoinType::InnerJoin,
            recipes::Relation::RecipeLevelTables.def(),
        )
        .join(JoinType::InnerJoin, item_with_amount::Relation::Items.def())
        .filter(items::Column::Name.like(&search_name));
    if let Some(rlv) = recipe_level {
        query = query.filter(recipes::Column::RecipeLevelId.eq(rlv))
    }
    if let Some(craft_type_id) = craft_type_id {
        query = query.filter(recipes::Column::CraftTypeId.eq(craft_type_id))
    }
    if let Some(job_level_min) = job_level_min {
        query = query.filter(recipe_level_tables::Column::ClassJobLevel.gte(job_level_min))
    }
    if let Some(job_level_max) = job_level_max {
        query = query.filter(recipe_level_tables::Column::ClassJobLevel.lte(job_level_max))
    }
    let paginate = query
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
    recipe_id: i32,
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

// #[derive(Serialize)]
// enum CollectablesMetadata {
//     CollectablesShopRefine(CollectablesShopRefine),
//     // HWDCrafterSupply,
//     // SatisfactionSupply,
//     // SharlayanCraftWorkSupply,
//     // CollectablesRefine,
//     Unknown(u16),
// }

#[tauri::command(async)]
async fn recipe_collectability(
    recipe_id: i32,
    app_state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<Option<collectables_shop_refine::Model>, String> {
    let db = app_state.get_db(app_handle).await?;
    let result = CollectablesShopRefine::find()
        .reverse_join(Recipes)
        .filter(recipes::Column::Id.eq(recipe_id))
        .one(db)
        .await
        .map_err(err_to_string)?;
    Ok(result)
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

#[derive(FromQueryResult, Debug)]
struct ItemFoodAction {
    name: String,
    level: u32,
    item_food_id: u16,
    item_food_duration: u16,
}

#[derive(Default, Serialize)]
struct Enhancer {
    name: String,
    level: u32,
    is_hq: bool,

    cm: Option<i32>,
    cm_max: Option<i32>,
    ct: Option<i32>,
    ct_max: Option<i32>,
    cp: Option<i32>,
    cp_max: Option<i32>,
}

const MEDICINE_SEARCH_ID: u32 = 43;
const MEALS_SEARCH_ID: u32 = 45;

#[tauri::command(async)]
async fn craft_type(
    app_state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<Vec<craft_types::Model>, String> {
    let db = app_state.get_db(app_handle).await?;
    let query = CraftTypes::find()
        .column_as(craft_types::Column::Id, "id")
        .column_as(craft_types::Column::Name, "name")
        .order_by(craft_types::Column::Id, Order::Asc);
    let result = query
        .into_model::<craft_types::Model>()
        .all(db)
        .await
        .map_err(err_to_string)?;
    Ok(result)
}

#[tauri::command(async)]
async fn medicine_table(
    app_state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<Vec<Enhancer>, String> {
    let db = app_state.get_db(app_handle).await?;
    query_enhancers(db, MEDICINE_SEARCH_ID).await
}

#[tauri::command(async)]
async fn meals_table(
    app_state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<Vec<Enhancer>, String> {
    let db = app_state.get_db(app_handle).await?;
    query_enhancers(db, MEALS_SEARCH_ID).await
}

async fn query_enhancers(
    conn: &DatabaseConnection,
    search_id: u32,
) -> Result<Vec<Enhancer>, String> {
    let crafting_item_food = ItemFood::find()
        .join(
            JoinType::InnerJoin,
            item_food::Relation::ItemFoodEffect.def(),
        )
        .select_with(ItemFoodEffect)
        .filter(item_food_effect::Column::BaseParam.is_in([11, 70, 71]))
        .all(conn)
        .await
        .map_err(err_to_string)?
        .into_iter()
        .map(|(item_food, effects)| (item_food.id, effects));
    let crafting_item_food = BTreeMap::from_iter(crafting_item_food);
    let result = Items::find()
        .select_only()
        .select_column_as(items::Column::Name, "name")
        .select_column_as(items::Column::Level, "level")
        .filter(items::Column::ItemSearchCategoryId.eq(search_id))
        .join(JoinType::InnerJoin, items::Relation::ItemAction.def())
        .select_column_as(item_action::Column::Data2, "item_food_id")
        .select_column_as(item_action::Column::Data3, "item_food_duration")
        .filter(item_action::Column::Type.between(844, 846))
        .filter(item_action::Column::Data2.is_in(crafting_item_food.iter().map(|v| *v.0)))
        .into_model::<ItemFoodAction>()
        .all(conn)
        .await
        .map_err(err_to_string)?;
    let result = result
        .into_iter()
        .flat_map(|item| {
            let mut enh = Enhancer {
                name: item.name.clone(),
                level: item.level,
                is_hq: false,
                ..Enhancer::default()
            };
            let mut enh_hq = Enhancer {
                name: item.name.clone(),
                level: item.level,
                is_hq: true,
                ..Enhancer::default()
            };
            for item_food in crafting_item_food
                .get(&(item.item_food_id as i32))
                .into_iter()
                .flatten()
            {
                match item_food.base_param {
                    11 => {
                        enh.cp = Some(item_food.value);
                        enh.cp_max = Some(item_food.max);
                        enh_hq.cp = Some(item_food.value_hq);
                        enh_hq.cp_max = Some(item_food.max_hq);
                    }
                    70 => {
                        enh.cm = Some(item_food.value);
                        enh.cm_max = Some(item_food.max);
                        enh_hq.cm = Some(item_food.value_hq);
                        enh_hq.cm_max = Some(item_food.max_hq);
                    }
                    71 => {
                        enh.ct = Some(item_food.value);
                        enh.ct_max = Some(item_food.max);
                        enh_hq.ct = Some(item_food.value_hq);
                        enh_hq.ct_max = Some(item_food.max_hq);
                    }
                    _ => {}
                }
            }
            [enh, enh_hq].into_iter()
        })
        .collect();
    Ok(result)
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
            .path()
            .resolve("assets/xiv.db", BaseDirectory::Resource)
            .map_err(|e| format!("database file not found: {e}"))?;
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
fn raphael_solve(
    status: Status,
    use_manipulation: bool,
    use_heart_and_soul: bool,
    use_quick_innovation: bool,
    use_trained_eye: bool,
    backload_progress: bool,
    adversarial: bool,
    unsound_branch_pruning: bool,
) -> Vec<Actions> {
    raphael::solve(
        status.clone(),
        use_manipulation,
        use_heart_and_soul,
        use_quick_innovation,
        use_trained_eye,
        backload_progress,
        adversarial,
        unsound_branch_pruning,
    )
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
    let window = app_handle.get_webview_window("main").unwrap();
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
) -> rand_simulations::Statistics {
    rand_simulations::stat(status, &actions, n, ignore_errors)
}

#[tauri::command(async)]
fn rand_collectables_simulation(
    status: Status,
    actions: Vec<Actions>,
    n: usize,
    ignore_errors: bool,
    collectables_shop_refine: rand_simulations::CollectablesShopRefine,
) -> rand_simulations::CollectableStatistics {
    rand_simulations::stat_collectables(
        status,
        &actions,
        n,
        ignore_errors,
        collectables_shop_refine,
    )
}

#[tauri::command(async)]
fn calc_attributes_scope(status: Status, actions: Vec<Actions>) -> Scope {
    app_libs::analyzer::scope_of_application::calc_scope(status, &actions)
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
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
            recipe_collectability,
            item_info,
            craft_type,
            medicine_table,
            meals_table,
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
            rand_collectables_simulation,
            calc_attributes_scope,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
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
        // .map_err(|err| msgbox::create("Error", err.to_string().as_str(), msgbox::IconType::Error))
        .map_err(|err| {
            use native_dialog::{MessageDialog, MessageType};
            MessageDialog::new()
                .set_type(MessageType::Error)
                .set_text(err.to_string().as_str())
                .show_confirm()
        })
        .unwrap();
}
