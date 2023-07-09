import { invoke } from "@tauri-apps/api";
import { Item, RecipeInfo, RecipeLevel } from "../../Craft";

export interface DataSource {
    recipeTable(page: number, searchName: string): Promise<RecipesSourceResult>
    recipeLevelTable(rlv: number): Promise<RecipeLevel>
    itemInfo(id: number): Promise<Item>
}

export interface RecipesSourceResult {
    recipes: RecipeInfo[],
    totalPages: number
}

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

    constructor(base: string, language?: 'en' | 'ja' | 'de' | 'fr') {
        this.base = base
        this.language = language
    }

    async recipeTable(page: number, searchName: string): Promise<RecipesSourceResult> {
        const query = new URLSearchParams({
            'indexes': 'Recipe',
            'page': String(page),
            'string': searchName,
            'columns': 'ID,Icon,ItemResult.Name,ItemResult.ID,CraftType.Name,DifficultyFactor,DurabilityFactor,QualityFactor,RecipeLevelTable.ID,RequiredCraftsmanship,RequiredControl,CanHq'
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

                required_craftsmanship: v.RequiredCraftsmanship,
                required_control: v.RequiredControl,

                can_hq: v.CanHq != 0,
            }),
            totalPages: data.Pagination.PageTotal
        }
    }

    async recipeLevelTable(rlv: number) {
        const url = new URL(`RecipeLevelTable/${rlv}`, this.base).toString() + '?' + new URLSearchParams({
            'columns': 'ID,Stars,ClassJobLevel,SuggestedCraftsmanship,SuggestedControl,Difficulty,Quality,Durability,ProgressDivider,QualityDivider,ProgressModifier,QualityModifier,ConditionsFlag'
        }).toString();
        const resp = await fetch(url, {
            method: 'GET',
            mode: 'cors'
        })
        let data = await resp.json()
        return <RecipeLevel>{
            id: data.ID,
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
        return <Item>{
            id: data.ID,
            name: data.Name,
            level: data.LevelItem,
            can_be_hq: data.CanBeHq != 0,
            category_id: undefined
        }
    }
}

export class LocalRecipeSource {
    async recipeTable(page: number, searchName: string) {
        let [recipes, totalPages]: [RecipeInfo[], number] = await invoke("recipe_table", { pageId: page - 1, searchName: "%" + searchName + "%" });
        return { recipes, totalPages }
    }
    async recipeLevelTable(rlv: number) {
        let result: RecipeLevel = await invoke("recipe_level_table", { rlv })
        return result
    }
    async itemInfo(itemId: number) {
        const { id, name, level, can_be_hq, category_id } = await invoke("item_info", { itemId }) as {
            id: number,
            name: string,
            level: number,
            can_be_hq: number,
            category_id?: number,
        };
        return { id, name, level, can_be_hq: can_be_hq != 0, category_id };
    }
}

export const CafeMakerApiBase = "https://cafemaker.wakingsands.com/"
export const XivapiBase = "https://xivapi.com/"
