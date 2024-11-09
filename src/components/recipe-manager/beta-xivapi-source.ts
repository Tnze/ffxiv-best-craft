// This file is part of BestCraft.
// Copyright (C) 2024 Tnze
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

import { Item, ItemWithAmount, RecipeInfo, RecipeLevel } from '@/libs/Craft';
import {
    CraftType,
    DataSourceResult,
    DataSourceType,
    RecipesSourceResult,
} from './source';
import { Enhancer } from '@/libs/Enhancer';

function assert<T>(v: T, name: string): T {
    if (v === undefined) throw `${name} is undefined`;
    return v;
}

export class BetaXivApiRecipeSource {
    base: string;
    language: 'en' | 'ja' | 'de' | 'fr' | undefined;
    public sourceType = DataSourceType.RemoteRealtime;

    constructor(base: string, language?: 'en' | 'ja' | 'de' | 'fr') {
        this.base = base;
        this.language = language;
    }

    async recipeTable(
        page: number,
        searchName?: string,
        rlv?: number,
        craftTypeId?: number,
        jobLevelMin?: number,
        jobLevelMax?: number,
    ): Promise<RecipesSourceResult> {
        const params: Record<string, string> = {
            fields: 'Icon,ItemResult.Name,CraftType.Name,DifficultyFactor,DurabilityFactor,QualityFactor,MaterialQualityFactor,RecipeLevelTable@as(raw),RequiredCraftsmanship,RequiredControl,CanHq',
            // 'limit': "100",
        };
        const query = new URLSearchParams(params);
        // Language?
        if (this.language != undefined) {
            query.set('language', this.language);
        }

        // Filters?
        const filters: string[] = [];
        if (searchName) {
            filters.push('ItemResult.Name~' + JSON.stringify(searchName));
        }
        if (rlv != undefined) {
            filters.push(`RecipeLevelTable=${rlv}`);
        }
        if (craftTypeId != undefined) {
            filters.push(`CraftType=${craftTypeId}`);
        }
        if (jobLevelMin != undefined) {
            filters.push(`RecipeLevelTable.ClassJobLevel>=${jobLevelMin}`);
        }
        if (jobLevelMax != undefined) {
            filters.push(`RecipeLevelTable.ClassJobLevel<=${jobLevelMax}`);
        }

        // Fetch
        if (filters.length > 0) {
            // If any filter exist, use search api
            query.set('query', filters.join(' +'));
            query.set('sheets', 'Recipe');
            const next = async () => {
                const url =
                    new URL('search', this.base).toString() +
                    '?' +
                    query.toString();
                const resp = await fetch(url, { method: 'GET', mode: 'cors' });
                let data = await resp.json();
                if (!resp.ok) this.checkRespError(data);
                if (query.has('query')) query.delete('query');
                if (data.next) query.set('cursor', data.next);
                else if (query.has('cursor')) query.delete('cursor');
                return {
                    results: data.results.map(
                        BetaXivApiRecipeSource.recipeRowsToRecipe,
                    ),
                    totalPages: 1,
                    next: query.has('cursor') ? next : undefined,
                };
            };
            return next();
        } else {
            // Use list rows api
            const next = async () => {
                const url =
                    new URL('sheet/Recipe', this.base).toString() +
                    '?' +
                    query.toString();
                const resp = await fetch(url, { method: 'GET', mode: 'cors' });
                let data = await resp.json();
                if (!resp.ok) this.checkRespError(data);
                if (data.rows.length > 0)
                    query.set('after', data.rows[data.rows.length - 1].row_id);
                else if (query.has('after')) query.delete('after');
                return {
                    results: data.rows.map(
                        BetaXivApiRecipeSource.recipeRowsToRecipe,
                    ),
                    totalPages: 1,
                    next: query.has('after') ? next : undefined,
                };
            };
            return next();
        }
    }

    private static recipeRowsToRecipe(v: any): RecipeInfo {
        return {
            id: assert(v?.row_id, 'recipe_id'),
            rlv: assert(v?.fields['RecipeLevelTable@as(raw)'], 'rlv'),
            item_id: assert(v?.fields?.ItemResult.row_id, 'item_id'),
            item_name: assert(v?.fields?.ItemResult?.fields.Name, 'item_name'),
            job: assert(v?.fields?.CraftType?.fields?.Name, 'job'),

            difficulty_factor: assert(
                v?.fields?.DifficultyFactor,
                'difficulty_factor',
            ),
            quality_factor: assert(v?.fields?.QualityFactor, 'quality_factor'),
            durability_factor: assert(
                v?.fields?.DurabilityFactor,
                'durability_factor',
            ),
            material_quality_factor: assert(
                v?.fields?.MaterialQualityFactor,
                'material_quality_factor',
            ),

            required_craftsmanship: assert(
                v?.fields?.RequiredCraftsmanship,
                'required_craftsmanship',
            ),
            required_control: assert(
                v?.fields?.RequiredControl,
                'required_control',
            ),

            can_hq: assert(v?.fields?.CanHq, 'can_hq'),
        };
    }

    async recipesIngredients(recipeId: number): Promise<ItemWithAmount[]> {
        const needs = new Map<number, ItemWithAmount>(); // item_id as key
        const query = new URLSearchParams({
            fields: 'AmountIngredient,Ingredient@as(raw)',
        });
        if (this.language != undefined) query.set('language', this.language);
        const url =
            new URL(`sheet/Recipe/${recipeId}`, this.base).toString() +
            '?' +
            query.toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors',
        });
        const data = await resp.json();
        if (!resp.ok) {
            this.checkRespError(data);
        }
        const Ingredient = data.fields['Ingredient@as(raw)'];
        const AmountIngredient = data.fields.AmountIngredient;
        const arrayLen = Math.min(Ingredient.length, AmountIngredient.length);
        for (let i = 0; i < arrayLen; i++) {
            const obj = <ItemWithAmount>{
                ingredient_id: Ingredient[i],
                amount: AmountIngredient[i],
            };
            if (obj.amount > 0) {
                let record = needs.get(obj.ingredient_id);
                if (record != null) {
                    record.ingredient_id += obj.amount;
                } else {
                    needs.set(obj.ingredient_id, obj);
                }
            }
        }
        return Array.from(needs.values()).sort(
            (a, b) => a.ingredient_id - b.ingredient_id,
        );
    }

    async recipeLevelTable(rlv: number): Promise<RecipeLevel> {
        const query = new URLSearchParams({
            fields: 'ID,Stars,ClassJobLevel,SuggestedCraftsmanship,SuggestedControl,Difficulty,Quality,Durability,ProgressDivider,QualityDivider,ProgressModifier,QualityModifier,ConditionsFlag',
        });
        if (this.language != undefined) query.set('language', this.language);
        const url =
            new URL(`sheet/RecipeLevelTable/${rlv}`, this.base).toString() +
            '?' +
            query.toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors',
        });
        let data = await resp.json();
        if (!resp.ok) {
            this.checkRespError(data);
        }
        data = data.fields;
        return {
            stars: assert(data.Stars, 'starts'),
            class_job_level: assert(data.ClassJobLevel, 'class_job_level'),

            suggested_craftsmanship: data.SuggestedCraftsmanship,
            suggested_control: data.SuggestedControl,

            difficulty: assert(data.Difficulty, 'difficulty'),
            quality: assert(data.Quality, 'quality'),
            durability: assert(data.Durability, 'durability'),

            progress_divider: assert(data.ProgressDivider, 'progress_divider'),
            quality_divider: assert(data.QualityDivider, 'quality_divider'),
            progress_modifier: assert(
                data.ProgressModifier,
                'progress_modifier',
            ),
            quality_modifier: assert(data.QualityModifier, 'quality_modifier'),

            conditions_flag: assert(data.ConditionsFlag, 'conditions_flag'),
        };
    }

    async recipeInfo(recipeId: number): Promise<RecipeInfo> {
        const query = new URLSearchParams({
            fields: 'Icon,ItemResult.Name,CraftType.Name,DifficultyFactor,DurabilityFactor,QualityFactor,MaterialQualityFactor,RecipeLevelTable@as(raw),RequiredCraftsmanship,RequiredControl,CanHq',
        });
        if (this.language != undefined) query.set('language', this.language);
        const url =
            new URL(`sheet/Recipe/${recipeId}`, this.base).toString() +
            '?' +
            query.toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors',
        });
        const data = await resp.json();
        if (!resp.ok) {
            this.checkRespError(data);
        }
        return BetaXivApiRecipeSource.recipeRowsToRecipe(data);
    }

    async itemInfo(id: number): Promise<Item> {
        const query = new URLSearchParams({
            fields: 'Name,LevelItem,CanBeHq,CategoryID',
        });
        if (this.language != undefined) query.set('language', this.language);
        const url =
            new URL(`sheet/Item/${id}`, this.base).toString() +
            '?' +
            query.toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors',
        });
        let data = await resp.json();
        if (!resp.ok) {
            this.checkRespError(data);
        }
        return {
            id: assert(data.row_id, 'item_id'),
            name: assert(data.fields.Name, 'name'),
            level: assert(data.fields.LevelItem.value, 'level'),
            can_be_hq: assert(data.fields.CanBeHq, 'can_be_hq'),
            category_id: undefined,
        };
    }

    async craftTypeList(): Promise<CraftType[]> {
        const query = new URLSearchParams({
            fields: 'Name',
        });
        if (this.language != undefined) query.set('language', this.language);
        const url =
            new URL(`sheet/CraftType`, this.base).toString() +
            '?' +
            query.toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors',
        });
        const data = await resp.json();
        if (!resp.ok) {
            this.checkRespError(data);
        }
        return data.rows.map(
            (v: any) =>
                <CraftType>{
                    id: assert(v.row_id, 'crafttype_id'),
                    name: assert(v.fields.Name, 'name'),
                },
        );
    }

    async medicineTable(page: number): Promise<DataSourceResult<Enhancer>> {
        return this.getEnhancers(page, MedicineID);
    }

    async mealsTable(page: number): Promise<DataSourceResult<Enhancer>> {
        return this.getEnhancers(page, Meals);
    }

    private async getEnhancers(
        _page: number,
        categoryID: number,
    ): Promise<DataSourceResult<Enhancer>> {
        const itemFoods = await this.craftingItemFoods();
        const itemFoodsQuery = itemFoods.map(v => `Data[]=${v[0]}`).join(' ');
        const query = [
            `+ItemAction.(${itemFoodsQuery})`,
            '+ItemAction.(Type=844 Type=845 Type=846)',
            `+ItemSearchCategory=${categoryID}`,
        ].join(' ');
        const data = await this.queryAll<{
            row_id: number;
            fields: {
                Name: string;
                ItemAction: any;
            };
        }>('Item', query, 'Name,ItemAction');
        const itemFoodsMap = new Map(itemFoods);
        return {
            totalPages: 1,
            results: data.flatMap(item => {
                const itemFood = itemFoodsMap.get(
                    item.fields.ItemAction.fields.Data[1],
                ); // [1] ItemFood (row_id), [2] Duration (sec)
                return (
                    itemFood?.map(v => ({
                        ...v,
                        name: item.fields.Name + v.name,
                    })) ?? []
                );
            }),
        };
    }

    private checkRespError(data: any) {
        if (data.code != 200) {
            console.error(data);
            throw data.message ?? 'Remote Error';
        }
    }

    async craftingItemFoods(): Promise<[number, [Enhancer, Enhancer]][]> {
        const data = await this.queryAll<{
            row_id: number;
            fields: {
                BaseParam: any[];
                Value: any[];
                Max: any[];
                ValueHQ: any[];
                MaxHQ: any[];
            };
        }>(
            'ItemFood',
            'BaseParam[]=11 BaseParam[]=70 BaseParam[]=71',
            'BaseParam@as(raw),Value,ValueHQ,Max,MaxHQ',
        );
        return data.map(row => [
            row.row_id,
            BetaXivApiRecipeSource.itemFoodToEnhancer('', row.fields),
        ]);
    }

    private static itemFoodToEnhancer(
        name: string,
        itemFood: any,
    ): [Enhancer, Enhancer] {
        const enh = <Enhancer>{ name };
        const enhHq = <Enhancer>{ name: name + ' HQ' };
        for (let i = 0; i < 3; i++) {
            switch (itemFood['BaseParam@as(raw)'][i]) {
                case 11: // CP
                    enh.cp = itemFood.Value[i];
                    enh.cp_max = itemFood.Max[i];
                    enhHq.cp = itemFood.ValueHQ[i];
                    enhHq.cp_max = itemFood.MaxHQ[i];
                    break;
                case 70: // Craftsmanship
                    enh.cm = itemFood.Value[i];
                    enh.cm_max = itemFood.Max[i];
                    enhHq.cm = itemFood.ValueHQ[i];
                    enhHq.cm_max = itemFood.MaxHQ[i];
                    break;
                case 71: // Control
                    enh.ct = itemFood.Value[i];
                    enh.ct_max = itemFood.Max[i];
                    enhHq.ct = itemFood.ValueHQ[i];
                    enhHq.ct_max = itemFood.MaxHQ[i];
                    break;
            }
        }
        return [enh, enhHq];
    }

    private async queryAll<T>(sheets: string, query: string, fields?: string) {
        const params = new URLSearchParams({ sheets, query });
        if (fields) params.set('fields', fields);
        if (this.language != undefined) params.set('language', this.language);

        const results = <T[]>[];
        let data: { next?: string; schema: string; results: T[] };
        do {
            const url =
                new URL(`search`, this.base).toString() +
                '?' +
                params.toString();
            const response = await fetch(url, { method: 'GET', mode: 'cors' });
            data = await response.json();
            if (!response.ok) {
                this.checkRespError(data);
            }
            results.push(...data.results);
            if (data.next) {
                params.set('cursor', data.next);
                params.delete('query');
            }
        } while (data.next !== undefined);

        return results;
    }
}

export const BetaXivapiBase = 'https://beta.xivapi.com/api/1/';

const MedicineID = 43;
const Meals = 45;
