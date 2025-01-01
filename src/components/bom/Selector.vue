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
import { ref, useTemplateRef, watch } from 'vue';
import {
    ElTable,
    ElTableColumn,
    ElInput,
    ElSpace,
    ElButton,
} from 'element-plus';
import type { TableInstance } from 'element-plus';
import { Search } from '@element-plus/icons-vue';

import useStore, { Item } from '@/stores/bom';
import useSettingStore from '@/stores/settings';
import { RecipeInfo } from '@/libs/Craft';

const emits = defineEmits<{
    clickItem: [item: Item];
    selectItems: [items: Item[]];
}>();

const store = useStore();
const settingStore = useSettingStore();

const search = ref('');
const recipeList = ref<RecipeInfo[]>([]);
const isLoading = ref(false);
const tableInstance = useTemplateRef<TableInstance>('table');

async function update() {
    try {
        isLoading.value = true;
        const source = await settingStore.getDataSource;
        const recipeTable = await source.recipeTable(1, search.value);
        recipeList.value = recipeTable.results;
    } finally {
        isLoading.value = false;
    }
}
watch(search, () => update(), { immediate: true });

const selected = ref<RecipeInfo[]>([]);
function selectionChanged(rows: RecipeInfo[]) {
    selected.value = rows;
}
function selectionConfirmed() {
    const items = selected.value.map(v => ({
        id: v.item_id,
        name: v.item_name,
    }));
    emits('selectItems', items);
    clearSelections();
}

async function selectItem(recipe: RecipeInfo) {
    emits('clickItem', { id: recipe.item_id, name: recipe.item_name });
    clearSelections();
}

function clearSelections() {
    selected.value.splice(0);
    tableInstance.value!.clearSelection();
}
</script>

<template>
    <el-table
        ref="table"
        :data="recipeList"
        @row-click="selectItem"
        max-height="300"
        v-tnze-loading="isLoading"
        @selection-change="selectionChanged"
    >
        <el-table-column type="selection" width="55" />
        <el-table-column prop="job" :label="$t('craft-type')" width="150" />
        <el-table-column prop="item_name" :label="$t('name')">
            <template #header>
                <el-space>
                    <span>{{ $t('name') }}</span>
                    <el-input
                        v-model="search"
                        size="small"
                        :placeholder="$t('type-to-search')"
                        :prefix-icon="Search"
                        style="width: 200px"
                    />
                </el-space>
            </template>
        </el-table-column>
    </el-table>
    <div class="buttons" v-if="selected.length > 0">
        <el-button type="primary" @click="selectionConfirmed">
            {{ $t('confirm') }}
        </el-button>
    </div>
</template>

<style scoped>
.el-table {
    user-select: none;
    border-top-left-radius: var(--tnze-content-raduis);
    --el-table-header-bg-color: transparent;
    --el-table-tr-bg-color: transparent;
}

.buttons {
    margin-top: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
}
</style>

<fluent locale="zh-CN">
craft-type = 制作类型
name = 名称

type-to-search = 输入以搜索
confirm = 确认
</fluent>

<fluent locale="en-US">
craft-type = Craft Type
name = Name

type-to-search = Type to search
confirm = Confirm
</fluent>
