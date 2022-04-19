<script setup lang="ts">
import { BottomRight, Close, Cpu, Printer, View } from '@element-plus/icons-vue'

const props = defineProps<{
    previewSolver: boolean
}>()

const emits = defineEmits<{
    (event: 'plus'): void
    (event: 'delete'): void
    (event: 'solver'): void
    (event: 'print'): void
    (event: 'update:previewSolver', value: boolean): void
}>();

</script>
<template>
    <div class="container">
        <el-tooltip content="保存工作区" placement="top" :show-after="1000">
            <el-icon class="button" :size="16" @click="emits('plus')">
                <bottom-right />
            </el-icon>
        </el-tooltip>
        <el-tooltip content="清空工作区" placement="top" :show-after="1000">
            <el-icon class="button" :size="16" @click="emits('delete')">
                <close />
            </el-icon>
        </el-tooltip>
        <el-tooltip content="求解器设置" placement="top" :show-after="1000">
            <el-icon class="button" :size="16" @click="emits('solver')">
                <cpu />
            </el-icon>
        </el-tooltip>
        <el-tooltip content="预览求解结果" placement="top" :show-after="1000">
            <el-icon class="button" :size="16" :color="previewSolver ? '#409EFC' : undefined"
                @click="emits('update:previewSolver', !previewSolver)">
                <View />
            </el-icon>
        </el-tooltip>
        <el-tooltip content="导出宏" placement="top" :show-after="1000">
            <el-icon class="button" :size="16" @click="emits('print')">
                <printer />
            </el-icon>
        </el-tooltip>
    </div>
</template>
<style scoped>
.container {
    display: flex;
    padding: 0px 0px 0px 3px;
    align-items: center;
}

.container> :hover {
    background-color: #ecf5ff;
    transition: all var(--el-transition-duration);
}

.button {
    padding: 6px 10px;
    cursor: pointer;
}
</style>