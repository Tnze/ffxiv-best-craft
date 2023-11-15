<script setup lang="ts">
import { ElButton, ElSpace, ElText, ElTag, ElIcon, ElButtonGroup } from "element-plus";
import { Delete, Upload, SuccessFilled, WarningFilled, CircleCloseFilled } from "@element-plus/icons-vue";
import { Sequence } from './Designer.vue'
import ActionQueue from "./ActionQueue.vue";
import { Jobs } from "@/libs/Craft";
import { computed } from "vue";

const props = defineProps<{
    seq: Sequence,
    displayJob: Jobs,
}>()

const emit = defineEmits<{
    (event: 'load'): void
    (event: 'delete'): void
}>()

const color = computed(() => {
    if (props.seq.status.progress < props.seq.status.recipe.difficulty)
        return '#F56C6C'
    else if (props.seq.status.quality < props.seq.status.recipe.quality)
        return '#E6A23C'
    else
        return '#67C23A'
})

const tagType = computed<"success" | "warning" | "info" | "danger">(() => {
    if (props.seq.status.progress < props.seq.status.recipe.difficulty)
        return 'danger'
    else if (props.seq.status.quality < props.seq.status.recipe.quality)
        return 'warning'
    else
        return 'success'
})
</script>

<template>
    <div class="savedqueue-item">
        <div class="savedqueue-item-right">
            <ActionQueue class="savedqueue-item-actions" :job="displayJob" :list="seq.slots" :err-list="seq.errors"
                disabled />
            <div class="savedqueue-item-above">
                <el-text size="small">
                    <el-space>
                        <el-icon :color="color" class="savedqueue-status">
                            <success-filled v-if="color == '#67C23A'" />
                            <warning-filled v-else-if="color == '#E6A23C'" />
                            <circle-close-filled v-else />
                        </el-icon>
                        <el-tag round :type="tagType">{{ $t('quality-tag', { quality: seq.status.quality }) }}</el-tag>
                        <el-tag round type="info">{{ $t('steps-tag', { steps: seq.status.step }) }}</el-tag>
                    </el-space>
                </el-text>
                <el-button-group>
                    <el-button :icon="Upload" size="small" round class="savedqueue-item-button" @click="emit('load')">
                        {{ $t('load') }}
                    </el-button>
                    <el-button :icon="Delete" size="small" round class="savedqueue-item-button"
                        @click="emit('delete')">
                        {{ $t('delete') }}
                    </el-button>
                </el-button-group>
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
load = 加载
delete = 删除
quality-tag = { quality }：{ $quality }
steps-tag = { steps }：{ $steps }
</fluent>

<fluent locale="en-US">
load = Load
delete = Delete
quality-tag = { quality }: { $quality }
steps-tag = { steps }: { $steps }
</fluent>
