<script setup lang="ts">
import { reactive, watchEffect } from 'vue'
import Action from './Action.vue'
import { Jobs, Actions, Status, allowedList, craftPointsList } from '../../Craft'

const props = defineProps<{
    job: Jobs,
    status?: Status
}>()

const emit = defineEmits<{
    (event: 'clickedAction', action: Actions): void
    (event: 'mouseoverAction', action: Actions): void
    (event: 'mouseleaveAction', action: Actions): void
}>()

const actions: Actions[][] = [
    [
        Actions.Reflect,
        Actions.MuscleMemory,
        Actions.TrainedEye,
    ],
    [
        Actions.MastersMend,
        Actions.WasteNot,
        Actions.WasteNotII,
        Actions.Manipulation,
    ],
    [
        Actions.Veneration,
        Actions.PrudentSynthesis,
        Actions.Groundwork,
        Actions.BasicSynthesis,
        Actions.CarefulSynthesis,
    ],
    [
        Actions.Innovation,
        Actions.PrudentTouch,
        Actions.PreparatoryTouch,
        Actions.BasicTouch,
        Actions.StandardTouch,
        Actions.AdvancedTouch,
        Actions.TrainedFinesse,
        Actions.GreatStrides,
        Actions.ByregotsBlessing,
    ],
    [
        Actions.TricksOfTheTrade,
        Actions.IntensiveSynthesis,
        Actions.PreciseTouch,
    ],
    [
        Actions.DelicateSynthesis,
        Actions.Observe,
        Actions.FocusedSynthesis,
        Actions.FocusedTouch,
    ],
    [
        Actions.FinalAppraisal,
        Actions.CarefulObservation,
        Actions.HeartAndSoul,
    ],
    [
        Actions.RapidSynthesis,
        Actions.HastyTouch,
    ]
]

const fail_actions: Actions[] = [
    Actions.RapidSynthesisFail,
    Actions.HastyTouchFail,
    Actions.FocusedSynthesisFail,
    Actions.FocusedTouchFail,
]

const isActived = (action: Actions) => {
    if (props.status == undefined)
        return false;
    switch (action) {
        case Actions.Reflect:
        case Actions.MuscleMemory:
        case Actions.TrainedEye:
            return props.status.step == 0
        case Actions.TricksOfTheTrade:
        case Actions.IntensiveSynthesis:
        case Actions.PreciseTouch:
            return props.status.condition == 'good' ||
                props.status.condition == 'excellent' ||
                props.status.buffs.heart_and_soul > 0
        case Actions.ByregotsBlessing:
            return props.status.buffs.inner_quiet > 0
        case Actions.StandardTouch:
            return props.status.buffs.touch_combo_stage == 1
        case Actions.AdvancedTouch:
            return props.status.buffs.touch_combo_stage == 2
        case Actions.FocusedSynthesis:
        case Actions.FocusedTouch:
            return props.status.buffs.observed > 0
    }
    return false;
}

const cachedAllowedList = reactive(new Map<Actions, string>())
const cachedCraftPointsList = reactive(new Map<Actions, number>())

watchEffect(() => {
    if (props.status == undefined) {
        cachedAllowedList.clear()
        return
    }
    const actions = Object.values(Actions)
    allowedList(props.status, actions).then(result => {
        actions.forEach((i, v) => {
            cachedAllowedList.set(i, result[v])
        })
    })
    craftPointsList(props.status, actions).then(result => {
        actions.forEach((i, v) => {
            cachedCraftPointsList.set(i, result[v])
        })
    })
})

</script>

<template>
    <div class="container" @click.stop.prevent.right>
        <div v-for="group in actions" class="group">
            <el-tooltip v-for="action in group" :hide-after="0" :enterable="false" :content="$t(action.replaceAll('_', '-'))"
                :show-after="1000">
                <Action :job="job" class="item" @click="emit('clickedAction', action)"
                    @mouseover="emit('mouseoverAction', action)" @mouseleave="emit('mouseleaveAction', action)"
                    :action="action" :active="isActived(action)"
                    :effect="cachedAllowedList.get(action) == 'ok' ? 'normal' : 'black'"
                    :cp="cachedCraftPointsList.get(action) || undefined" />
            </el-tooltip>
        </div>
        <div class="group">
            <Action :job="job" class="item" v-for="action in fail_actions" @click="emit('clickedAction', action)"
                @mouseover="emit('mouseoverAction', action)" @mouseleave="emit('mouseleaveAction', action)"
                :action="action" :active="isActived(action)"
                :effect="cachedAllowedList.get(action) == 'ok' ? 'red-cross' : 'black'"
                :cp="cachedCraftPointsList.get(action) || undefined" />
        </div>
    </div>
</template>

<style scoped>
.container {
    box-sizing: border-box;
    padding: 0px 6px;
    border-right: 1px solid var(--el-border-color);
}

.group {
    border-bottom: 1px solid var(--el-border-color);
    padding-top: 5px;
}

.item {
    transform: scale(0.8);
    margin: calc(-48px * 0.07);
}
</style>

<fluent locale="zh-CN">
action-panel = 技能面板
</fluent>

<fluent locale="en">
action-panel = Action Panel
</fluent>