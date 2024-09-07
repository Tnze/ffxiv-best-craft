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
import { ref, reactive, markRaw, watchEffect } from 'vue';
import { NGrid, NGi, NCheckbox, NMessageProvider, NCard, NTabs, NTabPane } from 'naive-ui';
import ActionQueue from '@/components/designer/ActionQueue.vue';
import Buffs from '@/components/designer/Buffs.vue';
import { Actions, newStatus, simulate, SimulateResult, Status } from '@/libs/Craft';
import { Sequence } from '@/components/designer/types';

import useFcoSimulatorStore from '../stores/simulator';
import useGearsetsStore from '@/stores/gearsets'

import ActionPanel from '../components/ActionPanel.vue';
import Attributes from '../components/Attributes.vue';
import CraftingState from '../components/CraftingState.vue';
import JobSelect from '../components/JobSelect.vue';
import RecipeSelect from '../components/RecipeSelect.vue';

const gearsetsStore = useGearsetsStore()
const store = useFcoSimulatorStore();

const rotation = reactive<Sequence>({
    slots: [
        { id: 0, action: Actions.TrainedEye },
        { id: 1, action: Actions.Groundwork },
    ],
    maxid: 2,
});
const initStatus = ref<Status>();
const simulateResult = ref<SimulateResult>();

// 计算初始状态
watchEffect(async () => {
    if (!store.recipe) {
        initStatus.value = undefined;
        return;
    }
    try {
        initStatus.value = await newStatus(
            gearsetsStore.attributes(store.job),
            store.recipe.recipe,
            store.recipe.recipeLevel
        );
    } catch (e: any) {
        console.error(e)
        initStatus.value = undefined;
    }
})

watchEffect(async () => {
    if (!initStatus.value) {
        simulateResult.value = undefined
        return;
    }
    simulateResult.value = await simulate(initStatus.value, rotation.slots.map(slot => slot.action));
})

function pushAction(action: Actions) {
    rotation.slots.push(markRaw({ id: rotation.maxid++, action }));
}

</script>

<template>
    <n-message-provider>
        <n-grid x-gap="20px" y-gap="20px" :cols="12" item-responsive responsive="screen">
            <n-gi :span="2">
                <n-checkbox>自定义配方</n-checkbox>
            </n-gi>
            <n-gi :span="2">
                <JobSelect />
            </n-gi>
            <n-gi :span="4">
                <RecipeSelect />
            </n-gi>
            <n-gi span="12 m:6">
                <CraftingState :job="store.job" :status="simulateResult?.status" />
            </n-gi>
            <n-gi span="12 m:6">
                <Attributes />
            </n-gi>
            <n-gi :span="12">
                <n-card>
                    <Buffs v-if="simulateResult" :buffs="simulateResult.status.buffs" />
                </n-card>
            </n-gi>
            <n-gi :span="12">
                <n-card>
                    <ActionQueue v-model:list="rotation.slots" :job="store.job" :err-list="simulateResult?.errors" />
                </n-card>
            </n-gi>
            <n-gi :span="12">
                <ActionPanel :job="store.job" :status="simulateResult?.status ?? initStatus"
                    @clicked-action="pushAction" />
            </n-gi>
            <n-gi :span="6">
                <n-tabs>
                    <n-tab-pane name="Random"> </n-tab-pane>
                    <n-tab-pane name="Macro"> </n-tab-pane>
                </n-tabs>
            </n-gi>
        </n-grid>
    </n-message-provider>
</template>

<style scoped></style>
