<script setup lang="ts">
import { computed, Ref, ref, unref, watch, watchEffect } from 'vue'
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

const props = defineProps<{
    itemName: string;
    job: Jobs | 'unknown';
    attributes: Attributes;
    recipe: Recipe;
}>()
const displayJob = computed(() => props.job == 'unknown' ? Jobs.Culinarian : props.job)

// Actions Queue
const actionQueue = ref<Slot[]>([])
const actions = computed(() => actionQueue.value.map(slot => slot.action))

// Simulation
const initStatus = ref<Status>(await newStatus(props.attributes, props.recipe))
watch(props, async p => {
    console.log('recipe updated')
    initStatus.value = await newStatus(p.attributes, p.recipe)
})
const { status, errors } = await (async () => {
    console.log('status updated')
    const { status, errors } = await simulate(initStatus.value, actions.value)
    return { status: ref(status), errors: ref(errors) }
})()
watch([initStatus, actions], async ([s, a]) => {
    try {
        let result = await simulate(s, a)
        status.value = result.status
        errors.value = result.errors
    } catch (err) {
        ElMessage({
            type: 'error',
            showClose: true,
            message: err as string,
        })
    }
})

// Solver result
const solverResult = ref<Actions[]>([])
watch(status, async (s) => {
    try {
        solverResult.value = actions.value.concat(await read_solver(s))
    } catch (err) {
        solverResult.value = []
    }
})

// Solver result displayer
const solverResultDisplay = ref<Slot[]>([])
watch(solverResult, async (newSolverResult) => {
    let display = [];
    let oldSolverResult = solverResultDisplay.value;
    let maxid = 0;
    if (oldSolverResult.length > 0)
        maxid = oldSolverResult.map(x => x.id).reduce((pv, cv) => Math.max(pv, cv));
    let oldID = new Map<Actions, number[]>()
    for (const slot of oldSolverResult) {
        if (oldID.get(slot.action)?.push(slot.id) == undefined)
            oldID.set(slot.action, [slot.id])
    }
    for (const skill of newSolverResult) {
        const i = oldID.get(skill)?.shift() || ++maxid;
        display.push({ id: i, action: skill })
    }
    solverResultDisplay.value = display
})

// Solver result status
const solverResultStatus = ref<Status>(initStatus.value)
watch([initStatus, solverResult], async ([s, a]) => {
    const result = await simulate(s, a)
    solverResultStatus.value = result.status
})

// Drawer status
const openSolverDrawer = ref(false)
const openExportMarco = ref(false)

const pushAction = (action: Actions) => {
    const maxid = actionQueue.value.map(v => v.id).reduce((a, b) => Math.max(a, b), 0)
    actionQueue.value.push({ id: maxid + 1, action })
}

const savedQueues = ref<Slot[][]>([])
const savedQueuesResults = ref<Status[]>([])
watchEffect(() => {
    savedQueuesResults.value = new Array(savedQueues.value.length)
    for (const i in savedQueues.value)
        simulate(initStatus.value, savedQueues.value[i].map(x => x.action))
            .then(result => savedQueuesResults.value[i] = result.status)
})

</script>

<template>
    <el-container>
        <el-drawer v-model="openSolverDrawer" title="求解器设置" size="45%">
            <SolverList :init-status="initStatus" :status="status" :recipe-name="itemName" />
        </el-drawer>
        <el-drawer v-model="openExportMarco" title="导出宏" direction="btt" size="95%">
            <MarcoExporter :actions="actions" />
        </el-drawer>
        <el-header>
            <h1>{{ itemName }}</h1>
        </el-header>
        <el-main>
            <div class="main-page">
                <StatusBar class="status-bar" :status="status!" />
                <div class="action-queue">
                    <ActionQueue :job="displayJob" :list="actionQueue" :err-list="errors" />
                </div>
                <div class="solver-and-savedqueue">
                    <Sidebar class="savedqueue-list-sidebar" @plus="savedQueues.push(actionQueue.slice())"
                        @delete="actionQueue = []" @solver="openSolverDrawer = true" @print="openExportMarco = true" />
                    <el-scrollbar class="solver-and-savedqueue-scrollbar">
                        <ul class="solver-and-savedqueue-list">
                            <li v-if="solverResult.length > 0" class="solver-and-savedqueue-item">
                                <QueueStatus :status="solverResultStatus" />
                                <ActionQueue :job="displayJob" :list="solverResultDisplay" disabled />
                                <el-link :icon="Edit" :underline="false" class="savedqueue-item-button"
                                    @click="actionQueue = solverResultDisplay" />
                            </li>
                            <li v-for="sq, i in savedQueues" class="solver-and-savedqueue-item">
                                <QueueStatus :status="savedQueuesResults[i]" />
                                <ActionQueue :job="displayJob" :list="sq" disabled />
                                <el-link :icon="Edit" :underline="false" class="savedqueue-item-button"
                                    @click="actionQueue = sq.slice()" />
                                <el-link :icon="Delete" :underline="false" class="savedqueue-item-button"
                                    @click="savedQueues.splice(i, 1)" />
                            </li>
                        </ul>
                    </el-scrollbar>
                </div>
                <ActionPanel class="action-panel" @clicked-action="pushAction" :job="displayJob" :status="status"
                    #lower />
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
