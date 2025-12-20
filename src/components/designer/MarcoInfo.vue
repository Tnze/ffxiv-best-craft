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
import { ElTag, ElSpace } from 'element-plus';
import { calcPostCastTime, calcWaitTime, Status } from '@/libs/Craft';
import { computed } from 'vue';
import { formatDuration } from '@/libs/Utils';
import { Sequence } from './types';

const props = defineProps<{
    seq: Sequence;
    status: Status;
}>();

const lengthTagType = computed<
    'success' | 'warning' | 'info' | 'danger' | undefined
>(() => {
    const length = props.seq.slots.length;
    if (length <= 15) return 'success';
    else if (length <= 30) return 'warning';
    else return 'danger';
});

const waitTime = computed(() => {
    const actions = props.seq.slots.map(v => v.action);
    return {
        macro: calcWaitTime(...actions),
        manual: calcPostCastTime(...actions),
    };
});
</script>

<template>
    <el-space>
        <el-tag round :type="lengthTagType">
            {{ $t('macro-length-tag', { length: props.seq.slots.length }) }}
        </el-tag>
        <el-tag round type="info">
            {{ $t('steps-tag', { steps: props.status.step }) }}
        </el-tag>
        <el-tag round type="info">
            {{
                $t('macro-duration-tag', {
                    duration: formatDuration(waitTime.macro * 1e3, 0),
                })
            }}
        </el-tag>
        <el-tag round type="info">
            {{
                $t('manual-duration-tag', {
                    duration: formatDuration(waitTime.manual * 1e3, 1),
                })
            }}
        </el-tag>
    </el-space>
</template>

<fluent locale="zh-CN">
macro-length-tag = 宏长度：{ $length }
steps-tag = { steps }：{ $steps }
macro-duration-tag = 宏耗时：{ $duration }
manual-duration-tag = 手搓耗时：{ $duration }
</fluent>

<fluent locale="zh-TW">
macro-length-tag = 巨集長度：{ $length }
steps-tag = { steps }：{ $steps }
macro-duration-tag = 巨集耗時：{ $duration }
manual-duration-tag = 手搓耗時：{ $duration }
</fluent>

<fluent locale="en-US">
macro-length-tag = Macro Length: { $length }
steps-tag = { steps }: { $steps }
macro-duration-tag = Macro Duration: { $duration }
manual-duration-tag = Manual Duration: { $duration }
</fluent>
