import { Item, ItemWithAmount, RecipeInfo, RecipeLevel } from "../../Craft";
import { RecipesSourceResult } from "./source";

export class LocalRecipeSource {
    invoke = import("@tauri-apps/api").then(pkg => pkg.invoke);

    async recipeTable(page: number, searchName: string): Promise<RecipesSourceResult> {
        let [recipes, totalPages]: [RecipeInfo[], number] = await (await this.invoke)("recipe_table", { pageId: page - 1, searchName: "%" + searchName + "%" });
        return { recipes, totalPages }
    }

    async recipesIngredients(recipeId: number): Promise<ItemWithAmount[]> {
        const ings: [number, number][] = await (await this.invoke)("recipes_ingredientions", { recipeId });
        return ings.map(x => ({ ingredient_id: x[0], amount: x[1] }))
    }

    async recipeLevelTable(rlv: number): Promise<RecipeLevel> {
        let result: RecipeLevel = {
            ...await (await this.invoke)("recipe_level_table", { rlv }),
            stars: 0,
        }
        return result
    }

    async recipeInfo(recipeId: number): Promise<RecipeInfo> {
        throw "todo"
    }

    async itemInfo(itemId: number): Promise<Item> {
        const { id, name, level, can_be_hq, category_id } = await (await this.invoke)("item_info", { itemId }) as {
            id: number,
            name: string,
            level: number,
            can_be_hq: number,
            category_id?: number,
        };
        return { id, name, level, can_be_hq: can_be_hq != 0, category_id };
    }
}
