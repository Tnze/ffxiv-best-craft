<script setup lang="ts">
import { ref, reactive } from 'vue'
import { Jobs } from '../Craft'

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

const filters = reactive({
    craftJob: null,
    name: ''
})


</script>

<template>
    <el-container>
        <el-header>
            <h1>配方</h1>
        </el-header>
        <el-main>
            <el-form class="filter" :inline="true" :model="filters">
                <el-form-item label="制作职业">
                    <el-select
                        v-model="filters.craftJob"
                        class="m-2"
                        placeholder="制作职业"
                        size="small"
                    >
                        <el-option
                            v-for="(v, i) in jobList"
                            :key="i"
                            :label="v.label"
                            :value="v.job"
                        />
                    </el-select>
                </el-form-item>
                <el-form-item label="搜索">
                    <el-input v-model="filters.name" placeholder="物品名称" size="small" />
                </el-form-item>
            </el-form>
            <el-empty description="尚未完成" />
        </el-main>
    </el-container>
</template>

<style scope >
.filter {
    padding: 10px 0px 0px 20px;
    border-top: 1px solid var(--el-border-color);
}
</style>
