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
import { ElContainer, ElHeader, ElMain, ElScrollbar, ElDialog, ElButton, ElTable, ElTableColumn, ElCard, ElSwitch } from 'element-plus';
import { computed, ref, watch } from 'vue';
import { Recipe, Item, Attributes, Jobs, newStatus, Status, Actions, Conditions, simulateOneStep, RecipeLevel } from '@/libs/Craft';
import { Enhancer } from "../attr-enhancer/Enhancer";
import StatusBarVue from './StatusBar.vue';
import ActionPanelVue from './ActionPanel.vue';
import ActionQueueVue from './ActionQueue.vue';
import AttrEnhSelector from '../attr-enhancer/AttrEnhSelector.vue';

const props = defineProps<{
    recipe: Recipe,
    recipeLevel: RecipeLevel,
    item: Item,
    attributes: Attributes,
    displayJob: Jobs,
}>()

interface Slot {
    id: number,
    action: Actions,
    condition: Conditions,
}

const attributesEnhancers = ref<Enhancer[]>([]);
const enhancedAttributes = computed<Attributes>(() => {
    let { level, craftsmanship, control, craft_points } = props.attributes;
    const sum = (prev: number, curr: number) => prev + curr;
    craftsmanship += attributesEnhancers.value
        .filter((v) => v.cm && v.cm_max)
        .map((v) => Math.min((craftsmanship * v.cm!) / 100, v.cm_max!))
        .reduce(sum, 0);
    control += attributesEnhancers.value
        .filter((v) => v.ct && v.ct_max)
        .map((v) => Math.min((control * v.ct!) / 100, v.ct_max!))
        .reduce(sum, 0);
    craft_points += attributesEnhancers.value
        .filter((v) => v.cp && v.cp_max)
        .map((v) => Math.min((craft_points * v.cp!) / 100, v.cp_max!))
        .reduce(sum, 0);
    return {
        level,
        craftsmanship,
        control,
        craft_points,
    };
});
const initStatus = ref<Status>({ ...await newStatus(enhancedAttributes.value, props.recipe, props.recipeLevel), quality: 0 });
watch([props, enhancedAttributes], async ([p, attr]) => { initStatus.value = { ...await newStatus(attr, p.recipe, p.recipeLevel), quality: 0 } });

const currentStatus = ref<Status>(initStatus.value)
const seq = ref<Slot[]>([])
const openAttrEnhSelector = ref(false)
const results = ref<Status[]>([])
const waiting = ref(false)
const preview = ref<Status | null>(null)
const rapidMode = ref(true)
let timer: any;

const sleep = (t: number) => new Promise((resolve) => setTimeout(resolve, t))
async function pushAction(action: Actions) {
    if (waiting.value) return;
    leaveAction()
    try {
        let wait;
        waiting.value = true
        if (!rapidMode.value) {
        }
        wait = sleep(rapidMode.value ? 300 : 1500)
        const { status, is_success } = await simulateOneStep(currentStatus.value, action, false);
        await wait;
        currentStatus.value = status
        if (!is_success) {
            action = <Actions>action.concat('_fail')
        }
        seq.value.push({
            id: seq.value.length, action,
            condition: currentStatus.value.condition,
        })
        if (status.progress >= status.recipe.difficulty || status.durability <= 0) {
            await sleep(2500);
            restart();
        }
    } catch (e: unknown) {
        console.error(e)
    } finally {
        if (timer != null) clearTimeout(timer)
        waiting.value = false
    }
}

function restart() {
    results.value.push(currentStatus.value)
    seq.value.splice(0)
    currentStatus.value = initStatus.value
}

function hoverAction(action: Actions) {
    if (timer != null) clearTimeout(timer)
    timer = setTimeout(() => {
        simulateOneStep(currentStatus.value, action, true)
            .then(v => preview.value = v.status)
            .catch(_e => { })
    }, 1000)
}

function leaveAction() {
    if (timer != null) clearTimeout(timer)
    preview.value = null
}

</script>

<template>
    <el-container>
        <el-dialog v-model="openAttrEnhSelector" :title="$t('meal-and-potion')">
            <AttrEnhSelector v-model="attributesEnhancers" />
        </el-dialog>
        <el-header>
            <h1>{{ $t('title', { recipe: item.name }) }}</h1>
        </el-header>
        <el-main>
            <div class="main-page">
                <StatusBarVue class="status-bar" :attributes="attributes" :enhancers="attributesEnhancers"
                    :status="preview ?? currentStatus" :disabled-init-quality="true" :show-condition="true"
                    @click-attributes="openAttrEnhSelector = true" />
                <el-scrollbar class="action-queue">
                    <ActionQueueVue :job="displayJob" :list="seq" disabled no-hover />
                </el-scrollbar>
                <div class="actionpanel">
                    <el-scrollbar class="action-panel">
                        <ActionPanelVue @clicked-action="pushAction" :disable="waiting" :job="displayJob"
                            :status="currentStatus" simulator-mode #lower @mousehover-action="hoverAction"
                            @mouseleave-action="leaveAction" />
                        <el-button class="drop" @click="restart" type="danger">{{ $t('restart') }}</el-button>
                        <el-switch v-model="rapidMode" inline-prompt active-text="高速模式" inactive-text="真实体验" />
                    </el-scrollbar>
                    <el-card class="results">
                        <el-scrollbar>
                            <el-table :data="results">
                                <el-table-column prop="step" :label="$t('steps')" />
                                <el-table-column prop="progress" :label="$t('progress')" />
                                <el-table-column prop="quality" :label="$t('quality')" />
                            </el-table>
                        </el-scrollbar>
                    </el-card>
                </div>
            </div>
        </el-main>
    </el-container>
</template>

<style scoped>
.el-main {
    background-color: transparent !important;
}

.main-page {
    height: 100%;
    display: flex;
    flex-direction: column;
}

.actionpanel {
    display: flex;
    flex: auto;
    overflow: hidden;
}

.action-queue {
    height: calc(48px * 1.7);
    margin: 5px 6px;
    padding: 5px 0px 0px 0px;
    border-top: 1px solid var(--el-border-color);
    border-bottom: 1px solid var(--el-border-color);
}

.action-panel {
    margin-bottom: 6px;
    max-width: 50%;
}

.results {
    flex: auto;
    margin: 20px;
}

.drop {
    margin: 10px;
}

.el-table {
    --el-table-header-bg-color: transparent;
    --el-table-tr-bg-color: transparent;
}
</style>

<fluent locale="zh-CN">
title = { $recipe } （模拟器模式）
meal-and-potion = 食物 & 药水
restart = 倒
</fluent>

<fluent locale="en-US">
title = { $recipe } (Simulator Mode)
meal-and-potion = Meal & Potions
restart = Restart
</fluent>../../libs/Craft