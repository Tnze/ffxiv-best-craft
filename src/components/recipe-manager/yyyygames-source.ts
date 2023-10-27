import { Item, ItemWithAmount, RecipeInfo, RecipeLevel } from "../../Craft";
import { DataSourceType, RecipesSourceResult } from "./source";

export class YYYYGamesSource {
    public sourceType = DataSourceType.SingleShot
    base: string

    constructor(base: string) {
        this.base = base
    }

    async recipeTable(page: number, searchName: string): Promise<RecipesSourceResult> {
        const query = new URLSearchParams({
            "page_id": String(page - 1),
            "search_name": "%" + searchName + "%"
        })
        const url = new URL("recipe_table", this.base).toString() + '?' + query.toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors'
        })
        const { data: recipes, p: totalPages } = await resp.json() as {
            data: RecipeInfo[],
            p: number
        }
        return { recipes, totalPages }
    }

    async recipesIngredients(recipeId: number): Promise<ItemWithAmount[]> {
        const query = new URLSearchParams({ "recipe_id": String(recipeId) })
        const url = new URL("recipes_ingredientions", this.base).toString() + '?' + query.toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors'
        })
        const ings: [number, number][] = await resp.json();
        return ings.map(x => ({ ingredient_id: x[0], amount: x[1] }))
    }

    async recipeLevelTable(rlv: number): Promise<RecipeLevel> {
        const query = new URLSearchParams({ 'rlv': String(rlv) })
        const url = new URL("recipe_level_table", this.base).toString() + '?' + query.toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors'
        })
        let result: RecipeLevel = {
            ...await resp.json(),
            stars: 0,
        }
        return result
    }

    async recipeInfo(recipeId: number): Promise<RecipeInfo> {
        throw "todo"
    }

    async itemInfo(itemId: number): Promise<Item> {
        const query = new URLSearchParams({ 'item_id': String(itemId) })
        const url = new URL("item_info", this.base).toString() + '?' + query.toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors'
        })
        const { id, name, level, can_be_hq, category_id } = await resp.json() as {
            id: number,
            name: string,
            level: number,
            can_be_hq: number,
            category_id?: number,
        }
        return { id, name, level, can_be_hq: can_be_hq != 0, category_id };
    }
}

export const YYYYGamesApiBase = "https://bc-api.yyyy.games/"
