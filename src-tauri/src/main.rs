// This file is part of BestCraft.
// Copyright (C) 2026 Tnze
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
    ffxiv_crafting::{Actions, Attributes, CastActionError, Recipe, Status},
    solver::{
        depth_first_search_solver, normal_progress_solver, raphael, reflect_solver, Solver,
        SolverHash,
    },
    SimulateOneStepResult, SimulateResult,
};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use rand::rng;
use sea_orm::{entity::*, query::*, Database, DatabaseConnection, FromQueryResult};
use serde::Serialize;
use tauri::{path::BaseDirectory, Manager};
use tokio::sync::{Mutex, OnceCell};

use app_db::{
    collectables_shop_refine, craft_types, item_action, item_food, item_food_effect,
    item_with_amount, items, prelude::*, recipe_level_tables, recipes, wks_mission_recipe,
    wks_mission_to_do, wks_mission_unit,
};

/// 创建新的Recipe对象，蕴含了模拟一次制作过程所必要的全部配方信息
#[tauri::command(async)]
async fn recipe_level_table(
    rlv: u32,
    app_state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<recipe_level_tables::Model, String> {
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
    stellar_steady_hand_count: u8,
) -> Result<Status, String> {
    app_libs::new_status(attrs, recipe, stellar_steady_hand_count)
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
    let mut rng = rng();
    app_libs::simulate_one_step(&mut status, action, force_success, &mut rng)
        .map(|is_success| SimulateOneStepResult { status, is_success })
        .map_err(err_to_string)
}

#[tauri::command(async)]
fn simulate_detail(status: Status, actions: Vec<Actions>) -> Vec<Result<Status, CastActionError>> {
    app_libs::simulate_detail(status, actions)
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
    item_amount: i32,
    job: String,

    difficulty_factor: u16,
    quality_factor: u16,
    durability_factor: u16,
    material_quality_factor: u8,

    required_craftsmanship: u16,
    required_control: u16,

    can_hq: bool,
    is_expert: bool,
    recipe_notebook_list: u32,
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
        .column_as(item_with_amount::Column::Amount, "item_amount")
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
        .column_as(recipes::Column::IsExpert, "is_expert")
        .column_as(recipes::Column::RecipeNotebookList, "recipe_notebook_list")
        .into_model::<RecipeInfo>()
        .paginate(db, 200);
    let p = paginate.num_pages().await.map_err(err_to_string)?;
    let data = paginate.fetch_page(page_id).await.map_err(err_to_string)?;
    Ok((data, p))
}

#[tauri::command(async)]
async fn recipe_level_table_by_job_level(
    job_level: i32,
    app_state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<Option<recipe_level_tables::Model>, String> {
    let db = app_state.get_db(app_handle).await.map_err(err_to_string)?;
    let result = RecipeLevelTables::find()
        .filter(recipe_level_tables::Column::ClassJobLevel.eq(job_level))
        .order_by_asc(recipe_level_tables::Column::Id)
        .one(db)
        .await
        .map_err(err_to_string)?;
    Ok(result)
}

#[tauri::command(async)]
async fn recipes_ingredientions(
    recipe_id: i32,
    app_state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<Vec<(u32, i32)>, String> {
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
    item_id: u32,
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
    #[allow(dead_code)]
    item_food_duration: u16,
}

#[derive(Default, Serialize)]
struct Enhancer {
    name: String,
    level: u32,
    is_hq: bool,

    cm: Option<i8>,
    cm_max: Option<i16>,
    ct: Option<i8>,
    ct_max: Option<i16>,
    cp: Option<i8>,
    cp_max: Option<i16>,
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
                .get(&(item.item_food_id as u32))
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

#[derive(Default, Serialize, FromQueryResult)]
struct TemporaryActionInfo {
    action: u32,
    count: u32,
}

#[tauri::command(async)]
async fn temporary_action_info(
    app_state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
    recipe_id: i32,
) -> Result<Option<TemporaryActionInfo>, String> {
    let db = app_state.get_db(app_handle).await?;
    let query = WksMissionUnit::find()
        .join(
            JoinType::InnerJoin,
            wks_mission_unit::Relation::WksMissionRecipe.def(),
        )
        .join(
            JoinType::LeftJoin,
            wks_mission_unit::Relation::WksMissionToDo3.def(),
        )
        .filter(
            wks_mission_recipe::Column::Recipe0Id
                .eq(recipe_id)
                .or(wks_mission_recipe::Column::Recipe1Id.eq(recipe_id))
                .or(wks_mission_recipe::Column::Recipe2Id.eq(recipe_id))
                .or(wks_mission_recipe::Column::Recipe3Id.eq(recipe_id))
                .or(wks_mission_recipe::Column::Recipe4Id.eq(recipe_id)),
        )
        .select_only()
        .column_as(wks_mission_to_do::Column::TemporaryAction, "action")
        .column_as(wks_mission_to_do::Column::TemporaryActionCount, "count");
    let result = query
        .into_model::<TemporaryActionInfo>()
        .one(db)
        .await
        .map_err(err_to_string)?;
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
    let solver = Box::new(reflect_solver::QualitySolver::new(
        status,
        use_manipulation,
        8 + 1,
        use_observe,
    ));
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
    target_quality: Option<u32>,
    use_manipulation: bool,
    use_heart_and_soul: bool,
    use_quick_innovation: bool,
    use_trained_eye: bool,
    backload_progress: bool,
    adversarial: bool,
    stellar_steady_hand_charges: u8,
) -> Vec<Actions> {
    raphael::solve(
        status.clone(),
        target_quality,
        use_manipulation,
        use_heart_and_soul,
        use_quick_innovation,
        use_trained_eye,
        backload_progress,
        adversarial,
        stellar_steady_hand_charges,
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
    #[allow(unused)]
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
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            recipe_level_table,
            recipe_level_table_by_job_level,
            new_status,
            simulate,
            simulate_one_step,
            simulate_detail,
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
            temporary_action_info,
            create_solver,
            read_solver,
            destroy_solver,
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
            use native_dialog::{DialogBuilder, MessageLevel};
            DialogBuilder::message()
                .set_level(MessageLevel::Error)
                .set_text(err.to_string())
                .confirm()
                .show()
        })
        .unwrap();
}
