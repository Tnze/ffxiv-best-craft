<script setup lang="ts">
import { ref, computed, watchEffect, reactive } from 'vue'
import 'element-plus/es/components/message/style/css'
import 'element-plus/es/components/message-box/style/css'
import { ElMessage, ElMessageBox } from 'element-plus'
import { EditPen } from '@element-plus/icons-vue'
import { Jobs, Recipe, newRecipe } from '../../Craft'

const xivapiBase = "https://cafemaker.wakingsands.com"

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

interface XivapiRecipe {
    ID: number;
    Icon: string;
    Name: string;
    CraftType: {
        Name: string // "铸甲"
    };
    DifficultyFactor: number// 100,
    DurabilityFactor: number// 100,
    QualityFactor: number,
    RecipeLevelTable: {
        ID: number // 560
    }
}

const pagination = reactive({
    Page: 1,
    PageTotal: 1
})
const displayTable = ref<XivapiRecipe[] | null>([])
watchEffect(async () => {
    displayTable.value = null
    const response = await fetch(xivapiBase + `/search?indexes=Recipe&page=${pagination.Page}&string=${searchText.value}&columns=ID%2CName%2CCraftType.Name%2CDifficultyFactor%2CDurabilityFactor%2CQualityFactor%2CRecipeLevelTable.ID`, { mode: 'cors' })
    const data: { Pagination: any, Results: XivapiRecipe[] } = await response.json()
    pagination.PageTotal = data.Pagination.PageTotal;
    displayTable.value = data.Results
})

const emits = defineEmits<{
    (event: 'change', job: Jobs | 'unknown', name: string, recipe: Recipe): void
}>()

const openFilter = ref(false)
const openCustomlizer = ref(false)

const selectRecipeRow = async (row: XivapiRecipe) => {
    try {
        await ElMessageBox.confirm(
            `将当前配方设置为“${row.Name}”吗？`,
            '请确认',
            {
                confirmButtonText: '确认',
                cancelButtonText: '取消',
                type: 'warning',
            }
        )
        const recipe = await newRecipe(
            row.RecipeLevelTable.ID,
            row.DifficultyFactor,
            row.QualityFactor,
            row.DurabilityFactor
        )
        selectRecipe(recipe, row.Name, row.CraftType.Name)
    } catch {
        // operation canceled by user
    }
}

const selectRecipe = (recipe: Recipe, name: string, job: string) => {
    emits('change', jobMaps[job], name, recipe)
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
                <el-table-column prop="ID" label="ID" />
                <el-table-column prop="RecipeLevelTable.ID" label="配方等级" />
                <el-table-column prop="Name" label="名称" width="250" />
                <el-table-column prop="CraftType.Name" label="类型" />
                <el-table-column prop="DifficultyFactor" label="难度因子" />
                <el-table-column prop="QualityFactor" label="品质因子" />
                <el-table-column prop="DurabilityFactor" label="耐久因子" />
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
