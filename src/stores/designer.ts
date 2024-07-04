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

import { Item, Jobs, Recipe, RecipeLevel, RecipeRequirements } from "@/libs/Craft"
import { defineStore } from "pinia"

export default defineStore('designer', {
    state: () => ({
        content: null as {
            item: Item,
            job?: Jobs,
            recipe: Recipe,
            recipeId?: number,
            recipeLevel: RecipeLevel,
            materialQualityFactor: number,
            requirements: RecipeRequirements,
            simulatorMode: boolean,
        } | null
    }),
    actions: {
        selectRecipe(payload: {
            job?: Jobs,
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
