<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import 'element-plus/es/components/message/style/css'
import 'element-plus/es/components/message-box/style/css'
import { ElMessage, ElMessageBox } from 'element-plus'
import { EditPen, Filter } from '@element-plus/icons-vue'
import { Jobs, RecipeRow, Recipe, newRecipeTable, newRecipe } from '../../Craft'

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

const recipeTable = ref<RecipeRow[]>([])
newRecipeTable().then((v) => {
    recipeTable.value = v.filter(r => r.name.length > 0)
})

const searchText = ref('')

const displayTable = computed(() => {
    return recipeTable.value.filter(r => r.name.includes(searchText.value))
})

const currentPage = ref(1)

const emits = defineEmits<{
    (event: 'change', job: Jobs | 'unknown', name: string, recipe: Recipe): void
}>()

const openFilter = ref(false)
const openCustomlizer = ref(false)

const selectRecipeRow = (row: RecipeRow) => {
    if (row == undefined)
        return
    ElMessageBox.confirm(
        `确认将当前配方设置为“${row.name}”吗`,
        'Warning',
        {
            confirmButtonText: '确认',
            cancelButtonText: '取消',
            type: 'warning'
        }
    ).then(() => {
        newRecipe(row.rlv, row.difficulty_factor, row.quality_factor, row.durability_factor)
            .then(r => {
                selectRecipe(r, row.name, row.job)
            })
    }).catch(() => {
        // operation canceled by user
    })
}

const selectRecipe = (recipe: Recipe, name: string, job: string) => {
    emits('change', jobMaps[job], name, recipe)
    ElMessage({
        type: 'success',
        duration: 5000,
        showClose: true,
        dangerouslyUseHTMLString: true,
        message: `
        配方设置已变更 rlv: ${recipe.rlv}<br/>
        难度: ${recipe.difficulty}
        品质: ${recipe.quality}
        耐久: ${recipe.durability}`
    })
}

const customRecipe = ref({
    rlv: 580,
    name: 'Recipe#580',
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
            <el-drawer v-model="openFilter" :show-close="false">
                <template #title>高级过滤</template>
                这里本来应该有通过职业过滤、品级过滤、等级过滤、秘籍过滤等等，但是被咕掉了。
            </el-drawer>
            <el-dialog v-model="openCustomlizer" title="自定义配方">
                <el-form
                    :model="customRecipe"
                    label-position="right"
                    label-width="100px"
                    style="max-width: 460px"
                >
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
                        <el-button
                            type="primary"
                            @click="openCustomlizer = false; selectRecipe(customRecipe, customRecipe.name, '自定义')"
                        >确认</el-button>
                    </span>
                </template>
            </el-dialog>
            <el-input v-model="searchText" class="search-input" placeholder="键入以搜索" clearable>
                <template #append>
                    <el-button :icon="Filter" @click="openFilter = true" />
                    <el-button :icon="EditPen" @click="openCustomlizer = true" />
                </template>
            </el-input>
            <el-table
                v-loading="recipeTable.length == 0"
                element-loading-text="请稍等..."
                highlight-current-row
                @row-dblclick="selectRecipeRow"
                :data="displayTable.slice((currentPage - 1) * 100, currentPage * 100)"
                height="100%"
                style="width: 100%"
            >
                <el-table-column prop="id" label="ID" width="100" />
                <el-table-column prop="rlv" label="rlv" width="60" />
                <el-table-column prop="job" label="制作职业" width="100" />
                <el-table-column prop="name" label="名称" />
                <el-table-column align="right" width="300">
                    <template #header>
                        <el-pagination
                            small
                            layout="prev, pager, next"
                            v-model:current-page="currentPage"
                            :page-count="Math.ceil(displayTable.length / 100)"
                        />
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
</style>
