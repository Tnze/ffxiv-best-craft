<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { Status } from "../../Craft"
import { create_solver, destroy_solver } from '../../Solver'
import { useFluent } from 'fluent-vue';

const props = defineProps<{
    initStatus: Status | undefined,
    status: Status | undefined,
    recipeName: string
}>()
const emits = defineEmits<{
    (event: 'solverLoad', solver: Solver): void
}>()
const { $t } = useFluent()

interface Solver {
    initStatus: Status,
    name: string,
    status: 'solving' | 'prepared'
}
const solvers = ref<Solver[]>([])
const useManipulation = ref(false)
const useMuscleMemory = ref(false)

const createSolver = async () => {
    const msg1 = ElMessage({
        showClose: true,
        duration: 0,
        type: 'info',
        message: $t('solving-info'),
    })
    let solver: Solver = {
        initStatus: props.initStatus!,
        name: props.recipeName,
        status: 'solving'
    }
    try {
        solvers.value.push(solver)
        const start_time = new Date().getTime();
        await create_solver(
            solver.initStatus,
            useMuscleMemory.value,
            useManipulation.value
        )
        const stop_time = new Date().getTime();
        ElMessage({
            showClose: true,
            duration: 0,
            type: 'success',
            message: $t('solve-finished', { solveTime: formatDuration(stop_time - start_time) }),
        })
        solver.status = 'prepared'
        emits('solverLoad', solver)
    } catch (err) {
        solvers.value.splice(solvers.value.indexOf(solver), 1)
        ElMessage({
            type: 'error',
            message: $t('error-with', { err: err as string }),
        })
        console.error(err)
    } finally {
        msg1.close()
    }
}

const destroySolver = (s: Solver) => {
    try {
        solvers.value.splice(solvers.value.indexOf(s), 1)
        destroy_solver(s.initStatus)
    } catch (err) {
        ElMessage({
            type: 'error',
            message: `${err}`,
        })
        console.error(err)
    }
}

function formatDuration(u: number): string {
    if (u < 1000) {
        return u + "ms"
    } else {
        const h = Math.floor(u / 1000 / 3600)
        const m = Math.floor(u / 1000 / 60) - h * 60
        const s = (u / 1000 - h * 3600 - m * 60).toFixed(3)
        return (h > 0 ? h + 'h' : '') + (m > 0 ? m + 'm' : '') + (s + 's')
    }
}

</script>

<template>
    <el-scrollbar class="container">
        <el-checkbox v-model="useManipulation" :label="$t('manipulation-select-info')" />
        <el-checkbox v-model="useMuscleMemory" :label="$t('muscle-memory-select-info')" />
        <el-button class="list-item" :disabled="initStatus == undefined" @click="createSolver">
            {{ $t('start-solver') }}
        </el-button>
        <el-button v-for="s in solvers" class="list-item" @click="destroySolver(s)" :disabled="s.status == 'solving'">
            {{ $t('close-solver', { solver: s.name }) }}
        </el-button>
    </el-scrollbar>
</template>

<style scoped>
.container {
    display: flex;
    flex-direction: column;
}

.list-item {
    height: 50px;
    width: 100%;
    margin: 0px;
}
</style>

<fluent locale="zh-CN">
manipulation-select-info = { manipulation }(时间&内存×9)
muscle-memory-select-info = { muscle-memory }(内存×2)
start-solver = 启动求解器
close-solver = 关闭求解器【{ $solver }】

solving-info = 求解器计算中，可能需要消耗大量内存，请稍等……
solve-finished = 求解器准备已完成({ $solveTime })
error-with = 错误：{ $err }
</fluent>

<fluent locale="en">
manipulation-select-info = { manipulation }(Time & Memory × 9)
muscle-memory-select-info = { muscle-memory }(Memory × 2)
start-solver = Create solver
close-solver = Release solver [{ $solver }]

solving-info = Solving could occupy lots of memory. Please wait...
solve-finished = Solve finished({ $solveTime })
error-with = Error: { $err }
</fluent>