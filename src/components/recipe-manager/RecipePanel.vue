<!-- 
    This file is part of BestCraft.
    Copyright (C) 2025  Tnze

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
import { ref, reactive, watch, onMounted, onActivated, watchEffect } from 'vue';
import {
    ElInput,
    ElButton,
    ElTable,
    ElTableColumn,
    ElPagination,
    ElMessage,
    ElForm,
    ElFormItem,
    ElSelect,
    ElOption,
    ElInputNumber,
} from 'element-plus';
import { EditPen } from '@element-plus/icons-vue';
import {
    CollectablesShopRefine,
    Item,
    newRecipe,
    Recipe,
    RecipeInfo,
} from '@/libs/Craft';
import { useRouter } from 'vue-router';
import { useFluent } from 'fluent-vue';
import {
    CraftType,
    DataSource,
    DataSourceType,
    RecipesSourceResult,
} from '@/datasource/source';
import useSettingsStore from '@/stores/settings';
import { useMediaQuery } from '@vueuse/core';
import ConfirmDialog from './ConfirmDialog.vue';

const emit = defineEmits<{
    (e: 'setTitle', title: string): void;
}>();
onActivated(() => emit('setTitle', 'select-recipe'));

const searchingDelayMs = 200;
const settingStore = useSettingsStore();
const router = useRouter();
const { $t } = useFluent();

const searchText = ref('');
const pagination = reactive({
    Page: 1,
    PageTotal: 1,
});
const infiniteRecipeNext = ref<() => Promise<RecipesSourceResult>>();
const displayTable = ref<RecipeInfo[]>([]);
const isRecipeTableLoading = ref(false);
const compactLayout = useMediaQuery('screen and (max-width: 500px)');
const filterCraftType = ref<number>();
const filterLevel = ref<number>();
const craftTypeOptions = ref<CraftType[]>([]);
const filterRecipeLevel = ref<number>();
const stellarSteadyHandCount = ref<number>(0);

async function craftTypeRemoteMethod() {
    const source = await settingStore.getDataSource();
    filterCraftType.value = undefined;
    craftTypeOptions.value = await source.craftTypeList();
}

let loadRecipeTableResult: Promise<{
    results: RecipeInfo[];
    totalPages: number;
}> | null = null;
async function updateRecipePage(
    dataSource: DataSource,
    pageNumber: number,
    searching: string,
) {
    // 对于已有缓存的加载会很快，只有较慢的情况才需要显示Loading
    let timer = setTimeout(() => (isRecipeTableLoading.value = true), 200);
    try {
        let promise = dataSource.recipeTable(
            pageNumber,
            searching,
            filterRecipeLevel.value,
            filterCraftType.value,
            filterLevel.value ? filterLevel.value * 10 - 9 : undefined,
            filterLevel.value ? filterLevel.value * 10 : undefined,
        );
        loadRecipeTableResult = promise;
        let { results, totalPages, next } = await promise;
        if (loadRecipeTableResult == promise) {
            displayTable.value = results;
            pagination.PageTotal = totalPages;
            loadRecipeTableResult = null;
            infiniteRecipeNext.value = next;
        }
    } catch (e: any) {
        ElMessage.error(String(e));
    } finally {
        clearTimeout(timer);
        isRecipeTableLoading.value = false;
    }
}

async function infiniteRecipeLoad() {
    if (infiniteRecipeNext.value == undefined) {
        return;
    }
    let timer = setTimeout(() => (isRecipeTableLoading.value = true), 20);
    try {
        const promise = infiniteRecipeNext.value();
        loadRecipeTableResult = promise;
        let { results, totalPages, next } = await promise;
        if (loadRecipeTableResult == promise) {
            displayTable.value = displayTable.value.concat(results);
            pagination.PageTotal = totalPages;
            loadRecipeTableResult = null;
            infiniteRecipeNext.value = next;
        }
    } catch (e: any) {
        ElMessage.error(String(e));
    } finally {
        clearTimeout(timer);
        isRecipeTableLoading.value = false;
    }
}

// 搜索时更新
let searchTimer: any = null;
watch(searchText, async searching => {
    const source = await settingStore.getDataSource();
    if (searchTimer != null) {
        clearTimeout(searchTimer);
    }
    switch (source.sourceType) {
        case DataSourceType.Realtime:
            updateRecipePage(source, pagination.Page, searching);
            break;
        case DataSourceType.RemoteRealtime:
            searchTimer = setTimeout(() => {
                pagination.Page = 1; // 触发搜索时应该翻回第一页，否则搜不到东西
                updateRecipePage(source, pagination.Page, searching);
                searchTimer = null;
            }, searchingDelayMs);
            break;
    }
});

// 翻页时更新
watch(
    () => pagination.Page,
    async pageNumber => {
        const source = await settingStore.getDataSource();
        await updateRecipePage(source, pageNumber, searchText.value);
    },
);

// 回车手动更新
async function triggerSearch() {
    const source = await settingStore.getDataSource();
    const pageNumber = pagination.Page;
    const searching = searchText.value;
    pagination.Page = 1; // 触发搜索时应该翻回第一页，否则搜不到东西
    await updateRecipePage(source, pageNumber, searching);
}

// 页面载入后更新
onMounted(async () => {
    triggerSearch();
    craftTypeRemoteMethod();
});

// 数据源切换时更新
watch(
    () => [settingStore.dataSource, settingStore.dataSourceLang],
    () => {
        triggerSearch();
        craftTypeRemoteMethod();
    },
);

// 接受跳转参数
watchEffect(() => {
    const recipeId = router.currentRoute.value.query.recipeId;
    if (recipeId !== undefined) {
        selectRecipeById(Number(recipeId));
    }
});

const confirmDialogVisible = ref(false);
const recipe = ref<Recipe>();
const recipeInfo = ref<RecipeInfo>();
const itemInfo = ref<Item>();
const collectability = ref<CollectablesShopRefine>();

async function selectRecipeRow(row: RecipeInfo) {
    try {
        isRecipeTableLoading.value = true;
        const source = await settingStore.getDataSource();

        var [recipeLevel, itemInfoTmp, collectabilityTmp, temporaryActionInfo] =
            await Promise.all([
                source.recipeLevelTable(row.rlv),
                source.itemInfo(row.item_id),
                (async () => {
                    if (source.recipeCollectableShopRefine == undefined) {
                        return undefined;
                    }
                    try {
                        return await source.recipeCollectableShopRefine(row.id);
                    } catch (e: any) {
                        console.error(
                            'Failed to fatch recipe collectability',
                            e,
                        );
                        return undefined; // in case the server doesn't support or any other situation;
                    }
                })(),
                source.temporaryActionInfo
                    ? source.temporaryActionInfo(row.id)
                    : undefined,
            ]);
    } catch (e: any) {
        ElMessage.error(String(e));
        return;
    } finally {
        isRecipeTableLoading.value = false;
    }
    recipe.value = await newRecipe(
        recipeLevel,
        row.difficulty_factor,
        row.quality_factor,
        row.durability_factor,
    );
    recipeInfo.value = row;
    itemInfo.value = itemInfoTmp;
    collectability.value = collectabilityTmp;
    confirmDialogVisible.value = true;
    stellarSteadyHandCount.value =
        temporaryActionInfo?.action == 46843 ? temporaryActionInfo.count : 0;
}

async function selectRecipeById(recipeId: number) {
    const source = await settingStore.getDataSource();
    if (source.recipeInfo == undefined) {
        ElMessage.error($t('datasource-unsupport-recipe-info'));
        return;
    }
    try {
        isRecipeTableLoading.value = true;
        var recipeInfo = await source.recipeInfo(recipeId);
        // isRecipeTableLoading.value = false; // Done by selectRecipeRow()
    } catch (e: any) {
        ElMessage.error($t('select-recipe-by-id-error', { err: String(e) }));
        isRecipeTableLoading.value = false;
        return;
    }
    await selectRecipeRow(recipeInfo);
}
</script>

<template>
    <div class="container">
        <ConfirmDialog
            v-if="recipe && recipeInfo && itemInfo"
            v-model="confirmDialogVisible"
            v-model:recipe="recipe"
            :recipe-info="recipeInfo"
            :item-info="itemInfo"
            :collectability="collectability"
            :stellarSteadyHandCount="stellarSteadyHandCount"
        />
        <el-input
            v-model="searchText"
            @keydown.enter="triggerSearch"
            class="search-input"
            :placeholder="$t('search')"
            clearable
        >
            <template #append>
                <el-button
                    :icon="EditPen"
                    @click="router.push('/recipe/customize')"
                >
                    {{ $t('custom-recipe') }}
                </el-button>
            </template>
        </el-input>
        <el-form :inline="true">
            <el-form-item :label="$t('craft-type')">
                <el-select
                    v-model="filterCraftType"
                    clearable
                    :remote-method="craftTypeRemoteMethod"
                    style="width: 180px"
                    @change="triggerSearch"
                >
                    <el-option
                        v-for="{ id, name } in craftTypeOptions"
                        :key="id"
                        :value="id"
                        :label="name"
                    />
                </el-select>
            </el-form-item>
            <el-form-item :label="$t('level')">
                <el-select
                    v-model="filterLevel"
                    style="width: 100px"
                    @change="triggerSearch"
                    clearable
                >
                    <el-option
                        v-for="i in 10"
                        :key="i"
                        :value="i"
                        :label="`${i * 10 - 9} ~ ${i * 10}`"
                    />
                </el-select>
            </el-form-item>
            <el-form-item :label="$t('recipe-level')">
                <el-input-number
                    v-model="filterRecipeLevel"
                    style="width: 100px"
                    clearable
                    :min="1"
                    :max="799"
                    :step="1"
                    step-strictly
                    :controls="false"
                    @change="triggerSearch"
                />
            </el-form-item>
        </el-form>
        <el-table
            v-tnze-loading="isRecipeTableLoading"
            :element-loading-text="$t('please-wait')"
            highlight-current-row
            @row-click="selectRecipeRow"
            :data="displayTable"
            height="100%"
            style="width: 100%"
            v-el-table-infinite-scroll="infiniteRecipeLoad"
            :infinite-scroll-disabled="infiniteRecipeNext == undefined"
            :infinite-scroll-immediate="false"
            :infinite-scroll-distance="100"
        >
            <el-table-column
                prop="id"
                label="ID"
                :width="compactLayout ? undefined : 100"
            />
            <el-table-column
                prop="rlv"
                :label="$t('recipe-level')"
                :width="compactLayout ? undefined : 100"
            />
            <el-table-column
                prop="job"
                :label="$t('type')"
                :width="compactLayout ? undefined : 200"
            />
            <el-table-column prop="item_name" :label="$t('name')" />
        </el-table>
        <el-pagination
            v-if="pagination.PageTotal > 1"
            layout="prev, pager, next"
            v-model:current-page="pagination.Page"
            :page-count="pagination.PageTotal"
        />
    </div>
</template>

<style scoped>
.container {
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    background-color: transparent !important;
}

.search-input {
    margin: 10px 0;
    width: 80%;
}

.el-table {
    user-select: none;
    --el-fill-color-blank: transparent;
}

.el-pagination {
    justify-content: center;
    /* margin-bottom: 10px; */
    --el-fill-color-blank: transparent;
}
</style>

<fluent locale="zh-CN">
datasource-unsupport-recipe-info = 当前数据源不支持从外部选择配方
select-recipe-by-id-error = 获取配方信息失败：{ $err }，请尝试切换数据源

search = 键入以搜索
please-wait = 请稍等...

type = 类型
craft-type = 制作类型
level = 等级
name = 名称
can-hq = 存在HQ
</fluent>

<fluent locale="zh-TW">
datasource-unsupport-recipe-info = 當前資料來源不支援從外部選擇配方
select-recipe-by-id-error = 獲取配方資訊失敗：{ $err }，請嘗試切換資料來源

search = 鍵入以搜尋
please-wait = 請稍等...

type = 職業
craft-type = 製作職業
level = 等級
name = 名稱
can-hq = 存在HQ
</fluent>

<fluent locale="en-US">
datasource-unsupport-recipe-info = Current data-source doesn't support choice recipe from external pages
select-recipe-by-id-error = Error fetching recipe data: { $err }. Please try choosing another DataSource

search = Search
please-wait = Please wait...

type = Type
craft-type = Craft Type
level = Level
name = Name
can-hq = Can HQ
</fluent>
