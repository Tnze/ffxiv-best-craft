<script setup lang="ts">
import { h, ref } from 'vue'
import 'element-plus/es/components/message/style/css'
import { ElMessage } from 'element-plus'
import { Actions, Status } from "../../Craft"
import { create_solver, destroy_solver } from '../../Solver'

const props = defineProps<{
    initStatus: Status | undefined,
    status: Status | undefined,
    recipeName: string
}>()
const emits = defineEmits<{
    (event: 'solverLoad', solver: Solver): void
}>()

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
        message: '求解器初始化中，请稍后……',
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
            message: `求解器创建成功(${formatDuration(stop_time - start_time)})`,
        })
        solver.status = 'prepared'
        emits('solverLoad', solver)
    } catch (err) {
        solvers.value.splice(solvers.value.indexOf(solver), 1)
        ElMessage({
            type: 'error',
            message: `错误: ${err}`,
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
            message: `错误: ${err}`,
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
        <el-checkbox v-model="useManipulation" label="掌握(时间&内存x9)" />
        <el-checkbox v-model="useMuscleMemory" label="坚信(内存x2)" />
        <el-button class="list-item" :disabled="initStatus == undefined" @click="createSolver">
            启动求解器
        </el-button>
        <el-button v-for="s in solvers" class="list-item" @click="destroySolver(s)" :disabled="s.status == 'solving'">
            关闭求解器【{{ s.name }}】
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