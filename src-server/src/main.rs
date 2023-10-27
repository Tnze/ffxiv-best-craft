use std::{collections::BTreeMap, env};

use salvo::cors::Cors;
use salvo::hyper::Method;
use salvo::prelude::*;
use salvo::{affix, cors};
use sea_orm::{entity::*, query::*, Database, DatabaseConnection, FromQueryResult};
use serde::Serialize;

mod db;
use db::{craft_types, item_with_amount, items, prelude::*, recipes};

type Result<T> = std::result::Result<T, StatusError>;

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    // get env vars
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // create post table if not exists
    let conn = Database::connect(&db_url).await.unwrap();
    let state = AppState { conn };

    let cors = Cors::new()
        .allow_origin(cors::Any)
        .allow_methods([Method::GET])
        .allow_headers(vec![
            "Access-Control-Request-Method",
            "Access-Control-Allow-Origin",
            "Access-Control-Allow-Headers",
            "Access-Control-Max-Age",
        ])
        .into_handler();

    let router = Router::with_hoop(cors)
        .hoop(affix::inject(state))
        .push(Router::with_path("recipe_level_table").get(recipe_level_table))
        .push(Router::with_path("recipe_table").get(recipe_table))
        .push(Router::with_path("recipes_ingredientions").get(recipes_ingredientions))
        .push(Router::with_path("item_info").get(item_info));
    let listener = TcpListener::new(server_url);
    let acceptor = listener.bind().await;
    Server::new(acceptor).serve(router).await;
}

#[derive(FromQueryResult, Serialize)]
struct RecipeInfo {
    id: u32,
    rlv: u32,
    item_id: u32,
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

// rlv: i32,
#[handler]
async fn recipe_level_table(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<()> {
    let state = depot
        .obtain::<AppState>()
        .map_err(|_| StatusError::internal_server_error())?;
    let rlv = req
        .query::<u32>("rlv")
        .ok_or_else(|| StatusError::bad_request())?;
    let Some(rt) = RecipeLevelTables::find_by_id(rlv)
        .one(&state.conn)
        .await
        .map_err(|_| StatusError::internal_server_error())?
    else {
        return Err(StatusError::bad_gateway());
    };
    res.render(Json(rt));
    Ok(())
}

// page_id: u64,
// search_name: String,
#[handler]
async fn recipe_table(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let state = depot
        .obtain::<AppState>()
        .map_err(|_| StatusError::internal_server_error().detail("Obtain AppState error"))?;
    let page_id = req
        .query::<u64>("page_id")
        .ok_or_else(|| StatusError::bad_request().detail("Need 'page_id'"))?;
    let search_name = req
        .query::<String>("search_name")
        .ok_or_else(|| StatusError::bad_request().detail("Need 'search_name'"))?;

    let query = Recipes::find()
        .join(JoinType::InnerJoin, recipes::Relation::CraftTypes.def())
        .join(JoinType::InnerJoin, recipes::Relation::ItemWithAmount.def())
        .join(
            JoinType::InnerJoin,
            recipes::Relation::RecipeLevelTables.def(),
        )
        .join(JoinType::InnerJoin, item_with_amount::Relation::Items.def())
        .select_only()
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
        .column_as(recipes::Column::CanHq, "can_hq");
    let paginate = query.into_model::<RecipeInfo>().paginate(&state.conn, 200);

    let p = paginate.num_pages().await.map_err(|err| {
        println!("Failed to get total page numbers: {err}");
        StatusError::internal_server_error().detail("Failed to get total page numbers")
    })?;
    let data = paginate.fetch_page(page_id).await.map_err(|err| {
        println!("Failed to get recipe data: {err}");
        StatusError::internal_server_error().detail("Failed to get recipe data")
    })?;

    #[derive(Serialize)]
    struct Resp {
        data: Vec<RecipeInfo>,
        p: u64,
    }
    res.render(Json(Resp { data, p }));
    Ok(())
}

// recipe_id: u32,
#[handler]
async fn recipes_ingredientions(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<()> {
    let state = depot
        .obtain::<AppState>()
        .map_err(|_| StatusError::internal_server_error())?;
    let recipe_id = req
        .query::<u32>("recipe_id")
        .ok_or_else(|| StatusError::bad_request())?;

    let mut needs = BTreeMap::new();
    let ing = ItemWithAmount::find()
        .filter(item_with_amount::Column::RecipeId.eq(recipe_id))
        .all(&state.conn)
        .await
        .map_err(|_| StatusError::internal_server_error())?;
    for v in &ing {
        needs
            .entry(v.ingredient_id)
            .and_modify(|e| *e += v.amount)
            .or_insert(v.amount);
    }
    let result: Vec<_> = needs.into_iter().collect();
    res.render(Json(result));
    Ok(())
}

// item_id: i32
#[handler]
async fn item_info(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let state = depot
        .obtain::<AppState>()
        .map_err(|_| StatusError::internal_server_error())?;
    let item_id = req
        .query::<u32>("item_id")
        .ok_or_else(|| StatusError::bad_request())?;
    let result = Items::find_by_id(item_id)
        .one(&state.conn)
        .await
        .map_err(|_| StatusError::internal_server_error())?
        .ok_or_else(|| StatusError::bad_gateway())?;
    res.render(Json(result));
    Ok(())
}
