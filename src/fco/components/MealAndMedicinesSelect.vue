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
import { ref } from 'vue';
import { NSelect } from 'naive-ui';

import { BetaXivapiBase, BetaXivApiRecipeSource } from '@/components/recipe-manager/beta-xivapi-source';
import { Enhancer } from '@/libs/Enhancer';

const dataSource = new BetaXivApiRecipeSource(BetaXivapiBase);
const enhancers = ref<Enhancer[]>()
const loading = ref(false)

const props = defineProps<{ type: 'meals' | 'medicines' }>()

const emits = defineEmits<{
    (event: 'select', v: Enhancer | undefined): void
}>()

async function loadMeals() {
    let i = 0, v;
    enhancers.value = []

    loading.value = true;
    do {
        if (v?.next) {
            v = await v.next();
        } else {
            v = await (props.type == 'meals'
                ? dataSource.mealsTable(i += 1)
                : dataSource.medicineTable(i += 1));
        }
        enhancers.value = enhancers.value.concat(v.results)
    } while (i < v.totalPages || v.next)
    loading.value = false;
}
loadMeals()

function handleUpdateValue(i: number) {
    emits('select', enhancers.value ? enhancers.value[i] : undefined)
}

</script>

<template>
    <n-select :placeholder="$t('select-meals')" :options="enhancers?.map((v, i) => ({ label: v.name, value: i }))"
        @update-value="handleUpdateValue" :loading="loading" clearable remote :reset-menu-on-options-change="false" />
</template>

<fluent locale="zh-CN">
select-meals = 选择{ meals }
select-medicines = 选择{ medicines }
</fluent>

<fluent locale="en-US">
select-meals = Select { meals }
select-medicines = Select { medicines }
</fluent>

<fluent locale="ja-JP">
</fluent>