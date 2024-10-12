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
import { ElScrollbar, ElTabs, ElTabPane, ElForm, ElFormItem, ElInputNumber } from 'element-plus'
import { onActivated, ref } from 'vue'
import useGearsetsStore from '@/stores/gearsets'
import Gearset from '@/components/Gearset.vue'

const emit = defineEmits<{
    (e: 'setTitle', title: string): void
}>()
onActivated(() => emit('setTitle', 'attributes'))

const store = useGearsetsStore()
const jobPage = ref('default')

</script>

<template>
    <el-scrollbar>
        <el-tabs class="page" v-model="jobPage" tab-position="left">
            <el-tab-pane name="default" :label="$t('default')">
                <el-form label-position="right" label-width="auto" :model="store.default">
                    <el-form-item :label="$t('level')">
                        <el-input-number :model-value="store.default.level" :min="1" :max="100"
                            :step-strictly="true" @change="x => store.default.level = x || 1" />
                    </el-form-item>
                    <el-form-item :label="$t('craftsmanship')">
                        <el-input-number :model-value="store.default.craftsmanship" :min="0"
                            :step-strictly="true" @change="x => store.default.craftsmanship = x || 0" />
                    </el-form-item>
                    <el-form-item :label="$t('control')">
                        <el-input-number :model-value="store.default.control" :min="0"
                            :step-strictly="true" @change="x => store.default.control = x || 0" />
                    </el-form-item>
                    <el-form-item :label="$t('craft-point')">
                        <el-input-number :model-value="store.default.craft_points" :min="0"
                            :step-strictly="true" @change="x => store.default.craft_points = x || 0" />
                    </el-form-item>
                </el-form>
            </el-tab-pane>
            <el-tab-pane v-for="v in store.special" :name="v.name" :label="$t(v.name)">
                <Gearset :job="v.name" />
            </el-tab-pane>
        </el-tabs>
    </el-scrollbar>
</template>

<style scoped>
.el-tabs {
    user-select: none;
}

.page {
    padding: 20px 0 0 0;
    background-color: transparent !important;
}

.el-form {
    max-width: 500px;
    margin-left: 20px;
}
</style>

<fluent locale="zh-CN">
default = 默认
</fluent>

<fluent locale="en-US">
default = Default
</fluent>

<fluent locale="ja-JP">
default = デフォルト
</fluent>