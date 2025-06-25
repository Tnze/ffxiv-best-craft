// This file is part of BestCraft.
// Copyright (C) 2025 Tnze
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

import {
    CollectablesShopRefine,
    Item,
    ItemWithAmount,
    RecipeInfo,
    RecipeLevel,
} from '@/libs/Craft';
import { Enhancer } from '@/libs/Enhancer';
import {
    CraftType,
    DataSourceResult,
    DataSourceType,
    RecipesSourceResult,
} from './source';

export class LocalRecipeSource {
    public sourceType = DataSourceType.Realtime;
    invoke = import('@tauri-apps/api/core').then(pkg => pkg.invoke);

    async recipeTable(
        page: number,
        searchName?: string,
        rlv?: number,
        craftTypeId?: number,
        jobLevelMin?: number,
        jobLevelMax?: number,
    ): Promise<RecipesSourceResult> {
        if (searchName === undefined) {
            searchName = '';
        }
        let [results, totalPages]: [RecipeInfo[], number] = await (
            await this.invoke
        )('recipe_table', {
            pageId: page - 1,
            searchName: '%' + searchName + '%',
            craftTypeId,
            recipeLevel: rlv,
            jobLevelMin,
            jobLevelMax,
        });
        return { results, totalPages };
    }

    async recipesIngredients(recipeId: number): Promise<ItemWithAmount[]> {
        const ings: [number, number][] = await (
            await this.invoke
        )('recipes_ingredientions', { recipeId });
        return ings.map(x => ({ ingredient_id: x[0], amount: x[1] }));
    }

    async recipeCollectableShopRefine(
        recipeId: number,
    ): Promise<CollectablesShopRefine> {
        return await (
            await this.invoke
        )('recipe_collectability', { recipeId });
    }

    async recipeLevelTable(rlv: number): Promise<RecipeLevel> {
        const invoke = await this.invoke;
        let result: RecipeLevel = {
            ...(await invoke('recipe_level_table', { rlv })),
            stars: 0,
        };
        return result;
    }

    async recipeLevelTablebyJobLevel(
        jobLevel: number,
    ): Promise<RecipeLevel | null> {
        const invoke = await this.invoke;
        const result: RecipeLevel = await invoke(
            'recipe_level_table_by_job_level',
            { jobLevel },
        );
        result.stars = 0;
        return result;
    }

    async itemInfo(itemId: number): Promise<Item> {
        const { id, name, level, can_be_hq, category_id } = (await (
            await this.invoke
        )('item_info', { itemId })) as {
            id: number;
            name: string;
            level: number;
            can_be_hq: number;
            category_id?: number;
        };
        return { id, name, level, can_be_hq: can_be_hq != 0, category_id };
    }
    async craftTypeList(): Promise<CraftType[]> {
        return await (
            await this.invoke
        )('craft_type');
    }
    async medicineTable(_page: number): Promise<DataSourceResult<Enhancer>> {
        const results: Enhancer[] = await (await this.invoke)('medicine_table');
        return { results, totalPages: 1 };
    }
    async mealsTable(_page: number): Promise<DataSourceResult<Enhancer>> {
        const results: Enhancer[] = await (await this.invoke)('meals_table');
        return { results, totalPages: 1 };
    }
}
