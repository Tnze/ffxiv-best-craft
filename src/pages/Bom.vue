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
import { onActivated, ref } from 'vue';
import { ElScrollbar, ElDivider, ElDialog, ElButton } from 'element-plus';

import BomItem from '@/components/bom/Item.vue';
import Selector from '@/components/bom/Selector.vue';
import useStore, { Item } from '@/stores/bom';

const emit = defineEmits<{
    (e: 'setTitle', title: string): void;
}>();
onActivated(() => emit('setTitle', 'bill-of-material'));

const store = useStore();
const selectorOpen = ref(false);

function addTarget(item: Item) {
    selectorOpen.value = false;
    store.addTarget(item);
    store.updateBom();
}
</script>

<template>
    <el-scrollbar>
        <el-dialog v-model="selectorOpen" :title="$t('select-recipe')">
            <Selector @click-item="addTarget" />
        </el-dialog>
        <div class="page">
            <el-scrollbar>
                <div class="row">
                    <BomItem
                        class="item"
                        v-for="item in store.targetItems"
                        :id="item.item.id"
                        :name="item.item.name"
                        :required-number="item.getFixRequiredNumber()"
                        @update:required-number="
                            v => item.setFixRequiredNumber(v)
                        "
                        :holding-number="
                            store.holdingItems.get(item.item.id) ?? 0
                        "
                        @update:holding-number="
                            v => store.holdingItems.set(item.item.id, v)
                        "
                    />
                    <el-button
                        class="item"
                        @click="selectorOpen = true"
                        style="height: 166px; width: 147px"
                    >
                        Add
                    </el-button>
                    <el-button @click="store.updateBom()">Test</el-button>
                </div>
            </el-scrollbar>
            <el-divider />
            <el-scrollbar>
                <div class="row" style="flex-wrap: wrap">
                    <BomItem
                        class="item"
                        v-for="item in store.ingredients.filter(
                            v => v.requiredNumber() > v.getFixRequiredNumber(),
                        )"
                        :key="item.item.id"
                        :name="item.item.name"
                        :required-number="
                            item.requiredNumber() - item.getFixRequiredNumber()
                        "
                        requiredInputDisabled
                        :holding-number="
                            store.holdingItems.get(item.item.id) ?? 0
                        "
                        @update:holding-number="
                            v => store.holdingItems.set(item.item.id, v)
                        "
                    />
                </div>
            </el-scrollbar>
        </div>
    </el-scrollbar>
</template>

<style scoped>
.page {
    padding: 10px;
}

.row {
    display: flex;
    padding-bottom: 5px;
}

.item {
    flex: none;
    margin: 5px;
}
</style>
