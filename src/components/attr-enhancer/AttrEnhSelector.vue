<!-- 
    This file is part of BestCraft.
    Copyright (C) 2023  Tnze

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
import { ElForm, ElFormItem, ElSelectV2, ElSwitch, ElDivider } from 'element-plus';
import { reactive, watch } from 'vue'
import meals from '@/assets/data/meal.json'
import potions from '@/assets/data/potions.json'
import useGearsets from '@/stores/gearsets'
import { Enhancer } from './Enhancer';
import { useFluent } from 'fluent-vue';
import { Jobs } from '@/libs/Craft';
import Gearset from '@/components/Gearset.vue';

const { $t } = useFluent();
const gearsets = useGearsets()

const 专家之证: Enhancer = {
    cm: 100,
    cm_max: 20,
    ct: 100,
    ct_max: 20,
    cp: 100,
    cp_max: 15,
    name: "【ilv.136】" + $t('soul-of-the-crafter')
}

const props = defineProps<{
    modelValue: Enhancer[],
    job?: Jobs
}>()

const emits = defineEmits<{
    (event: 'update:modelValue', v: Enhancer[]): void
}>()

function processData(enhancers: Enhancer[]) {
    return enhancers.map(value => ({
        label: value.name, value
    }))
}

const enhancers = reactive<{
    meal: Enhancer | null,
    potion: Enhancer | null,
    soulOfTheCrafter: boolean,
}>({ meal: null, potion: null, soulOfTheCrafter: false })

watch(enhancers, e => {
    const result = []
    if (e.meal != null) result.push(e.meal)
    if (e.potion != null) result.push(e.potion)
    if (e.soulOfTheCrafter) result.push(专家之证)
    emits('update:modelValue', result)
})

</script>

<template>
    <el-form :model="enhancers" label-width="auto">
        <el-form-item :label="$t('meal')">
            <el-select-v2 v-model="enhancers.meal" :placeholder="$t('none')" :options="processData(meals)" value-key="name"
                filterable clearable />
        </el-form-item>
        <el-form-item :label="$t('potion')">
            <el-select-v2 v-model="enhancers.potion" :placeholder="$t('none')" :options="processData(potions)"
                value-key="name" filterable clearable />
        </el-form-item>
        <el-form-item :label="$t('soul-of-the-crafter')">
            <el-switch v-model="enhancers.soulOfTheCrafter" />
        </el-form-item>
    </el-form>
    <template v-if="job != undefined">
        <el-divider />
        <Gearset :job="job" />
    </template>
</template>

<style scoped>
.el-select {
    width: 300px;
}

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
none = 无
meal = 食物
potion = 药水
soul-of-the-crafter = 专家之证
</fluent>

<fluent locale="en-US">
name = Name
none = None
meal = Meal
potion = Potion
soul-of-the-crafter = 专家之证
</fluent>

<fluent locale="en-US">
name = Name
none = None
soul-of-the-crafter = マイスターの証
</fluent>
