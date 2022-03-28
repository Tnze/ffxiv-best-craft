<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { Attributes, Jobs, Actions, simulate, Recipe, new_recipe, Status, new_status } from '../../Craft'
import ActionPanel from './ActionPanel.vue'
import Action from './Action.vue'
import Split from './Split.vue';
import ActionQueue from './ActionQueue.vue'
import StatusBar from './StatusBar.vue'

const props = defineProps<{
    itemName: string;
    job: Jobs;
    attributes: Attributes;
    recipe?: Recipe;
}>()
const actionQueue = ref<Slot[]>([])
const initStatus = ref<Status | undefined>(undefined)
const status = ref<Status | undefined>(undefined)
const castingErrors = ref<{ pos: number, err: string }[]>([])

watch(() => [props.attributes, props.recipe], async ([attr, recipe]) => {
    if (recipe == undefined) return
    let s = await new_status(attr as Attributes, recipe as Recipe)
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

const onClick = (action: Actions) => {
    actionQueue.value.push({ id: maxid.value, action })
    maxid.value++
}

</script>

<template>
    <el-container>
        <el-header>
            <h1>{{ itemName }}</h1>
        </el-header>
        <el-main>
            <Split>
                <template #upper>
                    <el-row :gutter="0">
                        <StatusBar :status="status!" />
                    </el-row>
                    <el-row class="action-queue">
                        <ActionQueue :job="job" :list="actionQueue" :err-list="castingErrors" />
                    </el-row>
                </template>
                <template #lower>
                    <el-scrollbar>
                        <ActionPanel @clicked-action="onClick" :job="job" #lower />
                    </el-scrollbar>
                </template>
            </Split>
        </el-main>
    </el-container>
</template>

<style scoped>
.action-queue {
    border-top: 1px solid var(--el-border-color);
    border-bottom: 1px solid var(--el-border-color);
}
.el-footer {
    padding: 0;
    height: 100px;
}
</style>
