<script setup lang="ts">
import { SuccessFilled, WarningFilled } from '@element-plus/icons-vue'
import { computed } from 'vue';
import { Status } from '../../Craft';

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
            <warning-filled v-else-if="color != undefined" />
        </el-icon>
    </el-tooltip>
</template>

<style scoped>
.savedqueue-status {
    margin: 0 -6px 0 8px;
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
