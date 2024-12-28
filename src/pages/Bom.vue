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
import { computed, onActivated, ref } from 'vue';
import {
    ElScrollbar,
    ElDivider,
    ElDialog,
    ElButton,
    ElAlert,
    ElMessage,
    MessageHandler,
} from 'element-plus';
import { Plus, Delete } from '@element-plus/icons-vue';

import BomItem from '@/components/bom/Item.vue';
import Selector from '@/components/bom/Selector.vue';
import useStore, { Item, Slot as BomSlot } from '@/stores/bom';
import { useFluent } from 'fluent-vue';

const emit = defineEmits<{
    (e: 'setTitle', title: string): void;
}>();
onActivated(() => emit('setTitle', 'bill-of-material'));

const { $t } = useFluent();
const store = useStore();
const selectorOpen = ref(false);
const errMsg = ref<string>();

const groupedIngs = computed(() => {
    if (store.ingredients.length == 0) {
        return [];
    }
    const maxDepth = store.ingredients[store.ingredients.length - 1].depth;
    const groups: BomSlot[][] = new Array(maxDepth);
    for (const ing of store.ingredients) {
        if (groups[ing.depth] == undefined) groups[ing.depth] = [ing];
        else groups[ing.depth].push(ing);
    }
    return groups.slice(1);
});

function addTarget(item: Item) {
    selectorOpen.value = false;
    store.addTarget(item);
    updateBom();
}

function clearSelection() {
    store.targetItems.splice(0);
    updateBom();
}

let alertHandle: MessageHandler | undefined = undefined;
async function updateBom() {
    try {
        errMsg.value = undefined;
        await store.updateBom();
    } catch (e: any) {
        errMsg.value = String(e);
    }
}
</script>

<template>
    <el-scrollbar>
        <el-dialog v-model="selectorOpen" :title="$t('select-recipe')">
            <Selector @click-item="addTarget" />
        </el-dialog>
        <div class="page">
            <el-scrollbar>
                <TransitionGroup class="row" tag="div">
                    <BomItem
                        class="item"
                        v-for="item in store.targetItems"
                        :id="item.item.id"
                        :name="item.item.name"
                        :key="item.item.id"
                        :required-number="item.getFixRequiredNumber()"
                        @update:required-number="
                            v => {
                                item.setFixRequiredNumber(v);
                                store.updateBom();
                            }
                        "
                        :holding-number="
                            store.holdingItems.get(item.item.id) ?? 0
                        "
                        @update:holding-number="
                            v => {
                                store.holdingItems.set(item.item.id, v);
                                store.updateBom();
                            }
                        "
                        :type="item.type"
                    />
                    <div class="item button-group" key="button-group">
                        <el-button
                            @click="selectorOpen = true"
                            style="width: 100%; flex: auto"
                            type="primary"
                            :icon="Plus"
                        >
                            {{ $t('add') }}
                        </el-button>
                        <el-button
                            @click="clearSelection"
                            style="width: 100%; margin: 10px 0 0 0; flex: auto"
                            :icon="Delete"
                        >
                            {{ $t('clear') }}
                        </el-button>
                    </div>
                </TransitionGroup>
            </el-scrollbar>
            <el-divider content-position="left">{{ $t('ings') }}</el-divider>
            <el-alert v-if="errMsg" type="error" show-icon :closable="false">
                {{ errMsg }}
            </el-alert>
            <el-scrollbar v-for="group in groupedIngs">
                <TransitionGroup class="row" tag="div">
                    <BomItem
                        class="item"
                        v-for="item in group"
                        :key="item.item.id"
                        :id="item.item.id"
                        :name="item.item.name"
                        :required-number="item.requiredNumber()"
                        requiredInputDisabled
                        :holding-number="
                            store.holdingItems.get(item.item.id) ?? 0
                        "
                        @update:holding-number="
                            v => {
                                store.holdingItems.set(item.item.id, v);
                                store.updateBom();
                            }
                        "
                        :type="item.type"
                    />
                </TransitionGroup>
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
    padding-bottom: 20px;
}

.item {
    flex: none;
    margin: 5px;
}

.button-group {
    height: auto;
    width: 147px;
    display: flex;
    flex-direction: column;
}

.v-move,
.v-enter-active,
.v-leave-active {
    transition: all 0.5s ease;
}

.v-enter-from,
.v-leave-to {
    opacity: 0;
    transform: translateX(30px);
}

.v-leave-active {
    position: absolute;
}
</style>

<style>
.el-divider__text {
    background-color: var(--tnze-main-bg-color);
}
</style>

<fluent locale="zh-CN">
ings = 材料
add = 添加
clear = 清空
</fluent>

<fluent locale="en-US">
</fluent>

<fluent locale="ja-JP">
</fluent>
