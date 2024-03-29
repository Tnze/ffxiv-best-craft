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

import { Item, ItemWithAmount, RecipeInfo, RecipeLevel } from "@/libs/Craft";
import { DataSourceType, RecipesSourceResult } from './source'

interface XivapiRecipeResult {
    Pagination: {
        Page: number
        PageNext?: number
        PagePrev?: number
        PageTotal: number
        Results: number
        ResultsPerPage: number
        ResultsTotal: number
    },
    Results: {
        ID: number
        Icon: string
        ItemResult: {
            ID: number
            Name: string
        }
        CraftType: { Name: string }
        DifficultyFactor: number
        DurabilityFactor: number
        QualityFactor: number
        MaterialQualityFactor: number
        RecipeLevelTable: {
            ID: number
        }
        RequiredCraftsmanship: number
        RequiredControl: number
        CanHq: number
    }[]
}

interface XivapiItemResult {
    ID: number,
    Name: string,
    LevelItem: number,
    CanBeHq: number,
}

export class XivApiRecipeSource {
    base: string;
    language: 'en' | 'ja' | 'de' | 'fr' | undefined
    public sourceType = DataSourceType.RemoteRealtime

    constructor(base: string, language?: 'en' | 'ja' | 'de' | 'fr') {
        this.base = base
        this.language = language
    }

    async recipeTable(page: number, searchName: string): Promise<RecipesSourceResult> {
        const query = new URLSearchParams({
            'indexes': 'Recipe',
            'page': String(page),
            'string': searchName,
            'columns': 'ID,Icon,ItemResult.Name,ItemResult.ID,CraftType.Name,DifficultyFactor,DurabilityFactor,QualityFactor,MaterialQualityFactor,RecipeLevelTable.ID,RequiredCraftsmanship,RequiredControl,CanHq'
        })
        if (this.language != undefined) query.set('language', this.language)
        const url = new URL('search', this.base).toString() + '?' + query.toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors'
        })
        let data: XivapiRecipeResult = await resp.json()
        return {
            recipes: data.Results.map(v => <RecipeInfo>{
                id: v.ID,
                rlv: v.RecipeLevelTable.ID,
                item_id: v.ItemResult.ID,
                item_name: v.ItemResult.Name,
                job: v.CraftType.Name,

                difficulty_factor: v.DifficultyFactor,
                quality_factor: v.QualityFactor,
                durability_factor: v.DurabilityFactor,
                material_quality_factor: v.MaterialQualityFactor,

                required_craftsmanship: v.RequiredCraftsmanship,
                required_control: v.RequiredControl,

                can_hq: v.CanHq != 0,
            }),
            totalPages: data.Pagination.PageTotal
        }
    }

    async recipesIngredients(recipeId: number): Promise<ItemWithAmount[]> {
        const nums = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        const needs = new Map<number, ItemWithAmount>() // item_id as key
        const url = new URL(`Recipe/${recipeId}`, this.base).toString() + '?' + new URLSearchParams({
            'columns': nums.map(n => `AmountIngredient${n},ItemIngredient${n}TargetID`).join(',')
        }).toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors'
        })
        const data = await resp.json()
        for (const n of nums) {
            const obj = {
                ingredient_id: data[`ItemIngredient${n}TargetID`] as number,
                amount: data[`AmountIngredient${n}`] as number,
            }
            if (obj.amount > 0) {
                let record = needs.get(obj.ingredient_id)
                if (record != null) {
                    record.ingredient_id += obj.amount
                } else {
                    needs.set(obj.ingredient_id, obj)
                }
            }
        }
        return Array.from(needs.values()).sort((a, b) => a.ingredient_id - b.ingredient_id)
    }

    async recipeLevelTable(rlv: number): Promise<RecipeLevel> {
        const url = new URL(`RecipeLevelTable/${rlv}`, this.base).toString() + '?' + new URLSearchParams({
            'columns': 'ID,Stars,ClassJobLevel,SuggestedCraftsmanship,SuggestedControl,Difficulty,Quality,Durability,ProgressDivider,QualityDivider,ProgressModifier,QualityModifier,ConditionsFlag'
        }).toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors'
        })
        const data = await resp.json()
        return {
            stars: data.Stars,
            class_job_level: data.ClassJobLevel,

            suggested_craftsmanship: data.SuggestedCraftsmanship,
            suggested_control: data.SuggestedControl,

            difficulty: data.Difficulty,
            quality: data.Quality,
            durability: data.Durability,

            progress_divider: data.ProgressDivider,
            quality_divider: data.QualityDivider,
            progress_modifier: data.ProgressModifier,
            quality_modifier: data.QualityModifier,

            conditions_flag: data.ConditionsFlag,
        }
    }

    async recipeInfo(recipeId: number): Promise<RecipeInfo> {
        throw "todo"
    }

    async itemInfo(id: number): Promise<Item> {
        const query = new URLSearchParams({
            'columns': 'ID,Name,LevelItem,CanBeHq'
        })
        if (this.language != undefined) query.set('language', this.language)
        const url = new URL(`Item/${id}`, this.base).toString() + '?' + query.toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors'
        })
        let data: XivapiItemResult = await resp.json()
        return {
            id: data.ID,
            name: data.Name,
            level: data.LevelItem,
            can_be_hq: data.CanBeHq != 0,
            category_id: undefined
        }
    }
}

export const CafeMakerApiBase = "https://cafemaker.wakingsands.com/"
export const XivapiBase = "https://xivapi.com/"
