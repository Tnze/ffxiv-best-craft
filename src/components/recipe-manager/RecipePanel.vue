<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import { Filter } from '@element-plus/icons-vue'
import { Jobs, RecipeRow, Recipe, recipe_table, new_recipe } from '../../Craft'

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

const recipeTable = ref<RecipeRow[]>([])
recipe_table().then((v) => {
    recipeTable.value = v.filter(r => r.name.length > 0)
})

const searchText = ref('')

const displayTable = computed(() => {
    return recipeTable.value.filter(r => r.name.includes(searchText.value))
})

const currentPage = ref(1)

const emits = defineEmits<{
    (event: 'change', job: Jobs, name: string, recipe: Recipe): void
}>()

const openFilter = ref(false)

const selectRecipe = (row: RecipeRow | undefined) => {
    if (row != undefined)
        new_recipe(row.rlv, row.difficulty_factor, row.quality_factor, row.durability_factor)
            .then((r) => { emits('change', jobMaps[row.job], row.name, r) })
}

</script>

<template>
    <el-container>
        <el-header>
            <h1>选择配方</h1>
        </el-header>
        <el-main class="container">
            <el-drawer v-model="openFilter" :show-close="false">
                <template #title>高级过滤</template>
            </el-drawer>
            <el-input v-model="searchText" class="search-input" placeholder="键入以搜索">
                <template #append>
                    <el-button :icon="Filter" @click="openFilter = true" />
                </template>
            </el-input>
            <el-table
                v-loading="recipeTable.length == 0"
                element-loading-text="请稍等..."
                highlight-current-row
                @current-change="selectRecipe"
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
                            :currentPage="currentPage"
                            :page-count="Math.ceil(displayTable.length / 100)"
                            @current-change="(v: number) => currentPage = v"
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
