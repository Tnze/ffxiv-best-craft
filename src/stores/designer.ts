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

import {
    CollectablesShopRefine,
    compareStatus,
    Item,
    Jobs,
    Recipe,
    RecipeRequirements,
    simulate,
    SimulateResult,
    Status,
} from '@/libs/Craft';
import { defineStore } from 'pinia';
import { Sequence } from '@/components/designer/types';

type StagedSequence = { key: number; seq: Sequence };

export default defineStore('designer', {
    state: () => ({
        content: null as {
            item: Item;
            job?: Jobs;
            recipe: Recipe;
            recipeId?: number;
            materialQualityFactor: number;
            requirements: RecipeRequirements;
            collectability?: CollectablesShopRefine;
            simulatorMode: boolean;
        } | null,
        rotations: {
            staged: <StagedSequence[]>[],
            maxid: 0,
        },
        options: {
            exportOptions: {
                addNotification: <boolean | 'auto'>'auto', // 宏执行完成是否提示
                sectionMethod: <'avg' | 'greedy' | 'disable'>'avg', // 宏分块的方式
                notifySound: ' <se.1>', // 宏完成提示音
                hasLock: false, // 添加锁定宏语句
                waitTimeInc: 0, // 增加等待时间(秒)
                oneclickCopy: true, // 一键复制
            },
            importOptions: {
                strictMode: false,
            },
            analyzerOptions: {
                ignoreErrors: true,
            },
        },
    }),
    getters: {
        toJson(): string {
            return JSON.stringify({
                rotations: this.rotations,
                options: this.options,
            });
        },
    },
    actions: {
        selectRecipe(payload: {
            job?: Jobs;
            item: Item;
            recipe: Recipe;
            recipeId?: number;
            materialQualityFactor: number;
            requirements: RecipeRequirements;
            collectability?: CollectablesShopRefine;
            simulatorMode: boolean;
        }) {
            this.content = payload;
        },

        pushRotation(seq: Sequence) {
            const key = this.rotations.maxid++;
            this.rotations.staged.push({ key, seq });
        },

        deleteRotation(i: number) {
            this.rotations.staged.splice(i, 1);
        },

        clearRotations() {
            this.rotations.staged.length = 0;
        },

        truncateRotations(n: number) {
            this.rotations.staged.length = n;
        },

        async sortRotations(initStatus: Status) {
            const results = new Map<number, SimulateResult>();
            await Promise.all(
                this.rotations.staged.map(async v => {
                    const actions = v.seq.slots.map(v => v.action);
                    const result = await simulate(initStatus, actions);
                    results.set(v.key, result);
                }),
            );
            this.rotations.staged.sort((a, b) => {
                const as = results.get(a.key)!.status;
                const bs = results.get(b.key)!.status;
                return compareStatus(bs, as) ?? a.key - b.key;
            });
        },

        fromJson(json: string) {
            try {
                const v = JSON.parse(json);
                if (v.rotations) this.rotations = v.rotations;
                if (v.options) {
                    this.$patch({ options: v.options });
                }
            } catch (e: any) {
                console.error(e);
            }
        },
    },
});
