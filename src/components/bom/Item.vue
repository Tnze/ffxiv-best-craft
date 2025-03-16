<!-- 
    This file is part of BestCraft.
    Copyright (C) 2025  Tnze

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
import {
    ElText,
    ElCard,
    ElInputNumber,
    ElTooltip,
    ElBadge,
} from 'element-plus';
import { toRef } from 'vue';

import { SlotType } from '@/stores/bom';

const props = defineProps<{
    id: number;
    name: string;
    wasted?: number;
    requiredInputDisabled?: boolean;
    holdingInputDisabled?: boolean;

    type?: SlotType;
}>();

const requiredNumber = defineModel<number>('requiredNumber', {
    required: true,
});
const holdingNumber = defineModel<number>('holdingNumber', { required: true });
const elemUiTypeMapping = new Map<
    SlotType | undefined,
    'warning' | 'success' | 'info' | 'danger' | undefined
>([
    [undefined, undefined],
    ['required', 'danger'],
    ['crafted', 'warning'],
    ['completed', 'success'],
    ['not-required', 'info'],
]);

defineExpose({ id: toRef(() => props.id) });
</script>

<template>
    <el-card :class="type" :body-class="$style.item" shadow="hover">
        <el-text :class="$style['id-badget']" size="small" type="info">
            #{{ id }}
        </el-text>
        <el-text :class="$style.elem" :type="elemUiTypeMapping.get(type)">
            {{ name }}
        </el-text>
        <div v-if="false" :class="$style.icon">假装这有图标</div>
        <el-badge
            :value="wasted ?? 0"
            :hidden="wasted == undefined || wasted == 0"
            type="info"
        >
            <el-tooltip
                :class="$style.elem"
                :content="$t('required-number')"
                effect="light"
                :enterable="false"
            >
                <el-input-number
                    size="small"
                    v-model="requiredNumber"
                    step-strictly
                    :min="0"
                    :disabled="requiredInputDisabled"
                >
                    <template #suffix>pcs</template>
                </el-input-number>
            </el-tooltip>
        </el-badge>
        <el-tooltip
            :class="$style.elem"
            :content="$t('holding-number')"
            effect="light"
            :enterable="false"
        >
            <el-input-number
                :class="$style.elem"
                size="small"
                v-model="holdingNumber"
                step-strictly
                :min="0"
                :disabled="holdingInputDisabled"
            >
                <template #suffix>pcs</template>
            </el-input-number>
        </el-tooltip>
    </el-card>
</template>

<style>
.required {
    background-color: var(--el-color-danger-light-9);
    border-color: var(--el-color-danger);
}

.crafted {
    background-color: var(--el-color-warning-light-9);
    border-color: var(--el-color-warning);
}

.completed {
    background-color: var(--el-color-success-light-9);
    border-color: var(--el-color-success);
}

.not-required {
    background-color: var(--el-color-info-light-9);
    /* border-color: var(--el-color-info); */
}
</style>

<style module>
.item {
    margin: 3px 5px;
    padding: 0;

    display: flex;
    flex-direction: column;
    align-items: center;
}

.id-badget {
    flex: auto;
    align-self: flex-start;
    margin-bottom: 0;
    user-select: none;
}

.elem {
    margin: 3px;
}

.icon {
    margin: 8px;
    height: 48px;
    width: 48px;
    border: 1px solid white;
    border-radius: 5px;
    overflow: hidden;
    user-select: none;
    color: var(--el-color-info);
}
</style>

<fluent locale="zh-CN">
required-number = 需求数量
holding-number = 持有数量
</fluent>

<fluent locale="en-US">
required-number = Required number
holding-number = Holding number
</fluent>

<fluent locale="ja-JP">
required-number = 需要数量
holding-number = 保有数量
</fluent>
