<!-- 
    This file is part of BestCraft.
    Copyright (C) 2024  Tnze

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
import { NPopover } from 'naive-ui';
import { computed, reactive, watchEffect } from 'vue';
import Action from '@/components/designer/Action.vue';
import {
    Jobs,
    Actions,
    Status,
    allowedList,
    craftPointsList,
    Conditions,
    LimitedActionState,
} from '@/libs/Craft';

const props = defineProps<{
    job: Jobs;
    status?: Status;
    simulatorMode?: boolean;
    disable?: boolean;
}>();

const emit = defineEmits<{
    (event: 'clickedAction', action: Actions): void;
    (event: 'mousehoverAction', action: Actions): void;
    (event: 'mouseleaveAction', action: Actions): void;
}>();

const actions: Actions[][] = [
    [
        Actions.Reflect,
        Actions.MuscleMemory,
        Actions.TrainedEye,
        Actions.CarefulObservation,
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
        Actions.HastyTouch, // Actions.DaringTouch,
        Actions.PrudentTouch,
        Actions.PreparatoryTouch,
        Actions.BasicTouch,
        Actions.StandardTouch,
        Actions.AdvancedTouch,
        Actions.TrainedFinesse,
        Actions.GreatStrides,
        Actions.QuickInnovation,
        Actions.ByregotsBlessing,
    ],
    [
        Actions.FinalAppraisal,
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
        Actions.ImmaculateMend,
        Actions.TrainedPerfection,
    ],
    [Actions.DelicateSynthesis, Actions.Observe],
];

const usedActions = computed(() =>
    actions.map(actions =>
        actions.map(action => {
            if (
                action === Actions.HastyTouch &&
                props.status != undefined &&
                props.status.buffs.expedience > 0
            ) {
                return Actions.DaringTouch;
            }
            return action;
        }),
    ),
);

const isActived = (action: Actions) => {
    if (props.status == undefined) return false;
    switch (action) {
        case Actions.Reflect:
        case Actions.MuscleMemory:
        case Actions.TrainedEye:
            return props.status.step == 0;
        case Actions.TricksOfTheTrade:
        case Actions.IntensiveSynthesis:
        case Actions.PreciseTouch:
            return (
                props.status.condition == Conditions.Good ||
                props.status.condition == Conditions.Excellent ||
                props.status.buffs.heart_and_soul == LimitedActionState.Active
            );
        case Actions.ByregotsBlessing:
            return props.status.buffs.inner_quiet > 0;
        case Actions.RefinedTouch:
        case Actions.StandardTouch:
            return props.status.buffs.touch_combo_stage == 1;
        case Actions.AdvancedTouch:
            return (
                props.status.buffs.touch_combo_stage == 2 ||
                props.status.buffs.observed > 0
            );
        case Actions.DaringTouch:
            return props.status.buffs.expedience > 0;
    }
    return false;
};

const cachedAllowedList = reactive(new Map<Actions, string>());
const cachedCraftPointsList = reactive(new Map<Actions, number>());

watchEffect(() => {
    if (props.status == undefined) {
        cachedAllowedList.clear();
        return;
    }
    const actions = Object.values(Actions);
    allowedList(props.status, actions).then(result => {
        actions.forEach((i, v) => {
            cachedAllowedList.set(i, result[v]);
        });
    });
    craftPointsList(props.status, actions).then(result => {
        actions.forEach((i, v) => {
            cachedCraftPointsList.set(i, result[v]);
        });
    });
});
</script>

<template>
    <div class="container" @click.stop.prevent.right>
        <div v-for="group in usedActions" class="group">
            <n-popover
                v-for="action in group"
                :delay="100"
                :duration="0"
                :offset="30"
            >
                <template #header>{{
                    $t(action.replaceAll('_', '-'))
                }}</template>
                <template #trigger>
                    <Action
                        :job="job"
                        class="item"
                        @click="emit('clickedAction', action)"
                        @mouseover="emit('mousehoverAction', action)"
                        @mouseleave="emit('mouseleaveAction', action)"
                        :action="action"
                        :active="isActived(action)"
                        :effect="
                            !status ||
                            (!disable && cachedAllowedList.get(action) == 'ok')
                                ? 'normal'
                                : 'black'
                        "
                        :cp="cachedCraftPointsList.get(action) || undefined"
                    />
                </template>
                {{ $t('desc-' + action.replaceAll('_', '-')) }}
            </n-popover>
        </div>
    </div>
</template>

<style scoped>
.container {
    box-sizing: border-box;
    padding: 0 6px 6px 6px;
}

.group {
    /* border-bottom: 1px dashed var(--el-border-color); */
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
