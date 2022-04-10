<script setup lang="ts">
import { computed, ref, watch, watchEffect } from 'vue'
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
    recipe: Recipe | null;
}>()
const displayJob = computed(() => props.job == 'unknown' ? Jobs.Culinarian : props.job)
const actionQueue = ref<Slot[]>([])
const actions = computed(() => actionQueue.value.map(slot => slot.action))
const initStatus = ref<Status | undefined>(undefined)
const status = ref<Status | undefined>(undefined)
const castingErrors = ref<{ pos: number, err: string }[]>([])
const openSolverDrawer = ref(false)
const openExportMarco = ref(false)
const solverResult = ref<Actions[]>([])
const solverResultDisplay = ref<Slot[]>([])

watchEffect(async () => {
    if (props.recipe == null) return
    try {
        let s = await newStatus(props.attributes as Attributes, props.recipe as Recipe)
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
    try {
        let result = await simulate(s!, actions.map(x => x.action))
        status.value = result.status
        castingErrors.value = result.errors
        try {
            solverResult.value = await read_solver(result.status)
        } catch (err) {
            solverResult.value = []
        }
    } catch (err) {
        ElMessage({
            type: 'error',
            showClose: true,
            message: err as string,
        })
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
                    <ActionQueue :job="displayJob" :list="actionQueue" :err-list="castingErrors" />
                </div>
                <div class="solver-and-savedqueue">
                    <Sidebar
                        class="savedqueue-list-sidebar"
                        @plus="saveQueue"
                        @delete="actionQueue = []"
                        @solver="openSolverDrawer = true"
                        @print="openExportMarco = true"
                    />
                    <el-scrollbar class="solver-and-savedqueue-scrollbar">
                        <ul class="solver-and-savedqueue-list">
                            <li v-if="solverResult.length > 0" class="solver-and-savedqueue-item">
                                <ActionQueue
                                    :job="displayJob"
                                    :list="solverResultDisplay"
                                    disabled
                                />
                            </li>
                            <li v-for="sq, i in savedQueues" class="solver-and-savedqueue-item">
                                <QueueStatus :status="simulate(initStatus!, sq.map(x => x.action))" />
                                <ActionQueue :job="displayJob" :list="sq" disabled />
                                <el-link
                                    :icon="Edit"
                                    :underline="false"
                                    class="savedqueue-item-status"
                                    @click="actionQueue = sq.slice()"
                                />
                                <el-link
                                    :icon="Delete"
                                    :underline="false"
                                    class="savedqueue-item-status"
                                    @click="savedQueues.splice(i, 1)"
                                />
                            </li>
                        </ul>
                    </el-scrollbar>
                </div>
                <ActionPanel
                    class="action-panel"
                    @clicked-action="pushAction"
                    :job="displayJob"
                    :status="status"
                    #lower
                />
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
}
.savedqueue-item-status {
    margin-right: 6px;
}
.el-link {
    margin-right: 8px;
}
</style>
