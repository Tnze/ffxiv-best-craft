use ironworks::excel::SheetMetadata;

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
            item_action_id: Some(row.field(15)?.into_u8()? as u32).filter(|x| *x != 0),
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

impl SheetMetadata for ItemFood {
    fn name(&self) -> String {
        String::from("ItemFood")
    }

    type Row = app_db::item_food::Model;
    type Error = Error;
    fn populate_row(&self, row: ironworks::excel::Row) -> Result<Self::Row, Self::Error> {
        Ok(Self::Row { id: row.row_id() })
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

pub struct RecipeRow {
    pub id: u32,
    pub number: i32,
    pub craft_type_id: u32,
    pub recipe_level_id: u32,
    pub item_result: Option<ItemWithAmount>,
    pub ingredients: [Option<ItemWithAmount>; 8],
    pub material_quality_factor: u8,
    pub difficulty_factor: u16,
    pub quality_factor: u16,
    pub durability_factor: u16,
    pub required_quality: u32,
    pub required_craftsmanship: u16,
    pub required_control: u16,
    pub can_hq: bool,
    pub is_expert: bool,
    pub collectables_metadata_key: u32,
    pub collectables_metadata: Option<u32>,
    pub recipe_notebook_list: u32,
}

pub struct ItemWithAmount {
    pub item_id: u32,
    pub amount: u8,
}

impl ItemWithAmount {
    fn from(item_id: u32, amount: u8) -> Option<Self> {
        if item_id == 0 || item_id == 0xFFFF_FFFF {
            return None;
        }
        Some(Self { item_id, amount })
    }
}

impl SheetMetadata for Recipe {
    fn name(&self) -> String {
        String::from("Recipe")
    }

    type Row = RecipeRow;
    type Error = Error;
    fn populate_row(&self, row: ironworks::excel::Row) -> Result<Self::Row, Self::Error> {
        let collectables_metadata_key = row.field(44)?.into_u8()? as u32;
        let collectables_metadata = match collectables_metadata_key {
            1 => Some(row.field(45)?.into_u16()? as u32),
            _ => None,
        };
        Ok(RecipeRow {
            id: row.row_id(),
            number: row.field(0)?.into_i32()?,
            craft_type_id: row.field(1)?.into_i32()? as u32,
            recipe_level_id: row.field(2)?.into_u16()? as u32,
            item_result: ItemWithAmount::from(
                row.field(4)?.into_i32()? as u32,
                row.field(5)?.into_u8()?,
            ),
            ingredients: [
                ItemWithAmount::from(row.field(6)?.into_i32()? as u32, row.field(7)?.into_u8()?),
                ItemWithAmount::from(row.field(8)?.into_i32()? as u32, row.field(9)?.into_u8()?),
                ItemWithAmount::from(row.field(10)?.into_i32()? as u32, row.field(11)?.into_u8()?),
                ItemWithAmount::from(row.field(12)?.into_i32()? as u32, row.field(13)?.into_u8()?),
                ItemWithAmount::from(row.field(14)?.into_i32()? as u32, row.field(15)?.into_u8()?),
                ItemWithAmount::from(row.field(16)?.into_i32()? as u32, row.field(17)?.into_u8()?),
                ItemWithAmount::from(row.field(18)?.into_i32()? as u32, row.field(19)?.into_u8()?),
                ItemWithAmount::from(row.field(20)?.into_i32()? as u32, row.field(21)?.into_u8()?),
            ],
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
