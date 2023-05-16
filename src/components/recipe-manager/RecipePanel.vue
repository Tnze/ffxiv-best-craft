<script setup lang="ts">
import { ref, watchEffect, reactive } from 'vue'
import { ElContainer, ElHeader, ElMain, ElForm, ElFormItem, ElInput, ElInputNumber, ElButton, ElDialog, ElTable, ElTableColumn, ElPagination, ElMessage, ElMessageBox } from 'element-plus'
import { EditPen } from '@element-plus/icons-vue'
import { Jobs, Recipe, newRecipe, recipeTable, RecipeInfo, Item, itemInfo } from '../../Craft'
import { useRouter } from 'vue-router';
import { useChecklistStore, useDesignerStore } from '../../store';
import { useFluent } from 'fluent-vue';

const checklistStore = useChecklistStore()
const designerStore = useDesignerStore()
const router = useRouter()
const { $t } = useFluent()

const jobMaps: { [key: string]: Jobs } = {
    '木工': Jobs.Carpenter,
    '锻冶': Jobs.Blacksmith,
    '铸甲': Jobs.Armorer,
    '雕金': Jobs.Goldsmith,
    '制革': Jobs.Leatherworker,
    '裁缝': Jobs.Weaver,
    '炼金': Jobs.Alchemist,
    '烹调': Jobs.Culinarian,
}

const searchText = ref('')
const pagination = reactive({
    Page: 1,
    PageTotal: 1
})
const displayTable = ref<RecipeInfo[]>([])
watchEffect(async () => {
    let [list, totalPages] = await recipeTable(pagination.Page, searchText.value)
    displayTable.value = list
    pagination.PageTotal = totalPages
})


const openCustomlizer = ref(false)

const selectRecipeRow = async (row: RecipeInfo) => {
    try {
        await ElMessageBox.confirm(
            $t('confirm-select', { itemName: row.item_name }),
            $t('please-confirm'),
            { type: 'warning' }
        )
        const recipe = await newRecipe(
            row.rlv,
            row.difficulty_factor,
            row.quality_factor,
            row.durability_factor
        )
        selectRecipe(recipe, row, await itemInfo(row.item_id), row.job)
        checklistStore.addToChecklist({ ingredient_id: row.item_id, amount: 1 })
    } catch {
        // operation canceled by user
    }
}

const selectRecipe = (recipe: Recipe, recipeInfo: RecipeInfo | undefined, item: Item, craftType: string) => {
    designerStore.selectRecipe({
        job: jobMaps[craftType] ?? Jobs.Culinarian,
        item,
        recipe,
        recipeInfo
    })
    router.push({ name: "designer" })
    ElMessage({
        type: 'success',
        showClose: true,
        message: $t('recipe-setting-changed')
    })
}

const customRecipe = ref({
    rlv: 611,
    job_level: 90,
    difficulty: 7480,
    quality: 13620,
    durability: 60,
    conditions_flag: 435,
})

</script>

<template>
    <el-container>
        <el-header>
            <h1>{{ $t('select-recipe') }}</h1>
        </el-header>
        <el-main class="container">
            <el-dialog v-model="openCustomlizer" :title="$t('custom-recipe', { rlv: customRecipe.rlv })">
                <el-form :model="customRecipe" label-position="right" label-width="100px" style="max-width: 460px">
                    <el-form-item :label="$t('recipe-level')">
                        <el-input-number v-model="customRecipe.rlv" :min="1"></el-input-number>
                    </el-form-item>
                    <el-form-item :label="$t('level')">
                        <el-input-number v-model="customRecipe.job_level" :min="1"></el-input-number>
                    </el-form-item>
                    <el-form-item :label="$t('difficulty')">
                        <el-input-number v-model="customRecipe.difficulty" :min="1"></el-input-number>
                    </el-form-item>
                    <el-form-item :label="$t('quality')">
                        <el-input-number v-model="customRecipe.quality" :min="1"></el-input-number>
                    </el-form-item>
                    <el-form-item :label="$t('durability')">
                        <el-input-number v-model="customRecipe.durability" :min="1"></el-input-number>
                    </el-form-item>
                    <el-form-item :label="$t('conditions-flag')">
                        <el-input-number v-model="customRecipe.conditions_flag"></el-input-number>
                    </el-form-item>
                </el-form>
                <template #footer>
                    <span class="dialog-footer">
                        <el-button @click="openCustomlizer = false">{{ $t('cancel') }}</el-button>
                        <el-button type="primary" @click="openCustomlizer = false; selectRecipe(customRecipe, undefined, {
                            id: -1,
                            name: $t('custom-recipe', { rlv: customRecipe.rlv }),
                            level: customRecipe.rlv,
                            can_be_hq: true
                        }, '')">
                            {{ $t('confirm') }}
                        </el-button>
                    </span>
                </template>
            </el-dialog>
            <el-input v-model="searchText" class="search-input" :placeholder="$t('search')" clearable>
                <template #append>
                    <el-button :icon="EditPen" @click="openCustomlizer = true" />
                </template>
            </el-input>
            <el-table :element-loading-text="$t('please-wait')" highlight-current-row @row-click="selectRecipeRow"
                :data="displayTable" height="100%" style="width: 100%">
                <el-table-column prop="id" label="ID" width="100" />
                <el-table-column prop="rlv" :label="$t('recipe-level')" width="100" />
                <!-- <el-table-column prop="Icon" label="图标" width="55">
                    <template #default="scope">
                        <div style="display: flex; align-items: center">
                            <el-image :src="'https://garlandtools.cn/files/icons/item/' + scope.row.number +'.png'" />
                        </div>
                    </template>
                </el-table-column> -->
                <el-table-column prop="job" :label="$t('type')" width="70" />
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
    --el-table-header-bg-color: transparent;
}

.el-pagination {
    justify-content: center;
    /* margin-bottom: 10px; */
}
</style>

<fluent locale="zh-CN">
confirm-select = 将当前配方设置为“{ $itemName }”吗？
please-confirm = 请确认
recipe-setting-changed = 配方设置已变更
select-recipe = 选择配方
custom-recipe = 自定义配方 #{ $rlv }
conditions-flag = 球色标志

cancel = 取消
confirm = 确认

search = 键入以搜索
please-wait = 请稍等...

type = 类型
name = 名称
</fluent>

<fluent locale="en-US">
confirm-select = Set current recipe to "{ $itemName }"?
please-confirm = Please confirm
recipe-setting-changed = Selected recipe is updated
select-recipe = Select Recipe
custom-recipe = Custom Recipe #{ $rlv }
conditions-flag = Cond. Flag

cancel = Cancel
confirm = Confirm

search = Search
please-wait = Please wait...

type = Type
name = Name
</fluent>
