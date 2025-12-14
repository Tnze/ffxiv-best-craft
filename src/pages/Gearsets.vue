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
    ElScrollbar,
    ElTabs,
    ElTabPane,
    TabPaneName,
    ElMessage,
} from 'element-plus';
import { onActivated, ref } from 'vue';
import useGearsetsStore from '@/stores/gearsets';
import { useFluent } from 'fluent-vue';
import Gearset from '@/components/Gearset.vue';
import { choiceGearsetDisplayName } from '@/libs/Gearsets';

const emit = defineEmits<{
    (e: 'setTitle', title: string): void;
}>();
const { $t } = useFluent();

onActivated(() => emit('setTitle', 'attributes'));
const store = useGearsetsStore();
const jobPage = ref(0);

function handleGearsetsEdit(
    target: TabPaneName | undefined,
    action: 'remove' | 'add',
) {
    if (action === 'add') {
        store.addGearset();
        return;
    }
    if (action === 'remove') {
        if (target == 0) {
            ElMessage({
                type: 'error',
                message: $t('cannot-remove-default-gearsets'),
            });
            return;
        }
        store.delGearset(target as number);
        if (jobPage.value === target) {
            // always works because 0 cannot be removed
            jobPage.value = target - 1;
        }
        return;
    }
}
</script>

<template>
    <el-scrollbar>
        <el-tabs
            class="page"
            v-model="jobPage"
            tab-position="left"
            editable
            @edit="handleGearsetsEdit"
        >
            <el-tab-pane
                v-for="(v, i) in store.gearsets"
                :name="v.id"
                :label="choiceGearsetDisplayName(v)"
                lazy
            >
                <Gearset :index="i" :id="v.id" />
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
    max-width: 700px;
    margin-left: 20px;
}
</style>

<fluent locale="zh-CN">
cannot-remove-default-gearsets = 不能删除默认配装
</fluent>

<fluent locale="zh-TW">
cannot-remove-default-gearsets = 不能刪除預設配裝
</fluent>

<fluent locale="en-US">
cannot-remove-default-gearsets = Can't remove default gearsets
</fluent>

<fluent locale="ja-JP">
cannot-remove-default-gearsets = デフォルトギアセットを削除できません
</fluent>
