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
import { ref, computed, watch } from 'vue'
import draggable from 'vuedraggable'
import { ElIcon } from 'element-plus';
import { Loading } from "@element-plus/icons-vue";

import { Actions, Jobs } from '../../Craft';
import { formatDuration } from "../../Solver";
import Action from './Action.vue'

interface Slot {
    id: number
    action: Actions
}

const props = defineProps<{
    list: Slot[]
    solverResult?: Slot[]
    loadingSolverResult?: boolean
    previewSolver?: boolean
    errList?: { pos: number, err: string }[]
    job: Jobs,
    disabled?: boolean
    noHover?: boolean
}>()

const isDragging = ref(false)
const solverAdds = computed(() => props.solverResult?.slice(props.list.length) ?? [])
const dragOptions = computed(() => {
    return {
        animation: 200,
        group: "description",
        disabled: props.disabled || false,
        ghostClass: "ghost"
    }
})

let startTime = 0;
let stopTimer: NodeJS.Timeout | null = null;
const solveTime = ref(0) // in micro second
const hideSolverResult = ref(false)
watch(() => props.loadingSolverResult, (newVal, oldVal) => {
    if (newVal) {
        startTime = new Date().getTime()
        stopTimer = setTimeout(() => { hideSolverResult.value = true }, 500)
    } else if (oldVal) {
        solveTime.value = new Date().getTime() - startTime
        if (stopTimer) clearTimeout(stopTimer)
        hideSolverResult.value = false
    }
})

function removeAction(index: number) {
    if (!props.disabled)
        props.list.splice(index, 1)
}

function calc_effect(index: number): 'normal' | 'red-cross' | 'black' {
    if (props.errList?.find((v) => v.pos == index) !== undefined)
        return 'black'
    else if (props.list[index].action.endsWith('_fail'))
        return 'red-cross'
    return 'normal'
}

</script>

<template>
    <div class="action-queue-container" @click.stop.prevent.right>
        <draggable item-key="id" :component-data="{
            name: !isDragging ? 'flip-list' : null, type: 'transtion-group'
        }" :list="list" v-bind="dragOptions" @start="isDragging = true" @end="isDragging = false">
            <template #item="{ element, index }">
                <div class="list-group-item">
                    <Action class="action-icon" :job="job" :action="element.action" :effect="calc_effect(index)" disabled
                        @click.stop.prevent.right="removeAction(index)" @click="removeAction(index)" :no-hover="noHover" />
                </div>
            </template>
            <template #footer>
                <div v-if="loadingSolverResult" class="list-group-item loading-icon">
                    <el-icon class="is-loading loading-icon-inner" :size="19.2">
                        <Loading />
                    </el-icon>
                </div>
                <div v-if="!hideSolverResult" v-for="elem in solverAdds" class="list-group-item">
                    <Action class="action-icon" :job="job" :action="elem.action" no-hover effect="sunken"
                        :opacity="previewSolver ? 1 : 0.4" disabled />
                </div>
                <span v-if="!loadingSolverResult && solverResult && solverResult.length > 0" class="solve-time">
                    {{ $t('solved-in', { 'time': formatDuration(solveTime) }) }}
                </span>
            </template>
        </draggable>
    </div>
</template>

<style scoped>
.action-queue-container {
    min-height: 43px;
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

.loading-icon {
    position: relative;
    height: calc(48px * 0.8);
}

.loading-icon-inner {
    position: absolute;
    top: 10px;
    left: 5px;
}

.action-icon {
    transform: scale(0.8);
    margin: calc(-48px * 0.1);
}

.solve-time {
    font-size: small;
    user-select: none;
    white-space: nowrap;
    color: var(--el-text-color-secondary);
}
</style>


<fluent locale="zh-CN">
solved-in = 求解完成（{ $time }）
</fluent>

<fluent locale="en-US">
solved-in = Solving finished ({ $time })
</fluent>
