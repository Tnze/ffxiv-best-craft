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
import { NSelect, SelectOption, SelectRenderLabel } from 'naive-ui';
import { useFluent } from 'fluent-vue';

import { Jobs } from '@/libs/Craft';
import useFcoSimulatorStore from '../stores/simulator';

const { $t } = useFluent()
const store = useFcoSimulatorStore();

const options: SelectOption[] = Object.values(Jobs).map(j => ({
    label: j as string,
    value: j,
}))

const renderTag: SelectRenderLabel = (option, selected) => {
    return $t(option.label as string)
}
</script>

<template>
    <n-select v-model:value="store.job" :options="options" :render-label="renderTag"></n-select>
</template>