<!-- 
    This file is part of BestCraft.
    Copyright (C) 2023  Tnze

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
import { computed } from 'vue';
import { ElForm, ElFormItem, ElSwitch, ElInputNumber } from 'element-plus';
import useGearsets from '@/stores/gearsets'
import { Jobs } from '@/libs/Craft';

const store = useGearsets()
const props = defineProps<{
    job: Jobs
}>()
const v = computed(() => store.special.find(v => v.name == props.job))

</script>

<template>
    <el-form v-if="v != undefined" label-position="right" label-width="auto" :model="v" style="max-width: 500px">
        <el-form-item :label="$t('attributes')">
            <el-switch :model-value="v.value == null" :active-text="$t('inherit-from-default')"
                @change="v.value = v.value == null ? { ...store.default } : null" />
        </el-form-item>
        <el-form-item :label="$t('level')">
            <el-input-number v-model="(v.value || store.default).level" :disabled="v.value == null" :min="0" :max="90"
                :step-strictly="true"></el-input-number>
        </el-form-item>
        <el-form-item :label="$t('craftsmanship')">
            <el-input-number v-model="(v.value || store.default).craftsmanship" :disabled="v.value == null" :min="0"
                :step-strictly="true"></el-input-number>
        </el-form-item>
        <el-form-item :label="$t('control')">
            <el-input-number v-model="(v.value || store.default).control" :disabled="v.value == null" :min="0"
                :step-strictly="true"></el-input-number>
        </el-form-item>
        <el-form-item :label="$t('craft-point')">
            <el-input-number v-model="(v.value || store.default).craft_points" :disabled="v.value == null" :min="0"
                :step-strictly="true"></el-input-number>
        </el-form-item>
    </el-form>
</template>

<fluent locale="zh-CN">
attributes = 装备属性
inherit-from-default = 继承自默认
</fluent>

<fluent locale="en-US">
attributes = Crafter Attributes
inherit-from-default = Inherit from default
</fluent>

<fluent locale="ja-JP">
attributes = 属性
inherit-from-default = デフォルトから継承
</fluent>