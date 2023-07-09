#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(int_roundings)]

use std::collections::{hash_map::Entry, BTreeMap, HashMap};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use ffxiv_crafting::{
    Actions, Attributes, CastActionError, Condition, ConditionIterator, Recipe, Status, RecipeLevel,
};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use rand::{random, seq::SliceRandom, thread_rng};
use sea_orm::{entity::*, query::*, Database, DatabaseConnection, FromQueryResult};
use serde::Serialize;
use tauri::Manager;
use tokio::sync::{Mutex, OnceCell};

mod db;
mod depth_first_search_solver;
mod hard_recipe;
mod memoization_solver;
mod muscle_memory_solver;
mod reflect_solver;
mod rika_solver;
mod rika_tnze_solver;
mod solver;

use crate::solver::Solver;
use db::{craft_types, item_with_amount, items, prelude::*, recipes};

/// 创建新的Recipe对象，蕴含了模拟一次制作过程所必要的全部配方信息
#[tauri::command(async)]
async fn recipe_level_table(
    rlv: i32,
    app_state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<db::recipe_level_tables::Model, String> {
    let db = app_state.get_db(app_handle).await.map_err(err_to_string)?;
    let Some(rt) = RecipeLevelTables::find_by_id(rlv).one(db).await.map_err(err_to_string)? else {
        return Err(String::from("unknown recipe level"))
    };
    Ok(rt)
}

/// 初始化一个表示一次制作的初始状态的Status对象，包含玩家属性、配方信息和初期品质
#[tauri::command(async)]
fn new_status(attrs: Attributes, recipe: Recipe, recipe_level: RecipeLevel) -> Result<Status, String> {
    if recipe.job_level > attrs.level + 5 {
        Err("Player level lower than recipe's require".to_string())
    } else {
        Ok(Status::new(attrs, recipe, recipe_level))
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

/// 模拟以指定初始状态按顺序执行一个技能序列后的结果，
/// 返回值`SimulateResult`包含了最终状态以及模拟过程中每个技能失败的位置及原因
#[tauri::command(async)]
fn simulate(status: Status, actions: Vec<Actions>) -> SimulateResult {
    let mut result = SimulateResult {
        status,
        errors: Vec::new(),
    };
    for (pos, sk) in actions.iter().enumerate() {
        match result.status.is_action_allowed(*sk) {
            Ok(_) => result.status.cast_action(*sk),
            Err(err) => result.errors.push(CastErrorPos { pos, err }),
        }
    }
    result
}

#[derive(Serialize)]
struct SimulateOneStepResult {
    status: Status,
    is_success: bool,
}

/// 只模拟一步制作，计算技能概率和制作状态，更新制作状态
#[tauri::command(async)]
fn simulate_one_step(
    mut status: Status,
    action: Actions,
    force_success: bool,
) -> Result<SimulateOneStepResult, String> {
    let mut rng = thread_rng();
    status.is_action_allowed(action).map_err(err_to_string)?;
    let is_success = force_success || status.success_rate(action) as f32 / 100.0 > random();
    if is_success {
        status.cast_action(action);
    } else {
        status.cast_action(match action {
            Actions::RapidSynthesis => Actions::RapidSynthesisFail,
            Actions::HastyTouch => Actions::HastyTouchFail,
            Actions::FocusedSynthesis => Actions::FocusedSynthesisFail,
            Actions::FocusedTouch => Actions::FocusedTouchFail,
            _ => unreachable!(),
        });
    }
    if !matches!(action, Actions::FinalAppraisal | Actions::HeartAndSoul) {
        status.condition = if force_success {
            Condition::Normal
        } else {
            ConditionIterator::new(
                status.recipe.conditions_flag as i32,
                status.attributes.level as i32,
            )
            .collect::<Vec<_>>()
            .choose_weighted(&mut rng, |c| c.1)
            .unwrap()
            .0
        };
    }
    Ok(SimulateOneStepResult { status, is_success })
}

/// 计算当前状态下可以释放技能的集合，用于模拟界面将不可释放技能置灰
#[tauri::command(async)]
fn allowed_list(status: Status, skills: Vec<Actions>) -> Vec<String> {
    skills
        .iter()
        .map(|&sk| match status.is_action_allowed(sk) {
            Ok(_) => "ok".to_string(),
            Err(err) => err.to_string(),
        })
        .collect()
}

/// 计算当前状态下，传入的技能列表中每个技能需要消耗的制作力，
/// 技能消耗的制作力会受到技能连击和当前制作状态（球色—）的影响
#[tauri::command(async)]
fn craftpoints_list(status: Status, skills: Vec<Actions>) -> Vec<i32> {
    skills.iter().map(|&sk| status.craft_point(sk)).collect()
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
    checklist: Vec<(i32, i32)>,
    app_state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<Vec<(i32, i32)>, String> {
    let db = app_state.get_db(app_handle).await?;
    let mut needs = BTreeMap::new();
    for (item_id, amount) in checklist {
        let r = Recipes::find()
            .join(JoinType::InnerJoin, recipes::Relation::ItemWithAmount.def())
            .filter(item_with_amount::Column::IngredientId.eq(item_id))
            .one(db)
            .await
            .map_err(err_to_string)?;
        match r {
            Some(r) => {
                let ing = ItemWithAmount::find()
                    .filter(item_with_amount::Column::RecipeId.eq(r.id))
                    .all(db)
                    .await
                    .map_err(err_to_string)?;
                for v in &ing {
                    needs
                        .entry(v.ingredient_id)
                        .and_modify(|e| *e += v.amount)
                        .or_insert(v.amount * amount);
                }
            }
            None => {
                needs
                    .entry(item_id)
                    .and_modify(|e| *e += amount)
                    .or_insert(amount);
            }
        }
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
    solver_list: Mutex<HashMap<solver::SolverHash, SolverInstance>>,
    db: OnceCell<DatabaseConnection>,
    should_be_transparent: AtomicBool,
}

impl AppState {
    fn new() -> Self {
        Self {
            solver_list: Mutex::new(HashMap::new()),
            db: OnceCell::new(),
            should_be_transparent: AtomicBool::new(false),
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
    let key = solver::SolverHash {
        attributes: status.attributes,
        recipe: status.recipe,
    };
    let solver_slot = {
        let mut list = app_state.solver_list.lock().await;
        match list.entry(key.clone()) {
            Entry::Vacant(o) => o.insert(Arc::new(Mutex::new(None))).clone(),
            Entry::Occupied(ref o) => {
                return match o.get().try_lock() {
                    Ok(_) => Err("Solver already exist".into()),
                    Err(_) => Err("Solver is creating".into()),
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
    let key = solver::SolverHash {
        attributes: status.attributes,
        recipe: status.recipe,
    };
    let result = app_state
        .solver_list
        .lock()
        .await
        .get(&key)
        .ok_or_else(|| "solver doesn't exists".to_string())?
        .lock()
        .await
        .as_ref()
        .ok_or_else(|| "solver isn't prepared".to_string())?
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
    depth_first_search_solver::solve(&status, depth, specialist)
}

/// 释放求解器
#[tauri::command(async)]
async fn destroy_solver(
    status: Status,
    app_state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let key = solver::SolverHash {
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

#[tauri::command]
fn should_be_transparent(app_state: tauri::State<AppState>) -> bool {
    app_state.should_be_transparent.load(Ordering::SeqCst)
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            recipe_level_table,
            new_status,
            simulate,
            simulate_one_step,
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
            should_be_transparent,
        ])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.set_decorations(true)?;

            let state = app.state::<AppState>();

            #[allow(unused_mut, unused_assignments)]
            let mut sbt = false;
            #[cfg(target_os = "macos")]
            {
                use window_vibrancy::NSVisualEffectMaterial;
                sbt = window_vibrancy::apply_vibrancy(
                    &window,
                    NSVisualEffectMaterial::HudWindow,
                    None,
                    None,
                )
                .is_ok();
            }

            #[cfg(target_os = "windows")]
            {
                sbt = window_vibrancy::apply_mica(&window).is_ok();
            }

            state
                .inner()
                .should_be_transparent
                .store(sbt, Ordering::SeqCst);

            Ok(())
        })
        .run(tauri::generate_context!())
        .map_err(|err| msgbox::create("Error", err.to_string().as_str(), msgbox::IconType::Error))
        .unwrap();
}
