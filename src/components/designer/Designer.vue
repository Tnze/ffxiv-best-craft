<script setup lang="ts">
import { ref, watch, watchEffect } from 'vue'
import 'element-plus/es/components/message/style/css'
import { ElMessage } from 'element-plus'
import { Attributes, Jobs, Actions, simulate, Recipe, Status, new_status } from '../../Craft'
import { read_solver } from '../../Solver'
import ActionPanel from './ActionPanel.vue'
import ActionQueue from './ActionQueue.vue'
import StatusBar from './StatusBar.vue'
import Sidebar from './Sidebar.vue'
import SolverList from './SolverList.vue'

interface Slot {
    id: number
    action: Actions
}

const props = defineProps<{
    itemName: string;
    job: Jobs;
    attributes: Attributes;
    recipe: Recipe | null;
}>()
const actionQueue = ref<Slot[]>([])
const initStatus = ref<Status | undefined>(undefined)
const status = ref<Status | undefined>(undefined)
const castingErrors = ref<{ pos: number, err: string }[]>([])
const openSolverDrawer = ref(false)
const solverResult = ref<Actions[]>([])
const solverResultDisplay = ref<Slot[]>([])

watchEffect(async () => {
    if (props.recipe == null) return
    try {
        let s = await new_status(props.attributes as Attributes, props.recipe as Recipe)
        initStatus.value = s
    } catch (e) {
        ElMessage({
            type: 'error',
            showClose: true,
            message: e as string,
        })
    }
})
watch([initStatus, actionQueue], async ([s, actions]) => {
    let result = await simulate(s!, actions.map(x => x.action))
    status.value = result.status
    castingErrors.value = result.errors
    try {
        solverResult.value = await read_solver(result.status)
    } catch (err) {
        solverResult.value = []
        console.log(err)
    }
}, { deep: true })

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

const maxid = ref(0)

const pushAction = (action: Actions) => {
    actionQueue.value.push({ id: maxid.value, action })
    maxid.value++
}

const savedQueues = ref<Slot[][]>([])

const saveQueue = () => {
    savedQueues.value.push(actionQueue.value.slice())
}

</script>

<template>
    <el-empty v-if="status == undefined" description="请先选择配方" style="height: 100%;" />
    <el-container v-else>
        <el-drawer v-model="openSolverDrawer" title="求解器设置" size="45%">
            <SolverList :init-status="initStatus" :recipe-name="itemName" />
        </el-drawer>
        <el-header>
            <h1>{{ itemName }}</h1>
        </el-header>
        <el-main>
            <div class="main-page">
                <StatusBar class="status-bar" :status="status!" />
                <div class="action-queue">
                    <ActionQueue :job="job" :list="actionQueue" :err-list="castingErrors" />
                </div>
                <div class="solver-and-options">
                    <Sidebar
                        class="options-list-sidebar"
                        @plus="saveQueue"
                        @delete="actionQueue = []"
                        @solver="openSolverDrawer = true"
                    />
                    <el-scrollbar class="solver-and-options-scrollbar">
                        <ul class="solver-and-options-list">
                            <li v-if="solverResult.length > 0" class="solver-and-options-item">
                                <ActionQueue :job="job" :list="solverResultDisplay" disabled />
                            </li>
                            <li v-for="sq in savedQueues" class="solver-and-options-item">
                                <ActionQueue :job="job" :list="sq" disabled />
                            </li>
                        </ul>
                    </el-scrollbar>
                </div>
                <ActionPanel class="action-panel" @clicked-action="pushAction" :job="job" #lower />
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
.solver-and-options {
    display: flex;
    flex: auto;
    overflow: hidden;
}
.options-list-sidebar {
    border-right: 1px solid var(--el-border-color);
}
.solver-and-options-scrollbar {
    flex: auto;
}
.action-panel {
    margin-bottom: 6px;
}
.solver-and-options-list {
    margin: 0px;
    padding: 0px;
}
.solver-and-options-item {
    display: flex;
    border-bottom: 1px solid var(--el-border-color);
}
</style>
