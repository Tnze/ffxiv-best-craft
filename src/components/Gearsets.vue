<!-- 
    This file is part of BestCraft.
    Copyright (C) 2023  <name of author>

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
import { ElContainer, ElHeader, ElMain, ElTabs, ElTabPane, ElForm, ElFormItem, ElInputNumber, ElSwitch } from 'element-plus'
import { ref } from 'vue'
import useGearsetsStore from '@/stores/gearsets'

const store = useGearsetsStore()
const jobPage = ref('default')

</script>

<template>
    <el-container>
        <el-header>
            <h1>{{ $t('attributes') }}</h1>
        </el-header>
        <el-main>
            <el-tabs v-model="jobPage" tab-position="left">
                <el-tab-pane name="default" :label="$t('default')">
                    <el-form label-position="right" label-width="130px" :model="store.default" style="max-width: 500px">
                        <el-form-item :label="$t('level')">
                            <el-input-number v-model="store.default.level" :min="1" :max="90"
                                :step-strictly="true"></el-input-number>
                        </el-form-item>
                        <el-form-item :label="$t('craftsmanship')">
                            <el-input-number v-model="store.default.craftsmanship" :min="0"
                                :step-strictly="true"></el-input-number>
                        </el-form-item>
                        <el-form-item :label="$t('control')">
                            <el-input-number v-model="store.default.control" :min="0"
                                :step-strictly="true"></el-input-number>
                        </el-form-item>
                        <el-form-item :label="$t('craft-point')">
                            <el-input-number v-model="store.default.craft_points" :min="0"
                                :step-strictly="true"></el-input-number>
                        </el-form-item>
                    </el-form>
                </el-tab-pane>
                <el-tab-pane v-for="v in store.special" :name="v.name" :label="$t(v.name)">
                    <el-form label-position="right" label-width="130px" :model="v" style="max-width: 500px">
                        <el-form-item :label="$t('attributes')">
                            <el-switch :model-value="v.value == null" :active-text="$t('inherit-from-default')"
                                @change="v.value = v.value == null ? { ...store.default } : null" />
                        </el-form-item>
                        <el-form-item :label="$t('level')">
                            <el-input-number v-model="(v.value || store.default).level" :disabled="v.value == null" :min="0"
                                :max="90" :step-strictly="true"></el-input-number>
                        </el-form-item>
                        <el-form-item :label="$t('craftsmanship')">
                            <el-input-number v-model="(v.value || store.default).craftsmanship" :disabled="v.value == null"
                                :min="0" :step-strictly="true"></el-input-number>
                        </el-form-item>
                        <el-form-item :label="$t('control')">
                            <el-input-number v-model="(v.value || store.default).control" :disabled="v.value == null"
                                :min="0" :step-strictly="true"></el-input-number>
                        </el-form-item>
                        <el-form-item :label="$t('craft-point')">
                            <el-input-number v-model="(v.value || store.default).craft_points" :disabled="v.value == null"
                                :min="0" :step-strictly="true"></el-input-number>
                        </el-form-item>
                    </el-form>
                </el-tab-pane>
            </el-tabs>
        </el-main>
    </el-container>
</template>

<style scoped>
.el-tabs {
    user-select: none;
}

.el-main {
    background-color: transparent !important;
}
</style>

<fluent locale="zh-CN">
attributes = 装备属性
default = 默认
inherit-from-default = 继承自默认
</fluent>

<fluent locale="en-US">
attributes = Crafter Attributes
default = Default
inherit-from-default = Inherit from default
</fluent>

<fluent locale="ja-JP">
attributes = 属性
default = デフォルト
inherit-from-default = デフォルトから継承
</fluent>