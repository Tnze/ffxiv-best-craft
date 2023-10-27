import { Item, ItemWithAmount, RecipeInfo, RecipeLevel } from "../../Craft";

export interface DataSource {
    sourceType: DataSourceType
    recipeTable(page: number, searchName: string): Promise<RecipesSourceResult>
    recipesIngredients(recipeId: number): Promise<ItemWithAmount[]>
    recipeLevelTable(rlv: number): Promise<RecipeLevel>
    recipeInfo(recipeId: number): Promise<RecipeInfo>
    itemInfo(id: number): Promise<Item>
}

export enum DataSourceType {
    Realtime, SingleShot
}

export interface RecipesSourceResult {
    recipes: RecipeInfo[],
    totalPages: number
}