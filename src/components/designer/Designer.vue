<script setup lang="ts">
import { save, open } from '@tauri-apps/api/dialog'
import { writeFile, readTextFile } from '@tauri-apps/api/fs'
import { computed, reactive, ref, watch } from "vue";
import { ElContainer, ElDrawer, ElDialog, ElHeader, ElMain, ElScrollbar, ElLink, ElMessage, ElMessageBox } from "element-plus";
import { Delete, Edit } from "@element-plus/icons-vue";
import { Attributes, Actions, simulate, Status, newStatus, compareStatus, Recipe, Jobs, Item } from "../../Craft";
import { read_solver } from "../../Solver";
import ActionPanel from "./ActionPanel.vue";
import ActionQueue from "./ActionQueue.vue";
import StatusBar from "./StatusBar.vue";
import Sidebar from "./Sidebar.vue";
import SolverList from "./SolverList.vue";
import MacroExporter from "./MacroExporter.vue";
import QueueStatus from "./QueueStatus.vue";
import InitialQualitySetting from './InitialQualitySetting.vue'
import AttrEnhSelector from "../attr-enhancer/AttrEnhSelector.vue";
import { Enhancer } from "../attr-enhancer/Enhancer";
import { useFluent } from 'fluent-vue';

interface Slot {
    id: number;
    action: Actions;
}

// 用于表示一个技能的序列，或者说一个宏
// 为了实现拖拽和删除等动画效果，我们需要给每个技能一个唯一的id
// maxid储存了当前序列中最大的标志，并用于生成下一个id
interface Sequence {
    slots: Slot[];
    maxid: number;
    status: Status;
    errors: { pos: number; err: string }[];
}

const props = defineProps<{
    recipe: Recipe,
    item: Item,
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
    return {
        level,
        craftsmanship,
        control,
        craft_points,
    };
});

// UI State
const isReadingSolver = ref(0);
const previewSolver = ref(false);
const openInitQualitySet = ref(false);
const openSolverDrawer = ref(false);
const openExportMacro = ref(false);
const openAttrEnhSelector = ref(false);

// Simulation Input
const initQuality = ref(0)
const initStatus = ref<Status>({ ...await newStatus(enhancedAttributes.value, props.recipe), quality: initQuality.value });
watch([props, enhancedAttributes, initQuality], async ([p, ea, iq]) => { initStatus.value = { ...await newStatus(ea, p.recipe), quality: iq } });

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
}

const displayedStatus = computed(() => {
    return previewSolver.value && solverResult.slots.length > 0
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
        for (const skill of newSolverResult) {
            const i = oldID.get(skill)?.shift() || solverResult.maxid++;
            display.push({ id: i, action: skill });
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
        <el-drawer v-model="openSolverDrawer" :title="$t('solvers')" size="45%">
            <SolverList :init-status="initStatus" :recipe-name="item.name" @solver-load="readSolver(activeSeq.status)"
                @solver-result="handleSolverResult" />
        </el-drawer>
        <el-drawer v-model="openExportMacro" :title="$t('export-macro')" direction="btt" size="80%">
            <MacroExporter :actions="displayActions" />
        </el-drawer>
        <el-dialog v-model="openAttrEnhSelector" :title="$t('meal-and-potion')">
            <AttrEnhSelector v-model="attributesEnhancers" />
        </el-dialog>
        <KeepAlive>
            <InitialQualitySetting v-model="initQuality" v-model:open="openInitQualitySet" :item="item"
                :recipe="recipe" />
        </KeepAlive>
        <el-header>
            <h1>{{ item.name }}</h1>
        </el-header>
        <el-main>
            <div class="main-page">
                <StatusBar class="status-bar" :attributes="attributes" :enhancers="attributesEnhancers" :status="
                    displayedStatus
                " @click-attributes="openAttrEnhSelector = true" @click-quality="openInitQualitySet = true" />
                <div class="actionpanel-and-savedqueue">
                    <el-scrollbar class="action-panel">
                        <ActionPanel @clicked-action="pushAction" :job="displayJob" :status="activeSeq.status" #lower />
                    </el-scrollbar>
                    <div class="actionqueue-and-savedqueue">
                        <div class="action-queue">
                            <ActionQueue :job="displayJob" :list="activeSeq.slots" :solver-result="solverResult.slots"
                                :preview-solver="previewSolver" :err-list="activeSeq.errors" />
                        </div>
                        <Sidebar class="savedqueue-list-sidebar" v-model:previewSolver="previewSolver"
                            @plus="saveSequence" @delete="clearSeq" @solver="openSolverDrawer = true"
                            @print="openExportMacro = true" @save-list="saveListToJSON" @open-list="openListFromJSON" />
                        <el-scrollbar class="savedqueue-scrollbar">
                            <TransitionGroup class="savedqueue-list" name="savedqueues" tag="ul">
                                <li v-for="({ key, seq }, i) in savedSeqs.ary" :key="key" class="savedqueue-item">
                                    <QueueStatus :status="seq.status" />
                                    <ActionQueue :job="displayJob" :list="seq.slots" :err-list="seq.errors" disabled />
                                    <el-link :icon="Edit" :underline="false" class="savedqueue-item-button"
                                        @click="loadSeq(seq)" />
                                    <el-link :icon="Delete" :underline="false" class="savedqueue-item-button"
                                        @click="savedSeqs.ary.splice(i, 1)" />
                                </li>
                            </TransitionGroup>
                        </el-scrollbar>
                    </div>
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

.status-bar {
    flex: none;
}

.action-queue {
    border-top: 1px solid var(--el-border-color);
    border-bottom: 1px solid var(--el-border-color);
}

.actionpanel-and-savedqueue {
    border-bottom: 1px solid var(--el-border-color);
    display: flex;
    flex: auto;
    overflow: hidden;
}

.savedqueue-list-sidebar {
    flex: auto;
}

.actionqueue-and-savedqueue {
    display: flex;
    flex-direction: column;
    flex: auto;
}

.savedqueue-scrollbar {
    flex: auto;
}

.action-panel {
    border-top: 1px solid var(--el-border-color);
    border-right: 1px solid var(--el-border-color);
    margin-bottom: 6px;
    max-width: 25%;
}

.savedqueue-list {
    margin: 0px;
    padding: 0px;
}

.savedqueue-item {
    display: flex;
    align-items: center;
    border-bottom: 1px solid var(--el-border-color);
    min-height: 50px;
}

.savedqueue-item-button {
    margin-right: 6px;
}

.savedqueues-enter-active,
.list-leave-active {
    transition: all 0.5s ease;
}

.savedqueues-enter-from,
.savedqueues-leave-to {
    opacity: 0;
    transform: translateX(30px);
}
</style>

<fluent locale="zh-CN">
solvers = 求解器
export-macro = 导出宏
meal-and-potion = 食物 & 药水

please-input-init-quality = 请输入初期品质
config-init-quality = 设置初期品质
please-input-integers = 请输入整数

number-of-macros-is-zero = 当前要保存的宏数量为0，是否继续？
waring = 警告

macro-file-type-name = BestCraft宏文件
save-file = 保存文件
save-success = 保存成功
save-fail = 保存失败：{ $reason }
open-file = 打开文件
read-n-macros = 读取了 { $n } 个宏
read-fail = 读取失败：{ $reason }
</fluent>

<fluent locale="en-US">
solvers = Solvers
export-macro = Export
meal-and-potion = Meal & Potions

please-input-init-quality = Please input initial quality
config-init-quality = Set initial quality
please-input-integers = Please input a integer

number-of-macros-is-zero = Number of macros is 0, continue?
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
</fluent>