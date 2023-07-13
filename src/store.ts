import { defineStore } from 'pinia'
import { Attributes, Item, ItemWithAmount, Jobs, Recipe, RecipeInfo, RecipeLevel, RecipeRequirements } from './Craft'
import { CafeMakerApiBase, DataSource, LocalRecipeSource, XivApiRecipeSource, XivapiBase } from './components/recipe-manager/source'

export const useGuideStore = defineStore('guide', {
    state: () => ({
        recipeInfo: <RecipeInfo | null>null,
    }),
    actions: {
        setRecipeInfo(info: RecipeInfo) {
            this.recipeInfo = info
        }
    }
})

export interface GearsetsRow {
    name: string
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
            { name: 'carpenter', value: null },
            { name: 'blacksmith', value: null },
            { name: 'armorer', value: null },
            { name: 'goldsmith', value: null },
            { name: 'leatherworker', value: null },
            { name: 'weaver', value: null },
            { name: 'alchemist', value: null },
            { name: 'culinarian', value: null },
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
            recipeLevel: RecipeLevel,
            requirements: RecipeRequirements,
            simulatorMode: boolean,
        }
    }),
    actions: {
        selectRecipe(payload: {
            job: Jobs,
            item: Item,
            recipe: Recipe,
            recipeLevel: RecipeLevel,
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
        dataSource: <'local' | 'xivapi' | 'cafe'>'local',
        dataSourceLang: <'en' | 'ja' | 'de' | 'fr' | undefined>undefined
    }),
    getters: {
        toJson(): string {
            return JSON.stringify({
                language: this.language,
            })
        },
        getDataSource(): DataSource {
            switch (this.dataSource) {
                case 'local': return new LocalRecipeSource()
                case 'xivapi': return new XivApiRecipeSource(XivapiBase, this.dataSourceLang)
                case 'cafe': return new XivApiRecipeSource(CafeMakerApiBase)
                default: return new LocalRecipeSource()
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
