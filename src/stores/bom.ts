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

export type SlotType = 'completed' | 'crafted' | 'required' | 'not-required';

export class Slot {
    item: Item;
    required: number;
    requiredBy: Map<ItemID, number>; // itemID, amount
    depth: number = 0;
    wasted: number = 0;
    type?: SlotType;

    constructor(item: Item) {
        this.item = item;
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
        ingredientsCache: new Map<RecipeID, ItemWithAmount[]>(),
        itemInfoCache: new Map<ItemID, Item>(),
        ingredients: <Slot[]>[],

        fetchingItem: <string | undefined>undefined,
    }),

    actions: {
        addTarget(...items: Item[]) {
            for (const item of items) {
                const slot = new Slot(item);
                slot.setFixRequiredNumber(1);
                this.targetItems.push(slot);
            }
        },

        removeTarget(i: number) {
            this.targetItems.splice(i, 1);
        },

        async updateBom() {
            const settingStore = useSettingStore();
            const ds = await settingStore.getDataSource();

            for (const slot of this.targetItems) {
                slot.requiredBy.clear();
            }

            // Discovering crafting DAG
            const queue = [...this.targetItems];
            const ings = new Map<ItemID, Slot>(queue.map(v => [v.item.id, v]));
            const successors = new Map<ItemID, ItemID[]>();
            while (queue.length > 0) {
                const v = queue.shift()!;
                ings.set(v.item.id, v);

                this.fetchingItem = v.item.name;

                const recipes = await this.findRecipe(
                    ds,
                    v.item.id,
                    v.item.name,
                );
                if (recipes.length == 0) continue;

                const r = recipes[0];
                const subIngs = await this.fetchIngredients(ds, r.id);

                for (const subIng of subIngs) {
                    let slot = ings.get(subIng.ingredient_id);
                    if (slot == undefined) {
                        const itemInfo = await this.fetchItemInfo(
                            ds,
                            subIng.ingredient_id,
                        );
                        slot = new Slot(itemInfo);
                        ings.set(itemInfo.id, slot);
                    }
                    queue.push(slot);
                }
                successors.set(
                    v.item.id,
                    subIngs.map(v => v.ingredient_id),
                );
            }
            this.fetchingItem = undefined;

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
            if (sorted.length != ings.size) {
                throw new Error('Topology sorting failed');
            }

            // Calculate amounts
            const holdings = new Map(this.holdingItems);
            const depths = new Map<ItemID, number>();
            for (const slot of sorted) {
                // calculate needs
                const r = slot.requiredNumber(); // required
                const h = holdings.get(slot.item.id) ?? 0; // holding
                const use = Math.min(r, h);
                holdings.set(slot.item.id, h - use);
                const n = r - use; // needs
                slot.type =
                    r > 0 ? (n > 0 ? 'required' : 'completed') : 'not-required';
                // find recipe
                const recipes = await this.findRecipe(
                    ds,
                    slot.item.id,
                    slot.item.name,
                );
                if (recipes.length == 0) {
                    slot.wasted = 0;
                    continue;
                }
                const recipe = recipes[0];
                if (recipe.item_amount == undefined)
                    throw 'unsupported data source';
                const crafts = Math.ceil(n / recipe.item_amount);
                slot.wasted = recipe.item_amount * crafts - n;

                for (const ing of await this.fetchIngredients(ds, recipe.id)) {
                    const subSlot = ings.get(ing.ingredient_id)!;
                    subSlot.addRequiredBy(slot.item.id, crafts * ing.amount);

                    depths.set(
                        subSlot.item.id,
                        Math.max(
                            depths.get(subSlot.item.id) ?? 0,
                            (depths.get(slot.item.id) ?? 0) + 1,
                        ),
                    );
                }
            }

            // Coloring / Depthing
            for (const slot of sorted.toReversed()) {
                slot.depth = depths.get(slot.item.id) ?? 0;
                if (slot.type != 'required') {
                    continue;
                }
                const ss = successors.get(slot.item.id);
                const isEnough = (s: number) => {
                    const typ = ings.get(s)?.type;
                    return typ == 'completed' || typ == 'crafted';
                };
                if (ss != undefined && ss.every(isEnough)) {
                    slot.type = 'crafted';
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

        async fetchIngredients(dataSource: DataSource, recipeId: RecipeID) {
            let result = this.ingredientsCache.get(recipeId);
            if (result == undefined) {
                result = await dataSource.recipesIngredients(recipeId);
                this.ingredientsCache.set(recipeId, result);
            }
            return result.filter(v => v.ingredient_id >= 20); // 过滤偏属性水晶
        },

        async fetchItemInfo(
            dataSource: DataSource,
            itemId: ItemID,
        ): Promise<Item> {
            let result = this.itemInfoCache.get(itemId);
            if (result == undefined) {
                result = await dataSource.itemInfo(itemId);
                this.itemInfoCache.set(itemId, result);
            }
            return result;
        },
    },
});
