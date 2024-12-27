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

export interface Slot {
    item: Item;
    requiredNumber(): number;
}

export interface RequiredBy {
    amount: number;
    depth: number;
}

class TargetSlot implements Slot {
    item: Item;
    required: number = 1;

    constructor(item: Item) {
        this.item = item;
    }

    requiredNumber(): number {
        return this.required;
    }

    setRequiredNumber(n: number) {
        this.required = n;
    }
}

class IngredientSlot implements Slot {
    item: Item;
    requiredBy: Map<ItemID, RequiredBy>;

    constructor(item: Item) {
        this.item = item;
        this.requiredBy = new Map();
    }

    requiredNumber(): number {
        let sum = 0;
        for (const req of this.requiredBy.values()) {
            sum += req.amount;
        }
        return sum;
    }

    addRequiredBy(item: ItemID, amount: number, depth: number) {
        const rb = this.requiredBy.get(item);
        if (rb != undefined) {
            rb.amount += amount;
            rb.depth = Math.max(rb.depth, depth);
            return;
        }

        this.requiredBy.set(item, {
            amount,
            depth,
        });
    }
}

export default defineStore('bom', {
    state: () => ({
        targetItems: <TargetSlot[]>[],
        holdingItems: new Map<ItemID, number>(), // item -> amount

        recipeCache: new Map<ItemID, RecipeInfo[]>(),
        ingredients: <IngredientSlot[]>[],
    }),

    actions: {
        addTarget(item: Item) {
            this.targetItems.push(new TargetSlot(item));
        },

        async updateBom() {
            const settingStore = useSettingStore();
            const ds = await settingStore.getDataSource;

            // 由于计算每种物品的需求数量需要先对整颗依赖树进行拓扑排序
            // 因此需要先在不考虑数量的情况下求出整颗依赖树
            const queue: Slot[] = [...this.targetItems];
            const ingredients = new Map<ItemID, IngredientSlot>();
            const holdings = new Map(this.holdingItems);
            while (queue.length > 0) {
                const v = queue.shift()!;
                let n = v.requiredNumber();

                const h = holdings.get(v.item.id);
                if (h != undefined && h > 0) {
                    const s = Math.min(n, h);
                    holdings.set(v.item.id, h - s);
                    n -= s;
                }

                const recipes = await this.findRecipe(
                    ds,
                    v.item.id,
                    v.item.name,
                );
                if (recipes.length == 0) {
                    // target item 是用户选择的目标物品，必须存在对应的配方
                    // TODO: 妥善处理没有对应配方的 target item
                    console.warn(`Cannot find a recipe for: ${v.item.name}`);
                    continue;
                }

                // TODO: 当存在多个配方时，允许用户选择使用哪个配方
                const r = recipes[0];
                const subIngs = await ds.recipesIngredients(r.id);

                const unknownIngs: ItemWithAmount[] = [];
                for (const subIng of subIngs) {
                    const slot = ingredients.get(subIng.ingredient_id);
                    if (slot != undefined) {
                        slot.addRequiredBy(v.item.id, n * subIng.amount, 0);
                    } else {
                        unknownIngs.push(subIng);
                    }
                }
                const fetched = await Promise.all(
                    unknownIngs.map(async v => ({
                        item: await ds.itemInfo(v.ingredient_id),
                        amount: v.amount,
                    })),
                );
                for (const { item, amount } of fetched) {
                    const slot = new IngredientSlot(item);
                    slot.addRequiredBy(v.item.id, n * amount, 0);
                    // fetched ings are promised not exist
                    ingredients.set(item.id, slot);
                }
            }

            // 基于目标物品的直接依赖物品扩展出整颗依赖树
            const queue2: IngredientSlot[] = [...ingredients.values()];
            while (queue2.length > 0) {
                const v = queue2.shift()!;
                
            }

            this.ingredients = ingredients.values().toArray();
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
