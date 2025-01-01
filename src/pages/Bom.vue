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
import { computed, onActivated, ref, useTemplateRef } from 'vue';
import {
    ElScrollbar,
    ElDivider,
    ElDialog,
    ElButton,
    ElAlert,
    ElIcon,
    ElText,
} from 'element-plus';
import { Plus, Delete, Loading, Refresh } from '@element-plus/icons-vue';
import { useElementBounding, UseElementBoundingReturn } from '@vueuse/core';
import { useFluent } from 'fluent-vue';

import BomItem from '@/components/bom/Item.vue';
import Selector from '@/components/bom/Selector.vue';
import Curves, { Relation } from '@/components/bom/Curves.vue';
import useStore, { Item, Slot as BomSlot, ItemID } from '@/stores/bom';

const emit = defineEmits<{
    (e: 'setTitle', title: string): void;
}>();
onActivated(() => emit('setTitle', 'bill-of-material'));

const { $t } = useFluent();
const store = useStore();
const selectorOpen = ref(false);
const errMsg = ref<string>();
const calculating = ref(false);

type BomItemType = InstanceType<typeof BomItem>;
const page = useTemplateRef<HTMLDivElement>('page');
const targetItems = useTemplateRef<BomItemType[]>('target-items');
const ingItems = useTemplateRef<BomItemType[]>('ing-items');

const pageBound = useElementBounding(page);

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
    return groups.splice(1);
});

function addTarget(item: Item) {
    selectorOpen.value = false;
    store.addTarget(item);
    updateBom();
}

function clearTargets() {
    store.targetItems.splice(0);
    updateBom();
}

let runningProcess: Promise<void> | undefined = undefined;
async function updateBom() {
    let p;
    try {
        errMsg.value = undefined;
        calculating.value = true;

        p = store.updateBom();
        runningProcess = p;
        await p;

        // 由于不知为何 vue 提供的 useTemplateRef 不会响应式更新，
        // 因此在此处手动从 targetItems 和 ingItems 中构造 itemsElems
        itemsElems.value = new Map(
            // 当存在重复键值时 new Map() 会用后面的覆盖前面的。
            // 因此当一个物品同时作为目标与素材时，我们的连线会连在素材上。
            [targetItems.value, ingItems.value]
                .filter(v => v != null)
                .flatMap(items =>
                    items
                        .filter(v => v.$el != undefined)
                        .map(v => [v.id, useElementBounding(v.$el)]),
                ),
        );
    } catch (e: any) {
        errMsg.value = String(e);
    } finally {
        if (p == runningProcess) {
            runningProcess = undefined;
            calculating.value = false;
        }
    }
}

const slots = computed(
    () => new Map(store.ingredients.map(v => [v.item.id, v])),
);
let itemsElems = ref(new Map<ItemID, UseElementBoundingReturn>());
const relations = computed(() => {
    const lines: Relation[] = [];
    for (const [itemId1, elem1] of itemsElems.value) {
        const slot = slots.value.get(itemId1);
        if (slot == undefined) continue;
        for (const [itemId2, amount] of slot.requiredBy) {
            const elem2 = itemsElems.value.get(itemId2);
            if (elem2 == undefined) continue;
            lines.push({
                p1: {
                    x: elem1.x + elem1.width / 2,
                    y: elem1.top,
                },
                p2: {
                    x: elem2.x + elem2.width / 2,
                    y: elem2?.bottom,
                },
                type: amount == 0 ? 'not-required' : slot.type,
            });
        }
    }
    return lines;
});
</script>

<template>
    <el-scrollbar ref="page">
        <el-dialog v-model="selectorOpen" :title="$t('select-recipe')">
            <Selector @click-item="addTarget" />
        </el-dialog>
        <div class="page">
            <el-scrollbar>
                <TransitionGroup class="row" tag="div">
                    <BomItem
                        class="item"
                        v-for="item of store.targetItems"
                        ref="target-items"
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
                            @click="clearTargets"
                            style="width: 100%; margin: 10px 0 0 0; flex: auto"
                            :icon="Delete"
                        >
                            {{ $t('clear') }}
                        </el-button>
                    </div>
                </TransitionGroup>
            </el-scrollbar>
            <el-divider content-position="left">
                <el-text>
                    <template v-if="calculating">
                        <el-text v-if="store.fetchingItem" type="info">
                            {{
                                store.fetchingItem != undefined
                                    ? $t('fetching-item', {
                                          itemName: store.fetchingItem,
                                      })
                                    : $t('calculating')
                            }}
                        </el-text>
                        <el-icon class="is-loading">
                            <Loading />
                        </el-icon>
                    </template>
                    <template v-else>
                        {{ $t('ings') }}
                        <el-button @click="updateBom()" text>
                            <el-icon>
                                <Refresh />
                            </el-icon>
                        </el-button>
                    </template>
                </el-text>
            </el-divider>
            <el-alert v-if="errMsg" type="error" show-icon :closable="false">
                {{ errMsg }}
            </el-alert>
            <el-scrollbar v-for="group of groupedIngs">
                <TransitionGroup class="row ings-row" tag="div">
                    <BomItem
                        class="item"
                        v-for="item of group"
                        ref="ing-items"
                        :key="item.item.id"
                        :id="item.item.id"
                        :name="item.item.name"
                        :wasted="item.wasted"
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
            <Curves :relations="relations" :clip-zone="pageBound" />
        </div>
    </el-scrollbar>
</template>

<style scoped>
.page {
    padding: 10px;
}

.row {
    display: flex;
}

.ings-row {
    padding-bottom: 30px;
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
calculating = 计算中
fetching-item = 正在查询 { $itemName }
add = 添加
clear = 清空
</fluent>

<fluent locale="en-US">
ings = Ingredients
calculating = Calculating
fetching-item = Fetching { $itemName }
add = Add
clear = Clear
</fluent>

<fluent locale="ja-JP">
ings = コンポーネント
calculating = ずるい
fetching-item = つかむ { $itemName }
add = 追加
clear = パージ
</fluent>
