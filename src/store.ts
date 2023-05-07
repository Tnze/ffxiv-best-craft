import { ref } from 'vue'
import { defineStore } from 'pinia'
import { Attributes, Item, ItemWithAmount, Jobs, Recipe } from './Craft'

export interface GearsetsRow {
    name: string
    value: Attributes | null
}

export interface State {
    gearsets: { default: Attributes, special: GearsetsRow[] }
    checklist: ItemWithAmount[]
    designer: null | {
        item: Item;
        job: Jobs;
        recipe: Recipe;
    }
    settings: {
        language: string
    }
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
    actions: {
        storeGearsets(newGearsets: any) {
            this.default = newGearsets.default
            this.special = newGearsets.special
        },
    }
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
            item: Item;
            job: Jobs;
            recipe: Recipe;
        }
    }),
    actions: {
        selectRecipe(payload: {
            job: Jobs,
            item: Item,
            recipe: Recipe
        }) {
            this.content = payload
        }
    }
})

export const useSettingsStore = defineStore('settings', {
    state: () => ({
        language: 'system'
    }),
    actions: {
        loadSettings(localSettings: { language: string }) {
            this.language = localSettings.language
        },
        selectLanguage(newLang: string) {
            this.language = newLang
        },
    }
})
