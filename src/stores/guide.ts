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

import { defineStore } from 'pinia'
import { Enhancer } from '@/components/attr-enhancer/Enhancer'
import { Actions, Attributes, Item, Jobs, Recipe, RecipeInfo, RecipeLevel } from '@/libs/Craft'

export default defineStore('guide', {
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