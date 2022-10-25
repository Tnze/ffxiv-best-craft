import { InjectionKey } from 'vue'
import { createStore, useStore as baseUseStore, Store } from 'vuex'
import { Attributes, ItemWithAmount, Jobs, Recipe } from './Craft'

export interface GearsetsRow {
    name: string
    value: Attributes | null
}

export interface State {
    gearsets: { default: Attributes, special: GearsetsRow[] }
    checklist: ItemWithAmount[]
    designer: null | {
        itemName: string;
        job: Jobs | "unknown";
        recipe: Recipe;
    }
    settings: {
        language: string
    }
}

export const key: InjectionKey<Store<State>> = Symbol()

export const store = createStore<State>({
    state: {
        gearsets: {
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
            ]
        },
        checklist: [],
        designer: null,
        settings: {
            language: "zh-CN"
        }
    },
    mutations: {
        storeGearsets(state, newGearsets) {
            state.gearsets = newGearsets
        },
        addToChecklist(state, payload: ItemWithAmount) {
            const elem = state.checklist.find(v => v.ingredient_id == payload.ingredient_id)
            if (elem == undefined)
                state.checklist.push(payload)
            else
                elem.amount += payload.amount
        },
        selectRecipe(state, payload: {
            job: Jobs | 'unknown',
            itemName: string,
            recipe: Recipe
        }) {
            state.designer = payload
        }
    }
})

export function useStore() {
    return baseUseStore(key)
}