<script setup lang="ts">
import { save, open } from '@tauri-apps/api/dialog'
import { writeFile, readTextFile } from '@tauri-apps/api/fs'
import { computed, reactive, ref, watch } from "vue";
import { ElMessage, ElMessageBox } from "element-plus";
import { Delete, Edit } from "@element-plus/icons-vue";
import {
    Attributes,
    Actions,
    simulate,
    Status,
    newStatus,
    isBetterThan,
    Recipe,
    Jobs,
    Item,
} from "../../Craft";
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

// Simulation
const initQuality = ref(0)
const initStatus = ref<Status>(
    await newStatus(enhancedAttributes.value, props.recipe, initQuality.value)
);
watch([props, enhancedAttributes, initQuality], async ([p, ea, iq]) => {
    initStatus.value = await newStatus(ea, p.recipe, iq);
});
// Actions Queue
const actionQueue = reactive<Sequence>({
    slots: [],
    maxid: 0,
    status: initStatus.value,
    errors: [],
});
const actions = computed(() => actionQueue.slots.map((slot) => slot.action));

watch([initStatus, actions], async ([s, a]) => {
    try {
        let result = await simulate(s, a);
        actionQueue.status = result.status;
        actionQueue.errors = result.errors;
    } catch (err) {
        ElMessage({
            type: "error",
            showClose: true,
            message: err as string,
        });
    }
});

// Solver result
const solverResult = reactive<Sequence>({
    slots: [],
    maxid: 0,
    status: initStatus.value,
    errors: [],
});
watch(() => actionQueue.status, readSolver);

// Drawer status
const openInitQualitySet = ref(false);
const openSolverDrawer = ref(false);
const openExportMacro = ref(false);
const openAttrEnhSelector = ref(false);

async function setInitQuality() {
    openInitQualitySet.value = true;
    return;
    try {
        const { value } = await ElMessageBox.prompt(
            $t('please-input-init-quality'),
            $t('config-init-quality'),
            {
                inputPattern: /^[0-9]+$/,
                inputErrorMessage: $t('please-input-integers'),
            }
        )
        initQuality.value = parseInt(value) || 0
    } catch {
        // canceled
    }
}

const savedQueues = reactive<Sequence[]>([]);
watch(initStatus, (newInitStatus) => {
    // recalculate all results of savedQueues
    for (let q of savedQueues) {
        simulate(
            newInitStatus,
            q.slots.map((x) => x.action)
        ).then((result) => {
            q.status = result.status;
            q.errors = result.errors;
        });
    }
});

function pushSequence(seq: Sequence) {
    for (const i in savedQueues) {
        if (isBetterThan(seq.status, savedQueues[i].status)) {
            savedQueues.splice(Number.parseInt(i), 0, seq)
            return;
        }
    }
    savedQueues.push(seq)
}

function pushAction(action: Actions) {
    actionQueue.slots.push({ id: actionQueue.maxid++, action });
}

function clearSequence() {
    actionQueue.slots = [];
    actionQueue.maxid = 0;
}

function saveSequence() {
    const queue = previewSolver.value
        ? solverResult
        : actionQueue
    pushSequence({
        slots: queue.slots.slice(),
        maxid: queue.maxid,
        status: queue.status,
        errors: queue.errors,
    });
}

function loadSequence(seq: Sequence) {
    actionQueue.slots = seq.slots.slice();
    actionQueue.maxid = seq.maxid;
}

const isReadingSolver = ref(0);
const previewSolver = ref(false);

const displayedStatus = computed(() => {
    return previewSolver.value && solverResult.slots.length > 0
        ? solverResult.status
        : actionQueue.status
})
watch(displayedStatus, (status) => {
    const cond1 = savedQueues.length == 0 && status.progress == status.recipe.difficulty;
    const cond2 = savedQueues.length > 0 && isBetterThan(status, savedQueues[0].status);
    if (cond1 || cond2)
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

async function saveListToJSON() {
    try {
        const queues = savedQueues.map(v => v.slots.map(s => s.action)).filter(v => v.length > 0)
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
            const queues = <Actions[][]>JSON.parse(content)
            for (const actions of queues) {
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
                message: $t('read-n-macros', { n: queues.length }),
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
        <el-drawer v-model="openSolverDrawer" :title="$t('solver-setting')" size="45%">
            <SolverList :init-status="initStatus" :status="actionQueue.status" :recipe-name="item.name"
                @solver-load="readSolver(actionQueue.status)" />
        </el-drawer>
        <el-drawer v-model="openExportMacro" :title="$t('export-macro')" direction="btt" size="80%">
            <MacroExporter :actions="actionQueue.slots.map((v) => v.action)" />
        </el-drawer>
        <el-dialog v-model="openAttrEnhSelector" :title="$t('meal-and-potion')">
            <AttrEnhSelector v-model="attributesEnhancers" />
        </el-dialog>
        <InitialQualitySetting v-model="initQuality" :open="openInitQualitySet" @close="openInitQualitySet = false"
            :item="item" :recipe="recipe" />
        <el-header>
            <h1>{{ item.name }}</h1>
        </el-header>
        <el-main>
            <div class="main-page">
                <StatusBar class="status-bar" :attributes="attributes" :enhancers="attributesEnhancers" :status="
                    displayedStatus
                " @click-attributes="openAttrEnhSelector = true" @click-quality="setInitQuality" />
                <div class="actionpanel-and-savedqueue">
                    <el-scrollbar class="action-panel">
                        <ActionPanel @clicked-action="pushAction" :job="displayJob" :status="actionQueue.status"
                            #lower />
                    </el-scrollbar>
                    <div class="actionqueue-and-savedqueue">
                        <div class="action-queue">
                            <ActionQueue :job="displayJob" :list="actionQueue.slots" :solver-result="solverResult.slots"
                                :preview-solver="previewSolver" :err-list="actionQueue.errors" />
                        </div>
                        <Sidebar class="savedqueue-list-sidebar" v-model:previewSolver="previewSolver"
                            @plus="saveSequence" @delete="clearSequence" @solver="openSolverDrawer = true"
                            @print="openExportMacro = true" @save-list="saveListToJSON" @open-list="openListFromJSON" />
                        <el-scrollbar class="solver-and-savedqueue-scrollbar">
                            <TransitionGroup class="solver-and-savedqueue-list" name="savedqueues" tag="ul">
                                <li v-for="(sq, i) in savedQueues" :key="sq.maxid" class="solver-and-savedqueue-item">
                                    <QueueStatus :status="sq.status" />
                                    <ActionQueue :job="displayJob" :list="sq.slots" :err-list="sq.errors" disabled />
                                    <el-link :icon="Edit" :underline="false" class="savedqueue-item-button"
                                        @click="loadSequence(sq)" />
                                    <el-link :icon="Delete" :underline="false" class="savedqueue-item-button"
                                        @click="savedQueues.splice(i, 1)" />
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

.solver-and-savedqueue-scrollbar {
    flex: auto;
}

.action-panel {
    border-top: 1px solid var(--el-border-color);
    margin-bottom: 6px;
    max-width: 25%;
}

.solver-and-savedqueue-list {
    margin: 0px;
    padding: 0px;
}

.solver-and-savedqueue-item {
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
solver-setting = 求解器设置
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

<fluent locale="en">
solver-setting = Solver setting
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