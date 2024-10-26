<!-- 
    This file is part of BestCraft.
    Copyright (C) 2024  Tnze

    BestCraft is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    BestCraft is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
-->

<script setup lang="ts">
import { ElButton, ElSpace, ElText, ElTag, ElIcon, ElButtonGroup } from "element-plus";
import { Delete, Upload, SuccessFilled, WarningFilled, CircleCloseFilled } from "@element-plus/icons-vue";
import { Sequence, SequenceSource } from './types';
import ActionQueue from "./ActionQueue.vue";
import { calcPostCastTime, calcWaitTime, Jobs, simulate, SimulateResult, Status } from "@/libs/Craft";
import { formatDuration } from "@/libs/Utils";
import { ref, computed, watchEffect } from "vue";

const props = defineProps<{
    seq: Sequence,
    status?: Status,
    displayJob: Jobs,
}>();

const emit = defineEmits<{
    (event: 'load'): void
    (event: 'delete'): void
}>();

const simulateResult = ref<SimulateResult>();
watchEffect(() => {
    if (props.status) {
        simulate(props.status, props.seq.slots.map(v => v.action))
            .then(rst => simulateResult.value = rst);
    }
})
const color = computed(() => {
    const status = simulateResult.value?.status;
    if (!status)
        return
    else if (status.progress < status.recipe.difficulty)
        return '#F56C6C'
    else if (status.quality < status.recipe.quality)
        return '#E6A23C'
    else
        return '#67C23A'
})

const tagType = computed<"success" | "warning" | "info" | "danger" | undefined>(() => {
    const status = simulateResult.value?.status;
    if (!status)
        return
    if (status.progress < status.recipe.difficulty)
        return 'danger'
    else if (status.quality < status.recipe.quality)
        return 'warning'
    else
        return 'success'
})

const waitTime = computed(() => {
    const actions = props.seq.slots.map(v => v.action)
    return {
        macro: calcWaitTime(...actions),
        manual: calcPostCastTime(...actions)
    }
})
</script>

<template>
    <div class="savedqueue-item">
        <div class="savedqueue-item-right">
            <div class="savedqueue-item-actions">
                <ActionQueue :job="displayJob" :list="seq.slots" :err-list="simulateResult?.errors" disabled no-hover />
            </div>
            <div class="savedqueue-item-above">
                <el-text size="small">
                    <el-space :wrap="true">
                        <el-icon :color="color" class="savedqueue-status">
                            <success-filled v-if="color == '#67C23A'" />
                            <warning-filled v-else-if="color == '#E6A23C'" />
                            <circle-close-filled v-else />
                        </el-icon>
                        <el-tag round v-if="simulateResult" :type="tagType">
                            {{ $t('quality-tag', { quality: simulateResult?.status.quality }) }}
                        </el-tag>
                        <el-tag round v-if="simulateResult" type="info">
                            {{ $t('steps-tag', { steps: simulateResult.status.step ?? seq.slots.length }) }}
                        </el-tag>
                        <el-tag round type="info">
                            {{ $t('macro-duration-tag', { duration: formatDuration(waitTime.macro * 1e3, 0) }) }}
                        </el-tag>
                        <el-tag round type="info">
                            {{ $t('manual-duration-tag', { duration: formatDuration(waitTime.manual * 1e3, 1) }) }}
                        </el-tag>
                        <el-tag round v-if="seq.source !== undefined"
                            :type="seq.source.includes('solver') ? 'success' : 'warning'">
                            {{ $t('source-tag', { typ: String(seq.source), source: $t(String(seq.source)) }) }}
                        </el-tag>
                        <el-tag round type="info" v-if="seq.itemName !== undefined">
                            {{ $t('itemname-tag', { itemName: String(seq.itemName) }) }}
                        </el-tag>
                        <el-button-group>
                            <el-button :icon="Upload" size="small" round class="savedqueue-item-button"
                                @click="emit('load')">
                                {{ $t('load') }}
                            </el-button>
                            <el-button :icon="Delete" size="small" round class="savedqueue-item-button"
                                @click="emit('delete')">
                                {{ $t('delete') }}
                            </el-button>
                        </el-button-group>
                    </el-space>
                </el-text>
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
}

.savedqueue-item-button {
    margin-right: 6px;
}

.savedqueue-item-actions {
    border-left: 5px solid var(--el-border-color);
}
</style>

<fluent locale="zh-CN">
load = 加载
delete = 删除
quality-tag = { quality }：{ $quality }
steps-tag = { steps }：{ $steps }
macro-duration-tag = 宏耗时：{ $duration }
manual-duration-tag = 手搓耗时：{ $duration }
itemname-tag = 物品名：{ $itemName }

# Sources
manual = 手动保存
auto-save = 自动保存
source-tag = 来源：{ $typ ->
    [auto-save] { auto-save }
    [manual] { manual }
    *[other] 「{ $source }」求解器
}
</fluent>

<fluent locale="en-US">
load = Load
delete = Delete
quality-tag = { quality }: { $quality }
steps-tag = { steps }: { $steps }
macro-duration-tag = Macro Duration: { $duration }
manual-duration-tag = Manual Duration: { $duration }
itemname-tag = Item Name: { $itemName }

# Sources
manual = Manual
auto-save = Auto Save
source-tag = Source: { $typ ->
    [auto-save] { auto-save }
    [manual] { manual }
    *[other] { $source } Solver
}
</fluent>
