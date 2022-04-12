<script setup lang="ts">
import { ref, watch, watchEffect } from 'vue'
import Action from './Action.vue'
import { Jobs, Actions, Status, allowedList } from '../../Craft'
import { computed } from '@vue/reactivity';

const props = defineProps<{
    job: Jobs,
    status?: Status
}>()

const emit = defineEmits<{
    (event: 'clickedAction', action: Actions): void
    (event: 'mouseoverAction', action: Actions): void
    (event: 'mouseleaveAction', action: Actions): void
}>()

const actions = Object.values(Actions)

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
        case Actions.PrudentTouch:
            return props.status.condition == 'good' || props.status.condition == 'excellent'
        case Actions.ByregotsBlessing:
            return props.status.buffs.inner_quiet > 0
        case Actions.StandardTouch:
            return props.status.buffs.standard_touch_prepared > 0
        case Actions.AdvancedTouch:
            return props.status.buffs.advanced_touch_prepared > 0
        case Actions.FocusedSynthesis:
        case Actions.FocusedTouch:
            return props.status.buffs.observed > 0
    }
    return false;
}

const cachedAllowedList = ref<string[]>([])

watchEffect(() => {
    if (props.status == undefined) {
        cachedAllowedList.value = []
        return
    }
    allowedList(props.status, actions).then((result) => {
        cachedAllowedList.value = result
    })
})

</script>

<template>
    <div class="container">
        <Action :job="job" class="item" v-for="action, i in actions" @click="emit('clickedAction', action)"
            @mouseover="emit('mouseoverAction', action)" @mouseleave="emit('mouseleaveAction', action)" :action="action"
            :active="isActived(action)" :effect="cachedAllowedList.at(i) == 'ok' ? 'normal' : 'black'" />
    </div>
</template>

<style scoped>
.container {
    box-sizing: border-box;
    padding: 3px 10px;
}

.item {
    transform: scale(0.8);
    margin: calc(-48px * 0.1);
}
</style>
