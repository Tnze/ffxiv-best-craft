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
import { computed, ref, watch } from 'vue';
import { NCard, NGrid, NGi } from 'naive-ui';
import useFcoSimulatorStore from '../stores/simulator';
import useGearsetsStore from '@/stores/gearsets';
import MealAndMedicinesSelect from './MealAndMedicinesSelect.vue';
import { Enhancer } from '@/libs/Enhancer';
import { Attributes } from '@/libs/Craft';

const gearsetsStore = useGearsetsStore()
const store = useFcoSimulatorStore();

defineProps<{
    initialQuality?: number,
    attributs: Attributes,
}>()

const emits = defineEmits<{
    'update:attributs': [Attributes]
}>()

const meal = ref<Enhancer>();
const medicine = ref<Enhancer>();

const attributes = computed(() => gearsetsStore.attributes(store.job))
const craftsmanshipAddons = ref(0);
const controlAddons = ref(0);
const craftPointAddons = ref(0);
watch([attributes, meal, medicine], ([{ level, craftsmanship, control, craft_points }, meal, medecine]) => {
    console.log(meal, medecine);
    const enhancers = [];
    if (meal) enhancers.push(meal);
    if (medecine) enhancers.push(medecine);

    const sum = (prev: number, curr: number) => prev + curr;
    craftsmanshipAddons.value = enhancers
        .filter((v) => v.cm && v.cm_max)
        .map((v) => Math.min((craftsmanship * v.cm!) / 100, v.cm_max!))
        .reduce(sum, 0);
    controlAddons.value = enhancers
        .filter((v) => v.ct && v.ct_max)
        .map((v) => Math.min((control * v.ct!) / 100, v.ct_max!))
        .reduce(sum, 0);
    craftPointAddons.value = enhancers
        .filter((v) => v.cp && v.cp_max)
        .map((v) => Math.min((craft_points * v.cp!) / 100, v.cp_max!))
        .reduce(sum, 0);
    // Store
    emits('update:attributs', {
        level,
        craftsmanship: craftsmanship + craftsmanshipAddons.value,
        control: control + controlAddons.value,
        craft_points: craft_points + craftPointAddons.value,
    });
});

</script>

<template>
    <n-card>
        <template #header>状态参数</template>
        <n-grid x-gap="12" y-gap="16" :cols="6">
            <n-gi :span="3">
                <span>{{ $t('meals') }}</span>
                <MealAndMedicinesSelect type="meals" @select="v => meal = v" />
            </n-gi>
            <n-gi :span="3">
                <span>{{ $t('medicines') }}</span>
                <MealAndMedicinesSelect type="medicines" @select="v => medicine = v" />
            </n-gi>

            <n-gi :span="2">{{ $t('craftsmanship') }}</n-gi>
            <n-gi>{{ attributes?.craftsmanship }}</n-gi>
            <n-gi>+ {{ craftsmanshipAddons }}</n-gi>
            <n-gi>+ 0</n-gi>
            <n-gi>= {{ attributs.craftsmanship }}</n-gi>

            <n-gi :span="2">{{ $t('control') }}</n-gi>
            <n-gi>{{ attributes?.control }}</n-gi>
            <n-gi>+ {{ controlAddons }}</n-gi>
            <n-gi>+ 0</n-gi>
            <n-gi>= {{ attributs.control }}</n-gi>

            <n-gi :span="2">{{ $t('craft-point') }}</n-gi>
            <n-gi>{{ attributes?.craft_points }}</n-gi>
            <n-gi>+ {{ craftPointAddons }}</n-gi>
            <n-gi>+ 0</n-gi>
            <n-gi>= {{ attributs.craft_points }}</n-gi>

            <n-gi :span="2">{{ $t('initial-quality') }}</n-gi>
            <n-gi>{{ initialQuality }}</n-gi>
            <n-gi>+ 0</n-gi>
            <n-gi>+ 0</n-gi>
            <n-gi>= {{ initialQuality }}</n-gi>
        </n-grid>
    </n-card>
</template>
