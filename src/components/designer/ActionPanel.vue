<!-- 
    This file is part of BestCraft.
    Copyright (C) 2023  <name of author>

    BestCraft is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    BestCraft is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
-->

<script setup lang="ts">
import { ElPopover } from 'element-plus';
import { computed, reactive, watchEffect } from 'vue'
import Action from './Action.vue'
import { Jobs, Actions, Status, allowedList, craftPointsList, Conditions } from '../../Craft'

const props = defineProps<{
    job: Jobs,
    status?: Status,
    simulatorMode?: boolean,
    disable?: boolean,
}>()

const emit = defineEmits<{
    (event: 'clickedAction', action: Actions): void
    (event: 'mousehoverAction', action: Actions): void
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

const actionsForSimulator: Actions[][] = [
    [
        Actions.Reflect,
        Actions.MuscleMemory,
        Actions.TrainedEye,
    ],
    [
        Actions.Veneration,
        Actions.RapidSynthesis,
        Actions.PrudentSynthesis,
        Actions.Groundwork,
        Actions.BasicSynthesis,
        Actions.CarefulSynthesis,
    ],
    [
        Actions.Innovation,
        Actions.HastyTouch,
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
        Actions.FinalAppraisal,
        Actions.CarefulObservation,
        Actions.HeartAndSoul,
        Actions.TricksOfTheTrade,
        Actions.IntensiveSynthesis,
        Actions.PreciseTouch,
    ],
    [
        Actions.MastersMend,
        Actions.WasteNot,
        Actions.WasteNotII,
        Actions.Manipulation,
    ],
    [
        Actions.DelicateSynthesis,
        Actions.Observe,
        Actions.FocusedSynthesis,
        Actions.FocusedTouch,
    ]
]

const usedActions = computed(() => props.simulatorMode ? actionsForSimulator : actions)

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
            return props.status.condition == Conditions.Good ||
                props.status.condition == Conditions.Excellent ||
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
        <div v-for="group in usedActions" class="group">
            <el-popover v-for="action in group" :show-after="1000" :hide-after="0"
                :title="$t(action.replaceAll('_', '-'))" :content="$t('desc-' + action.replaceAll('_', '-'))">
                <template #reference>
                    <Action :job="job" class="item" @click="emit('clickedAction', action)"
                        @mouseover="emit('mousehoverAction', action)" @mouseleave="emit('mouseleaveAction', action)"
                        :action="action" :active="isActived(action)"
                        :effect="!disable && cachedAllowedList.get(action) == 'ok' ? 'normal' : 'black'"
                        :cp="cachedCraftPointsList.get(action) || undefined" />
                </template>
            </el-popover>
        </div>
        <div class="group" v-if="!simulatorMode">
            <Action :job="job" class="item" v-for="action in fail_actions" @click="emit('clickedAction', action)"
                @mouseover="emit('mousehoverAction', action)" @mouseleave="emit('mouseleaveAction', action)"
                :action="action" :active="isActived(action)"
                :effect="!disable && cachedAllowedList.get(action) == 'ok' ? 'red-cross' : 'black'"
                :cp="cachedCraftPointsList.get(action) || undefined" />
        </div>
    </div>
</template>

<style scoped>
.container {
    box-sizing: border-box;
    padding: 0px 6px;
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