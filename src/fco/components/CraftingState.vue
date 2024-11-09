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
import { watchEffect, ref } from 'vue';
import { NCard, NProgress, NDivider, NFlex } from 'naive-ui';

import { Jobs, Status, high_quality_probability } from '@/libs/Craft';

const props = defineProps<{
    job?: Jobs;
    status?: Status;
    successRate?: number;
}>();

const hqRate = ref<number>();
watchEffect(async () => {
    if (props.status) {
        hqRate.value =
            (await high_quality_probability(props.status)) ?? undefined;
    }
});
</script>

<template>
    <n-card>
        <template #header>
            Lv.{{ status?.attributes.level ?? 0 }}
            {{ job ? $t(job) : '未知职业' }}
        </template>
        <span>{{ $t('progress') }}</span>
        <n-progress
            type="line"
            :height="24"
            :percentage="
                status ? (status.progress / status.recipe.difficulty) * 100 : 0
            "
        >
            {{ status?.progress ?? 0 }} / {{ status?.recipe.difficulty ?? 0 }}
        </n-progress>
        <span>{{ $t('quality') }}</span>
        <n-progress
            type="line"
            :height="24"
            :percentage="
                status ? (status.quality / status.recipe.quality) * 100 : 0
            "
        >
            {{ status?.quality ?? 0 }} / {{ status?.recipe.quality ?? 0 }}
        </n-progress>
        <span>{{ $t('craft-point') }}</span>
        <n-progress
            type="line"
            :height="24"
            :percentage="
                status
                    ? (status.craft_points / status.attributes.craft_points) *
                      100
                    : 0
            "
        >
            {{ status?.craft_points ?? 0 }} /
            {{ status?.attributes.craft_points ?? 0 }}
        </n-progress>
        <n-divider />
        <n-flex justify="space-between">
            <span
                >耐久度 {{ status?.durability ?? 0 }} /
                {{ status?.recipe.durability ?? 0 }}</span
            >
            <span>优质率 {{ hqRate ?? 0 }}%</span>
            <span>成功率 {{ successRate ?? 100 }}%</span>
        </n-flex>
    </n-card>
</template>
