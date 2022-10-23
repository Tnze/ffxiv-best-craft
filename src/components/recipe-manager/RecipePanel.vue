<script setup lang="ts">
import { ref, watchEffect, reactive } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { EditPen } from '@element-plus/icons-vue'
import { Jobs, Recipe, newRecipe, recipeTable, RecipeRow } from '../../Craft'
import { useRouter } from 'vue-router';
import { useStore } from '../../store';

const store = useStore()
const router = useRouter()

const jobMaps: { [key: string]: Jobs | 'unknown' } = {
    '木工': Jobs.Carpenter,
    '锻冶': Jobs.Blacksmith,
    '铸甲': Jobs.Armorer,
    '雕金': Jobs.Goldsmith,
    '制革': Jobs.Leatherworker,
    '裁缝': Jobs.Weaver,
    '炼金': Jobs.Alchemist,
    '烹调': Jobs.Culinarian,
    '自定义': 'unknown',
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
            `将当前配方设置为“${row.name}”吗？`,
            '请确认',
            {
                confirmButtonText: '确认',
                cancelButtonText: '取消',
                type: 'warning',
            }
        )
        const recipe = await newRecipe(
            row.rlv,
            row.difficulty_factor,
            row.quality_factor,
            row.durability_factor
        )
        selectRecipe(recipe, row.name, row.job)
    } catch {
        // operation canceled by user
    }
}

const selectRecipe = (recipe: Recipe, itemName: string, craftType: string) => {
    const job = jobMaps[craftType]
    console.log(job)
    store.commit('selectRecipe', { job, itemName, recipe })
    router.push({ name: "designer" })
    ElMessage({
        type: 'success',
        showClose: true,
        message: '配方设置已变更'
    })
}

const customRecipe = ref({
    rlv: 580,
    job_level: 90,
    difficulty: 3900,
    quality: 10920,
    durability: 70,
    conditions_flag: 15,
})

</script>

<template>
    <el-container>
        <el-header>
            <h1>选择配方</h1>
        </el-header>
        <el-main class="container">
            <el-dialog v-model="openCustomlizer" title="自定义配方">
                <el-form :model="customRecipe" label-position="right" label-width="100px" style="max-width: 460px">
                    <el-form-item label="rlv">
                        <el-input-number v-model="customRecipe.rlv" :min="1"></el-input-number>
                    </el-form-item>
                    <el-form-item label="等级">
                        <el-input-number v-model="customRecipe.job_level" :min="1"></el-input-number>
                    </el-form-item>
                    <el-form-item label="难度">
                        <el-input-number v-model="customRecipe.difficulty" :min="1"></el-input-number>
                    </el-form-item>
                    <el-form-item label="品质">
                        <el-input-number v-model="customRecipe.quality" :min="1"></el-input-number>
                    </el-form-item>
                    <el-form-item label="耐久">
                        <el-input-number v-model="customRecipe.durability" :min="1"></el-input-number>
                    </el-form-item>
                </el-form>
                <template #footer>
                    <span class="dialog-footer">
                        <el-button @click="openCustomlizer = false">取消</el-button>
                        <el-button type="primary"
                            @click="openCustomlizer = false; selectRecipe(customRecipe, 'Recipe#' + customRecipe.rlv, '自定义')">
                            确认
                        </el-button>
                    </span>
                </template>
            </el-dialog>
            <el-input v-model="searchText" class="search-input" placeholder="键入以搜索" clearable>
                <template #append>
                    <el-button :icon="EditPen" @click="openCustomlizer = true" />
                </template>
            </el-input>
            <el-table v-loading="displayTable == null" element-loading-text="请稍等..." highlight-current-row
                @row-click="selectRecipeRow" :data="displayTable" height="100%" style="width: 100%">
                <el-table-column prop="id" label="ID" />
                <el-table-column prop="rlv" label="配方等级" />
                <!-- <el-table-column prop="Icon" label="图标" width="55">
                    <template #default="scope">
                        <div style="display: flex; align-items: center">
                            <el-image :src="'https://garlandtools.cn/files/icons/item/' + scope.row.item +'.png'" />
                        </div>
                    </template>
                </el-table-column> -->
                <el-table-column prop="job" label="类型" />
                <el-table-column prop="name" label="名称" width="250" />
                <!-- <el-table-column prop="difficulty_factor" label="难度因子" /> -->
                <!-- <el-table-column prop="quality_factor" label="品质因子" /> -->
                <!-- <el-table-column prop="durability_factor" label="耐久因子" /> -->
                <el-table-column align="right" width="300">
                    <template #header>
                        <el-pagination small layout="prev, pager, next" v-model:current-page="pagination.Page"
                            :page-count="pagination.PageTotal" />
                    </template>
                </el-table-column>
            </el-table>
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
</style>