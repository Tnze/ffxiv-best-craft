<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import { Search, Filter } from '@element-plus/icons-vue'
import type { TableColumnCtx } from 'element-plus/es/components/table/src/table-column/defaults'
import { Jobs, RecipeRow, recipe_table } from '../../Craft'

const jobList = [
    { job: Jobs.Carpenter, label: "刻木匠" },
    { job: Jobs.Blacksmith, label: "锻铁匠" },
    { job: Jobs.Armorer, label: "铸甲匠" },
    { job: Jobs.Goldsmith, label: "雕金匠" },
    { job: Jobs.Leatherworker, label: "制革匠" },
    { job: Jobs.Weaver, label: "裁衣匠" },
    { job: Jobs.Alchemist, label: "炼金术士" },
    { job: Jobs.Culinarian, label: "烹调师" },
]

const recipeTable = ref<RecipeRow[]>([])
recipe_table().then((v) => {
    console.log(v)
    recipeTable.value = v.filter(r => r.name.length > 0)
})

const currentPage = ref(1)

const emits = defineEmits<{
    (event: 'change', job: Jobs, name: string): void
}>()

const input3 = ref('')
const openFilter = ref(false)
</script>

<template>
    <el-container>
        <el-header>
            <h1>配方</h1>
        </el-header>
        <el-main>
            <el-drawer v-model="openFilter" :show-close="false">
                <template #title>高级搜索</template>
            </el-drawer>
            <el-table
                :data="recipeTable.slice((currentPage - 1) * 100, currentPage * 100)"
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
                            :page-count="Math.ceil(recipeTable.length / 100)"
                            @current-change="(v: number) => currentPage = v"
                        />
                    </template>
                </el-table-column>
            </el-table>
        </el-main>
    </el-container>
</template>

<style scope >
</style>
