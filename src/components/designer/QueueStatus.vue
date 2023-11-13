<!-- 
    This file is part of BestCraft.
    Copyright (C) 2023  <name of author>

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
import { ElTooltip, ElIcon } from 'element-plus';
import { SuccessFilled, WarningFilled, CircleCloseFilled } from '@element-plus/icons-vue'
import { computed } from 'vue';
import { Status } from '@/libs/Craft';

const props = defineProps<{
    status: Status
}>()

const color = computed(() => {
    if (props.status.progress < props.status.recipe.difficulty)
        return '#F56C6C'
    else if (props.status.quality < props.status.recipe.quality)
        return '#E6A23C'
    else
        return '#67C23A'
})

</script>

<template>
    <el-tooltip :hide-after="0" placement="left" :content="$t('status-text', {
        progress: status.progress,
        quality: status.quality,
        craftPoint: status.craft_points,
        durability: status.durability,
        steps: status.step
    })">
        <el-icon v-if="status != undefined" :color="color" class="savedqueue-status">
            <success-filled v-if="color == '#67C23A'" />
            <warning-filled v-else-if="color == '#E6A23C'" />
            <circle-close-filled v-else />
        </el-icon>
    </el-tooltip>
</template>

<style scoped>
.savedqueue-status {
    margin: 0 -5px 0 0;
}
</style>

<fluent locale="zh-CN">
status-text = { quality }：{ $quality }
    { steps }：{ $steps }
</fluent>

<fluent locale="en-US">
status-text = { quality }: { $quality }
    { steps }: { $steps }
</fluent>
