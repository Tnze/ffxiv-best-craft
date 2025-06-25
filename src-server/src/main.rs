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

use std::{
    collections::{BTreeMap, HashMap},
    env,
};

use salvo::cors;
use salvo::cors::Cors;
use salvo::hyper::Method;
use salvo::prelude::*;
use sea_orm::{entity::*, query::*, Database, DatabaseConnection, FromQueryResult};
use serde::{Deserialize, Serialize};

use app_db::{
    craft_types, item_action, item_food, item_food_effect, item_with_amount, items, prelude::*,
    recipe_level_tables, recipes,
};

type Result<T> = std::result::Result<T, StatusError>;

#[derive(Deserialize, Debug)]
struct ServerConfig {
    lang: HashMap<String, LanguageConfig>,
}

#[derive(Deserialize, Debug)]
struct LanguageConfig {
    database: String,
}

#[derive(Clone)]
struct AppState {
    connections: HashMap<String, DatabaseConnection>,
}

#[tokio::main]
async fn main() {
    println!("hello, world");

    // parse configs
    let config_str = std::fs::read_to_string("config.toml").unwrap();
    let config: ServerConfig = toml::from_str(&config_str).unwrap();

    // get env vars
    dotenvy::dotenv().unwrap();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // create post table if not exists
    let mut connections = HashMap::new();
    for (lang, lang_cfg) in config.lang.into_iter() {
        let url = format!("{db_url}/{}", lang_cfg.database);
        let conn = Database::connect(&url).await.unwrap();
        connections.insert(lang, conn);
    }
    let state = AppState { connections };

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
        .hoop(affix_state::inject(state))
        .push(
            Router::with_path("{lang}")
                .push(Router::with_path("recipe_level_table").get(recipe_level_table))
                .push(Router::with_path("recipe_table").get(recipe_table))
                .push(Router::with_path("recipe_info").get(recipe_info))
                .push(Router::with_path("recipes_ingredientions").get(recipes_ingredientions))
                .push(Router::with_path("recipe_collectability").get(recipe_collectability))
                .push(Router::with_path("item_info").get(item_info))
                .push(Router::with_path("craft_type").get(craft_type))
                .push(Router::with_path("medicine_table").get(medicine_table))
                .push(Router::with_path("meals_table").get(meals_table)),
        );
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
    item_amount: i32,
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
    let lang = req.params().get::<str>("lang").unwrap();
    let conn = state
        .connections
        .get(lang)
        .ok_or_else(|| StatusError::bad_request())?;
    let rlv = req
        .query::<u32>("rlv")
        .ok_or_else(|| StatusError::bad_request())?;
    let Some(rt) = RecipeLevelTables::find_by_id(rlv)
        .one(conn)
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
    let lang = req.params().get::<str>("lang").unwrap();
    let conn = state
        .connections
        .get(lang)
        .ok_or_else(|| StatusError::bad_request())?;
    let page_id = req
        .query::<u64>("page_id")
        .ok_or_else(|| StatusError::bad_request().detail("Need 'page_id'"))?;
    let search_name = req
        .query::<String>("search_name")
        .ok_or_else(|| StatusError::bad_request().detail("Need 'search_name'"))?;

    let mut query = Recipes::find()
        .join(JoinType::InnerJoin, recipes::Relation::CraftTypes.def())
        .join(JoinType::InnerJoin, recipes::Relation::ItemWithAmount.def())
        .join(
            JoinType::InnerJoin,
            recipes::Relation::RecipeLevelTables.def(),
        )
        .join(JoinType::InnerJoin, item_with_amount::Relation::Items.def())
        .select_only()
        .filter(items::Column::Name.like(&search_name));
    if let Some(rlv) = req.query::<u32>("rlv") {
        query = query.filter(recipes::Column::RecipeLevelId.eq(rlv))
    }
    if let Some(craft_type_id) = req.query::<u32>("craft_type_id") {
        query = query.filter(recipes::Column::CraftTypeId.eq(craft_type_id))
    }
    if let Some(job_level_min) = req.query::<u32>("job_level_min") {
        query = query.filter(recipe_level_tables::Column::ClassJobLevel.gte(job_level_min))
    }
    if let Some(job_level_max) = req.query::<u32>("job_level_max") {
        query = query.filter(recipe_level_tables::Column::ClassJobLevel.lte(job_level_max))
    }
    let query = query
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
        .order_by(recipes::Column::Id, Order::Asc);
    let paginate = query.into_model::<RecipeInfo>().paginate(conn, 200);

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

#[handler]
async fn craft_type(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let state = depot
        .obtain::<AppState>()
        .map_err(|_| StatusError::internal_server_error())?;
    let lang = req.params().get::<str>("lang").unwrap();
    let conn = state
        .connections
        .get(lang)
        .ok_or_else(|| StatusError::bad_request())?;
    let query = CraftTypes::find()
        .column_as(craft_types::Column::Id, "id")
        .column_as(craft_types::Column::Name, "name")
        .order_by(craft_types::Column::Id, Order::Asc);
    let result = query
        .into_model::<craft_types::Model>()
        .all(conn)
        .await
        .map_err(|err| {
            println!("Failed to get craft type list: {err}");
            StatusError::internal_server_error().detail("Failed to get craft type list")
        })?;
    res.render(Json(result));
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
    let lang = req.params().get::<str>("lang").unwrap();
    let conn = state
        .connections
        .get(lang)
        .ok_or_else(|| StatusError::bad_request())?;
    let recipe_id = req
        .query::<u32>("recipe_id")
        .ok_or_else(|| StatusError::bad_request())?;

    let mut needs = BTreeMap::new();
    let ing = ItemWithAmount::find()
        .filter(item_with_amount::Column::RecipeId.eq(recipe_id))
        .all(conn)
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

#[handler]
async fn recipe_info(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let state = depot
        .obtain::<AppState>()
        .map_err(|_| StatusError::internal_server_error().detail("Obtain AppState error"))?;
    let lang = req.params().get::<str>("lang").unwrap();
    let conn = state
        .connections
        .get(lang)
        .ok_or_else(|| StatusError::bad_request())?;
    let recipe_id = req
        .query::<u32>("recipe_id")
        .ok_or_else(|| StatusError::bad_request().detail("Need 'recipe_id'"))?;
    let result = Recipes::find_by_id(recipe_id)
        .join(JoinType::InnerJoin, recipes::Relation::CraftTypes.def())
        .join(JoinType::InnerJoin, recipes::Relation::ItemWithAmount.def())
        .join(
            JoinType::InnerJoin,
            recipes::Relation::RecipeLevelTables.def(),
        )
        .join(JoinType::InnerJoin, item_with_amount::Relation::Items.def())
        .select_only()
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
        .into_model::<RecipeInfo>()
        .one(conn)
        .await
        .map_err(|e| {
            println!("recipe_info error: {e:?}");
            StatusError::internal_server_error()
        })?
        .ok_or_else(|| StatusError::bad_request().detail("Recipe not found"))?;
    res.render(Json(result));
    Ok(())
}

#[handler]
async fn recipe_collectability(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<()> {
    let state = depot
        .obtain::<AppState>()
        .map_err(|_| StatusError::internal_server_error().detail("Obtain AppState error"))?;
    let lang = req.params().get::<str>("lang").unwrap();
    let conn = state
        .connections
        .get(lang)
        .ok_or_else(|| StatusError::bad_request())?;
    let recipe_id = req
        .query::<u32>("recipe_id")
        .ok_or_else(|| StatusError::bad_request().detail("Need 'recipe_id'"))?;
    let result = CollectablesShopRefine::find()
        .reverse_join(Recipes)
        .filter(recipes::Column::Id.eq(recipe_id))
        .one(conn)
        .await
        .map_err(|_| StatusError::internal_server_error())?;
    res.render(Json(result));
    Ok(())
}

// item_id: i32
#[handler]
async fn item_info(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let state = depot
        .obtain::<AppState>()
        .map_err(|_| StatusError::internal_server_error())?;
    let lang = req.params().get::<str>("lang").unwrap();
    let conn = state
        .connections
        .get(lang)
        .ok_or_else(|| StatusError::bad_request())?;
    let item_id = req
        .query::<u32>("item_id")
        .ok_or_else(|| StatusError::bad_request())?;
    let result = Items::find_by_id(item_id)
        .one(conn)
        .await
        .map_err(|_| StatusError::internal_server_error())?
        .ok_or_else(|| StatusError::bad_gateway())?;
    res.render(Json(result));
    Ok(())
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

#[handler]
async fn medicine_table(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let state = depot
        .obtain::<AppState>()
        .map_err(|_| StatusError::internal_server_error())?;
    let lang = req.params().get::<str>("lang").unwrap();
    let conn = state
        .connections
        .get(lang)
        .ok_or_else(|| StatusError::bad_request())?;
    res.render(Json(query_enhancers(conn, MEDICINE_SEARCH_ID).await?));
    Ok(())
}

#[handler]
async fn meals_table(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let state = depot
        .obtain::<AppState>()
        .map_err(|_| StatusError::internal_server_error())?;
    let lang = req.params().get::<str>("lang").unwrap();
    let conn = state
        .connections
        .get(lang)
        .ok_or_else(|| StatusError::bad_request())?;
    res.render(Json(query_enhancers(conn, MEALS_SEARCH_ID).await?));
    Ok(())
}

async fn query_enhancers(conn: &DatabaseConnection, search_id: u32) -> Result<Vec<Enhancer>> {
    let crafting_item_food = ItemFood::find()
        .join(
            JoinType::InnerJoin,
            item_food::Relation::ItemFoodEffect.def(),
        )
        .select_with(ItemFoodEffect)
        .filter(item_food_effect::Column::BaseParam.is_in([11, 70, 71]))
        .all(conn)
        .await
        .map_err(|err| StatusError::internal_server_error().detail(err.to_string()))?
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
        .map_err(|err| StatusError::internal_server_error().detail(err.to_string()))?;
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
