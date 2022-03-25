<script setup lang="ts">
import { ref, computed } from 'vue'
import { Actions, Jobs } from '../../Craft';
import Action from './Action.vue'

interface Slot {
    id: number
    action: Actions
}

const props = defineProps<{
    list: Slot[]
    job: Jobs,
}>()


</script>

<template>
    <div class="action-queue-container">
        <transition-group name="list-complete" tag="p">
            <div
                v-for="(item, index) in list"
                @click.stop.prevent.right="list.splice(index, 1)"
                :key="item.id"
                class="list-complete-item"
            >
                <Action
                    :scale="0.7"
                    :job="Jobs.Armorer"
                    :action="item.action"
                    @click.stop.prevent.right="list.splice(index, 1)"
                />
            </div>
        </transition-group>
    </div>
</template>

<style scoped>
.action-queue-container {
    margin: 10px;
}
.list-complete-item {
    transition: all 0.3s ease;
    display: inline-block;
}

.list-complete-enter-from,
.list-complete-leave-to {
    opacity: 0;
    transform: translateY(30px);
}

.list-complete-leave-active {
    position: absolute;
}
</style>