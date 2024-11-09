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
    ElSpace,
} from 'element-plus';
import { onMounted, reactive, watch, ref, defineAsyncComponent, h } from 'vue';
import { Enhancer, calculateEnhancedAttributs } from '@/libs/Enhancer';
import { useFluent } from 'fluent-vue';
import { Attributes, Jobs } from '@/libs/Craft';
import settingStore from '@/stores/settings';
import { DataSource } from '../../recipe-manager/source';
import AttrEnhSelectorOption from './AttrEnhSelectorOption.vue';

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
    attributs?: Attributes;
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
        })();
    }
}

function enhancerToOptions(enhancers: Enhancer[] | undefined) {
    return (
        enhancers
            ?.map(value => {
                const newName = value.name + (value.is_hq ? ' \uE03C' : '');
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
    const list = [];
    if (props.inc.cm) list.push($t('craftsmanship') + ` +${props.inc.cm}`);
    if (props.inc.ct) list.push($t('control') + ` +${props.inc.ct}`);
    if (props.inc.cp) list.push($t('craft-point') + ` +${props.inc.cp}`);
    return h(ElSpace, list);
}
</script>

<template>
    <el-form :model="enhancers" label-width="auto">
        <el-form-item :label="$t('meal')">
            <el-select-v2
                v-model="enhancers.meal"
                :options="enhancerToOptions(meals)"
                value-key="name"
                clearable
                remote
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
                :inc="calculateEnhancedAttributs(attributs, enhancers.meal)[1]"
            />
        </el-form-item>
        <el-form-item :label="$t('medicine')">
            <el-select-v2
                v-model="enhancers.potion"
                :options="enhancerToOptions(medicine)"
                value-key="name"
                clearable
                remote
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
                    calculateEnhancedAttributs(attributs, enhancers.potion)[1]
                "
            />
        </el-form-item>
        <el-form-item :label="$t('soul-of-the-crafter')">
            <el-switch v-model="enhancers.soulOfTheCrafter" />
            <EnhIncComponent
                class="enhnacer-info"
                v-if="enhancers.soulOfTheCrafter && attributs"
                :inc="calculateEnhancedAttributs(attributs, 专家之证)[1]"
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
soul-of-the-crafter = 专家之证
</fluent>

<fluent locale="ja-JP">
meal = 調理品
medicine = 薬品
soul-of-the-crafter = マイスターの証
</fluent>
