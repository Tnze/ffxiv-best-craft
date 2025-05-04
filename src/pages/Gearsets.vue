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
import {
    ElScrollbar,
    ElTabs,
    ElTabPane,
    ElIcon
} from 'element-plus';
import { onActivated } from 'vue';
import { Close } from '@element-plus/icons-vue';
import useGearsetsStore, { labelWrapper } from '@/stores/gearsets';
import Gearset from '@/components/Gearset.vue';

const emit = defineEmits<{
    (e: 'setTitle', title: string): void;
}>();
onActivated(() => emit('setTitle', 'attributes'));

const store = useGearsetsStore();
</script>

<template>
    <el-scrollbar>
        <el-tabs class="page" v-model="store.jobPage" tab-position="left" :addable="store.showAddSetButton" @tab-add="store.addSet" >
            <el-tab-pane
                v-for="v in store.special"
                :name="v.name"
            >
                <template #label>
                    <span>{{ labelWrapper(v) }}</span>
                        <el-icon
                            v-if="v.name != store.defaultSet.name"
                            @click.stop="store.removeSet(v.name)"
                        >
                            <Close />
                        </el-icon>
                </template>
                <Gearset :name="v.name" />
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
