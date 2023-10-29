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
import { ElTag, ElTable, ElTableColumn } from 'element-plus';
import { ref } from 'vue'
import meal from '@/assets/data/meal.json'
import potions from '@/assets/data/potions.json'
import { Enhancer } from './Enhancer';
import { useFluent } from 'fluent-vue';

const { $t } = useFluent();

const 专家之证: Enhancer = {
    cm: 100,
    cm_max: 20,
    ct: 100,
    ct_max: 20,
    cp: 100,
    cp_max: 15,
    name: "【ilv.136】" + $t('soul-of-the-crafter')
}
const items: Enhancer[] = [专家之证].concat(meal).concat(potions)

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