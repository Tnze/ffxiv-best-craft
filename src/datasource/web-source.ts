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
import {
    CraftType,
    DataSourceResult,
    DataSourceType,
    RecipesSourceResult,
} from './source';
import { Enhancer } from '@/libs/Enhancer';
import { PageTranslator } from '@/libs/ZhConvertor';
import useSettingsStore from '@/stores/settings';

export class WebSource {
    public sourceType = DataSourceType.RemoteRealtime;
    base: string;

    constructor(base: string, lang: string) {
        if (lang !== undefined)
            this.base = new URL(lang + '/', base).toString();
        else this.base = base;
    }

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
        const settingStore = useSettingsStore();
        if (settingStore.language.startsWith('zh') && settingStore.dataSourceLang == 'zh') //雙語言互通查詢
            searchName = translator.simplize(searchName)
        const query = new URLSearchParams({
            page_id: String(page - 1),
            search_name: '%' + searchName + '%',
        });
        if (rlv !== undefined) {
            query.set('rlv', String(rlv));
        }
        if (craftTypeId !== undefined) {
            query.set('craft_type_id', String(craftTypeId));
        }
        if (jobLevelMin !== undefined) {
            query.set('job_level_min', String(jobLevelMin));
        }
        if (jobLevelMax !== undefined) {
            query.set('job_level_max', String(jobLevelMax));
        }

        const url =
            new URL('recipe_table', this.base).toString() +
            '?' +
            query.toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors',
        });
        const { data: results, p: totalPages } = (await jsonZhConvert(resp)) as {
            data: RecipeInfo[];
            p: number;
        };
        return { results, totalPages };
    }

    async recipesIngredients(recipeId: number): Promise<ItemWithAmount[]> {
        const query = new URLSearchParams({ recipe_id: String(recipeId) });
        const url = new URL('recipes_ingredientions', this.base);
        url.search = query.toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors',
        });
        const ings: [number, number][] = await resp.json();
        return ings.map(x => ({ ingredient_id: x[0], amount: x[1] }));
    }

    async recipeLevelTable(rlv: number): Promise<RecipeLevel> {
        const query = new URLSearchParams({ rlv: String(rlv) });
        const url = new URL('recipe_level_table', this.base);
        url.search = query.toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors',
        });
        let result: RecipeLevel = {
            id: rlv,
            stars: 0, // TODO
            ...(await resp.json()),
        };
        return result;
    }

    async recipeLevelTablebyJobLevel(
        job_level: number,
    ): Promise<RecipeLevel | null> {
        const query = new URLSearchParams({ job_level: String(job_level) });
        const url = new URL('recipe_level_table_by_job_level', this.base);
        url.search = query.toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors',
        });
        const result: RecipeLevel | null = await resp.json();
        if (result == null) return null;
        return {
            ...result,
            stars: 0, // TODO
        };
    }

    async recipeCollectableShopRefine(
        recipeId: number,
    ): Promise<CollectablesShopRefine> {
        const query = new URLSearchParams({ recipe_id: String(recipeId) });
        const url = new URL('recipe_collectability', this.base);
        url.search = query.toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors',
        });
        return resp.json();
    }

    async recipeInfo(recipeId: number): Promise<RecipeInfo> {
        const query = new URLSearchParams({ recipe_id: String(recipeId) });
        const url =
            new URL('recipe_info', this.base).toString() +
            '?' +
            query.toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors',
        });
        if (!resp.ok) {
            throw resp.statusText;
        }
        return await jsonZhConvert(resp);
    }

    async itemInfo(itemId: number): Promise<Item> {
        const query = new URLSearchParams({ item_id: String(itemId) });
        const url = new URL('item_info', this.base);
        url.search = query.toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors',
        });
        const { id, name, level, can_be_hq, category_id } =
            (await jsonZhConvert(resp)) as {
                id: number;
                name: string;
                level: number;
                can_be_hq: number;
                category_id?: number;
            };
        return { id, name, level, can_be_hq: can_be_hq != 0, category_id };
    }

    async craftTypeList(): Promise<CraftType[]> {
        const url = new URL('craft_type', this.base).toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors',
        });
        return (await jsonZhConvert(resp)) as CraftType[];
    }

    async medicineTable(_page: number): Promise<DataSourceResult<Enhancer>> {
        const url = new URL('medicine_table', this.base).toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors',
        });
        const results = (await jsonZhConvert(resp)) as Enhancer[];
        return { results, totalPages: 1 };
    }

    async mealsTable(_page: number): Promise<DataSourceResult<Enhancer>> {
        const url = new URL('meals_table', this.base).toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors',
        });
        const results = (await jsonZhConvert(resp)) as Enhancer[];
        return { results, totalPages: 1 };
    }
}

export const YYYYGamesApiBase = 'https://tnze.yyyy.games/api/datasource/';

const translator = new PageTranslator({
  translateButtonId: "translateButtonId",
  targetEncodingCookie: "targetEncodingCookie",

  msgToTraditionalChinese: "繁體",
  msgToSimplifiedChinese: "简体",

  defaultEncoding: 1,
  currentEncoding: 1,
  targetEncoding: 2,

  translateDelay: 300,

  translateBody: () => {},
});

export async function jsonZhConvert<T>(resp: Response): Promise<T> {
    const settingStore = useSettingsStore();
    const raw = await resp.text();
    switch (settingStore.language) {
        case 'zh-TW' :
            return JSON.parse(translator.traditionalize(raw)) as T;
        default:
            return JSON.parse(raw) as T;
    }
}