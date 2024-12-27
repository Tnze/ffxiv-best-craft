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
import { ItemWithAmount, RecipeInfo } from '@/libs/Craft';

export type ItemID = number;
export type RecipeID = number;

export interface Item {
    id: ItemID;
    name: string;
}

class Slot {
    item: Item;
    required: number;
    requiredBy: Map<ItemID, number>; // itemID, amount
    depth: number;

    constructor(item: Item, depth: number) {
        this.item = item;
        this.depth = depth;
        this.required = 0;
        this.requiredBy = new Map();
    }

    setFixRequiredNumber(n: number) {
        this.required = n;
    }

    getFixRequiredNumber() {
        return this.required;
    }

    requiredNumber(): number {
        let sum = this.required;
        for (const amount of this.requiredBy.values()) {
            sum += amount;
        }
        return sum;
    }

    addRequiredBy(item: ItemID, amount: number) {
        const oldAmount = this.requiredBy.get(item) ?? 0;
        this.requiredBy.set(item, oldAmount + amount);
    }
}

export default defineStore('bom', {
    state: () => ({
        targetItems: <Slot[]>[],
        holdingItems: new Map<ItemID, number>(), // item -> amount

        recipeCache: new Map<ItemID, RecipeInfo[]>(),
        ingredients: <Slot[]>[],
    }),

    actions: {
        addTarget(item: Item) {
            const slot = new Slot(item, 0);
            slot.setFixRequiredNumber(1);
            this.targetItems.push(slot);
        },

        async updateBom() {
            const settingStore = useSettingStore();
            const ds = await settingStore.getDataSource;

            for (const slot of this.targetItems) {
                slot.requiredBy.clear();
            }

            // Discovering crafting DAG
            const ings = new Map<ItemID, Slot>();
            const queue = [...this.targetItems];
            const successors = new Map<ItemID, ItemID[]>();
            while (queue.length > 0) {
                const v = queue.shift()!;
                ings.set(v.item.id, v);

                const recipes = await this.findRecipe(
                    ds,
                    v.item.id,
                    v.item.name,
                );
                if (recipes.length == 0) continue;

                const r = recipes[0];
                const subIngs = await ds.recipesIngredients(r.id);

                for (const subIng of subIngs) {
                    let slot = ings.get(subIng.ingredient_id);
                    if (slot == undefined) {
                        const itemInfo = await ds.itemInfo(
                            subIng.ingredient_id,
                        );
                        slot = new Slot(itemInfo, v.depth + 1);
                        ings.set(itemInfo.id, slot);
                    }
                    queue.push(slot);
                }
                successors.set(
                    v.item.id,
                    subIngs.map(v => v.ingredient_id),
                );
            }

            // Sorting with Kahn's algorithm
            const indegrees = new Map<ItemID, number>();
            for (const v of successors.values()) {
                v.forEach(successor => {
                    indegrees.set(
                        successor,
                        (indegrees.get(successor) ?? 0) + 1,
                    );
                });
            }
            for (const ing of ings.values()) {
                if ((indegrees.get(ing.item.id) ?? 0) == 0) {
                    queue.push(ing);
                }
            }
            const sorted = [];
            while (queue.length > 0) {
                const v = queue.shift()!;
                sorted.push(v);

                const ss = successors.get(v.item.id);
                if (ss != undefined) {
                    for (const successor of ss!) {
                        const newIndegree = indegrees.get(successor)! - 1;
                        indegrees.set(successor, newIndegree);
                        if (newIndegree == 0) {
                            queue.push(ings.get(successor)!);
                        }
                    }
                }
            }
            console.assert(sorted.length == ings.size, 'Sorting failed');

            // Calculate numbers
            const holdings = new Map(this.holdingItems);
            for (const slot of sorted) {
                // calculate needs
                const r = slot.requiredNumber(); // required
                const h = holdings.get(slot.item.id) ?? 0; // holding
                const use = Math.min(r, h);
                holdings.set(slot.item.id, h - use);
                const n = r - use; // needs

                // find recipe
                const recipes = await this.findRecipe(
                    ds,
                    slot.item.id,
                    slot.item.name,
                );
                if (recipes.length == 0) continue;
                const recipe = recipes[0];
                if (recipe.item_amount == undefined)
                    throw 'unsupported data source';
                const crafts = Math.ceil(n / recipe.item_amount);
                const wasted = recipe.item_amount * crafts - n;
                if (wasted > 0) {
                    console.warn(`${slot.item.name} ×${wasted} will be wasted`);
                }

                for (const ing of await ds.recipesIngredients(recipe.id)) {
                    const slot = ings.get(ing.ingredient_id)!;
                    slot.addRequiredBy(slot.item.id, crafts * ing.amount);
                }
            }

            this.ingredients = sorted;
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
