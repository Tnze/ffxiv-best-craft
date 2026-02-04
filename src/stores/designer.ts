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
    Item,
    Jobs,
    Recipe,
    RecipeRequirements,
} from '@/libs/Craft';
import { defineStore } from 'pinia';

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
            stellarSteadyHandCount: number;
        } | null,
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
            stellarSteadyHandCount: number;
        }) {
            this.content = payload;
        },

        fromJson(json: string) {
            try {
                const v = JSON.parse(json);
                if (v.options) {
                    this.$patch({ options: v.options });
                }
            } catch (e: any) {
                console.error(e);
            }
        },
    },
});
