<script setup lang="ts">
import { save, open } from '@tauri-apps/api/dialog'
import { writeFile, readTextFile } from '@tauri-apps/api/fs'
import { computed, reactive, ref, watch } from "vue";
import { ElMessage, ElMessageBox } from "element-plus";
import { Delete, Edit } from "@element-plus/icons-vue";
import {
    Attributes,
    Jobs,
    Actions,
    simulate,
    Recipe,
    Status,
    newStatus,
} from "../../Craft";
import { read_solver } from "../../Solver";
import ActionPanel from "./ActionPanel.vue";
import ActionQueue from "./ActionQueue.vue";
import StatusBar from "./StatusBar.vue";
import Sidebar from "./Sidebar.vue";
import SolverList from "./SolverList.vue";
import MarcoExporter from "./MarcoExporter.vue";
import QueueStatus from "./QueueStatus.vue";
import AttrEnhSelector from "../attr-enhancer/AttrEnhSelector.vue";
import { Enhancer } from "../attr-enhancer/Enhancer";
import { useStore } from '../../store';

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

const store = useStore()
const attributes = computed(() =>
    store.state.gearsets.special.find(v => v.name == store.state.designer!.job)?.value || store.state.gearsets.default
)
const displayJob = computed(() =>
    store.state.designer!.job == "unknown" ? Jobs.Culinarian : store.state.designer!.job
);

// 食物和药水效果
const attributesEnhancers = ref<Enhancer[]>([]);
const enhancedAttributes = computed<Attributes>(() => {
    let { level, craftsmanship, control, craft_points } = attributes.value;
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
    await newStatus(enhancedAttributes.value, store.state.designer!.recipe, initQuality.value)
);
watch([store.state.designer!, enhancedAttributes, initQuality], async ([p, ea, iq]) => {
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
const openSolverDrawer = ref(false);
const openExportMarco = ref(false);
const openAttrEnhSelector = ref(false);

async function setInitQuality() {
    try {
        const { value } = await ElMessageBox.prompt('请输入初期品质', '设置初期品质', {
            confirmButtonText: '确定',
            cancelButtonText: '取消',
            inputPattern: /^[0-9]+$/,
            inputErrorMessage: '请输入一个整数',
        })
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

function pushAction(action: Actions) {
    actionQueue.slots.push({ id: actionQueue.maxid++, action });
}

function clearSequence() {
    actionQueue.slots = [];
    actionQueue.maxid = 0;
}

function saveSequence() {
    const slots = previewSolver.value
        ? actionQueue.slots.concat(solverResult.slots)
        : actionQueue.slots.slice()
    savedQueues.push({
        slots,
        maxid: actionQueue.maxid,
        status: actionQueue.status,
        errors: actionQueue.errors,
    });
}

function loadSequence(seq: Sequence) {
    actionQueue.slots = seq.slots.slice();
    actionQueue.maxid = seq.maxid;
}

const isReadingSolver = ref(0);
const previewSolver = ref(false);

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
        const queues = [actionQueue].concat(savedQueues).map(v => v.slots.map(s => s.action)).filter(v => v.length > 0)
        try {
            if (queues.length == 0) {
                await ElMessageBox.confirm(
                    '当前要保存的宏数量为0，是否继续？',
                    '警告',
                    {
                        confirmButtonText: 'OK',
                        cancelButtonText: 'Cancel',
                        type: 'warning',
                    }
                )
            }
        } catch {
            return
        }
        const { level, craftsmanship, control, craft_points } = enhancedAttributes.value
        const path = await save({
            defaultPath: `${store.state.designer!.itemName}-${level}-${craftsmanship}-${control}-${craft_points}`,
            filters: [{ name: 'BestCraft宏文件', extensions: ['json'] }],
            title: '保存文件'
        })
        if (!path) {
            return
        }
        await writeFile({ path, contents: JSON.stringify(queues) })
        ElMessage({
            type: "success",
            showClose: true,
            message: '保存成功',
        });
    } catch (err) {
        ElMessage({
            type: "error",
            showClose: true,
            message: '保存失败：' + err as string,
        });
    }
}

async function openListFromJSON() {
    const pathlist = <string[]>await open({
        filters: [{ name: 'BestCraft宏文件', extensions: ['json'] }],
        multiple: true,
        title: '打开文件'
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
                savedQueues.push({
                    slots,
                    maxid: slots.length - 1,
                    status,
                    errors,
                });
            }
            ElMessage({
                type: "success",
                showClose: true,
                message: `读取了 ${queues.length} 个宏`,
            });
        } catch (err) {
            ElMessage({
                type: "error",
                showClose: true,
                message: `读取失败：` + err as string,
            });
        }
    }
}
</script>

<template>
    <el-container>
        <el-drawer v-model="openSolverDrawer" title="求解器设置" size="45%">
            <SolverList :init-status="initStatus" :status="actionQueue.status"
                :recipe-name="store.state.designer!.itemName" @solver-load="readSolver(actionQueue.status)" />
        </el-drawer>
        <el-drawer v-model="openExportMarco" title="导出宏" direction="btt" size="80%">
            <MarcoExporter :actions="actionQueue.slots.map((v) => v.action)" />
        </el-drawer>
        <el-dialog v-model="openAttrEnhSelector" title="食物 & 药水">
            <AttrEnhSelector v-model="attributesEnhancers" />
        </el-dialog>
        <el-header>
            <h1>{{ store.state.designer!.itemName }}</h1>
        </el-header>
        <el-main>
            <div class="main-page">
                <StatusBar class="status-bar" :attributes="attributes" :enhancers="attributesEnhancers" :status="
                    previewSolver && solverResult.slots.length > 0
                        ? solverResult.status
                        : actionQueue.status
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
                            @print="openExportMarco = true" @save-list="saveListToJSON" @open-list="openListFromJSON" />
                        <el-scrollbar class="solver-and-savedqueue-scrollbar">
                            <ul class="solver-and-savedqueue-list">
                                <li v-for="(sq, i) in savedQueues" class="solver-and-savedqueue-item">
                                    <QueueStatus :status="sq.status" />
                                    <ActionQueue :job="displayJob" :list="sq.slots" :err-list="sq.errors" disabled />
                                    <el-link :icon="Edit" :underline="false" class="savedqueue-item-button"
                                        @click="loadSequence(sq)" />
                                    <el-link :icon="Delete" :underline="false" class="savedqueue-item-button"
                                        @click="savedQueues.splice(i, 1)" />
                                </li>
                            </ul>
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
</style>
