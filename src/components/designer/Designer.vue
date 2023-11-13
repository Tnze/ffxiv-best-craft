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
import { save, open } from '@tauri-apps/api/dialog'
import { writeFile, readTextFile } from '@tauri-apps/api/fs'
import { computed, reactive, ref, watch } from "vue";
import { ElContainer, ElHeader, ElMain, ElScrollbar, ElLink, ElMessage, ElMessageBox, ElAlert, ElTabs, ElTabPane, ElCheckboxButton, ElButton, ElButtonGroup } from "element-plus";
import { Bottom, Close } from "@element-plus/icons-vue";
import { Attributes, Actions, simulate, Status, newStatus, compareStatus, Recipe, Jobs, Item, RecipeLevel, RecipeRequirements } from "@/libs/Craft";
import { read_solver } from "@/libs/Solver";
import ActionPanel from "./ActionPanel.vue";
import ActionQueue from "./ActionQueue.vue";
import StatusBar from "./StatusBar.vue";
import SolverList from "./SolverList.vue";
import MacroExporter from "./MacroExporter.vue";
import QueueStatus from "./QueueStatus.vue";
import InitialQualitySetting from './InitialQualitySetting.vue'
import AttrEnhSelector from "../attr-enhancer/AttrEnhSelector.vue";
import { Enhancer } from "../attr-enhancer/Enhancer";
import { useFluent } from 'fluent-vue';
import StagedActionQueueItem from './StagedActionQueueItem.vue';

interface Slot {
    id: number;
    action: Actions;
}

// 用于表示一个技能的序列，或者说一个宏
// 为了实现拖拽和删除等动画效果，我们需要给每个技能一个唯一的id
// maxid储存了当前序列中最大的标志，并用于生成下一个id
export interface Sequence {
    slots: Slot[];
    maxid: number;
    status: Status;
    errors: { pos: number; err: string }[];
}

const props = defineProps<{
    recipe: Recipe,
    recipeId?: number,
    recipeLevel: RecipeLevel,
    requirements: RecipeRequirements,
    item: Item,
    materialQualityFactor: number,
    attributes: Attributes,
    displayJob: Jobs,
}>()
const { $t } = useFluent()

// 食物和药水效果
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
    return { level, craftsmanship, control, craft_points };
});

// Attribution Alert
var attributionAlert = computed(() => {
    let { required_craftsmanship, required_control } = props.requirements;
    let { craftsmanship, control } = enhancedAttributes.value;
    let notMeet = [] as string[]
    if (required_craftsmanship > craftsmanship) {
        notMeet.push($t('craftsmanship'))
    }
    if (required_control > control) {
        notMeet.push($t('control'))
    }
    let num = notMeet.length
    if (num > 0) {
        let attribute = notMeet[0]
        if (num > 1) {
            attribute = $t('and', { a: notMeet[0], b: notMeet[1] })
        }
        return {
            title: $t('attributes-do-not-meet-the-requirements', { num, attribute }),
            descryption: $t('attributes-requirements', {
                craftsmanship: required_craftsmanship,
                control: required_control
            })
        }
    }
    return;
})
// UI State
const isReadingSolver = ref(0);
const isReadingSolverDisplay = ref(false); // This is basicly (isReadingSolver != 0), with a 500ms delay on rising edge
const previewSolver = ref(false);
const openInitQualitySet = ref(false);
const openAttrEnhSelector = ref(false);
const activeTab = ref('staged')

let isReadingSolverDisplayStopTimer: NodeJS.Timeout | null = null;
watch(isReadingSolver, (irs, irsPrev) => {
    if (irs > 0) {
        if (irsPrev == 0) isReadingSolverDisplayStopTimer = setTimeout(
            () => { isReadingSolverDisplay.value = true },
            500,
        )
    } else if (irsPrev > 0) {
        if (isReadingSolverDisplayStopTimer)
            clearTimeout(isReadingSolverDisplayStopTimer)
        isReadingSolverDisplay.value = false
    }
})

// Simulation Input
const initQuality = ref(0)
const initStatus = ref<Status>({ ...await newStatus(enhancedAttributes.value, props.recipe, props.recipeLevel), quality: initQuality.value });
watch([props, enhancedAttributes, initQuality], async ([p, ea, iq]) => { initStatus.value = { ...await newStatus(ea, p.recipe, p.recipeLevel), quality: iq } });

// Active Sequence
const activeSeq = reactive<Sequence>({
    slots: [],
    maxid: 0,
    status: initStatus.value,
    errors: [],
});
const actions = computed(() => activeSeq.slots.map(slot => slot.action));
const displayActions = computed(() => {
    return previewSolver.value && solverResult.slots.length > 0
        ? solverResult.slots.map(v => v.action)
        : activeSeq.slots.map(v => v.action)
})
function pushAction(action: Actions) {
    activeSeq.slots.push({ id: activeSeq.maxid++, action });
}
function loadSeq(seq: Sequence) {
    activeSeq.slots = seq.slots.slice();
    activeSeq.maxid = seq.maxid;
}
function clearSeq() {
    activeSeq.slots = [];
    activeSeq.maxid = 0;
}

// Solver result
const solverResult = reactive<Sequence>({
    slots: [],
    maxid: 0,
    status: initStatus.value,
    errors: [],
});
watch(() => activeSeq.status, readSolver);

// Saved Sequence
const savedSeqs = reactive<{ maxid: number, ary: { key: number, seq: Sequence }[] }>({ maxid: 0, ary: [] });
watch(initStatus, async (newInitStatus) => {
    // re-simulate all savedQueues
    const results = await Promise.all(
        savedSeqs.ary.map(({ seq }) => simulate(
            newInitStatus,
            seq.slots.map((x) => x.action)
        ))
    )
    savedSeqs.ary.forEach(({ seq }, i) => {
        seq.status = results[i].status;
        seq.errors = results[i].errors;
    })
    savedSeqs.ary.sort((a, b) => {
        const ord = compareStatus(b.seq.status, a.seq.status)
        return ord != 0 ? ord : a.key - b.key
    });

    // re-simulate activeSeq
    let result = await simulate(newInitStatus, actions.value);
    activeSeq.status = result.status;
    activeSeq.errors = result.errors;
});
watch(actions, async (a) => {
    let result = await simulate(initStatus.value, a);
    activeSeq.status = result.status;
    activeSeq.errors = result.errors;
});
function saveSequence() {
    const queue = previewSolver.value
        ? solverResult
        : activeSeq
    pushSequence({
        slots: queue.slots.slice(),
        maxid: queue.maxid,
        status: queue.status,
        errors: queue.errors,
    });
}
function pushSequence(seq: Sequence) {
    const key = savedSeqs.maxid++;
    savedSeqs.ary.push({ key, seq });
    savedSeqs.ary.sort((a, b) => {
        const ord = compareStatus(b.seq.status, a.seq.status)
        return ord != 0 ? ord : a.key - b.key
    });
    activeTab.value = 'staged'
}

const displayedStatus = computed(() => {
    return previewSolver.value && !isReadingSolverDisplay.value && solverResult.slots.length > 0
        ? solverResult.status
        : activeSeq.status
})
watch(displayedStatus, (status) => {
    if (status.progress < status.recipe.difficulty)
        return;
    if (savedSeqs.ary.some(v => compareStatus(v.seq.status, status) >= 0))
        return;
    saveSequence()
})

async function readSolver(s: Status) {
    try {
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

        const result = await simulate(initStatus.value, newSolverResult);
        solverResult.status = result.status;
        solverResult.errors = result.errors;
    } catch (err) {
        solverResult.slots = [];
    } finally {
        isReadingSolver.value--;
    }
}

async function handleSolverResult(actions: Actions[]) {
    let slots: Slot[] = [];
    for (const i in actions)
        slots.push({ action: actions[i], id: Number.parseInt(i) })
    const { status, errors } = await simulate(initStatus.value, actions)
    pushSequence({
        slots,
        maxid: actions.length,
        status,
        errors,
    });
}

async function saveListToJSON() {
    try {
        const queues = savedSeqs.ary.map(v => v.seq.slots.map(s => s.action)).filter(v => v.length > 0)
        try {
            if (queues.length == 0) {
                await ElMessageBox.confirm(
                    $t('number-of-macros-is-zero'),
                    $t('waring'),
                    { type: 'warning' }
                )
            }
        } catch {
            return
        }
        const { level, craftsmanship, control, craft_points } = enhancedAttributes.value
        const path = await save({
            defaultPath: `${props.item.name}-${level}-${craftsmanship}-${control}-${craft_points}`,
            filters: [{ name: $t('macro-file-type-name'), extensions: ['json'] }],
            title: $t('save-file')
        })
        if (!path) {
            return
        }
        await writeFile({ path, contents: JSON.stringify(queues) })
        ElMessage({
            type: "success",
            showClose: true,
            message: $t('save-success'),
        });
    } catch (err) {
        ElMessage({
            type: "error",
            showClose: true,
            message: $t('save-fail', { reason: err as string }),
        });
    }
}

async function openListFromJSON() {
    const pathlist = <string[]>await open({
        filters: [{ name: $t('macro-file-type-name'), extensions: ['json'] }],
        multiple: true,
        title: $t('open-file')
    })
    if (!pathlist)
        return
    for (const filepath of pathlist) {
        try {
            const content = await readTextFile(filepath)
            const sequences = <Actions[][]>JSON.parse(content)
            for (const actions of sequences) {
                const slots = actions.map((action, index) => { return { id: index, action } })
                const { status, errors } = await simulate(initStatus.value, actions)
                pushSequence({
                    slots,
                    maxid: slots.length - 1,
                    status,
                    errors,
                });
            }
            ElMessage({
                type: "success",
                showClose: true,
                message: $t('read-n-macros', { n: sequences.length }),
            });
        } catch (err) {
            ElMessage({
                type: "error",
                showClose: true,
                message: $t('read-fail', { reason: err as string }),
            });
        }
    }
}
</script>

<template>
    <el-container>
        <el-header>
            <h1>{{ item.name }}</h1>
        </el-header>
        <el-main>
            <div class="main-page">
                <div class="crafting-alerts">
                    <el-alert v-if="attributionAlert != undefined" :title="attributionAlert.title"
                        :description="attributionAlert.descryption" type="warning" show-icon center :closable="false" />
                </div>
                <StatusBar class="status-bar" :attributes="attributes" :enhancers="attributesEnhancers"
                    :status="displayedStatus" :disabled-init-quality="recipeId == undefined"
                    @click-attributes="openAttrEnhSelector = true" @click-quality="openInitQualitySet = true"
                    :show-condition="false" />
                <el-tabs v-model="activeTab" tab-position="left" style="height: auto;" class="above-panel">
                    <el-tab-pane :label="$t('action-editor')" name="staged" class="staged-panel">
                        <el-scrollbar class="staged-left-panel">
                            <ActionPanel @clicked-action="pushAction" :job="displayJob" :status="activeSeq.status" #lower />
                        </el-scrollbar>
                        <div class="staged-right-panel">
                            <div class="action-queue">
                                <ActionQueue :job="displayJob" v-model:list="activeSeq.slots" :solver-result="solverResult.slots"
                                    :preview-solver="previewSolver" :err-list="activeSeq.errors"
                                    :loading-solver-result="isReadingSolverDisplay" />
                            </div>
                            <el-button-group>
                                <el-button @click="saveSequence" :icon="Bottom">
                                    {{ $t('save-workspace') }}
                                </el-button>
                                <el-button @click="clearSeq" :icon="Close">
                                    {{ $t('clear-workspace') }}
                                </el-button>
                                <el-checkbox-button v-model:model-value="previewSolver"
                                    v-if="solverResult.slots.length > 0">
                                    {{ $t('apply-solver') }}
                                </el-checkbox-button>
                            </el-button-group>
                            <el-scrollbar class="savedqueue-list">
                                <TransitionGroup name="savedqueues" tag="div">
                                    <StagedActionQueueItem v-for="({ key, seq }, i) in savedSeqs.ary" :key="key" :seq="seq"
                                        :display-job="displayJob" @load="loadSeq(seq)"
                                        @delete="savedSeqs.ary.splice(i, 1)" />
                                </TransitionGroup>
                            </el-scrollbar>
                        </div>
                    </el-tab-pane>
                    <el-tab-pane :label="$t('init-quality')" name="init-quality">
                        <InitialQualitySetting v-if="recipeId != undefined" v-model="initQuality"
                            v-model:open="openInitQualitySet" :item="item" :recipe="recipe" :recipe-id="recipeId"
                            :material-quality-factor="materialQualityFactor" />
                    </el-tab-pane>
                    <el-tab-pane :label="$t('attributes-enhance')" name="attributes-enhance">
                        <AttrEnhSelector v-model="attributesEnhancers" />
                    </el-tab-pane>
                    <el-tab-pane :label="$t('export-macro')" name="export-macro">
                        <MacroExporter :actions="displayActions" />
                    </el-tab-pane>
                    <el-tab-pane :label="$t('solvers')" name="solver-list">
                        <SolverList :init-status="initStatus" :recipe-name="item.name" :can-hq="item.can_be_hq"
                            @solver-load="readSolver(activeSeq.status)" @solver-result="handleSolverResult" />
                    </el-tab-pane>
                </el-tabs>
            </div>
        </el-main>
    </el-container>
</template>

<style scoped>
.el-main {
    background-color: transparent !important;
}

.el-card {
    --el-card-padding: 4px 5px;
    --el-card-border-radius: 15px;
    margin: 15px;
}

.el-tabs {
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
    border-top: 1px solid var(--el-border-color);
}

.above-panel :deep(.el-tabs__content) {
    flex: auto;
    display: flex;
}

.staged-panel {
    display: flex;
    flex-direction: row;
    flex: auto;
}

.staged-left-panel {
    width: 233px;
    height: auto;
}

.staged-right-panel {
    display: flex;
    flex-direction: column;
    flex: 1;
    height: 100%;
    margin-left: 5px;
}

.action-queue {
    border-left: 5px solid var(--el-border-color);
    margin-bottom: 5px;
}

.savedqueue-list {
    margin: 5px 0;
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
solvers = 求解器
export-macro = 导出宏
attributes-enhance = 食药&装备
init-quality = 初期品质
action-editor = 编排技能

save-workspace = 暂存
clear-workspace = 清空
apply-solver = 应用求解结果

number-of-macros-is-zero = 当前要保存的宏数量为0，是否继续？
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
attributes-enhance = Attributes Enhance
staged = Staged

save-workspace = Save
clear-workspace = Clear
apply-solver = Apply solver result

number-of-macros-is-zero = Number of macros is 0. Continue?
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
and = { $a }と{ $b }
attributes-do-not-meet-the-requirements = { $attribute }が足りないため
attributes-requirements = 製作可能条件：{ craftsmanship }{ $craftsmanship}以上 と { control }{ $control }以上
</fluent>