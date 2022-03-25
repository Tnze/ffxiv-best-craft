<script setup lang="ts">
import { ref } from 'vue'
import {
    Notebook,
    Suitcase,
    Cpu,
    Setting,
} from '@element-plus/icons-vue'

const emit = defineEmits<{
    (event: 'select', page: number): void
}>()

const handleSelect = (key: number, keyPath: string) => {
    emit('select', key)
}
</script>

<template>
    <el-menu default-active="2" @select="handleSelect" :collapse="true">
        <el-menu-item index="0">
            <el-icon>
                <notebook />
            </el-icon>
            <template #title>配方</template>
        </el-menu-item>
        <el-menu-item index="1">
            <el-icon>
                <suitcase />
            </el-icon>
            <template #title>装备</template>
        </el-menu-item>
        <el-menu-item index="2">
            <el-icon>
                <cpu />
            </el-icon>
            <template #title>设计</template>
        </el-menu-item>
        <el-menu-item index="3">
            <el-icon>
                <setting />
            </el-icon>
            <template #title>设置</template>
        </el-menu-item>
    </el-menu>
</template>

<style>
.el-menu {
    height: 100%;
}
</style>
