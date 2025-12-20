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
import { ElScrollbar, ElAlert, ElTabs, ElTabPane } from 'element-plus';
import { useMediaQuery, useElementSize } from '@vueuse/core';

import {
    Attributes,
    Actions,
    simulate,
    Status,
    newStatus,
    Recipe,
    Jobs,
    Item,
    RecipeRequirements,
    SimulateResult,
    CollectablesShopRefine,
} from '@/libs/Craft';
import { read_solver } from '@/libs/Solver';
import { calculateEnhancedAttributsAbs, Enhancer } from '@/libs/Enhancer';
import { GearsetsRow } from '@/libs/Gearsets';
import useDesignerStore from '@/stores/designer';
import useGearsetsStore from '@/stores/gearsets';

import AttrEnhSelector from './tabs/AttrEnhSelector.vue';
import InitialQualitySetting from './tabs/InitialQualitySetting.vue';
import MacroExporter from './tabs/MacroExporter.vue';
import MacroImporter from './tabs/MacroImporter.vue';

import ActionPanel from './ActionPanel.vue';
import ActionQueue from './ActionQueue.vue';
import StatusBar from './StatusBar.vue';
import SolverList from './solvers/List.vue';
import { useFluent } from 'fluent-vue';
import Analyzers from './tabs/Analyzers.vue';
import { activeSeqKey, displayJobKey } from './injectionkeys';
import { Slot, Sequence, SequenceSource } from './types';
import MarcoInfo from './MarcoInfo.vue';

const props = defineProps<{
    recipe: Recipe;
    recipeId?: number;
    requirements: RecipeRequirements;
    collectableShopRefine?: CollectablesShopRefine;
    item: Item;
    materialQualityFactor: number;
    isCustomRecipe: boolean;
}>();

const store = useDesignerStore();
const gearsetsStore = useGearsetsStore();
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

// 装备属性
const gearsetId = ref(0);
const selectedGearsetRow = computed<GearsetsRow | undefined>(() => {
    return gearsetsStore.gearsets.find(v => v.id == gearsetId.value);
});
const attributes = computed<Attributes>(
    () => (selectedGearsetRow.value ?? gearsetsStore.default).value,
);
// Gearsets changed
gearsetsStore.$subscribe((_, state) => {
    selectDefaultGearset();
});
// Recipe changed
watch(
    [displayJob, () => props.isCustomRecipe],
    ([displayJob, isCustomRecipe], old) => {
        const currentGearset = selectedGearsetRow.value;
        if (
            currentGearset == undefined ||
            currentGearset.id == 0 ||
            (!isCustomRecipe &&
                !currentGearset.compatibleJobs.includes(displayJob))
        ) {
            // Current selected gearset doesn't fit current recipe
            selectDefaultGearset();
        }
    },
    { immediate: true },
);

function selectDefaultGearset() {
    if (props.isCustomRecipe) {
        gearsetId.value = 0;
        return;
    }
    const job = displayJob.value;
    const newGearset = gearsetsStore.gearsets.find(
        (v, i) => i != 0 && v.compatibleJobs.includes(job),
    );
    gearsetId.value = newGearset?.id ?? 0;
}

// 食物和药水效果
const attributesEnhancers = ref<Enhancer[]>([]);
const enhancedAttributes = computed<Attributes>(() =>
    calculateEnhancedAttributsAbs(
        attributes.value,
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
const DEFAULT_TAB = 'solver-list';
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
});
watch(actions, async a => {
    activeRst.value = await simulate(initStatus.value, a);
});

const displayedStatus = computed(() => {
    return previewSolver.value &&
        !isReadingSolverDisplay.value &&
        solverResult.slots.length > 0
        ? (solverResultRst.value?.status ??
              activeRst.value?.status ??
              initStatus.value)
        : (activeRst.value?.status ?? initStatus.value);
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
    additional: boolean,
) {
    if (additional) {
        for (const i in actions) pushAction(actions[i]);
    } else {
        const seq = {
            slots: actions.map((action, id) => ({ action, id })),
            maxid: actions.length,
            source: solverName,
            itemName: store.content?.item.name,
        };
        loadSeq(seq);
    }
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
                    <MarcoInfo
                        style="margin-left: 9px"
                        :seq="activeSeq"
                        :status="displayedStatus"
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
                                v-model:gearset-id="gearsetId"
                                :job="isCustomRecipe ? undefined : displayJob"
                                :attributes="attributes"
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
                                :current-status="displayedStatus"
                                :recipe-name="item.name"
                                :can-hq="item.can_be_hq"
                                @solver-load="readSolver()"
                                @solver-result="handleSolverResult"
                                :collectable-shop-refine="collectableShopRefine"
                            />
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

<fluent locale="zh-TW">
solvers = 求解
export-macro = 匯出
import-macro = 匯入
attributes-enhance = 食藥&裝備
init-quality = 初期品質
store = 儲存
analyzer = 分析
action-panel = 技能面板

waring = 警告

macro-file-type-name = BestCraft巨集檔案
save-file = 儲存檔案
save-success = 儲存成功
save-fail = 儲存失敗：{ $reason }
open-file = 開啟檔案
read-n-macros = 讀取了 { $n } 個巨集
read-fail = 讀取失敗：{ $reason }

edit = 編輯
delete = 刪除

and = { $a }和{ $b }
attributes-do-not-meet-the-requirements = 裝備{ $attribute }不滿足配方要求
attributes-requirements = 製作該配方要求：作業精度 ≥ { $craftsmanship } 且 加工精度 ≥ { $control }
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
