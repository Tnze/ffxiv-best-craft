// This file is part of BestCraft.
// Copyright (C) 2023 Tnze
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

import { Item, ItemWithAmount, RecipeInfo, RecipeLevel } from "../../libs/Craft";
import { DataSourceType, RecipesSourceResult } from "./source";

export class LocalRecipeSource {
    public sourceType = DataSourceType.Realtime
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
