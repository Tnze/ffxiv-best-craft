<script setup lang="ts">
import { ElContainer, ElHeader, ElMain, ElScrollbar, ElLink, ElMessage, ElMessageBox, ElAlert, ElTabs, ElTabPane, ElCheckboxButton, ElButton, ElButtonGroup } from "element-plus";
import { Delete, Edit } from "@element-plus/icons-vue";
import { Sequence } from './Designer.vue'
import ActionQueue from "./ActionQueue.vue";
import QueueStatus from "./QueueStatus.vue";
import { Jobs } from "@/libs/Craft";

defineProps<{
    key: number,
    seq: Sequence,
    displayJob: Jobs,
}>()

const emit = defineEmits<{
    (event: 'load'): void
    (event: 'delete'): void
}>()
</script>

<template>
    <div class="savedqueue-item" :key="key">
        <QueueStatus :status="seq.status" />
        <div class="savedqueue-item-right">
            <ActionQueue class="savedqueue-item-actions" :job="displayJob" :list="seq.slots" :err-list="seq.errors"
                disabled />
            <div class="savedqueue-item-above">
                <el-link :icon="Edit" :underline="false" class="savedqueue-item-button" @click="emit('load')">
                    {{ $t('edit') }}
                </el-link>
                <el-link :icon="Delete" :underline="false" class="savedqueue-item-button" @click="emit('delete')">
                    {{ $t('delete') }}
                </el-link>
            </div>
        </div>
    </div>
</template>

<style scoped>
.savedqueue-item {
    display: flex;
    align-items: center;
    min-height: 50px;
    margin-bottom: 3px;
}

.savedqueue-item-right {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
}

.savedqueue-item-above {
    display: flex;
    margin-left: 10px;
}

.savedqueue-item-button {
    margin-right: 6px;
}

.savedqueue-item-actions {
    padding: 3px 0 0 8px;
    border-left: 5px solid var(--el-border-color);
}

</style>

<fluent locale="zh-CN">
edit = 编辑
delete = 删除
</fluent>

<fluent locale="en-US">
edit = Edit
delete = Delete
</fluent>
