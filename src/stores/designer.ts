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

import { compareStatus, Item, Jobs, Recipe, RecipeLevel, RecipeRequirements } from "@/libs/Craft"
import { defineStore } from "pinia"
import { Sequence } from "@/components/designer/types"

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
        } | null,
        rotations: {
            staged: <{ key: number, seq: Sequence }[]>[],
            maxid: 0,
        }
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
        },

        pushRotation(seq: Sequence) {
            const key = this.rotations.maxid++;
            this.rotations.staged.push({ key, seq });
            this.rotations.staged.sort((a, b) => {
                const ord = compareStatus(b.seq.status, a.seq.status)
                return ord != 0 ? ord : a.key - b.key
            });
        },

        deleteRotation(i: number) {
            this.rotations.staged.splice(i, 1)
        },

        clearRotations() {
            this.rotations.staged.length = 0
        },
    }
})
