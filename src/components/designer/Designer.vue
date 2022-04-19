<script setup lang="ts">
import { computed, reactive, ref, watch, } from 'vue'
import 'element-plus/es/components/message/style/css'
import { ElMessage } from 'element-plus'
import { Delete, Edit } from '@element-plus/icons-vue'
import { Attributes, Jobs, Actions, simulate, Recipe, Status, newStatus } from '../../Craft'
import { read_solver } from '../../Solver'
import ActionPanel from './ActionPanel.vue'
import ActionQueue from './ActionQueue.vue'
import StatusBar from './StatusBar.vue'
import Sidebar from './Sidebar.vue'
import SolverList from './SolverList.vue'
import MarcoExporter from './MarcoExporter.vue'
import QueueStatus from './QueueStatus.vue'

interface Slot {
    id: number
    action: Actions
}

// 用于表示一个技能的序列，或者说一个宏
// 为了实现拖拽和删除等动画效果，我们需要给每个技能一个唯一的id
// maxid储存了当前序列中最大的标志，并用于生成下一个id
interface Sequence {
    slots: Slot[]
    maxid: number
    status: Status
    errors: { pos: number, err: string }[]
}

const props = defineProps<{
    itemName: string;
    job: Jobs | 'unknown';
    attributes: Attributes;
    recipe: Recipe;
}>()
const displayJob = computed(() => props.job == 'unknown' ? Jobs.Culinarian : props.job)

// Simulation
const initStatus = ref<Status>(await newStatus(props.attributes, props.recipe))
watch(props, async p => {
    initStatus.value = await newStatus(p.attributes, p.recipe)
})
// Actions Queue
const actionQueue = reactive<Sequence>({ slots: [], maxid: 0, status: initStatus.value, errors: [] })
const actions = computed(() => actionQueue.slots.map(slot => slot.action))

watch([initStatus, actions], async ([s, a]) => {
    try {
        let result = await simulate(s, a)
        actionQueue.status = result.status
        actionQueue.errors = result.errors
    } catch (err) {
        ElMessage({
            type: 'error',
            showClose: true,
            message: err as string,
        })
    }
})

// Solver result
const solverResult = reactive<Sequence>({ slots: [], maxid: 0, status: initStatus.value, errors: [] })
watch(() => actionQueue.status, readSolver)
// Drawer status
const openSolverDrawer = ref(false)
const openExportMarco = ref(false)

const savedQueues = reactive<Sequence[]>([])
watch(initStatus, (newInitStatus) => {
    // recalculate all results of savedQueues
    for (let q of savedQueues) {
        simulate(newInitStatus, q.slots.map(x => x.action))
            .then(result => {
                q.status = result.status
                q.errors = result.errors
            })
    }
})

function pushAction(action: Actions) {
    actionQueue.slots.push({ id: actionQueue.maxid++, action })
}

function clearSequence() {
    actionQueue.slots = []
    actionQueue.maxid = 0
}

function saveSequence() {
    savedQueues.push({
        slots: actionQueue.slots.slice(),
        maxid: actionQueue.maxid,
        status: actionQueue.status,
        errors: actionQueue.errors,
    })
}

function loadSequence(seq: Sequence) {
    actionQueue.slots = seq.slots.slice()
    actionQueue.maxid = seq.maxid
}

async function readSolver(s: Status) {
    try {
        const newSolverResult = actions.value.concat(await read_solver(s))
        let display = [];
        let oldID = new Map<Actions, number[]>()
        for (const slot of solverResult.slots) {
            if (oldID.get(slot.action)?.push(slot.id) == undefined)
                oldID.set(slot.action, [slot.id])
        }
        for (const skill of newSolverResult) {
            const i = oldID.get(skill)?.shift() || solverResult.maxid++;
            display.push({ id: i, action: skill })
        }
        solverResult.slots = display

        const result = await simulate(initStatus.value, newSolverResult)
        solverResult.status = result.status
        solverResult.errors = result.errors
    } catch (err) {
        solverResult.slots = []
    }
}

</script>

<template>
    <el-container>
        <el-drawer v-model="openSolverDrawer" title="求解器设置" size="45%">
            <SolverList :init-status="initStatus" :status="actionQueue.status" :recipe-name="itemName"
                @solver-load="readSolver(actionQueue.status)" />
        </el-drawer>
        <el-drawer v-model="openExportMarco" title="导出宏" direction="btt" size="80%">
            <MarcoExporter :actions="actionQueue.slots.map(v => v.action)" />
        </el-drawer>
        <el-header>
            <h1>{{ itemName }}</h1>
        </el-header>
        <el-main>
            <div class="main-page">
                <StatusBar class="status-bar" :status="actionQueue.status" />
                <div class="action-queue">
                    <ActionQueue :job="displayJob" :list="actionQueue.slots" :err-list="actionQueue.errors" />
                </div>
                <div class="solver-and-savedqueue">
                    <Sidebar class="savedqueue-list-sidebar" @plus="saveSequence" @delete="clearSequence"
                        @solver="openSolverDrawer = true" @print="openExportMarco = true" />
                    <el-scrollbar class="solver-and-savedqueue-scrollbar">
                        <ul class="solver-and-savedqueue-list">
                            <li v-if="solverResult.slots.length > 0" class="solver-and-savedqueue-item">
                                <QueueStatus :status="solverResult.status" />
                                <ActionQueue :job="displayJob" :list="solverResult.slots" disabled />
                                <el-link :icon="Edit" :underline="false" class="savedqueue-item-button"
                                    @click="loadSequence(solverResult)" />
                            </li>
                            <li v-for="sq, i in savedQueues" class="solver-and-savedqueue-item">
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
                <ActionPanel class="action-panel" @clicked-action="pushAction" :job="displayJob"
                    :status="actionQueue.status" #lower />
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
    background-color: #fafafa;
}

.solver-and-savedqueue {
    display: flex;
    flex: auto;
    overflow: hidden;
}

.savedqueue-list-sidebar {
    border-right: 1px solid var(--el-border-color);
}

.solver-and-savedqueue-scrollbar {
    flex: auto;
}

.action-panel {
    margin-bottom: 6px;
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
