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
import { Actions, Jobs } from '@/libs/Craft';

import { NGrid, NGi, NCheckbox, NSelect, NCard, NTabs, NTabPane } from 'naive-ui';
import CraftingState from '../components/CraftingState.vue';
import Attributes from '../components/Attributes.vue';

import ActionQueue from '@/components/designer/ActionQueue.vue';

const slots = [
    { id: 0, action: Actions.TrainedEye },
    { id: 0, action: Actions.Groundwork },
];

</script>

<template>
    <n-grid x-gap="20px" y-gap="20px" :cols="2" item-responsive responsive="screen">
        <n-gi :span="2">
            <n-checkbox>自定义配方</n-checkbox>
            <n-select></n-select>
            <n-select></n-select>
        </n-gi>
        <n-gi span="2 m:1">
            <CraftingState />
        </n-gi>
        <n-gi span="1 m:1">
            <Attributes />
        </n-gi>
        <n-gi :span="2">
            <n-card>
                <ActionQueue :list="slots" :job="Jobs.Carpenter" />
            </n-card>
        </n-gi>
        <n-gi :span="2">
            <n-tabs>
                <n-tab-pane name="Random"> </n-tab-pane>
                <n-tab-pane name="Macro"> </n-tab-pane>
            </n-tabs>
        </n-gi>
    </n-grid>
</template>

<style scoped></style>
