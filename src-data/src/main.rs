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

use std::{error::Error, fmt::Display, path::Path};

use clap::Parser;
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

#[derive(Clone, Copy, Debug)]
struct LanguageOption(Language);

impl Display for LanguageOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

fn language_value_parser(s: &str) -> Result<Language, String> {
    let input_lower = s.to_ascii_lowercase();
    Language::iter()
        .filter(|l| !matches!(l, Language::None | Language::Unknown(_)))
        .find(|l| format!("{l:?}").to_ascii_lowercase() == input_lower)
        .ok_or_else(|| format!("unknown language: {s}"))
}

#[derive(Parser)]
#[command(name = "app-data")]
#[command(about = "Import FFXIV game data into a database", long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        help = "Path to FFXIV installation. If not provided, auto-search will be used (interactive fallback)"
    )]
    install_path: Option<String>,

    #[arg(short, long = "lang", value_parser = language_value_parser)]
    lang: Option<Language>,

    #[arg(long = "db")]
    db_url: Option<String>,

    #[arg(short = 's', long, default_value_t = 200)]
    batch_size: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("hello, world");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    let args = Args::parse();

    let install = if let Some(ref path) = args.install_path {
        Some(Install::at(Path::new(path)))
    } else {
        let install = Install::search();
        let found_path = if let Some(install) = &install {
            println!("Install found: {:#?}", install);
            Confirm::new()
                .with_prompt("Found FFXIV installation, continue?")
                .interact()
                .unwrap_or(false)
        } else {
            false
        };
        if found_path {
            install
        } else {
            println!("Install not found");
            let path = Input::<String>::new()
                .with_prompt("Input FFXIV installation path?")
                .default(r"C:\WeGameApps\rail_apps\ffxiv(2000340)".to_string())
                .interact()
                .unwrap();
            Some(Install::at(Path::new(&path)))
        }
    };
    let sqpack = SqPack::new(install.unwrap());
    let ironworks = Ironworks::new().with_resource(sqpack);

    let language = if let Some(lang) = args.lang {
        lang
    } else {
        let variants: Vec<LanguageOption> = Language::iter()
            .filter(|l| !matches!(l, Language::None | Language::Unknown(_)))
            .map(LanguageOption)
            .collect();
        let selection = Select::new()
            .items(&variants)
            .with_prompt("Select a language")
            .default(4)
            .interact()
            .unwrap();
        variants[selection].0
    };

    let excel = Excel::new(ironworks).with_default_language(language);

    let db_url = if let Some(ref url) = args.db_url {
        url.clone()
    } else {
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
        Input::<String>::new()
            .with_prompt("Input Database URL")
            .default(db_url_defaults[db_type].to_string())
            .interact()
            .unwrap()
    };
    let mut db_opt = ConnectOptions::new(db_url);
    db_opt.sqlx_logging(false);
    let db = Database::connect(db_opt).await?;

    // Create tables
    create_tables(&db).await?;

    insert_sheets(&excel, &db, args.batch_size).await?;

    println!("Success!");
    Ok(())
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

#[rustfmt::skip]
async fn insert_sheets(excel: &Excel, db: &DatabaseConnection, batch_size: usize) -> Result<(), Box<dyn Error>> {
    use app_db::prelude::*;
    use metadata as md;
    
    insert_excel_sheet(ItemUiCategories, excel, db, md::ItemUICategory, batch_size).await?;
    insert_excel_sheet(ItemSearchCategories, excel, db, md::ItemSearchCategory, batch_size).await?;
    insert_excel_sheet(CraftTypes, excel, db, md::CraftType, batch_size).await?;
    insert_excel_sheet(ItemAction, excel, db, md::ItemAction, batch_size).await?;
    insert_excel_sheet(Items, excel, db, md::Item, batch_size).await?;
    insert_excel_sheet(RecipeLevelTables, excel, db, md::RecipeLevelTable, batch_size).await?;
    // insert_excel_sheet(ItemFood, &excel, &db, md::ItemFood).await?;
    insert_item_food(&excel, &db, batch_size).await?;
    insert_excel_sheet(CollectablesShopRefine, excel, db, md::CollectablesShopRefine, batch_size).await?;
    insert_excel_sheet(Recipes, excel, db, md::Recipe, batch_size).await?;
    insert_excel_sheet(WksMissionRecipe, excel, db, md::WKSMissionRecipe, batch_size).await?;
    insert_excel_sheet(WksMissionToDo, excel, db, md::WKSMissionToDo, batch_size).await?;
    insert_excel_sheet(WksMissionUnit, excel, db, md::WKSMissionUnit, batch_size).await?;
    Ok(())
}

async fn insert_excel_sheet<E>(
    entity: E,
    excel: &Excel,
    db: &DatabaseConnection,
    metadata: impl SheetMetadata<Row = impl Into<E::ActiveModel>>,
    batch_size: usize,
) -> Result<(), Box<dyn Error>>
where
    E: EntityTrait,
{
    for (batch_id, batch) in excel
        .sheet(metadata)?
        .into_iter()
        .filter_map(|x| x.inspect_err(|e| println!("{e}")).ok())
        .map(|x| x.into())
        .chunks(batch_size)
        .into_iter()
        .enumerate()
    {
        println!(
            "Pushing {} batch {}",
            entity.table_name(),
            batch_id * batch_size
        );
        E::insert_many(batch).exec(db).await?;
    }
    Ok(())
}

async fn insert_item_food(
    excel: &Excel,
    db: &DatabaseConnection,
    batch_size: usize,
) -> Result<(), Box<dyn Error>> {
    for (batch_id, batch) in excel
        .sheet(metadata::ItemFood)?
        .into_iter()
        .filter_map(|x| x.inspect_err(|e| println!("{e}")).ok())
        .chunks(batch_size)
        .into_iter()
        .enumerate()
    {
        println!("Pushing ItemFood batch {}", batch_id * batch_size);
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
