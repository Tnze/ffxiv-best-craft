<!-- 
    This file is part of BestCraft.
    Copyright (C) 2025  Tnze

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
import {
    ElForm,
    ElFormItem,
    ElInputNumber,
    ElInput,
    ElCheckboxGroup,
    ElCheckboxButton,
} from 'element-plus';
import { Jobs } from '@/libs/Craft';
import useGearsets from '@/stores/gearsets';
import { choiceGearsetDisplayName } from '@/libs/Gearsets';

const store = useGearsets();
const props = defineProps<{
    index: number;
    simplify?: boolean;
}>();
</script>

<template>
    <el-form label-position="right" label-width="auto">
        <template v-if="!simplify && store.gearsets[index].id != 0">
            <el-form-item :label="$t('gearset-name')">
                <el-input
                    v-model="store.gearsets[index].name"
                    class="set-name-input"
                    :maxlength="10"
                    :placeholder="
                        choiceGearsetDisplayName(store.gearsets[index])
                    "
                />
            </el-form-item>
            <el-form-item :label="$t('job')">
                <el-checkbox-group
                    v-model="store.gearsets[index].compatibleJobs"
                    size="small"
                >
                    <el-checkbox-button
                        v-for="job in Object.values(Jobs)"
                        :label="$t(job)"
                        :value="job"
                    />
                </el-checkbox-group>
            </el-form-item>
        </template>
        <el-form-item :label="$t('level')">
            <el-input-number
                v-model="store.gearsets[index].value.level"
                :min="1"
                :max="100"
                :step-strictly="true"
                :value-on-clear="0"
            />
        </el-form-item>
        <el-form-item :label="$t('craftsmanship')">
            <el-input-number
                v-model="store.gearsets[index].value.craftsmanship"
                :min="0"
                :step-strictly="true"
                :value-on-clear="0"
            />
        </el-form-item>
        <el-form-item :label="$t('control')">
            <el-input-number
                v-model="store.gearsets[index].value.control"
                :min="0"
                :step-strictly="true"
                :value-on-clear="0"
            />
        </el-form-item>
        <el-form-item :label="$t('craft-point')">
            <el-input-number
                v-model="store.gearsets[index].value.craft_points"
                :min="0"
                :step-strictly="true"
                :value-on-clear="0"
            />
        </el-form-item>
    </el-form>
</template>

<style scoped>
.set-name-input {
    width: 200px;
}
</style>

<fluent locale="zh-CN">
gearset-name = 配装名称
job = 适配职业
attributes = 装备属性
inherit-from-default = 继承自默认
</fluent>

<fluent locale="en-US">
gearset-name = Gearset Name
job = Fit Job
attributes = Crafter Attributes
inherit-from-default = Inherit from default
</fluent>

<fluent locale="ja-JP">
gearset-name = ギアセット名
attributes = 属性
inherit-from-default = デフォルトから継承
</fluent>
