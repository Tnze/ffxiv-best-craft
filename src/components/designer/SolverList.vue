<script setup lang="ts">
import { h, ref } from 'vue'
import 'element-plus/es/components/message/style/css'
import { ElMessage, ElProgress } from 'element-plus'
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

const synthList = [
    // Actions.MuscleMemory,
    Actions.BasicSynthesis,
    Actions.Observe,
    Actions.WasteNot,
    Actions.Veneration,
    Actions.WasteNotII,
    Actions.CarefulSynthesis,
    Actions.Groundwork,
    Actions.DelicateSynthesis,
    Actions.IntensiveSynthesis,
    Actions.PrudentSynthesis,
];

const touchList = [
    Actions.BasicSynthesis,
    Actions.BasicTouch,
    Actions.MastersMend,
    Actions.WasteNot,
    Actions.Veneration,
    Actions.StandardTouch,
    Actions.GreatStrides,
    Actions.Innovation,
    Actions.WasteNotII,
    Actions.ByregotsBlessing,
    Actions.CarefulSynthesis,
    // Actions.Manipulation,
    Actions.PrudentTouch,
    Actions.PreparatoryTouch,
    Actions.Groundwork,
    Actions.DelicateSynthesis,
    Actions.AdvancedTouch,
    Actions.PrudentSynthesis,
    Actions.TrainedFinesse,
]

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
            synthList,
            useManipulation.value ? touchList.concat(Actions.Manipulation) : touchList,
            useMuscleMemory.value,
            useManipulation.value
        )
        const stop_time = new Date().getTime();
        ElMessage({
            showClose: true,
            duration: 0,
            type: 'success',
            message: `求解器创建成功(${(stop_time - start_time)}ms)`,
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

</script>e

<template>
    <el-scrollbar class="container">
        <el-checkbox v-model="useManipulation" label="掌握(时间x9)" />
        <el-checkbox v-model="useMuscleMemory" label="坚信(时间x3)" />
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