<script setup lang="ts">
import { SuccessFilled, WarningFilled } from '@element-plus/icons-vue'
import { computed } from '@vue/reactivity';
import { Status } from '../../Craft';

const props = defineProps<{
    status?: Status
}>()

const color = computed(() => {
    if (props.status == undefined)
        return undefined
    else if (props.status.progress < props.status.recipe.difficulty)
        return '#F56C6C'
    else if (props.status.quality < props.status.recipe.quality)
        return '#E6A23C'
    else
        return '#67C23A'
})

</script>

<template>
    <el-icon v-if="status != undefined" :color="color" class="savedqueue-status">
        <success-filled v-if="color == '#67C23A'" />
        <warning-filled v-else-if="color != undefined" />
    </el-icon>
</template>

<style scoped>
.savedqueue-status {
    margin: 0 -6px 0 8px;
}
</style>