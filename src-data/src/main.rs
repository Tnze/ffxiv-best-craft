use std::{error::Error, fmt::Display, path::Path};

use dialoguer::{Confirm, Input, Select};
use ironworks::{
    Ironworks,
    excel::{Excel, Language, SheetMetadata},
    sqpack::{Install, SqPack},
};
use itertools::Itertools;
use sea_orm::{
    ActiveValue, ConnectOptions, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection,
    EntityTrait, Statement,
};

mod metadata;

struct LanguageOption(Language);

impl Display for LanguageOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

async fn create_tables(db: &sea_orm::DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    let backend = db.get_database_backend();
    let schema = sea_orm::Schema::new(backend);
    let stats = [
        schema.create_table_from_entity(app_db::item_ui_categories::Entity),
        schema.create_table_from_entity(app_db::item_search_categories::Entity),
        schema.create_table_from_entity(app_db::craft_types::Entity),
        schema.create_table_from_entity(app_db::item_action::Entity),
        schema.create_table_from_entity(app_db::items::Entity),
        schema.create_table_from_entity(app_db::recipe_level_tables::Entity),
        schema.create_table_from_entity(app_db::item_food::Entity),
        schema.create_table_from_entity(app_db::item_food_effect::Entity),
        schema.create_table_from_entity(app_db::collectables_shop_refine::Entity),
        schema.create_table_from_entity(app_db::item_with_amount::Entity),
        schema.create_table_from_entity(app_db::recipes::Entity),
        schema.create_table_from_entity(app_db::wks_mission_recipe::Entity),
        schema.create_table_from_entity(app_db::wks_mission_to_do::Entity),
        schema.create_table_from_entity(app_db::wks_mission_unit::Entity),
    ];
    if let DatabaseBackend::MySql = backend {
        db.execute(Statement::from_string(
            backend,
            "SET FOREIGN_KEY_CHECKS = 0;",
        ))
        .await?;
    }

    for mut stat in stats {
        db.execute(backend.build(stat.if_not_exists())).await?;
    }

    if let DatabaseBackend::MySql = backend {
        db.execute(Statement::from_string(
            backend,
            "SET FOREIGN_KEY_CHECKS = 1;",
        ))
        .await?;
    }
    Ok(())
}

async fn insert_excel_sheet<E>(
    entity: E,
    excel: &Excel,
    db: &DatabaseConnection,
    metadata: impl SheetMetadata<Row = impl Into<E::ActiveModel>>,
) -> Result<(), Box<dyn Error>>
where
    E: EntityTrait,
{
    for (batch_id, batch) in excel
        .sheet(metadata)?
        .into_iter()
        .filter_map(|x| x.inspect_err(|e| println!("{e}")).ok())
        .map(|x| x.into())
        .chunks(1000)
        .into_iter()
        .enumerate()
    {
        println!("Pushing {} batch {}", entity.table_name(), batch_id * 1000);
        E::insert_many(batch).exec(db).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("hello, world");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    let mut install = Install::search();
    let mut found_path = false;
    if let Some(install) = &install {
        println!("Install found: {:#?}", install);
        found_path = Confirm::new()
            .with_prompt("Found FFXIV installation, continue?")
            .interact()
            .unwrap();
    }
    if !found_path {
        println!("Install not found");
        let path = Input::<String>::new()
            .with_prompt("Input FFXIV installation path?")
            .default(r"C:\WeGameApps\rail_apps\ffxiv(2000340)".to_string())
            .interact()
            .unwrap();
        install = Some(Install::at(Path::new(&path)))
    }
    let sqpack = SqPack::new(install.unwrap());
    let ironworks = Ironworks::new().with_resource(sqpack);

    let language = Language::from(
        Select::new()
            .items(Language::iter().map(LanguageOption))
            .with_prompt("Select a language")
            .default(5)
            .interact()
            .unwrap() as u8,
    );

    let excel = Excel::new(ironworks).with_default_language(language);

    let db_type = Select::new()
        .items(&["SQLite", "MySQL", "Postgres"])
        .with_prompt("Select a Database")
        .default(0)
        .interact()
        .unwrap();

    let db_url_defaults = [
        "sqlite://xiv.db?mode=rwc",
        "mysql://username:password@host/database",
        "postgres://username:password@host/database?options=--search_path=my_schema",
    ];
    let db_url = Input::<String>::new()
        .with_prompt("Input Database URL")
        .default(db_url_defaults[db_type].to_string())
        .interact()
        .unwrap();
    let mut db_opt = ConnectOptions::new(db_url);
    db_opt.sqlx_logging(false);
    let db = Database::connect(db_opt).await?;

    // Create tables
    create_tables(&db).await?;

    use app_db::prelude::*;
    use metadata as md;
    insert_excel_sheet(ItemUiCategories, &excel, &db, md::ItemUICategory).await?;
    insert_excel_sheet(ItemSearchCategories, &excel, &db, md::ItemSearchCategory).await?;
    insert_excel_sheet(CraftTypes, &excel, &db, md::CraftType).await?;
    insert_excel_sheet(ItemAction, &excel, &db, md::ItemAction).await?;
    insert_excel_sheet(Items, &excel, &db, md::Item).await?;
    insert_excel_sheet(RecipeLevelTables, &excel, &db, md::RecipeLevelTable).await?;
    // insert_excel_sheet(ItemFood, &excel, &db, md::ItemFood).await?;
    insert_item_food(&excel, &db).await?;
    insert_excel_sheet(
        CollectablesShopRefine,
        &excel,
        &db,
        md::CollectablesShopRefine,
    )
    .await?;
    insert_recipes(&excel, &db).await?;
    insert_excel_sheet(WksMissionRecipe, &excel, &db, md::WKSMissionRecipe).await?;
    insert_excel_sheet(WksMissionToDo, &excel, &db, md::WKSMissionToDo).await?;
    insert_excel_sheet(WksMissionUnit, &excel, &db, md::WKSMissionUnit).await?;

    println!("Success!");
    Ok(())
}

async fn insert_item_food(excel: &Excel, db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
    for (batch_id, batch) in excel
        .sheet(metadata::ItemFood)?
        .into_iter()
        .filter_map(|x| x.inspect_err(|e| println!("{e}")).ok())
        .chunks(1000)
        .into_iter()
        .enumerate()
    {
        println!("Pushing ItemFood batch {}", batch_id * 1000);
        let (item_foods, item_food_effects): (Vec<_>, Vec<_>) = batch
            .into_iter()
            .map(|x| {
                (
                    app_db::item_food::ActiveModel {
                        id: ActiveValue::Set(x.id),
                    },
                    x.effects
                        .into_iter()
                        .map(move |effect| effect.to_model(x.id)),
                )
            })
            .unzip();
        app_db::item_food::Entity::insert_many(item_foods)
            .exec(db)
            .await?;
        app_db::item_food_effect::Entity::insert_many(
            item_food_effects
                .into_iter()
                .flatten()
                .filter(|x| !matches!(x.base_param, ActiveValue::Set(0))),
        )
        .do_nothing()
        .exec(db)
        .await?;
    }
    Ok(())
}

async fn insert_recipes(excel: &Excel, db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
    for (i, row) in excel
        .sheet(metadata::Recipe)?
        .into_iter()
        .filter_map(|x| x.inspect_err(|e| println!("{e}")).ok())
        .enumerate()
    {
        if i % 100 == 0 {
            println!("Pushing Recipe {}", i);
        }
        let Some(recipe_result) =
            row.item_result
                .map(|item_result| app_db::item_with_amount::ActiveModel {
                    id: ActiveValue::NotSet,
                    ingredient_id: ActiveValue::Set(item_result.item_id),
                    amount: ActiveValue::Set(item_result.amount),
                    recipe_id: ActiveValue::Set(None),
                })
        else {
            continue;
        };
        let item_result_id = app_db::item_with_amount::Entity::insert(recipe_result)
            .exec(db)
            .await
            .unwrap()
            .last_insert_id;

        let recipe = app_db::recipes::ActiveModel {
            id: ActiveValue::Set(row.id),
            number: ActiveValue::Set(row.number),
            craft_type_id: ActiveValue::Set(row.craft_type_id),
            recipe_level_id: ActiveValue::Set(row.recipe_level_id),
            item_result_id: ActiveValue::Set(item_result_id),
            material_quality_factor: ActiveValue::Set(row.material_quality_factor),
            difficulty_factor: ActiveValue::Set(row.difficulty_factor),
            quality_factor: ActiveValue::Set(row.quality_factor),
            durability_factor: ActiveValue::Set(row.durability_factor),
            required_quality: ActiveValue::Set(row.required_quality),
            required_craftsmanship: ActiveValue::Set(row.required_craftsmanship),
            required_control: ActiveValue::Set(row.required_control),
            can_hq: ActiveValue::Set(row.can_hq),
            is_expert: ActiveValue::Set(row.is_expert),
            collectables_metadata_key: ActiveValue::Set(row.collectables_metadata_key),
            collectables_metadata: ActiveValue::Set(row.collectables_metadata),
            recipe_notebook_list: ActiveValue::Set(row.recipe_notebook_list),
        };

        let recipe_id = app_db::recipes::Entity::insert(recipe)
            .exec(db)
            .await
            .unwrap()
            .last_insert_id;

        let ingredients = row.ingredients.into_iter().flatten().map(move |row| {
            app_db::item_with_amount::ActiveModel {
                id: ActiveValue::NotSet,
                ingredient_id: ActiveValue::Set(row.item_id),
                amount: ActiveValue::Set(row.amount),
                recipe_id: ActiveValue::Set(Some(recipe_id)),
            }
        });
        app_db::item_with_amount::Entity::insert_many(ingredients)
            .do_nothing()
            .exec(db)
            .await
            .unwrap();
    }
    Ok(())
}
