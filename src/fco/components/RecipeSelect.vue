<!-- 
    This file is part of BestCraft.
    Copyright (C) 2024  Tnze

    BestCraft is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    BestCraft is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
-->

<script setup lang="ts">
import { ref } from 'vue';
import { NModal, NCard, NSelect, SelectOption, useMessage, useLoadingBar } from 'naive-ui';

import useFcoSimulatorStore from '../stores/simulator';
import { WebSource, YYYYGamesApiBase } from '@/components/recipe-manager/web-source';
import { RecipesSourceResult } from '@/components/recipe-manager/source';
import { newRecipe, RecipeInfo } from '@/libs/Craft';
import JobSelect from './JobSelect.vue';

const loadingRecipeList = ref(false);
const options = ref<SelectOption[]>([]);
const store = useFcoSimulatorStore();
const message = useMessage();
const loadingBar = useLoadingBar();

const dataSource = new WebSource(YYYYGamesApiBase);
var lastResult: RecipesSourceResult | undefined;
var currentPage = 1;
var currentQuery: string | undefined = undefined;
var recipeInfos: Map<number, RecipeInfo> = new Map();

async function loadOptions() {
    if (loadingRecipeList.value) {
        return;
    }
    loadingRecipeList.value = true;
    const query = currentQuery;
    try {
        if (!lastResult) {
            lastResult = await dataSource.recipeTable(currentPage, query);
        } else if (lastResult.next) {
            lastResult = await lastResult.next();
        } else if (currentPage < lastResult.totalPages) {
            currentPage += 1;
            lastResult = await dataSource.recipeTable(currentPage, query);
        } else {
            return;
        }
        if (currentQuery == query) {
            options.value = options.value.concat(
                lastResult.results.map(
                    recipe => ({
                        label: recipe.item_name,
                        value: recipe.id,
                    })
                )
            );
            for (const recipeInfo of lastResult.results) {
                recipeInfos.set(recipeInfo.id, recipeInfo)
            }
        }
    } catch (e: any) {
        message.error(String(e));
    } finally {
        if (currentQuery == query) {
            loadingRecipeList.value = false;
        }
    }
}
loadOptions()

function handleScroll(e: Event) {
    const currentTarget = e.currentTarget as HTMLElement
    if (
        currentTarget.scrollTop + currentTarget.offsetHeight
        >= currentTarget.scrollHeight - 100
    ) {
        loadOptions();
    }
}

function setQuery(query: string) {
    if (currentQuery != query) {
        lastResult = undefined;
        currentQuery = query;
        options.value.splice(0);
        loadingRecipeList.value = false;
        loadOptions();
    }
}

async function updateValue(value: number) {
    const recipeInfo = recipeInfos.get(value);
    if (recipeInfo == undefined) {
        message.error('Invalid Selection');
        return;
    }
    try {
        loadingBar.start();
        var [recipeLevel, _info] = await Promise.all([
            dataSource.recipeLevelTable(recipeInfo.rlv),
            dataSource.itemInfo(recipeInfo.item_id)
        ])
        const recipe = await newRecipe(
            recipeInfo.rlv,
            recipeLevel,
            recipeInfo.difficulty_factor,
            recipeInfo.quality_factor,
            recipeInfo.durability_factor
        )
        store.recipe = { recipe, recipeLevel, recipeInfo }
        loadingBar.finish();
    } catch (e: any) {
        message.error(String(e))
        loadingBar.error();
    }
}

</script>
<template>
    <n-modal style="width: 90%;">
        <n-card>
            <JobSelect />
            <n-select :options="options" :loading="loadingRecipeList" :reset-menu-on-options-change="false"
                @scroll="handleScroll" @search="setQuery" filterable remote :placeholder="$t('select-recipe')"
                @update-value="updateValue"></n-select>
        </n-card>
    </n-modal>
</template>

<fluent locale="zh-CN">
select-recipe = 选择配方
</fluent>

<fluent locale="en-US">
select-recipe = Select Recipe
</fluent>

<fluent locale="ja-JP">
</fluent>
