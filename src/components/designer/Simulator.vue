<script setup lang="ts">
import { ElContainer, ElHeader, ElMain, ElScrollbar, ElDialog, ElButton, ElTable, ElTableColumn } from 'element-plus';
import { computed, ref } from 'vue';
import { Recipe, Item, Attributes, Jobs, newStatus, Status, Actions, Conditions, simulateOneStep } from '../../Craft';
import { Enhancer } from "../attr-enhancer/Enhancer";
import StatusBarVue from './StatusBar.vue';
import ActionPanelVue from './ActionPanel.vue';
import ActionQueueVue from './ActionQueue.vue';
import AttrEnhSelector from '../attr-enhancer/AttrEnhSelector.vue';

const props = defineProps<{
    recipe: Recipe,
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
const initStatus = ref<Status>({ ...await newStatus(enhancedAttributes.value, props.recipe), quality: 0 });
const currentStatus = ref<Status>(initStatus.value)
const seq = ref<Slot[]>([])
const openAttrEnhSelector = ref(false)
const results = ref<Status[]>([])
const waiting = ref(false)

const sleep = (t: number) => new Promise((resolve) => setTimeout(resolve, t))
async function pushAction(action: Actions) {
    if (waiting.value) return;
    seq.value.push({
        id: seq.value.length, action,
        condition: currentStatus.value.condition,
    })
    try {
        waiting.value = true
        const wait = sleep(1000)
        const newState = await simulateOneStep(currentStatus.value, action);
        await wait;
        currentStatus.value = newState
        if (newState.progress >= newState.recipe.difficulty || newState.durability <= 0) {
            await sleep(2000);
            restart();
        }
    } catch (e: unknown) {
        console.error(e)
    } finally {
        waiting.value = false
    }
}

function restart() {
    results.value.push(currentStatus.value)
    seq.value.splice(0);
    currentStatus.value = initStatus.value
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
                    :status="currentStatus" :disabled-init-quality="true"
                    @click-attributes="openAttrEnhSelector = true" />
                <el-scrollbar class="action-queue">
                    <ActionQueueVue :job="displayJob" :list="seq" disabled no-hover />
                </el-scrollbar>
                <div class="actionpanel">
                    <el-scrollbar class="action-panel">
                        <ActionPanelVue @clicked-action="pushAction" :disable="waiting" :job="displayJob"
                            :status="currentStatus" simulator-mode #lower />
                        <el-button class="drop" @click="restart" type="danger">{{ $t('restart') }}</el-button>
                    </el-scrollbar>
                    <el-scrollbar class="results">
                        <el-table :data="results">
                            <el-table-column prop="step" :label="$t('steps')" />
                            <el-table-column prop="progress" :label="$t('progress')" />
                            <el-table-column prop="quality" :label="$t('quality')" />
                        </el-table>
                    </el-scrollbar>
                </div>
            </div>
        </el-main>
    </el-container>
</template>

<style scoped>
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
    height: calc(48px * 1);
    margin: 5px 6px;
    padding: 5px 0px 0px 0px;
    border-top: 1px solid var(--el-border-color);
    border-bottom: 1px solid var(--el-border-color);
}

.action-panel {
    margin-bottom: 6px;
}

.results {
    flex: auto;
}

.drop {
    margin: 10px;
}

.el-table {
    --el-table-header-bg-color: transparent;
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
</fluent>