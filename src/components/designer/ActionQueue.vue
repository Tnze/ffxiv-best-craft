<script setup lang="ts">
import { ref, computed } from 'vue'
import draggable from 'vuedraggable'
import { Actions, Jobs } from '../../Craft';
import Action from './Action.vue'

interface Slot {
    id: number
    action: Actions
}

const props = defineProps<{
    list: Slot[]
    errList?: { pos: number, err: string }[]
    job: Jobs,
    disabled?: boolean
}>()

const dragOptions = computed(() => {
    return {
        animation: 200,
        group: "description",
        disabled: props.disabled || false,
        ghostClass: "ghost"
    }
})

const isDragging = ref(false)
const onRightClick = (index: number) => {
    if (!props.disabled)
        props.list.splice(index, 1)
}

function calc_effect(index: number): string {
    if (props.errList?.find((v) => v.pos == index) !== undefined)
        return 'black'
    else if (props.list[index].action.endsWith('_fail'))
        return 'red-cross'
    return 'normal'
}

</script>

<template>
    <div class="action-queue-container" @click.stop.prevent.right>
        <draggable item-key="id" tag="transition-group" :component-data="{
            name: !isDragging ? 'flip-list' : null,
        }" :list="list" v-bind="dragOptions" @start="isDragging = true" @end="isDragging = false">
            <template #item="{ element, index }">
                <div class="list-group-item">
                    <Action class="action-icon" :job="job" :action="element.action" :effect="calc_effect(index)"
                        disabled @click.stop.prevent.right="onRightClick(index)" />
                </div>
            </template>
        </draggable>
    </div>
</template>

<style scoped>
.action-queue-container {
    margin: 7px 10px 3px 10px;
    min-height: 42px;
}

.flip-list-move,
.flip-list-enter-active,
.flip-list-leave-active {
    transition: all 0.3s ease;
}

.flip-list-leave-active {
    position: absolute;
}

.flip-list-enter-from,
.flip-list-leave-to {
    opacity: 0;
    transform: translateX(30px);
}

.ghost {
    opacity: 0.5;
}

.list-group-item {
    display: inline-block;
}

.action-icon {
    transform: scale(0.8);
    margin: calc(-48px * 0.1);
}
</style>