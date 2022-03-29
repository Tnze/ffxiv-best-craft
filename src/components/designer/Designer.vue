<script setup lang="ts">
import { ref, watch, watchEffect } from 'vue'
import { Attributes, Jobs, Actions, simulate, Recipe, Status, new_status } from '../../Craft'
import ActionPanel from './ActionPanel.vue'
import ActionQueue from './ActionQueue.vue'
import StatusBar from './StatusBar.vue'
import Sidebar from './Sidebar.vue'

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

watchEffect(async () => {
    console.log(props.attributes, props.recipe)
    if (props.recipe == null) return
    let s = await new_status(props.attributes as Attributes, props.recipe as Recipe)
    initStatus.value = s
})
watch([initStatus, actionQueue], async ([s, actions]) => {
    let result = await simulate(s!, actions.map(x => x.action))
    status.value = result.status
    castingErrors.value = result.errors
}, { deep: true })

interface Slot {
    id: number
    action: Actions
}

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
    <el-empty v-if="status == undefined" description="未加载配方" style="height: 100%;" />
    <el-container v-else>
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
                    />
                    <el-scrollbar class="solver-and-options-scrollbar">
                        <ul class="solver-and-options-list">
                            <li v-for="sq in savedQueues" class="solver-and-options-item">
                                <ActionQueue :job="job" :list="sq" :err-list="[]" disabled />
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
    height: 50px;
}
</style>
