#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(generic_const_exprs)]
#![feature(async_closure)]

use std::{
    collections::{hash_map::Entry, HashMap},
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{http::StatusCode, routing, Json, Router};
use ffxiv_crafting::{Attributes, CastActionError, Recipe, Skills, Status};
use futures::{prelude::*, stream::FuturesOrdered};
use sea_orm::{entity::*, query::*, Database, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use tauri::Manager;
use tokio::sync::oneshot;

mod db;
mod ordinary_solver;
mod preprogress_solver;
mod solver;

use crate::ordinary_solver::{ProgressSolver, QualitySolver};
use crate::preprogress_solver::PreprogressSolver;
use crate::solver::Solver;

#[tauri::command(async)]
fn new_recipe(
    rlv: i32,
    difficulty_factor: u16,
    quality_factor: u16,
    durability_factor: u16,
) -> Recipe {
    Recipe::new(rlv, difficulty_factor, quality_factor, durability_factor)
}

#[tauri::command(async)]
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

#[tauri::command(async)]
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

#[tauri::command(async)]
fn allowed_list(status: Status, skills: Vec<Skills>) -> Vec<String> {
    skills
        .iter()
        .map(|&sk| match status.is_action_allowed(sk) {
            Ok(_) => "ok".to_string(),
            Err(err) => err.to_string(),
        })
        .collect()
}

#[tauri::command(async)]
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

#[tauri::command(async)]
async fn recipe_table(
    page_id: usize,
    app_state: tauri::State<'_, AppState>,
) -> Result<Vec<RecipeRow>, String> {
    let db = &{
        let mut db = app_state.db.lock().await;
        if let None = *db {
            let new_db = Database::connect("sqlite:./assets/xiv.db?mode=ro").await;
            *db = Some(new_db.unwrap());
        }
        db.as_ref().unwrap().clone()
    };
    let paginate = db::prelude::Recipes::find().paginate(db, 1000);
    // let p = paginate.num_pages().await;
    let recipes_iter = paginate
        .fetch_page(page_id)
        .await
        .map_err(|e| e.to_string())?;
    let data = recipes_iter
        .iter()
        .map(async move |recipe| -> Result<RecipeRow, String> {
            let (result, result_item) = recipe
                .find_related(db::prelude::ItemWithAmount)
                .find_also_related(db::prelude::Items)
                .one(db)
                .await
                .map_err(|e| e.to_string())?
                .ok_or("None".to_string())?;
            let result_item = result_item.ok_or("None".to_string())?;
            Ok(RecipeRow {
                id: recipe.number as usize,
                rlv: recipe.recipe_level,
                name: result_item.name,
                job: "".to_string(),
                difficulty_factor: recipe.difficulty_factor as u16,
                quality_factor: recipe.quality_factor as u16,
                durability_factor: recipe.durability_factor as u16,
            })
        })
        .collect::<FuturesOrdered<_>>();
    let data: Vec<Result<RecipeRow, String>> = data.collect().await;
    let data = data
        .into_iter()
        .flat_map(|x| x.ok())
        .collect::<Vec<RecipeRow>>();
    dbg!("after collect");
    Ok(data)
}

struct AppState {
    solver_list: Mutex<HashMap<ordinary_solver::SolverHash, Option<Box<dyn Solver + Send + Sync>>>>,
    db: tokio::sync::Mutex<Option<DatabaseConnection>>,
    shutdown_signal: Mutex<Option<oneshot::Sender<()>>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            solver_list: Mutex::new(HashMap::new()),
            db: tokio::sync::Mutex::new(None),
            shutdown_signal: Mutex::new(None),
        }
    }
}

#[tauri::command(async)]
fn create_solver(
    status: Status,
    use_muscle_memory: bool,
    use_manipulation: bool,
    app_state: tauri::State<AppState>,
) -> Result<(), String> {
    let key = ordinary_solver::SolverHash {
        attributes: status.attributes,
        recipe: status.recipe,
    };
    {
        let mut list = app_state
            .solver_list
            .lock()
            .map_err(|err| err.to_string())?;
        match list.entry(key.clone()) {
            Entry::Occupied(e) => match e.get() {
                Some(_) => Err("solver already exists".to_string()),
                None => Err("solver is creating".to_string()),
            },
            Entry::Vacant(e) => {
                e.insert(None); // tell others we are already take this place
                Ok(())
            }
        }
    }
    .and_then(|_| {
        let solver: Box<dyn Solver + Send + Sync> = if use_muscle_memory {
            let progress_list = preprogress_list(&status);
            if use_manipulation {
                let mut driver = ProgressSolver::new(status.clone());
                driver.init();
                let mut solver =
                    PreprogressSolver::<8, 8>::new(status, progress_list, Arc::new(driver));
                solver.init();
                Box::new(solver)
            } else {
                let mut driver = ProgressSolver::new(status.clone());
                driver.init();
                let mut solver =
                    PreprogressSolver::<0, 8>::new(status, progress_list, Arc::new(driver));
                solver.init();
                Box::new(solver)
            }
        } else {
            if use_manipulation {
                let mut driver = ProgressSolver::new(status.clone());
                driver.init();
                let mut solver = QualitySolver::<8, 8>::new(status, Arc::new(driver));
                solver.init();
                Box::new(solver)
            } else {
                let mut driver = ProgressSolver::new(status.clone());
                driver.init();
                let mut solver = QualitySolver::<0, 8>::new(status, Arc::new(driver));
                solver.init();
                Box::new(solver)
            }
        };
        let mut list = app_state
            .solver_list
            .lock()
            .map_err(|err| err.to_string())?;
        *list.get_mut(&key).unwrap() = Some(solver); // we are sure that there is a None value so we can successfully get it
        Ok(())
    })
}

fn preprogress_list(status: &Status) -> Vec<u16> {
    let level_based = |level, e1, e2| {
        if status.attributes.level < level {
            e1
        } else {
            e2
        }
    };
    vec![
        status.calc_synthesis(level_based(31, 1.0, 1.2)), // basic synth
        status.calc_synthesis(level_based(82, 1.5, 1.8)), // careful synth
    ]
}

#[tauri::command(async)]
fn read_solver(status: Status, app_state: tauri::State<AppState>) -> Result<Vec<Skills>, String> {
    let key = ordinary_solver::SolverHash {
        attributes: status.attributes,
        recipe: status.recipe,
    };
    let mut list = app_state
        .solver_list
        .lock()
        .map_err(|err| err.to_string())?;
    match list.entry(key) {
        Entry::Occupied(e) => {
            if let Some(v) = e.get() {
                Ok(v.read_all(&status))
            } else {
                Err("solver not prepared".to_string())
            }
        }
        _ => Err("solver not exists".to_string()),
    }
}

#[tauri::command(async)]
fn destroy_solver(status: Status, app_state: tauri::State<AppState>) -> Result<(), String> {
    let key = ordinary_solver::SolverHash {
        attributes: status.attributes,
        recipe: status.recipe,
    };
    let mut list = app_state.solver_list.lock().unwrap();
    match list.entry(key) {
        Entry::Occupied(e) => match e.get() {
            Some(_) => {
                e.remove();
                Ok(())
            }
            None => Err("solver is creating".to_string()), // we can't take the solver when it is a None, because the creating thread expect it will not be remove
        },
        Entry::Vacant(_) => Err("solver not exists".to_string()),
    }
}

#[tauri::command(async)]
async fn start_http_server(
    addr: String,
    app_state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    #[derive(Deserialize, Clone, Debug)]
    struct ActionStep {
        action: i32,
        progress: u16,
        quality: u32,
        durability: u16,
        condition: i8,
    }
    #[derive(Serialize, Clone)]
    struct ActionStepJS {
        action: Skills,
        progress: u16,
        quality: u32,
        durability: u16,
        condition: i8,
    }
    let action = async move |Json(payload): Json<ActionStep>| {
        println!("{:?}", payload);
        match ffxiv_crafting::data::action_table(payload.action) {
            Some(sk) => app_handle
                .emit_all(
                    "action-step",
                    ActionStepJS {
                        action: sk,
                        progress: payload.progress,
                        quality: payload.quality,
                        durability: payload.durability,
                        condition: payload.condition,
                    },
                )
                .unwrap(),
            None => return (StatusCode::BAD_REQUEST, ()),
        };
        (StatusCode::NO_CONTENT, ())
    };

    println!("starting http server");
    let rx = {
        let mut current_tx = app_state.shutdown_signal.lock().unwrap();
        if let Some(_) = *current_tx {
            return Err(String::from("http server is running"));
        }
        let (tx, rx) = oneshot::channel::<()>();
        *current_tx = Some(tx);
        rx
    };
    let server = Router::new().route("/action", routing::post(action));
    let addr: SocketAddr = addr.parse().unwrap();
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .with_graceful_shutdown(wait_for_shutdown(rx))
        .await
        .unwrap();
    Ok(())
}

#[allow(unused_must_use)]
async fn wait_for_shutdown(rx: oneshot::Receiver<()>) {
    rx.await;
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            new_recipe,
            new_status,
            simulate,
            allowed_list,
            craftpoints_list,
            recipe_table,
            create_solver,
            read_solver,
            destroy_solver,
            start_http_server,
        ])
        .run(tauri::generate_context!())
        .map_err(|err| {
            msgbox::create(
                "错误",
                format!("error while running tauri application: {}", err).as_str(),
                msgbox::IconType::Error,
            )
        })
        .unwrap();
}
