use std::{error::Error, fmt::Display, path::Path};

use dialoguer::{Confirm, Input, Select};
use ironworks::{
    Ironworks,
    excel::{Excel, Language},
    sqpack::{Install, SqPack},
};
use sea_orm::{ActiveValue, ConnectOptions, ConnectionTrait, Database, EntityTrait};

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
        schema.create_table_from_entity(app_db::item_food_effect::Entity),
        schema.create_table_from_entity(app_db::item_food::Entity),
        schema.create_table_from_entity(app_db::collectables_shop_refine::Entity),
        schema.create_table_from_entity(app_db::item_with_amount::Entity),
        schema.create_table_from_entity(app_db::recipes::Entity),
        schema.create_table_from_entity(app_db::wks_mission_recipe::Entity),
        schema.create_table_from_entity(app_db::wks_mission_to_do::Entity),
        schema.create_table_from_entity(app_db::wks_mission_unit::Entity),
    ];
    for mut stat in stats {
        db.execute(backend.build(stat.if_not_exists())).await?;
    }
    Ok(())
}

macro_rules! insert_excel_sheet {
    ($($db_mod:ident)::+, $metadata:expr, $excel:expr, $db:expr) => {{
        use itertools::Itertools;
        for batch in &$excel
                .sheet($metadata)?
                .into_iter()
                .filter_map(|x| x.inspect_err(|e| println!("{e}")).ok())
                .map($($db_mod)::+::ActiveModel::from)
                .chunks(1000)
        {
            $($db_mod)::+::Entity::insert_many(batch)
                .exec($db)
                .await
                .inspect_err(|e| println!("Failed to insert many: {e}"))
                .unwrap();
        }
    }};
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
            .default(r"C:\Program Files (x86)\上海数龙科技有限公司\最终幻想XIV".to_string())
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

    insert_excel_sheet!(
        app_db::item_ui_categories,
        metadata::ItemUICategory,
        excel,
        &db
    );
    insert_excel_sheet!(
        app_db::item_search_categories,
        metadata::ItemSearchCategory,
        excel,
        &db
    );
    insert_excel_sheet!(app_db::craft_types, metadata::CraftType, excel, &db);
    insert_excel_sheet!(app_db::item_action, metadata::ItemAction, excel, &db);
    insert_excel_sheet!(app_db::items, metadata::Item, excel, &db);
    insert_excel_sheet!(
        app_db::recipe_level_tables,
        metadata::RecipeLevelTable,
        excel,
        &db
    );
    insert_excel_sheet!(app_db::item_food, metadata::ItemFood, excel, &db);
    insert_excel_sheet!(
        app_db::collectables_shop_refine,
        metadata::CollectablesShopRefine,
        excel,
        &db
    );
    // insert_excel_sheet!(app_db::recipes, metadata::Recipe, excel, &db);
    use itertools::Itertools;
    for (batch_id, batch) in excel
        .sheet(metadata::Recipe)?
        .into_iter()
        .filter_map(|x| x.inspect_err(|e| println!("{e}")).ok())
        .chunks(1000)
        .into_iter()
        .enumerate()
    {
        println!("Start push recipe batch {batch_id}");
        let ((recipe_results, mut recipes), ingredients): ((Vec<_>, Vec<_>), Vec<_>) = batch
            .map(|row| {
                let recipe_result =
                    row.item_result
                        .map(|item_result| app_db::item_with_amount::ActiveModel {
                            id: ActiveValue::NotSet,
                            ingredient_id: ActiveValue::Set(item_result.item_id),
                            amount: ActiveValue::Set(item_result.amount),
                            recipe_id: ActiveValue::Set(None),
                        });
                let recipe = app_db::recipes::ActiveModel {
                    id: ActiveValue::Set(row.id),
                    number: ActiveValue::Set(row.number),
                    craft_type_id: ActiveValue::Set(row.craft_type_id),
                    recipe_level_id: ActiveValue::Set(row.recipe_level_id),
                    item_result_id: ActiveValue::Set(None),
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
                ((recipe_result, recipe), (row.id, row.ingredients))
            })
            .unzip();

        let (idx, models): (Vec<_>, Vec<_>) = recipe_results
            .into_iter()
            .enumerate()
            .filter_map(|(i, opt_val)| opt_val.map(|val| (i, val)))
            .unzip();
        let keys = app_db::item_with_amount::Entity::insert_many(models)
            .exec_with_returning_keys(&db)
            .await?;
        for (key, i) in keys.into_iter().zip(idx) {
            recipes[i].item_result_id = ActiveValue::Set(Some(key));
        }
        app_db::recipes::Entity::insert_many(recipes)
            .exec(&db)
            .await?;

        app_db::item_with_amount::Entity::insert_many(ingredients.into_iter().flat_map(
            |(recipe_id, item_result)| {
                item_result.into_iter().flatten().map(move |row| {
                    app_db::item_with_amount::ActiveModel {
                        id: ActiveValue::NotSet,
                        ingredient_id: ActiveValue::Set(row.item_id),
                        amount: ActiveValue::Set(row.amount),
                        recipe_id: ActiveValue::Set(Some(recipe_id)),
                    }
                })
            },
        ))
        .exec(&db)
        .await?;
    }

    // app_db::item_with_amount::Entity::
    insert_excel_sheet!(
        app_db::wks_mission_recipe,
        metadata::WKSMissionRecipe,
        excel,
        &db
    );
    insert_excel_sheet!(
        app_db::wks_mission_to_do,
        metadata::WKSMissionToDo,
        excel,
        &db
    );
    insert_excel_sheet!(
        app_db::wks_mission_unit,
        metadata::WKSMissionUnit,
        excel,
        &db
    );

    println!("Success!");
    Ok(())
}
