<script setup lang="ts">
import { ref } from 'vue'
import Action from './Action.vue'
import { Jobs, Actions } from '../../Craft'

const props = defineProps<{
    job: Jobs,
}>()

const emit = defineEmits<{
    (event: 'clickedAction', action: Actions): void
    (event: 'mouseoverAction', action: Actions): void
    (event: 'mouseleaveAction', action: Actions): void
}>()

</script>

<template>
    <div class="container">
        <Action
            :job="job"
            v-for="action in Actions"
            @click="emit('clickedAction', action)"
            @mouseover="emit('mouseoverAction', action)"
            @mouseleave="emit('mouseleaveAction', action)"
            :action="action"
        />
    </div>
</template>

<style scoped>
.container {
    box-sizing: border-box;
    padding: 6px;
}
</style>
