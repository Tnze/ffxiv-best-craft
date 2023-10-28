<!-- 
    This file is part of BestCraft.
    Copyright (C) 2023  <name of author>

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
import { ref, reactive, watch, onMounted } from 'vue'
import { ElContainer, ElHeader, ElMain, ElInput, ElButton, ElDialog, ElTable, ElTableColumn, ElPagination, ElMessage, ElMessageBox } from 'element-plus'
import { EditPen } from '@element-plus/icons-vue'
import { newRecipe, RecipeInfo } from '../../Craft'
import { useRouter } from 'vue-router';
import { useChecklistStore, useSettingsStore } from '../../store';
import { useFluent } from 'fluent-vue';
import { selectRecipe } from './common';
import { DataSource, DataSourceType } from './source';


// let jsonData = JSON.stringify(data.map(elem => ({
//     cm: elem.craftsmanship_percent,
//     cm_max: elem.craftsmanship_value,
//     cp: elem.cp_percent,
//     cp_max: elem.cp_value,
//     ct: elem.control_percent,
//     ct_max: elem.control_value,
//     name: elem.name.cn + (elem.hq ? " HQ" : "")
// })), null, '    ')

const checklistStore = useChecklistStore()
const settingStore = useSettingsStore()
const router = useRouter()
const { $t } = useFluent()

const searchText = ref('')
const pagination = reactive({
    Page: 1,
    PageTotal: 1
})
const displayTable = ref<RecipeInfo[]>([])
const isRecipeTableLoading = ref(false)

let loadRecipeTableResult: Promise<{ recipes: RecipeInfo[], totalPages: number }> | null = null

async function updateRecipePage(dataSource: DataSource, pageNumber: number, searching: string) {
    // 对于已有缓存的加载会很快，只有较慢的情况才需要显示Loading
    let timer = setTimeout(() => isRecipeTableLoading.value = true, 200)
    try {
        let promise = dataSource.recipeTable(pageNumber, searching)
        loadRecipeTableResult = promise
        let { recipes, totalPages } = await promise
        if (loadRecipeTableResult == promise) {
            displayTable.value = recipes
            pagination.PageTotal = totalPages
            loadRecipeTableResult = null
        }
    } catch (e: any) {
        ElMessage.error(String(e))
    } finally {
        clearTimeout(timer)
        isRecipeTableLoading.value = false
    }
}

// 搜索时更新
watch(searchText, async searching => {
    const source = await settingStore.getDataSource
    if (source.sourceType == DataSourceType.Realtime) {
        await updateRecipePage(source, pagination.Page, searching)
    }
})

// 翻页时更新
watch(() => pagination.Page, async pageNumber => {
    const source = await settingStore.getDataSource
    await updateRecipePage(source, pageNumber, searchText.value)
})

// 回车手动更新
async function triggerSearch() {
    const source = await settingStore.getDataSource
    const pageNumber = pagination.Page
    const searching = searchText.value
    await updateRecipePage(source, pageNumber, searching)
}

// 页面载入后更新
onMounted(triggerSearch)

const confirmDialogVisible = ref(false)
let confirmDialogCallback: ((mode: 'designer' | 'simulator') => void) | null = null

const selectRecipeRow = async (row: RecipeInfo) => {
    try {
        isRecipeTableLoading.value = true
        var [recipeLevel, info] = await Promise.all([
            (await settingStore.getDataSource).recipeLevelTable(row.rlv),
            (await settingStore.getDataSource).itemInfo(row.item_id)
        ])
    } catch (e: any) {
        ElMessage.error(String(e))
        return
    } finally {
        isRecipeTableLoading.value = false
    }
    const recipe = await newRecipe(
        row.rlv,
        recipeLevel,
        row.difficulty_factor,
        row.quality_factor,
        row.durability_factor
    )

    if ((recipe.conditions_flag & ~15) == 0) {
        try {
            await ElMessageBox.confirm(
                $t('confirm-select', { itemName: row.item_name }),
                $t('please-confirm'),
                { type: 'warning' }
            )
            selectRecipe(recipe, row.id, recipeLevel, row.material_quality_factor, row, info, row.job, false)
            router.push({ name: "designer" })
            checklistStore.addToChecklist({ ingredient_id: row.item_id, amount: 1 })
        } catch {
            // operation canceled by user
        }
    } else {
        confirmDialogCallback = (mode: 'designer' | 'simulator') => {
            selectRecipe(recipe, row.id, recipeLevel, row.material_quality_factor, row, info, row.job, mode == 'simulator')
            router.push({ name: "designer" })
            confirmDialogVisible.value = false
            confirmDialogCallback = null
        }
        confirmDialogVisible.value = true
    }
}

</script>

<template>
    <el-container>
        <el-header>
            <h1>{{ $t('select-recipe') }}</h1>
        </el-header>
        <el-main class="container">
            <el-dialog v-model="confirmDialogVisible" :title="$t('please-confirm')" :align-center="true">
                <span>
                    {{ $t('confirm-select2') }}
                </span>
                <template #footer>
                    <span>
                        <el-button @click="confirmDialogVisible = false">
                            {{ $t('cancel') }}
                        </el-button>
                        <el-button type="primary" @click=" confirmDialogCallback!('simulator')">
                            {{ $t('simulator-mode') }}
                        </el-button>
                        <el-button type="primary" @click=" confirmDialogCallback!('designer')">
                            {{ $t('designer-mode') }}
                        </el-button>
                    </span>
                </template>
            </el-dialog>
            <el-input v-model="searchText" @keydown.enter="triggerSearch" class="search-input" :placeholder="$t('search')"
                clearable>
                <template #append>
                    <el-button :icon="EditPen" @click="router.push('/recipe/customize')">{{ $t('custom-recipe') }}</el-button>
                </template>
            </el-input>
            <el-table v-tnze-loading="isRecipeTableLoading" :element-loading-text="$t('please-wait')" highlight-current-row
                @row-click="selectRecipeRow" :data="displayTable" height="100%" style="width: 100%">
                <el-table-column prop="id" label="ID" width="100" />
                <el-table-column prop="rlv" :label="$t('recipe-level')" width="100" />
                <!-- <el-table-column prop="Icon" la\bel="图标" width="55">
                    <template #default="scope">
                        <div style="display: flex; align-items: center">
                            <el-image :src="'https://garlandtools.cn/files/icons/item/' + scope.row.number +'.png'" />
                        </div>
                    </template>
                </el-table-column> -->
                <el-table-column prop="job" :label="$t('type')" width="200" />
                <el-table-column prop="item_name" :label="$t('name')" />
                <!-- <el-table-column prop="difficulty_factor" label="难度因子" /> -->
                <!-- <el-table-column prop="quality_factor" label="品质因子" /> -->
                <!-- <el-table-column prop="durability_factor" label="耐久因子" /> -->
            </el-table>
            <el-pagination layout="prev, pager, next" v-model:current-page="pagination.Page"
                :page-count="pagination.PageTotal" />
        </el-main>
    </el-container>
</template>

<style scoped >
.container {
    display: flex;
    flex-direction: column;
    align-items: center;
    background-color: transparent !important;
}

.search-input {
    width: 80%;
}

.el-table {
    user-select: none;
    border-top-left-radius: var(--tnze-content-raduis);
    --el-table-header-bg-color: transparent;
    --el-table-tr-bg-color: transparent;
}

.el-pagination {
    justify-content: center;
    /* margin-bottom: 10px; */
}
</style>

<fluent locale="zh-CN">
select-recipe = 选择配方
custom-recipe = 自定义配方

confirm-select = 确定要制作“{ $itemName }”吗？
confirm-select2 = 这是一个高难度配方，请选择模式。
please-confirm = 请确认

cancel = 取消
confirm = 确认
designer-mode = 普通模式
simulator-mode = 高难模式

search = 键入以搜索
please-wait = 请稍等...

type = 类型
name = 名称
</fluent>

<fluent locale="en-US">
select-recipe = Select Recipe
custom-recipe = Custom Recipe

confirm-select = Start crafting "{ $itemName }"?
confirm-select2 = This is a 高难度配方. Please make a choice.
please-confirm = Please confirm

cancel = Cancel
confirm = Confirm
designer-mode = Normal Mode
simulator-mode = Simulator Mode

search = Search
please-wait = Please wait...

type = Type
name = Name
</fluent>
