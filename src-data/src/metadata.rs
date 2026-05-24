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

use ironworks::excel::SheetMetadata;
use sea_orm::ActiveValue;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("ironwork error: {0}")]
    IronworkError(#[from] ironworks::Error),

    #[error("convert SeString error: {0}")]
    SeStringConvertError(#[from] ironworks::sestring::Error),

    #[error("Cannot convert field type: {field:?}")]
    DataTypeError { field: ironworks::excel::Field },
}

impl From<ironworks::excel::Field> for Error {
    fn from(field: ironworks::excel::Field) -> Self {
        Self::DataTypeError { field }
    }
}

pub struct ItemUICategory;
pub struct ItemSearchCategory;
pub struct CraftType;
pub struct ItemAction;
pub struct Item;
pub struct RecipeLevelTable;
pub struct ItemFood;
pub struct CollectablesShopRefine;
pub struct Recipe;
pub struct WKSMissionRecipe;
pub struct WKSMissionToDo;
pub struct WKSMissionUnit;

impl SheetMetadata for ItemUICategory {
    fn name(&self) -> String {
        String::from("ItemUICategory")
    }

    type Row = app_db::item_ui_categories::Model;
    type Error = Error;
    fn populate_row(&self, row: ironworks::excel::Row) -> Result<Self::Row, Self::Error> {
        Ok(Self::Row {
            id: row.row_id(),
            name: row.field(0)?.into_string()?.format()?,
        })
    }
}

impl SheetMetadata for ItemSearchCategory {
    fn name(&self) -> String {
        String::from("ItemSearchCategory")
    }

    type Row = app_db::item_search_categories::Model;
    type Error = Error;
    fn populate_row(&self, row: ironworks::excel::Row) -> Result<Self::Row, Self::Error> {
        Ok(Self::Row {
            id: row.row_id(),
            name: row.field(0)?.into_string()?.format()?,
        })
    }
}

impl SheetMetadata for CraftType {
    fn name(&self) -> String {
        String::from("CraftType")
    }

    type Row = app_db::craft_types::Model;
    type Error = Error;
    fn populate_row(&self, row: ironworks::excel::Row) -> Result<Self::Row, Self::Error> {
        Ok(Self::Row {
            id: row.row_id(),
            name: row.field(2)?.into_string()?.format()?,
        })
    }
}

impl SheetMetadata for ItemAction {
    fn name(&self) -> String {
        String::from("ItemAction")
    }

    type Row = app_db::item_action::Model;
    type Error = Error;
    fn populate_row(&self, row: ironworks::excel::Row) -> Result<Self::Row, Self::Error> {
        Ok(Self::Row {
            id: row.row_id(),
            r#type: row.field(4 /* Action */)?.into_u16()?,
            data1: row.field(5)?.into_u16()?,
            data2: row.field(6)?.into_u16()?,
            data3: row.field(7)?.into_u16()?,
            data4: row.field(8)?.into_u16()?,
            data5: row.field(9)?.into_u16()?,
            data6: row.field(10)?.into_u16()?,
            data7: row.field(11)?.into_u16()?,
            data8: row.field(12)?.into_u16()?,
            data9: row.field(13)?.into_u16()?,
            data_hq1: row.field(14)?.into_u16()?,
            data_hq2: row.field(15)?.into_u16()?,
            data_hq3: row.field(16)?.into_u16()?,
            data_hq4: row.field(17)?.into_u16()?,
            data_hq5: row.field(18)?.into_u16()?,
            data_hq6: row.field(19)?.into_u16()?,
            data_hq7: row.field(20)?.into_u16()?,
            data_hq8: row.field(21)?.into_u16()?,
            data_hq9: row.field(22)?.into_u16()?,
        })
    }
}

impl SheetMetadata for Item {
    fn name(&self) -> String {
        String::from("Item")
    }

    type Row = app_db::items::Model;
    type Error = Error;
    fn populate_row(&self, row: ironworks::excel::Row) -> Result<Self::Row, Self::Error> {
        Ok(Self::Row {
            id: row.row_id(),
            name: row.field(9)?.into_string()?.format()?,
            level: row.field(11)?.into_u16()? as u32,
            can_be_hq: row.field(27)?.into_bool()?,
            item_ui_category_id: Some(row.field(15)?.into_u8()? as u32).filter(|x| *x != 0),
            item_search_category_id: Some(row.field(16)?.into_u8()? as u32).filter(|x| *x != 0),
            item_action_id: Some(row.field(30)?.into_u16()? as u32).filter(|x| *x != 0),
            is_collectable: row.field(37)?.into_bool()?,
            always_collectable: row.field(38)?.into_bool()?,
        })
    }
}

impl SheetMetadata for RecipeLevelTable {
    fn name(&self) -> String {
        String::from("RecipeLevelTable")
    }

    type Row = app_db::recipe_level_tables::Model;
    type Error = Error;
    fn populate_row(&self, row: ironworks::excel::Row) -> Result<Self::Row, Self::Error> {
        Ok(Self::Row {
            id: row.row_id(),
            class_job_level: row.field(0)?.into_u8()?,
            suggested_craftsmanship: row.field(2)?.into_u16()?,
            difficulty: row.field(3)?.into_u16()?,
            quality: row.field(4)?.into_u32()?,
            progress_divider: row.field(5)?.into_u8()?,
            quality_divider: row.field(6)?.into_u8()?,
            progress_modifier: row.field(7)?.into_u8()?,
            quality_modifier: row.field(8)?.into_u8()?,
            durability: row.field(9)?.into_u16()?,
            conditions_flag: row.field(10)?.into_u16()?,
        })
    }
}

pub struct ItemFoodRow {
    pub id: u32,
    pub effects: [ItemFoodEffect; 3],
}

pub struct ItemFoodEffect {
    pub base_param: u8,
    pub value: i8,
    pub max: i16,
    pub value_hq: i8,
    pub max_hq: i16,
}

impl ItemFoodEffect {
    pub fn to_model(&self, item_food_id: u32) -> app_db::item_food_effect::ActiveModel {
        app_db::item_food_effect::ActiveModel {
            id: ActiveValue::NotSet,
            base_param: ActiveValue::Set(self.base_param),
            value: ActiveValue::Set(self.value),
            max: ActiveValue::Set(self.max),
            value_hq: ActiveValue::Set(self.value_hq),
            max_hq: ActiveValue::Set(self.max_hq),
            item_food_id: ActiveValue::Set(item_food_id),
        }
    }
}

impl SheetMetadata for ItemFood {
    fn name(&self) -> String {
        String::from("ItemFood")
    }

    type Row = ItemFoodRow;
    type Error = Error;
    fn populate_row(&self, row: ironworks::excel::Row) -> Result<Self::Row, Self::Error> {
        Ok(Self::Row {
            id: row.row_id(),
            effects: [
                ItemFoodEffect {
                    base_param: row.field(1)?.into_u8()?,
                    value: row.field(3)?.into_i8()?,
                    max: row.field(4)?.into_i16()?,
                    value_hq: row.field(5)?.into_i8()?,
                    max_hq: row.field(6)?.into_i16()?,
                },
                ItemFoodEffect {
                    base_param: row.field(7)?.into_u8()?,
                    value: row.field(9)?.into_i8()?,
                    max: row.field(10)?.into_i16()?,
                    value_hq: row.field(11)?.into_i8()?,
                    max_hq: row.field(12)?.into_i16()?,
                },
                ItemFoodEffect {
                    base_param: row.field(13)?.into_u8()?,
                    value: row.field(15)?.into_i8()?,
                    max: row.field(16)?.into_i16()?,
                    value_hq: row.field(17)?.into_i8()?,
                    max_hq: row.field(18)?.into_i16()?,
                },
            ],
        })
    }
}

impl SheetMetadata for CollectablesShopRefine {
    fn name(&self) -> String {
        String::from("CollectablesShopRefine")
    }

    type Row = app_db::collectables_shop_refine::Model;
    type Error = Error;
    fn populate_row(&self, row: ironworks::excel::Row) -> Result<Self::Row, Self::Error> {
        Ok(Self::Row {
            id: row.row_id(),
            low_collectability: row.field(0)?.into_u16()?,
            mid_collectability: row.field(1)?.into_u16()?,
            high_collectability: row.field(2)?.into_u16()?,
        })
    }
}

fn read_ingredient(
    row: &ironworks::excel::Row,
    id_field: usize,
    amt_field: usize,
) -> Result<(u32, u8), Error> {
    let item_id = row.field(id_field)?.into_i32()? as u32;
    let amount = row.field(amt_field)?.into_u8()?;
    if item_id == 0 || item_id == 0xFFFF_FFFF {
        Ok((0, 0))
    } else {
        Ok((item_id, amount))
    }
}

impl SheetMetadata for Recipe {
    fn name(&self) -> String {
        String::from("Recipe")
    }

    type Row = app_db::recipes::Model;
    type Error = Error;
    fn populate_row(&self, row: ironworks::excel::Row) -> Result<Self::Row, Self::Error> {
        let collectables_metadata_key = row.field(44)?.into_u8()? as u32;
        let collectables_metadata = match collectables_metadata_key {
            1 => Some(row.field(45)?.into_u16()? as u32),
            _ => None,
        };
        let (item_result_id, item_result_amount) = read_ingredient(&row, 4, 5)?;
        let (ingredient0, ingredient_amount0) = read_ingredient(&row, 6, 7)?;
        let (ingredient1, ingredient_amount1) = read_ingredient(&row, 8, 9)?;
        let (ingredient2, ingredient_amount2) = read_ingredient(&row, 10, 11)?;
        let (ingredient3, ingredient_amount3) = read_ingredient(&row, 12, 13)?;
        let (ingredient4, ingredient_amount4) = read_ingredient(&row, 14, 15)?;
        let (ingredient5, ingredient_amount5) = read_ingredient(&row, 16, 17)?;
        let (ingredient6, ingredient_amount6) = read_ingredient(&row, 18, 19)?;
        let (ingredient7, ingredient_amount7) = read_ingredient(&row, 20, 21)?;
        Ok(app_db::recipes::Model {
            id: row.row_id(),
            number: row.field(0)?.into_i32()?,
            craft_type_id: row.field(1)?.into_i32()? as u32,
            recipe_level_id: row.field(2)?.into_u16()? as u32,
            item_result_id,
            item_result_amount,
            material_quality_factor: row.field(25)?.into_u8()?,
            difficulty_factor: row.field(26)?.into_u16()?,
            quality_factor: row.field(27)?.into_u16()?,
            durability_factor: row.field(28)?.into_u16()?,
            required_quality: row.field(29)?.into_u32()?,
            required_craftsmanship: row.field(30)?.into_u16()?,
            required_control: row.field(31)?.into_u16()?,
            can_hq: row.field(37)?.into_bool()?,
            is_expert: row.field(43)?.into_bool()?,
            collectables_metadata_key,
            collectables_metadata,
            recipe_notebook_list: row.field(22)?.into_u16()? as u32,
            ingredient0,
            ingredient_amount0,
            ingredient1,
            ingredient_amount1,
            ingredient2,
            ingredient_amount2,
            ingredient3,
            ingredient_amount3,
            ingredient4,
            ingredient_amount4,
            ingredient5,
            ingredient_amount5,
            ingredient6,
            ingredient_amount6,
            ingredient7,
            ingredient_amount7,
        })
    }
}

impl SheetMetadata for WKSMissionRecipe {
    fn name(&self) -> String {
        String::from("WKSMissionRecipe")
    }

    type Row = app_db::wks_mission_recipe::Model;
    type Error = Error;
    fn populate_row(&self, row: ironworks::excel::Row) -> Result<Self::Row, Self::Error> {
        Ok(Self::Row {
            id: row.row_id(),
            recipe0_id: Some(row.field(0)?.into_u32()?).filter(|x| *x != 0),
            recipe1_id: Some(row.field(1)?.into_u32()?).filter(|x| *x != 0),
            recipe2_id: Some(row.field(2)?.into_u32()?).filter(|x| *x != 0),
            recipe3_id: Some(row.field(3)?.into_u32()?).filter(|x| *x != 0),
            recipe4_id: Some(row.field(4)?.into_u32()?).filter(|x| *x != 0),
            is_expert: row.field(5)?.into_bool()?,
        })
    }
}

impl SheetMetadata for WKSMissionToDo {
    fn name(&self) -> String {
        String::from("WKSMissionToDo")
    }

    type Row = app_db::wks_mission_to_do::Model;
    type Error = Error;
    fn populate_row(&self, row: ironworks::excel::Row) -> Result<Self::Row, Self::Error> {
        Ok(Self::Row {
            id: row.row_id(),
            temporary_action: row.field(0)?.into_u32()?,
            temporary_action_count: row.field(1)?.into_u8()? as u16,
        })
    }
}

impl SheetMetadata for WKSMissionUnit {
    fn name(&self) -> String {
        String::from("WKSMissionUnit")
    }

    type Row = app_db::wks_mission_unit::Model;
    type Error = Error;
    fn populate_row(&self, row: ironworks::excel::Row) -> Result<Self::Row, Self::Error> {
        Ok(Self::Row {
            id: row.row_id(),
            name: row.field(0)?.into_string()?.format()?,
            recipe_id: Some(row.field(21)?.into_u16()? as u32).filter(|x| *x != 0),
            to_do0_id: Some(row.field(13)?.into_u16()? as u32).filter(|x| *x != 0),
            to_do1_id: Some(row.field(14)?.into_u16()? as u32).filter(|x| *x != 0),
            to_do2_id: Some(row.field(15)?.into_u16()? as u32).filter(|x| *x != 0),
        })
    }
}
