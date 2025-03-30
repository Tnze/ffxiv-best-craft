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
import {
    ElForm,
    ElFormItem,
    ElSelectV2,
    ElSwitch,
    ElDivider,
    ElSpace,
} from 'element-plus';
import { onMounted, reactive, watch, ref, defineAsyncComponent, h } from 'vue';
import { Enhancer, calculateEnhancedAttributsAddon } from '@/libs/Enhancer';
import { useFluent } from 'fluent-vue';
import { Attributes, Jobs } from '@/libs/Craft';
import settingStore from '@/stores/settings';
import { DataSource } from '../../../datasource/source';
import AttrEnhSelectorOption from './AttrEnhSelectorOption.vue';

const Gearset = defineAsyncComponent(() => import('@/components/Gearset.vue'));

const { $t } = useFluent();
const setting = settingStore();
const meals = ref<Enhancer[]>();
const medicine = ref<Enhancer[]>();
const mealSearchKeyword = ref('');
const medicineSearchKeyword = ref('');

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
    attributs?: Attributes;
}>();

const emits = defineEmits<{
    (event: 'update:modelValue', v: Enhancer[]): void;
}>();

onMounted(async () => loadMealsAndMedicine(setting.getDataSource));
watch(() => setting.getDataSource, loadMealsAndMedicine);
watch(mealSearchKeyword, async () => loadMeals(await setting.getDataSource));
watch(medicineSearchKeyword, async () =>
    loadMedicines(await setting.getDataSource),
);

async function loadMealsAndMedicine(datasource: Promise<DataSource>) {
    let ds = await datasource;
    await Promise.all([loadMeals(ds), loadMedicines(ds)]);
}

async function loadMeals(ds: DataSource) {
    let i = 0;
    let results: Enhancer[] = [];
    do {
        var v = await ds.mealsTable((i += 1));
        results.push(...v.results);
    } while (i < v.totalPages);
    meals.value = results;
}

async function loadMedicines(ds: DataSource) {
    let i = 0;
    let results: Enhancer[] = [];
    do {
        var v = await ds.medicineTable((i += 1));
        results.push(...v.results);
    } while (i < v.totalPages);
    medicine.value = results;
}

function enhancerToOptions(enhancers: Enhancer[] | undefined, search: string) {
    return (
        enhancers
            ?.filter(v => v.name.includes(search))
            .map(value => {
                let newName = value.name;
                if (value.is_hq) newName += ' \uE03C';
                return {
                    label: newName,
                    value: {
                        ...value,
                        name: newName, // el-select needs a unique key
                    },
                };
            })
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

function EnhIncComponent(props: {
    inc: { cm: number; ct: number; cp: number };
}) {
    return h(ElSpace, () => {
        const list = [];
        if (props.inc.cm) list.push($t('craftsmanship') + ` +${props.inc.cm}`);
        if (props.inc.ct) list.push($t('control') + ` +${props.inc.ct}`);
        if (props.inc.cp) list.push($t('craft-point') + ` +${props.inc.cp}`);
        return list;
    });
}
</script>

<template>
    <el-form :model="enhancers" label-width="auto">
        <el-form-item :label="$t('meal')">
            <el-select-v2
                v-model="enhancers.meal"
                :options="enhancerToOptions(meals, mealSearchKeyword)"
                value-key="name"
                clearable
                filterable
                remote
                :remote-method="(kw: string) => (mealSearchKeyword = kw)"
                :loading="!meals"
                :item-height="50"
            >
                <template #default="{ item }">
                    <AttrEnhSelectorOption :enh="item.value" />
                </template>
            </el-select-v2>
            <EnhIncComponent
                class="enhnacer-info"
                v-if="enhancers.meal && attributs"
                :inc="
                    calculateEnhancedAttributsAddon(attributs, enhancers.meal)
                "
            />
        </el-form-item>
        <el-form-item :label="$t('medicine')">
            <el-select-v2
                v-model="enhancers.potion"
                :options="enhancerToOptions(medicine, medicineSearchKeyword)"
                value-key="name"
                clearable
                filterable
                remote
                :remote-method="(kw: string) => (medicineSearchKeyword = kw)"
                :loading="!medicine"
                :item-height="50"
            >
                <template #default="{ item }">
                    <AttrEnhSelectorOption :enh="item.value" />
                </template>
            </el-select-v2>
            <EnhIncComponent
                class="enhnacer-info"
                v-if="enhancers.potion && attributs"
                :inc="
                    calculateEnhancedAttributsAddon(attributs, enhancers.potion)
                "
            />
        </el-form-item>
        <el-form-item :label="$t('soul-of-the-crafter')">
            <el-switch v-model="enhancers.soulOfTheCrafter" />
            <EnhIncComponent
                class="enhnacer-info"
                v-if="enhancers.soulOfTheCrafter && attributs"
                :inc="calculateEnhancedAttributsAddon(attributs, 专家之证)"
            />
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

.enhnacer-info {
    color: var(--el-color-info);
    font-size: var(--el-font-size-extra-small);
    margin-inline-start: 8px;
}
</style>

<fluent locale="zh-CN">
meal = 食物
medicine = 药水
soul-of-the-crafter = 专家之证
</fluent>

<fluent locale="en-US">
meal = Meal
medicine = Potion
soul-of-the-crafter = Soul of the Crafter
</fluent>

<fluent locale="ja-JP">
meal = 調理品
medicine = 薬品
soul-of-the-crafter = マイスターの証
</fluent>
