import { defineStore } from 'pinia'
import { Actions, Attributes, Item, ItemWithAmount, Jobs, Recipe, RecipeInfo, RecipeLevel, RecipeRequirements } from './Craft'
import { CafeMakerApiBase, XivApiRecipeSource, XivapiBase } from './components/recipe-manager/remote-source'
import { DataSource } from './components/recipe-manager/source'
import { Enhancer } from './components/attr-enhancer/Enhancer'

export const useGuideStore = defineStore('guide', {
    state: () => ({
        currentPage: 'welcome',
        recipeInfo: <RecipeInfo | null>null,
        recipe: <Recipe | null>null,
        recipeLevel: <RecipeLevel | null>null,
        itemInfo: <Item | null>null,
        craftType: <Jobs | null>null,
        craftTypeAttr: <Attributes | null>null,
        food: <Enhancer | undefined>undefined,
        potion: <Enhancer | undefined>undefined,
        manipulation: false,
        bestResult: <Actions[] | null>null,
    }),
    actions: {
        setCurrentPage(url: string) { this.currentPage = url },
        setRecipeInfo(info: RecipeInfo) { this.recipeInfo = info },
        setRecipe(r: Recipe) { this.recipe = r },
        setRecipeLevel(rlv: RecipeLevel) { this.recipeLevel = rlv },
        setItemInfo(item: Item) { this.itemInfo = item },
        setAttributes(classJob: Jobs, attr: Attributes) {
            this.craftType = classJob
            this.craftTypeAttr = attr
        },
        setBestResult(actions: Actions[]) { this.bestResult = actions }
    },
    getters: {
        finalAttr(): Attributes | null {
            if (this.craftTypeAttr == null) return null
            let { level, craftsmanship, control, craft_points } = this.craftTypeAttr;
            for (const v of [this.food, this.potion]) {
                if (!v) continue;
                if (v.cm && v.cm_max) craftsmanship += Math.min((craftsmanship * v.cm) / 100, v.cm_max)
                if (v.ct && v.ct_max) control += Math.min((control * v.ct) / 100, v.ct_max)
                if (v.cp && v.cp_max) craft_points += Math.min((craft_points * v.cp) / 100, v.cp_max)
            }
            return { level, craftsmanship, control, craft_points }
        }
    }
})

export interface GearsetsRow {
    name: Jobs
    value: Attributes | null
}

export const useGearsetsStore = defineStore('gearsets', {
    state: () => ({
        default: {
            level: 90,
            craftsmanship: 2786,
            control: 2764,
            craft_points: 533,
        },
        special: [
            { name: Jobs.Carpenter, value: null },
            { name: Jobs.Blacksmith, value: null },
            { name: Jobs.Armorer, value: null },
            { name: Jobs.Goldsmith, value: null },
            { name: Jobs.Leatherworker, value: null },
            { name: Jobs.Weaver, value: null },
            { name: Jobs.Alchemist, value: null },
            { name: Jobs.Culinarian, value: null },
        ] as GearsetsRow[]
    }),
    getters: {
        toJson(): string {
            return JSON.stringify({
                default: this.default,
                special: this.special
            })
        }
    },
    actions: {
        fromJson(json: string) {
            let v = JSON.parse(json)
            this.default = v.default
            this.special = v.special
        },
    },
})

export const useChecklistStore = defineStore('checklist', {
    state: () => ({
        items: [] as ItemWithAmount[],
    }),
    actions: {
        addToChecklist(payload: ItemWithAmount) {
            const elem = this.items.find(v => v.ingredient_id == payload.ingredient_id)
            if (elem == undefined)
                this.items.push(payload)
            else
                elem.amount += payload.amount
        },
        changeChecklist(payload: { idx: number, amount: number }) {
            if (payload.amount > 0)
                this.items[payload.idx].amount = payload.amount
            else
                this.items.splice(payload.idx, 1)
        },
    },
})

export const useDesignerStore = defineStore('designer', {
    state: () => ({
        content: null as null | {
            item: Item,
            job: Jobs,
            recipe: Recipe,
            recipeId?: number,
            recipeLevel: RecipeLevel,
            materialQualityFactor: number,
            requirements: RecipeRequirements,
            simulatorMode: boolean,
        }
    }),
    actions: {
        selectRecipe(payload: {
            job: Jobs,
            item: Item,
            recipe: Recipe,
            recipeId?: number,
            recipeLevel: RecipeLevel,
            materialQualityFactor: number,
            requirements: RecipeRequirements,
            simulatorMode: boolean,
        }) {
            this.content = payload
        }
    }
})

export const useSettingsStore = defineStore('settings', {
    state: () => ({
        language: 'system',
        dataSource: <'local' | 'xivapi' | 'cafe'>(import.meta.env.VITE_BESTCRAFT_TARGET == "tauri" ? 'local' : 'xivapi'),
        dataSourceLang: <'en' | 'ja' | 'de' | 'fr' | undefined>undefined
    }),
    getters: {
        toJson(): string {
            return JSON.stringify({
                language: this.language,
            })
        },
        async getDataSource(): Promise<DataSource> {
            if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
                var { LocalRecipeSource } = await import('./components/recipe-manager/local-source')
                switch (this.dataSource) {
                    case 'local':
                        return new LocalRecipeSource()
                    case 'xivapi': return new XivApiRecipeSource(XivapiBase, this.dataSourceLang)
                    case 'cafe': return new XivApiRecipeSource(CafeMakerApiBase)
                    default: return new LocalRecipeSource()
                }
            } else {
                switch (this.dataSource) {
                    case 'xivapi': return new XivApiRecipeSource(XivapiBase, this.dataSourceLang)
                    case 'cafe': return new XivApiRecipeSource(CafeMakerApiBase)
                    default: return new XivApiRecipeSource(CafeMakerApiBase)
                }
            }
        }
    },
    actions: {
        loadSettings(localSettings: any) {
            this.$patch(localSettings)
            this.language = localSettings.language
            this.dataSource = localSettings.dataSource
            if (localSettings.dataSourceLang)
                this.dataSourceLang = localSettings.dataSourceLang
            else {
                if (this.language.startsWith('en')) {
                    this.dataSourceLang = 'en'
                } else if (this.language.startsWith('ja')) {
                    this.dataSourceLang = 'ja'
                }
            }
        },
        fromJson(json: string) {
            this.$patch(JSON.parse(json))
        },
    }
})
