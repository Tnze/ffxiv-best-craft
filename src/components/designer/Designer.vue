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
    Ref,
    computed,
    inject,
    markRaw,
    provide,
    reactive,
    ref,
    watch,
} from 'vue';
import {
    ElScrollbar,
    ElAlert,
    ElTabs,
    ElTabPane,
    ElCheckboxButton,
    ElButton,
    ElButtonGroup,
    ElPopconfirm,
} from 'element-plus';
import { Bottom, Close } from '@element-plus/icons-vue';
import { useMediaQuery, useElementSize } from '@vueuse/core';

import {
    Attributes,
    Actions,
    simulate,
    Status,
    newStatus,
    compareStatus,
    Recipe,
    Jobs,
    Item,
    RecipeRequirements,
    SimulateResult,
    CollectablesShopRefine,
} from '@/libs/Craft';
import { read_solver } from '@/libs/Solver';
import { calculateEnhancedAttributsAbs, Enhancer } from '@/libs/Enhancer';
import useDesignerStore from '@/stores/designer';

import AttrEnhSelector from './tabs/AttrEnhSelector.vue';
import InitialQualitySetting from './tabs/InitialQualitySetting.vue';
import MacroExporter from './tabs/MacroExporter.vue';
import MacroImporter from './tabs/MacroImporter.vue';
import StagedActionQueueItem from './tabs/StagedActionQueueItem.vue';

import ActionPanel from './ActionPanel.vue';
import ActionQueue from './ActionQueue.vue';
import StatusBar from './StatusBar.vue';
import SolverList from './solvers/List.vue';
import { useFluent } from 'fluent-vue';
import Analyzers from './tabs/Analyzers.vue';
import { activeSeqKey, displayJobKey } from './injectionkeys';
import { Slot, Sequence, SequenceSource } from './types';

const props = defineProps<{
    recipe: Recipe;
    recipeId?: number;
    requirements: RecipeRequirements;
    collectableShopRefine?: CollectablesShopRefine;
    item: Item;
    materialQualityFactor: number;
    attributes: Attributes;
    isCustomRecipe: boolean;
}>();

const store = useDesignerStore();
const { $t } = useFluent();
const displayJob = inject(displayJobKey) as Ref<Jobs>;
const foldMultiFunctionArea = useMediaQuery('screen and (max-width: 480px)');
watch(foldMultiFunctionArea, fold => {
    if (!fold && activeTab.value == 'action-panel') {
        activeTab.value = DEFAULT_TAB;
    }
});

const actionQueueElem = ref();
const { height: actionQueueHeight } = useElementSize(actionQueueElem);

// 食物和药水效果
const attributesEnhancers = ref<Enhancer[]>([]);
const enhancedAttributes = computed<Attributes>(() =>
    calculateEnhancedAttributsAbs(
        props.attributes,
        ...attributesEnhancers.value,
    ),
);

// Attribution Alert
var attributionAlert = computed(() => {
    let { required_craftsmanship, required_control } = props.requirements;
    let { craftsmanship, control } = enhancedAttributes.value;
    let notMeet = [] as string[];
    if (required_craftsmanship > craftsmanship) {
        notMeet.push($t('craftsmanship'));
    }
    if (required_control > control) {
        notMeet.push($t('control'));
    }
    let num = notMeet.length;
    if (num > 0) {
        let attribute = notMeet[0];
        if (num > 1) {
            attribute = $t('and', { a: notMeet[0], b: notMeet[1] });
        }
        return {
            title: $t('attributes-do-not-meet-the-requirements', {
                num,
                attribute,
            }),
            descryption: $t('attributes-requirements', {
                craftsmanship: required_craftsmanship,
                control: required_control,
            }),
        };
    }
    return;
});
// UI States
const DEFAULT_TAB = "solver-list"
const isReadingSolver = ref(0);
const isReadingSolverDisplay = ref(false); // This is basicly (isReadingSolver != 0), with a 500ms delay on rising edge
const previewSolver = ref(false);
const activeTab = ref(DEFAULT_TAB);

let isReadingSolverDisplayStopTimer: NodeJS.Timeout | null = null;
watch(isReadingSolver, (irs, irsPrev) => {
    if (irs > 0) {
        if (irsPrev == 0)
            isReadingSolverDisplayStopTimer = setTimeout(() => {
                isReadingSolverDisplay.value = true;
            }, 500);
    } else if (irsPrev > 0) {
        if (isReadingSolverDisplayStopTimer)
            clearTimeout(isReadingSolverDisplayStopTimer);
        isReadingSolverDisplay.value = false;
    }
});

// Simulation Input
const initQuality = ref(0);
const initStatus = ref<Status>({
    ...(await newStatus(enhancedAttributes.value, props.recipe)),
    quality: initQuality.value,
});
watch([props, enhancedAttributes, initQuality], async ([p, ea, iq]) => {
    initStatus.value = {
        ...(await newStatus(ea, p.recipe)),
        quality: iq,
    };
});

// Active Sequence
const activeSeq = reactive<Sequence>({
    slots: [],
    maxid: 0,
});
const activeRst = ref<SimulateResult>();
simulate(initStatus.value, []).then(v => (activeRst.value = v));

provide(activeSeqKey, ref(activeSeq));
const actions = computed(() => activeSeq.slots.map(slot => slot.action));
const displayActions = computed(() => {
    return previewSolver.value && solverResult.slots.length > 0
        ? solverResult.slots.map(v => v.action)
        : activeSeq.slots.map(v => v.action);
});
function pushAction(action: Actions) {
    activeSeq.slots.push(markRaw({ id: activeSeq.maxid++, action }));
}
function loadSeq(seq: Sequence) {
    const length = activeSeq.slots.length;
    activeSeq.slots.splice(0, length, ...seq.slots);
    activeSeq.maxid = seq.maxid;
}
function userImport(actions: Actions[]) {
    const slots = actions.map((action, id) => <Slot>{ id, action });
    loadSeq(<Sequence>{ slots, maxid: slots.length });
}

// Solver result
const solverResult = reactive<Sequence>({
    slots: [],
    maxid: 0,
});
const solverResultRst = ref<SimulateResult>();
watch(initStatus, readSolver);

// Saved Sequence
watch(initStatus, async newInitStatus => {
    // re-simulate activeSeq
    activeRst.value = await simulate(newInitStatus, actions.value);
    store.sortRotations(newInitStatus);
});
watch(actions, async a => {
    activeRst.value = await simulate(initStatus.value, a);
});
function saveSequence(isManual: boolean) {
    const queue = previewSolver.value ? solverResult : activeSeq;
    store.pushRotation({
        slots: queue.slots.slice(),
        maxid: queue.maxid,
        source: isManual ? SequenceSource.Manual : SequenceSource.AutoSave,
        itemName: store.content?.item.name,
    });
    store.sortRotations(initStatus.value);
    activeTab.value = 'store';
}

const displayedStatus = computed(() => {
    return previewSolver.value &&
        !isReadingSolverDisplay.value &&
        solverResult.slots.length > 0
        ? (solverResultRst.value?.status ??
              activeRst.value?.status ??
              initStatus.value)
        : (activeRst.value?.status ?? initStatus.value);
});
watch(displayedStatus, async status => {
    if (status.progress < status.recipe.difficulty) return;
    const staged = await Promise.all(
        store.rotations.staged
            .map(v => v.seq.slots.map(v => v.action))
            .map(v => simulate(initStatus.value, v)),
    );
    if (staged.some(v => compareStatus(v.status, status) >= 0)) return;
    saveSequence(false);
});

async function readSolver() {
    try {
        const s = activeRst.value?.status;
        if (!s) return;
        isReadingSolver.value++;
        const newSolverResult = actions.value.concat(await read_solver(s));
        let display = [];
        let oldID = new Map<Actions, number[]>();
        for (const slot of solverResult.slots) {
            if (oldID.get(slot.action)?.push(slot.id) == undefined)
                oldID.set(slot.action, [slot.id]);
        }
        for (const action of newSolverResult) {
            const i = oldID.get(action)?.shift() || solverResult.maxid++;
            display.push({ id: i, action: action });
        }
        solverResult.slots = display;

        solverResultRst.value = await simulate(
            initStatus.value,
            newSolverResult,
        );
    } catch (err) {
        solverResult.slots = [];
    } finally {
        isReadingSolver.value--;
    }
}

async function handleSolverResult(
    actions: Actions[],
    solverName: SequenceSource,
) {
    let slots: Slot[] = [];
    for (const i in actions)
        slots.push({ action: actions[i], id: Number.parseInt(i) });
    const seq = {
        slots,
        maxid: actions.length,
        source: solverName,
        itemName: store.content?.item.name,
    };
    loadSeq(seq);
    store.pushRotation(seq);
    store.sortRotations(initStatus.value);
    // activeTab.value = 'store';
}
</script>

<template>
    <div class="main-page">
        <div class="crafting-alerts">
            <el-alert
                v-if="attributionAlert != undefined"
                :title="attributionAlert.title"
                :description="attributionAlert.descryption"
                type="warning"
                show-icon
                center
                :closable="false"
            />
        </div>
        <StatusBar
            class="status-bar"
            :attributes="enhancedAttributes"
            :status="displayedStatus"
            :show-condition="false"
            :collectable-shop-refine="collectableShopRefine"
        />
        <div class="above-panel">
            <el-scrollbar
                v-if="!foldMultiFunctionArea"
                class="above-left-panel"
            >
                <ActionPanel
                    @clicked-action="pushAction"
                    :job="displayJob"
                    :status="activeRst?.status"
                    #lower
                />
            </el-scrollbar>

            <div class="above-right-panel" style="overflow: hidden">
                <div ref="actionQueueElem" class="action-queue">
                    <ActionQueue
                        :job="displayJob"
                        v-model:list="activeSeq.slots"
                        :solver-result="solverResult.slots"
                        :preview-solver="previewSolver"
                        :err-list="activeRst?.errors"
                        :loading-solver-result="isReadingSolverDisplay"
                        clearable
                    />
                </div>
                <el-tabs
                    class="above-tabs"
                    v-model="activeTab"
                    tab-position="top"
                    :style="`height: calc(100% - ${actionQueueHeight + 10}px)`"
                >
                    <el-tab-pane
                        v-if="foldMultiFunctionArea"
                        :label="$t('action-panel')"
                        name="action-panel"
                        class="multi-function-area"
                    >
                        <ActionPanel
                            @clicked-action="pushAction"
                            :status="activeRst?.status"
                        />
                    </el-tab-pane>
                    <el-tab-pane
                        :label="$t('init-quality')"
                        name="init-quality"
                        class="multi-function-area"
                    >
                        <el-scrollbar style="flex: auto; padding-left: 30px">
                            <InitialQualitySetting
                                v-model="initQuality"
                                :item="item"
                                :recipe="recipe"
                                :recipe-id="recipeId"
                                :material-quality-factor="materialQualityFactor"
                            />
                        </el-scrollbar>
                    </el-tab-pane>
                    <el-tab-pane
                        :label="$t('attributes-enhance')"
                        name="attributes-enhance"
                        class="multi-function-area"
                    >
                        <el-scrollbar style="flex: auto; padding-left: 30px">
                            <AttrEnhSelector
                                v-model="attributesEnhancers"
                                :job="isCustomRecipe ? undefined : displayJob"
                                :attributs="attributes"
                            />
                        </el-scrollbar>
                    </el-tab-pane>
                    <el-tab-pane
                        :label="$t('export-macro')"
                        name="export-macro"
                        class="multi-function-area"
                    >
                        <el-scrollbar style="flex: auto">
                            <MacroExporter :actions="displayActions" />
                        </el-scrollbar>
                    </el-tab-pane>
                    <el-tab-pane
                        :label="$t('import-macro')"
                        name="import-macro"
                        class="multi-function-area"
                    >
                        <el-scrollbar style="flex: auto">
                            <MacroImporter @on-recognized="userImport" />
                        </el-scrollbar>
                    </el-tab-pane>
                    <el-tab-pane
                        :label="$t('solvers')"
                        name="solver-list"
                        class="multi-function-area"
                    >
                        <el-scrollbar style="flex: auto">
                            <SolverList
                                :init-status="initStatus"
                                :recipe-name="item.name"
                                :can-hq="item.can_be_hq"
                                @solver-load="readSolver()"
                                @solver-result="handleSolverResult"
                                :collectable-shop-refine="collectableShopRefine"
                            />
                        </el-scrollbar>
                    </el-tab-pane>
                    <el-tab-pane
                        :label="$t('store')"
                        name="store"
                        class="multi-function-area"
                    >
                        <el-scrollbar class="savedqueue-list">
                            <el-button-group>
                                <el-button
                                    @click="saveSequence(true)"
                                    :icon="Bottom"
                                >
                                    {{ $t('save-workspace') }}
                                </el-button>
                                <el-popconfirm
                                    :title="$t('confirm-clear-store')"
                                    @confirm="store.clearRotations()"
                                    :confirm-button-text="$t('confirm')"
                                    :cancel-button-text="$t('cancel')"
                                >
                                    <template #reference>
                                        <el-button :icon="Close">
                                            {{ $t('clear-store') }}
                                        </el-button>
                                    </template>
                                </el-popconfirm>
                                <el-checkbox-button
                                    v-model:model-value="previewSolver"
                                    v-if="solverResult.slots.length > 0"
                                >
                                    {{ $t('apply-solver') }}
                                </el-checkbox-button>
                            </el-button-group>
                            <TransitionGroup name="savedqueues" tag="div">
                                <StagedActionQueueItem
                                    v-for="({ key, seq }, i) in store.rotations
                                        .staged"
                                    :key="key"
                                    :seq="seq"
                                    :status="initStatus"
                                    :display-job="displayJob"
                                    @load="loadSeq(seq)"
                                    @delete="store.deleteRotation(i)"
                                />
                            </TransitionGroup>
                        </el-scrollbar>
                    </el-tab-pane>
                    <el-tab-pane
                        :label="$t('analyzer')"
                        name="analyzer"
                        class="multi-function-area"
                    >
                        <el-scrollbar style="flex: auto">
                            <Analyzers
                                :init-status="initStatus"
                                :actions="displayActions"
                                :collectable-shop-refine="collectableShopRefine"
                            />
                        </el-scrollbar>
                    </el-tab-pane>
                </el-tabs>
            </div>
        </div>
    </div>
</template>

<style scoped>
.el-card {
    --el-card-padding: 4px 5px;
    --el-card-border-radius: 15px;
    margin: 15px;
}

.above-tabs {
    /* height: 100%; */
    user-select: none;
}

.main-page {
    height: 100%;
    display: flex;
    flex-direction: column;
}

.crafting-alerts {
    margin: 5px 15px;
}

.status-bar {
    flex: none;
}

.above-panel {
    display: flex;
    flex: auto;
    overflow: hidden;
    flex-direction: row;
}

.above-panel :deep(.el-tabs__content) {
    display: flex;
    flex: auto;
}

.above-left-panel {
    flex: 0 0 270px;
    min-width: 140px;
    height: auto;
}

.above-right-panel {
    display: flex;
    flex-direction: column;
    flex: 1 1 auto;
    margin-left: 5px;
    height: 100%;
}

@media screen and (max-width: 480px) {
    .above-panel {
        flex-direction: column;
    }

    .above-left-panel {
        order: 1;
        flex: 1 1 auto;
    }

    .above-right-panel {
        flex: 0 0 auto;
        height: inherit;
    }
}

.multi-function-area {
    height: 100%;
    width: 100%;
    overflow: hidden;
}

.action-queue {
    border-left: 5px solid var(--el-border-color);
    margin-bottom: 5px;
}

.savedqueue-list {
    margin: 0;
    flex: auto;
}

.savedqueues-move,
.savedqueues-enter-active,
.savedqueues-leave-active {
    transition: all 0.5s ease;
}

.savedqueues-enter-from,
.savedqueues-leave-to {
    opacity: 0;
    transform: translateX(30px);
}

.savedqueues-leave-active {
    position: absolute;
}
</style>

<fluent locale="zh-CN">
solvers = 求解
export-macro = 导出
import-macro = 导入
attributes-enhance = 食药&装备
init-quality = 初期品质
store = 储存
analyzer = 分析
action-panel = 技能面板

save-workspace = 储存
clear-store = 清空
confirm-clear-store = 确定要清空保存区域吗？
apply-solver = 应用求解结果
confirm = 确认
cancel = 取消

waring = 警告

macro-file-type-name = BestCraft宏文件
save-file = 保存文件
save-success = 保存成功
save-fail = 保存失败：{ $reason }
open-file = 打开文件
read-n-macros = 读取了 { $n } 个宏
read-fail = 读取失败：{ $reason }

edit = 编辑
delete = 删除

and = { $a }和{ $b }
attributes-do-not-meet-the-requirements = 装备{ $attribute }不满足配方要求
attributes-requirements = 制作该配方要求：作业精度 ≥ { $craftsmanship } 且 加工精度 ≥ { $control }
</fluent>

<fluent locale="en-US">
solvers = Solvers
export-macro = Export
import-macro = Import
attributes-enhance = Medicines & Meals
init-quality = Quality
store = Store
analyzer = Analyzer
action-panel = Action Panel

save-workspace = Save
clear-store = Clear
confirm-clear-store = Are you sure to clear the storeage?
apply-solver = Apply solver result
confirm = Confirm
cancel = Cancel

waring = Warning

macro-file-type-name = BestCraft saved macros file
save-file = Save file
save-success = Saving successed
save-fail = Saving failed: { $reason }
open-file = Open file
read-n-macros = Read { $n -> 
    [one] one macro
    *[other] { $n } macros
}
read-fail = Reading failed: { $reason }

edit = Edit
delete = Delete

and = { $a } and { $b }
attributes-do-not-meet-the-requirements = 
    { $attribute }
    { $num ->
        [one] does
        *[other] do
    }
    not meet the requirements.
attributes-requirements = Require: craftsmanship ≥ { $craftsmanship } and control ≥ { $control }
</fluent>

<fluent locale="ja-JP">
attributes-enhance = 薬品・調理品
init-quality = 初期品質
and = { $a }と{ $b }
attributes-do-not-meet-the-requirements = { $attribute }が足りないため
attributes-requirements = 製作可能条件：{ craftsmanship }{ $craftsmanship}以上 と { control }{ $control }以上
</fluent>
