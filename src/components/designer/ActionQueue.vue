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

</script>

<template>
    <div class="action-queue-container">
        <draggable
            item-key="id"
            tag="transition-group"
            :component-data="{
                name: !isDragging ? 'flip-list' : null,
            }"
            :list="list"
            v-bind="dragOptions"
            @start="isDragging = true"
            @end="isDragging = false"
        >
            <template #item="{ element, index }">
                <div class="list-group-item">
                    <Action
                        :scale="0.7"
                        :job="Jobs.Armorer"
                        :action="element.action"
                        :effect="errList?.find((v) => v.pos == index) !== undefined ? 'red-cross' : 'normal'"
                        :disabled="true"
                        @click.stop.prevent.right="list.splice(index, 1)"
                    />
                </div>
            </template>
        </draggable>
    </div>
</template>

<style scoped>
.action-queue-container {
    margin: 7px 10px 3px 10px;
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
</style>