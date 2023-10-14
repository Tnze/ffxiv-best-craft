<script setup lang="ts">
import { ref, watchEffect, reactive } from 'vue'
import { ElContainer, ElHeader, ElMain, ElInput, ElButton, ElDialog, ElTable, ElTableColumn, ElPagination, ElMessage, ElMessageBox } from 'element-plus'
import { EditPen } from '@element-plus/icons-vue'
import { newRecipe, RecipeInfo } from '../../Craft'
import { useRouter } from 'vue-router';
import { useChecklistStore, useSettingsStore } from '../../store';
import { useFluent } from 'fluent-vue';
import { selectRecipe } from './common';


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
watchEffect(async () => {
    try {
        isRecipeTableLoading.value = true
        let promise = settingStore.getDataSource.recipeTable(pagination.Page, searchText.value)
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
        isRecipeTableLoading.value = false
    }
})

const confirmDialogVisible = ref(false)
let confirmDialogCallback: ((mode: 'designer' | 'simulator') => void) | null = null

const selectRecipeRow = async (row: RecipeInfo) => {
    try {
        isRecipeTableLoading.value = true
        var [recipeLevel, info] = await Promise.all([
            settingStore.getDataSource.recipeLevelTable(row.rlv),
            settingStore.getDataSource.itemInfo(row.item_id)
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
            <el-input v-model="searchText" class="search-input" :placeholder="$t('search')" clearable>
                <template #append>
                    <el-button :icon="EditPen" @click="router.push('/recipe/customize')" />
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
