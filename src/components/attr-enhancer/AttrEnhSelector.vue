<script setup lang="ts">
import { ElTag, ElTable, ElTableColumn } from 'element-plus';
import { ref } from 'vue'
import meal from '../../assets/data/meal.json'
import potions from '../../assets/data/potions.json'
import { Enhancer } from './Enhancer';
import { useFluent } from 'fluent-vue';

const { $t } = useFluent();

interface Item {
    // craftsmanship
    cm?: number;
    cm_max?: number;

    // touch
    ct?: number;
    ct_max?: number;

    // craft-point
    cp?: number;
    cp_max?: number;

    name: string;
}

const 专家之证: Item = {
    cm: 100,
    cm_max: 20,
    ct: 100,
    ct_max: 20,
    cp: 100,
    cp_max: 15,
    name: "【ilv.136】" + $t('soul-of-the-crafter')
}
const items: Item[] = [专家之证].concat(meal).concat(potions)

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
        <el-table-column prop="name" :label="$t('name')" />
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

<fluent locale="zh-CN">
name = 名称
soul-of-the-crafter = 专家之证
</fluent>

<fluent locale="en-US">
name = Name
</fluent>

<fluent locale="en-US">
name = Name
soul-of-the-crafter = マイスターの証
</fluent>