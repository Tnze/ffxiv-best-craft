<script setup lang="ts">
import { BottomRight, Close, Cpu, Printer } from '@element-plus/icons-vue'

const emits = defineEmits<{
    (event: 'plus'): void
    (event: 'delete'): void
    (event: 'solver'): void
    (event: 'print'): void
}>();
</script>
<template>
    <div class="container">
        <el-tooltip content="保存工作区" placement="right" :show-after="1000">
            <el-icon class="button" :size="16" @click="emits('plus')">
                <bottom-right />
            </el-icon>
        </el-tooltip>
        <el-tooltip content="清空工作区" placement="right" :show-after="1000">
            <el-icon class="button" :size="16" @click="emits('delete')">
                <close />
            </el-icon>
        </el-tooltip>
        <el-tooltip content="求解器设置" placement="right" :show-after="1000">
            <el-icon class="button" :size="16" @click="emits('solver')">
                <cpu />
            </el-icon>
        </el-tooltip>
        <el-tooltip content="导出宏" placement="right" :show-after="1000">
            <el-icon class="button" :size="16" @click="emits('print')">
                <printer />
            </el-icon>
        </el-tooltip>
    </div>
</template>
<style scoped>
.container {
    display: flex;
    flex-direction: column;
    padding: 3px 0px 0px 0px;
    width: 40px;
    align-items: center;
}

.container> :hover {
    background-color: #ecf5ff;
    transition: all var(--el-transition-duration);
}

.button {
    padding: 10px 0px;
    cursor: pointer;
    width: 100%;
}
</style>