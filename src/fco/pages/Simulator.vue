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
import { NGrid, NGi, NMessageProvider, NCard, NTabs, NTabPane, NButton, NFlex, NModalProvider } from 'naive-ui';
import ActionQueue from '@/components/designer/ActionQueue.vue';
import Buffs from '@/components/designer/Buffs.vue';
import { Actions, Attributes, newStatus, simulate, SimulateResult, Status } from '@/libs/Craft';
import { Sequence } from '@/components/designer/types';

import useFcoSimulatorStore from '../stores/simulator';
import useGearsetsStore from '@/stores/gearsets'

import ActionPanel from '../components/ActionPanel.vue';
import AttributesVue from '../components/Attributes.vue';
import CraftingState from '../components/CraftingState.vue';
import JobSelect from '../components/JobSelect.vue';
import RecipeSelect from '../components/RecipeSelect.vue';
import Analyzer from '../components/Analyzer.vue';
import SolveModal from '../components/SolveModal.vue';
import Macros from '../components/Macros.vue';

const gearsetsStore = useGearsetsStore()
const store = useFcoSimulatorStore();

// 持久化 Store
store.$subscribe((_mutation, state) => {
    localStorage.setItem('fco-simulator', JSON.stringify(state))
})

const rotation = reactive<Sequence>({
    slots: store.rotation.map((action, id) => ({ id, action })),
    maxid: store.rotation.length,
});
const attributs = ref<Attributes>(gearsetsStore.attributes(store.job))
const initStatus = ref<Status>();
const simulateResult = ref<SimulateResult>();
const editMode = ref(false);
const solverModalShow = ref(false)

// 计算初始状态
watchEffect(async () => {
    if (!store.recipe) {
        initStatus.value = undefined;
        return;
    }
    try {
        initStatus.value = await newStatus(
            attributs.value,
            store.recipe.recipe,
            store.recipe.recipeLevel
        );
    } catch (e: any) {
        console.error(e)
        initStatus.value = undefined;
    }
})

// 计算模拟结果
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

function save() {
    editMode.value = false;
    store.rotation = rotation.slots.map(v => v.action);
}


</script>

<template>
    <n-message-provider>
        <n-grid x-gap="20px" y-gap="20px" :cols="12" item-responsive responsive="screen">
            <n-gi span="12 m:2">
                <!-- <n-checkbox>自定义配方</n-checkbox> -->
            </n-gi>
            <n-gi span="6 m:2">
                <JobSelect />
            </n-gi>
            <n-gi span="12 m:4">
                <RecipeSelect />
            </n-gi>

            <n-gi span="12 m:6">
                <CraftingState :job="store.job" :status="simulateResult?.status" />
            </n-gi>
            <n-gi span="12 m:6">
                <AttributesVue v-model:attributs="attributs" />
            </n-gi>

            <n-gi v-if="editMode" :span="12">
                <n-card>
                    <Buffs v-if="simulateResult" :buffs="simulateResult.status.buffs" />
                </n-card>
            </n-gi>
            <n-gi :span="12">
                <n-card>
                    <ActionQueue v-model:list="rotation.slots" :job="store.job" :err-list="simulateResult?.errors"
                        :disabled="!editMode" />
                </n-card>
            </n-gi>
            <n-gi :span="12">
                <n-flex>
                    <template v-if="!editMode">
                        <n-button type="primary" @click="editMode = true">{{ $t('edit') }}</n-button>
                        <n-button type="primary" @click="solverModalShow = true">{{ $t('solve') }}</n-button>
                    </template>
                    <template v-else>
                        <n-button type="primary" @click="save">{{ $t('save') }}</n-button>
                        <n-button type="warning" @click="rotation.slots.splice(0)">{{ $t('clear') }}</n-button>
                    </template>
                </n-flex>
                <n-modal-provider>
                    <SolveModal v-model:show="solverModalShow" />
                </n-modal-provider>
            </n-gi>
            <n-gi v-if="editMode" :span="12">
                <ActionPanel :job="store.job" :status="simulateResult?.status ?? initStatus"
                    @clicked-action="pushAction" />
            </n-gi>
            <n-gi v-if="initStatus" :span="12">
                <n-tabs>
                    <n-tab-pane name="random" :tab="$t('random')">
                        <Analyzer :init-status="initStatus" :actions="rotation.slots.map(s => s.action)" />
                    </n-tab-pane>
                    <n-tab-pane name="macro" :tab="$t('macro')">
                        <Macros :actions="rotation.slots.map(s => s.action)" />
                    </n-tab-pane>
                </n-tabs>
            </n-gi>
        </n-grid>
    </n-message-provider>
</template>

<style scoped></style>

<fluent locale="zh-CN">
edit = 编辑
solve = 求解

cancel = 取消
save = 保存
clear = 清空

random = 随机模拟
macro = 宏指令
</fluent>

<fluent locale="en-US">
edit = Edit
solve = Solve

cancel = Cancel
save = Save
clear = Clear

random = Random
macro = Macro
</fluent>
