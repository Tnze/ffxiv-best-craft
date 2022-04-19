<script setup lang="ts">
import {
    Notebook,
    Suitcase,
    Edit,
    Setting,
} from '@element-plus/icons-vue'

const props = defineProps<{
    modelValue: number
}>()

const emit = defineEmits<{
    (event: 'update:modelValue', page: number): void
}>()

const handleSelect = (key: number, keyPath: string) => {
    emit('update:modelValue', key)
}
</script>

<template>
    <el-menu :default-active="modelValue" @select="handleSelect" :collapse="true">
        <el-menu-item index="0">
            <el-icon>
                <suitcase />
            </el-icon>
            <template #title>装备</template>
        </el-menu-item>
        <el-menu-item index="1">
            <el-icon>
                <notebook />
            </el-icon>
            <template #title>配方</template>
        </el-menu-item>
        <el-menu-item index="2">
            <el-icon>
                <edit />
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
