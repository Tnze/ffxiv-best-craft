<script setup lang="ts">
import { ref, computed } from 'vue'
import { Attributes } from '../Craft'

const props = defineProps<{
    attr: Attributes
}>()

const emits = defineEmits<{
    (event: 'update:attr', attr: Attributes): void
}>()


</script>

<template>
    <el-container>
        <el-header>
            <h1>装备属性</h1>
        </el-header>
        <el-main>
            <el-empty description="尚未完成" />
        </el-main>
    </el-container>
</template>

<style scoped>
</style>
