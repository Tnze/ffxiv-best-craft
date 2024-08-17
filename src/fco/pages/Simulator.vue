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

import { NGrid, NGi, NCheckbox, NSelect, NCard, NTabs, NTabPane } from 'naive-ui';
import ActionQueue from '@/components/designer/ActionQueue.vue';
import { Actions, Jobs } from '@/libs/Craft';
import { reactive, markRaw } from 'vue';
import { Sequence } from '@/components/designer/types';

import ActionPanel from '../components/ActionPanel.vue';
import Attributes from '../components/Attributes.vue';
import CraftingState from '../components/CraftingState.vue';

const rotation = reactive<Sequence>({
    slots: [
        { id: 0, action: Actions.TrainedEye },
        { id: 1, action: Actions.Groundwork },
    ],
    maxid: 2,
});

function pushAction(action: Actions) {
    rotation.slots.push(markRaw({ id: rotation.maxid++, action }));
}

</script>

<template>
    <n-grid x-gap="20px" y-gap="20px" :cols="12" item-responsive responsive="screen">
        <n-gi :span="2">
            <n-checkbox>自定义配方</n-checkbox>
        </n-gi>
        <n-gi :span="2">
            <n-select></n-select>
        </n-gi>
        <n-gi :span="4">
            <n-select></n-select>
        </n-gi>
        <n-gi span="12 m:6">
            <CraftingState />
        </n-gi>
        <n-gi span="12 m:6">
            <Attributes />
        </n-gi>
        <n-gi :span="12">
            <n-card>
                <ActionQueue v-model:list="rotation.slots" :job="Jobs.Carpenter" />
            </n-card>
        </n-gi>
        <n-gi :span="12">
            <ActionPanel :job="Jobs.Carpenter"  @clicked-action="pushAction" />
        </n-gi>
        <n-gi :span="6">
            <n-tabs>
                <n-tab-pane name="Random"> </n-tab-pane>
                <n-tab-pane name="Macro"> </n-tab-pane>
            </n-tabs>
        </n-gi>
    </n-grid>
</template>

<style scoped></style>
