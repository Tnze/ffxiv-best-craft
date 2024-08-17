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
import { NTabs, NTabPane, NForm, NFormItem, NInputNumber } from 'naive-ui';
import useGearsetsStore from '@/stores/gearsets'

const store = useGearsetsStore()
</script>

<template>
    <n-tabs>
        <n-tab-pane v-for="v in store.special" :name="v.name" :label="$t(v.name)">
            <n-form :model="v" label-placement="left" label-width="auto">
                <n-form-item :label="$t('level')">
                    <n-input-number v-model:value="(v.value || store.default).level" :disable="!v.value" />
                </n-form-item>
                <n-form-item :label="$t('craftsmanship')">
                    <n-input-number v-model:value="(v.value || store.default).craftsmanship" :disable="!v.value" />
                </n-form-item>
                <n-form-item :label="$t('control')">
                    <n-input-number v-model:value="(v.value || store.default).control" :disable="!v.value" />
                </n-form-item>
                <n-form-item :label="$t('craft-point')">
                    <n-input-number v-model:value="(v.value || store.default).craft_points" :disable="!v.value" />
                </n-form-item>
            </n-form>
        </n-tab-pane>
    </n-tabs>
</template>