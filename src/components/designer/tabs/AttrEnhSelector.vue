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
import {
    ElForm,
    ElFormItem,
    ElSelectV2,
    ElSwitch,
    ElDivider,
} from 'element-plus';
import {
    onMounted,
    reactive,
    watch,
    ref,
    defineAsyncComponent,
    Ref,
} from 'vue';
import { Enhancer } from '@/libs/Enhancer';
import { useFluent } from 'fluent-vue';
import { Jobs } from '@/libs/Craft';
import settingStore from '@/stores/settings';
import { DataSource } from '../../recipe-manager/source';

const Gearset = defineAsyncComponent(() => import('@/components/Gearset.vue'));

const { $t } = useFluent();
const setting = settingStore();
const meals = ref<Enhancer[]>();
const medicine = ref<Enhancer[]>();

const 专家之证: Enhancer = {
    cm: 100,
    cm_max: 20,
    ct: 100,
    ct_max: 20,
    cp: 100,
    cp_max: 15,
    name: $t('soul-of-the-crafter'),
};

const props = defineProps<{
    modelValue: Enhancer[];
    job?: Jobs;
}>();

const emits = defineEmits<{
    (event: 'update:modelValue', v: Enhancer[]): void;
}>();

onMounted(async () => loadMealsAndMedicine(setting.getDataSource));
watch(() => setting.getDataSource, loadMealsAndMedicine);

async function loadMealsAndMedicine(ds: Promise<DataSource>) {
    let datasource = await ds;
    if (datasource.mealsTable) {
        (async () => {
            let i = 0,
                v;
            meals.value = [];
            do {
                v = await datasource.mealsTable((i += 1));
                meals.value = meals.value.concat(v.results);
            } while (i < v.totalPages);
            fix_enhancer_name(meals);
        })();
    }
    if (datasource.medicineTable) {
        (async () => {
            let i = 0,
                v;
            medicine.value = [];
            do {
                v = await datasource.medicineTable((i += 1));
                medicine.value = medicine.value.concat(v.results);
            } while (i < v.totalPages);
            fix_enhancer_name(medicine);
        })();
    }
}

function fix_enhancer_name(group: Ref<Enhancer[] | undefined>) {
    if (group.value == undefined) return;
    for (const i in group.value) {
        const oldName = group.value[i].name;
        if (oldName.endsWith(' HQ')) {
            group.value[i].name = oldName.replace(' HQ', ' \uE03C');
        }
    }
}

function enhancerToOptions(enhancers: Enhancer[] | undefined) {
    return (
        enhancers
            ?.map(value => ({
                label: value.name,
                value,
            }))
            .reverse() ?? []
    );
}

const enhancers = reactive<{
    meal: Enhancer | null;
    potion: Enhancer | null;
    soulOfTheCrafter: boolean;
}>({ meal: null, potion: null, soulOfTheCrafter: false });

watch(enhancers, e => {
    const result = [];
    if (e.meal != null) result.push(e.meal);
    if (e.potion != null) result.push(e.potion);
    if (e.soulOfTheCrafter) result.push(专家之证);
    emits('update:modelValue', result);
});
</script>

<template>
    <el-form :model="enhancers" label-width="auto">
        <el-form-item :label="$t('meal')">
            <el-select-v2
                v-model="enhancers.meal"
                :placeholder="$t('none')"
                :options="enhancerToOptions(meals)"
                value-key="name"
                clearable
                remote
                :loading="!meals"
            />
        </el-form-item>
        <el-form-item :label="$t('medicine')">
            <el-select-v2
                v-model="enhancers.potion"
                :placeholder="$t('none')"
                :options="enhancerToOptions(medicine)"
                value-key="name"
                clearable
                remote
                :loading="!medicine"
            />
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
medicine = 药水
soul-of-the-crafter = 专家之证
</fluent>

<fluent locale="en-US">
name = Name
none = None
meal = Meal
medicine = Potion
soul-of-the-crafter = 专家之证
</fluent>

<fluent locale="ja-JP">
meal = 調理品
medicine = 薬品
soul-of-the-crafter = マイスターの証
</fluent>
