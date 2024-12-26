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

import { defineStore } from 'pinia';
import useSettingStore from '@/stores/settings';
import { DataSource } from '@/datasource/source';
import { RecipeInfo } from '@/libs/Craft';

export type ItemID = number;
export type RecipeID = number;

export interface Item {
    id: ItemID;
    name: string;
}

export interface Slot {
    item: Item;
    requiredNummber: number;
    holdingNumber: number;
}

export default defineStore('bom', {
    state: () => ({
        targetItems: <Slot[]>[],
        holdingItems: new Map<ItemID, number>(), // item -> amount

        recipeCache: new Map<ItemID, RecipeInfo[]>(),
    }),

    actions: {
        addTarget(item: Item) {
            this.targetItems.push({
                item,
                requiredNummber: 1,
                holdingNumber: 0,
            });
        },

        async updateBom() {
            const settingStore = useSettingStore();
            const dataSource = await settingStore.getDataSource;

            const targets: { id: number; name: string; demanded: number }[] =
                [];
            for (const target of this.targetItems) {
                const demanded = target.requiredNummber - target.holdingNumber;
                if (demanded > 0) {
                    targets.push({
                        id: target.item.id,
                        name: target.item.name,
                        demanded,
                    });
                }
            }

            const ingredients = new Map<ItemID, number>();
            for (const required of targets) {
                const recipes = await this.findRecipe(
                    dataSource,
                    required.id,
                    required.name,
                );
                if (recipes.length == 0) {
                    // Final Item
                    console.log(
                        `Final Item: "${required.name}" ×${required.demanded}`,
                    );
                    continue;
                }

                const selectedRecipe = recipes[0];
                // Calculate how many times is required crafting this item.
                const resultAmount = selectedRecipe.item_amount;
                if (resultAmount == undefined)
                    throw 'data source does not support item_amount';
                const n = Math.ceil(required.demanded / resultAmount);

                const result = await dataSource.recipesIngredients(
                    selectedRecipe.id,
                );
                for (const ingredient of result) {
                    const count =
                        ingredients.get(ingredient.ingredient_id) ?? 0;
                    ingredients.set(
                        ingredient.ingredient_id,
                        count + ingredient.amount * n,
                    );
                }
            }
            console.log(ingredients);
        },

        async findRecipe(
            dataSource: DataSource,
            itemId: ItemID,
            itemName: string,
        ): Promise<RecipeInfo[]> {
            let result = this.recipeCache.get(itemId); // read cache
            if (result != undefined) {
                return result;
            }

            // TODO: be smarter, get rid of itemName
            const table = await dataSource.recipeTable(1, itemName);
            result = table.results.filter(v => v.item_id == itemId);

            this.recipeCache.set(itemId, result); // write cache
            return result;
        },
    },
});
