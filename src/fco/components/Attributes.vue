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
import { NCard, NSelect, NGrid, NGi } from 'naive-ui';
</script>

<template>
    <n-card>
        <template #header>状态参数</template>
        <n-grid x-gap="12" :cols="2">
            <n-gi>
                <span>{{ $t('meals') }}</span>
                <n-select :placeholder="$t('select-meals')"></n-select>
            </n-gi>
            <n-gi>
                <span>{{ $t('medicines') }}</span>
                <n-select :placeholder="$t('select-medicines')"></n-select>
            </n-gi>

            <n-gi :span="2">
                <span>{{ $t('craftsmanship') }}</span>
            </n-gi>
            <n-gi :span="2">
                <span>{{ $t('control') }}</span>
            </n-gi>
            <n-gi :span="2">
                <span>{{ $t('craft-point') }}</span>
            </n-gi>
            <n-gi :span="2">
                <span>{{ $t('initial-quality') }}</span>
            </n-gi>
        </n-grid>
    </n-card>
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
