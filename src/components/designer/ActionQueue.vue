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
import { ref, computed, watch, nextTick, toRaw } from 'vue'
import { VueDraggable } from 'vue-draggable-plus'
import { ElIcon } from 'element-plus';
import { Loading, CircleClose } from "@element-plus/icons-vue";

import { Actions, Jobs } from '@/libs/Craft';
import { formatDuration } from "@/libs/Utils";
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
    clearable?: boolean
}>()

const emit = defineEmits<{
    (event: 'update:list', v: Slot[]): void
}>()

const isDragging = ref(false)
const solverAdds = computed(() => props.solverResult?.slice(props.list.length) ?? [])
const dragOptions = computed(() => {
    return {
        animation: 150,
        group: "description",
        disabled: props.disabled || false,
        ghostClass: "ghost"
    }
})
const listBinded = computed({
    get: () => props.list,
    set: (v: Slot[]) => emit('update:list', toRaw(v.filter(x => x != undefined)))
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

function removeAction(id: number) {
    if (!props.disabled) {
        const index = listBinded.value.findIndex(elem => elem.id == id)
        if (index != -1) {
            const list = toRaw(props.list).slice()
            list.splice(index, 1)
            listBinded.value = list
        }
    }
}

function clear() {
    listBinded.value = []
}

function calcEffect(index: number): 'normal' | 'red-cross' | 'black' {
    if (props.errList?.find((v) => v.pos == index) !== undefined)
        return 'black'
    else if (props.list[index].action.endsWith('_fail'))
        return 'red-cross'
    return 'normal'
}

</script>

<template>
    <div class="action-queue-container" @click.stop.prevent.right>
        <VueDraggable v-model="listBinded" v-bind="dragOptions" target=".sort-target" @start="isDragging = true"
            @end="nextTick(() => isDragging = false)">
            <TransitionGroup class="sort-target" type="transition" tag="div"
                :name="!isDragging ? 'flip-list' : undefined">
                <div v-for="(element, index) in listBinded" class="list-group-item" :key="element.id">
                    <Action class="action-icon" :job="job" :action="element.action" :effect="calcEffect(index)" disabled
                        @click.stop.prevent.right="removeAction(element.id)" @click="removeAction(element.id)"
                        :no-hover="noHover" />
                </div>
                <div v-if="!hideSolverResult" v-for="elem in solverAdds" class="list-group-item" :key="elem.id">
                    <Action class="action-icon" :job="job" :action="elem.action" no-hover effect="sunken"
                        :opacity="previewSolver ? 1 : 0.4" disabled />
                </div>
                <div v-if="loadingSolverResult" class="list-group-item following-icon" key="loading-icon">
                    <el-icon class="is-loading following-icon-inner" :size="19.2">
                        <Loading />
                    </el-icon>
                </div>
                <div v-if="clearable && listBinded.length > 0" class="list-group-item following-icon" key="clear-icon"
                    @click="clear">
                    <el-icon class="following-icon-inner" :size="19.2">
                        <CircleClose />
                    </el-icon>
                </div>
            </TransitionGroup>
            <span v-if="!loadingSolverResult && solverResult && solverResult.length > 0" class="solve-time">
                {{ $t('solved-in', { 'time': formatDuration(solveTime) }) }}
            </span>
        </VueDraggable>
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

.following-icon {
    position: relative;
    height: calc(48px * 0.8);
}

.following-icon-inner {
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
