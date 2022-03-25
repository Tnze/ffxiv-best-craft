<script setup lang="ts">
import { ref } from 'vue'
import Action from './Action.vue'
import { Jobs } from '../../Craft'

const props = defineProps<{
    job: Jobs,
}>()

const emit = defineEmits<{
    (event: 'clickedAction', actionName: string): void
}>()

const actionList = [
    'basic_synthesis',
    'basic_touch',
    'masters_mend',
    'hasty_touch',
    'rapid_synthesis',
    'observe',
    'tricks_of_the_trade',
    'waste_not',
    'veneration',
    'standard_touch',
    'great_strides',
    'innovation',
    'final_appraisal',
    'waste_not_ii',
    'byregot_s_blessing',
    'precise_touch',
    'muscle_memory',
    'careful_synthesis',
    'manipulation',
    'prudent_touch',
    'focused_synthesis',
    'focused_touch',
    'reflect',
    'preparatory_touch',
    'groundwork',
    'delicate_synthesis',
    'intensive_synthesis',
    'trained_eye',
    'advanced_touch',
    'prudent_synthesis',
    'trained_finesse',
    'careful_observation',
    'heart_and_soul',
]

</script>

<template>
    <div class="container">
        <Action
            :job="job"
            v-for="actionName in actionList"
            @click="emit('clickedAction', actionName)"
            :name="actionName"
        />
    </div>
</template>

<style scoped>
.container {
    width: 100%;
    text-align: left;
    margin: 6px;
}
</style>
