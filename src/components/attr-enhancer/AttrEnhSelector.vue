<script setup lang="ts">
import { ElTable } from 'element-plus';
import { ref } from 'vue'
import meal from '../../assets/data/meal.json'
import potions from '../../assets/data/potions.json'
import { Enhancer } from './Enhancer';

const items = meal.concat(potions)

const props = defineProps<{
    modelValue: Enhancer[]
}>()

const emits = defineEmits<{
    (event: 'update:modelValue', v: Enhancer[]): void
}>()

const multipleTableRef = ref<InstanceType<typeof ElTable>>()

const handleSelectionChange = (val: Enhancer[]) => {
    emits('update:modelValue', val)
}
const cancelSelection = (e: Enhancer) => {
    multipleTableRef.value!.toggleRowSelection(e, false)
}
</script>

<template>
    <el-tag v-for="tag in modelValue" class="item" @close="cancelSelection(tag)" closable>
        {{ tag.name }}
    </el-tag>
    <el-table class="enhancer-table" :data="items" max-height="400" @selection-change="handleSelectionChange"
        ref="multipleTableRef">
        <el-table-column type="selection" width="50" />
        <el-table-column prop="name" label="名称" />
    </el-table>
</template>

<style scoped>
.item {
    margin-right: 10px;
    margin-bottom: 5px;
}

.enhancer-table {
    width: 100%;
}
</style>