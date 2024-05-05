<script setup lang="ts">
import { rand_simulation } from "@/libs/Analyzer"
import { Actions, Status } from "@/libs/Craft";


const props = defineProps<{
    initStatus: Status,
    actions: Actions[],
}>()

async function runSimulatios() {
    let result = await rand_simulation(props.initStatus, props.actions, 10000)
    console.table(result)
}
</script>

<template>
    <div>
        <button @click="runSimulatios">Run simulations</button>
    </div>
</template>