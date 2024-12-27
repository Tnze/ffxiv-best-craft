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
    required: number = 1;
    requiredBy: Map<ItemID, number>; // itemID, amount
    depth: number;

    constructor(item: Item, depth: number) {
        this.item = item;
        this.depth = depth;
        this.requiredBy = new Map();
    }

    setFixRequiredNumber(n: number) {
        this.required = n;
    }

    getFixRequiredNumber() {
        return this.required;
    }

    requiredNumber(): number {
        let sum = 0;
        for (const amount of this.requiredBy.values()) {
            sum += amount;
        }
        return sum;
    }

    addRequiredBy(item: ItemID, amount: number, depth: number) {
        const oldAmount = this.requiredBy.get(item) ?? 0;
        this.requiredBy.set(item, oldAmount + amount);
        this.depth = Math.max(this.depth, depth);
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
            this.targetItems.push(new Slot(item, 0));
        },

        async updateBom() {
            const settingStore = useSettingStore();
            const ds = await settingStore.getDataSource;

            const ings = new Map<ItemID, Slot>();
            const queue = [...this.targetItems];
            const indegrees = new Map<ItemID, number>();
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
                    indegrees.set(
                        subIng.ingredient_id,
                        (indegrees.get(subIng.ingredient_id) ?? 0) + 1,
                    );

                    let slot = ings.get(subIng.ingredient_id);
                    if (slot != undefined) {
                        slot.addRequiredBy(v.item.id, 0, v.depth + 1);
                    } else {
                        const itemInfo = await ds.itemInfo(
                            subIng.ingredient_id,
                        );
                        slot = new Slot(itemInfo, v.depth + 1);
                        slot.addRequiredBy(v.item.id, 0, v.depth + 1);
                        ings.set(itemInfo.id, slot);
                    }
                    queue.push(slot);
                }
                successors.set(
                    v.item.id,
                    subIngs.map(v => v.ingredient_id),
                );
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
