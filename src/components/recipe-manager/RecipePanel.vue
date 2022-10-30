<script setup lang="ts">
import { ref, watchEffect, reactive } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { EditPen } from '@element-plus/icons-vue'
import { Jobs, Recipe, newRecipe, recipeTable, RecipeRow } from '../../Craft'
import { useRouter } from 'vue-router';
import { useStore } from '../../store';
import { useFluent } from 'fluent-vue';

const store = useStore()
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
const displayTable = ref<RecipeRow[] | null>([])
watchEffect(async () => {
    let [list, totalPages] = await recipeTable(pagination.Page, searchText.value)
    displayTable.value = list
    pagination.PageTotal = totalPages
})


const openFilter = ref(false)
const openCustomlizer = ref(false)

const selectRecipeRow = async (row: RecipeRow) => {
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
        selectRecipe(recipe, row.item_name, row.job)
        store.commit('addToChecklist', { ingredient_id: row.item_id, amount: 1 })
    } catch {
        // operation canceled by user
    }
}

const selectRecipe = (recipe: Recipe, itemName: string, craftType: string) => {
    store.commit('selectRecipe', { job: jobMaps[craftType] ?? Jobs.Culinarian, itemName, recipe })
    router.push({ name: "designer" })
    ElMessage({
        type: 'success',
        showClose: true,
        message: $t('recipe-setting-changed')
    })
}

const customRecipe = ref({
    rlv: 610,
    job_level: 90,
    difficulty: 5060,
    quality: 12628,
    durability: 70,
    conditions_flag: 15,
})

</script>

<template>
    <el-container>
        <el-header>
            <h1>{{ $t('select-recipe') }}</h1>
        </el-header>
        <el-main class="container">
            <el-dialog v-model="openCustomlizer" :title="$t('custom-recipe')">
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
                </el-form>
                <template #footer>
                    <span class="dialog-footer">
                        <el-button @click="openCustomlizer = false">{{ $t('cancel') }}</el-button>
                        <el-button type="primary"
                            @click="openCustomlizer = false; selectRecipe(customRecipe, $t('custom-recipe', {rlv: customRecipe.rlv}), '')">
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
            <el-table v-loading="displayTable == null" :element-loading-text="$t('please-wait')" highlight-current-row
                @row-click="selectRecipeRow" :data="displayTable" height="100%" style="width: 100%">
                <el-table-column prop="id" label="ID" width="100" />
                <el-table-column prop="rlv" :label="$t('recipe-level')" width="100" />
                <!-- <el-table-column prop="Icon" label="图标" width="55">
                    <template #default="scope">
                        <div style="display: flex; align-items: center">
                            <el-image :src="'https://garlandtools.cn/files/icons/item/' + scope.row.item +'.png'" />
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

cancel = 取消
confirm = 确认

search = 键入以搜索
please-wait = 请稍等...

type = 类型
name = 名称
</fluent>

<fluent locale="en">
confirm-select = Set cuurent recipe to "{ $itemName }"?
please-confirm = Please confirm
recipe-setting-changed = Recipe is updated
select-recipe = Select Recipe
custom-recipe = Custom Recipe #{ $rlv }

cancel = Cancel
confirm = Confirm

search = Search
please-wait = Please wait...

type = Type
name = Name
</fluent>