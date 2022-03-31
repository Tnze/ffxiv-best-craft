<script setup lang="ts">
import { ref } from 'vue'
import 'element-plus/es/components/message/style/css'
import { ElMessage } from 'element-plus'
import { Actions, Status } from "../../Craft"
import { create_solver, init_solver } from '../../Solver'

const props = defineProps<{
    initStatus: Status | undefined,
}>()

const allowedList = [
    Actions.BasicSynthesis,
    Actions.Observe,
    Actions.WasteNot,
    Actions.Veneration,
    Actions.WasteNotII,
    // Actions.MuscleMemory,
    Actions.CarefulSynthesis,
    Actions.Manipulation,
    Actions.FocusedSynthesis,
    Actions.Groundwork,
    Actions.DelicateSynthesis,
    Actions.IntensiveSynthesis,
    Actions.PrudentSynthesis,
];

const solvers = ref([])

const createSolver = async () => {
    try {
        await create_solver(props.initStatus!)
        const msg1 = ElMessage({
            showClose: true,
            duration: 0,
            type: 'info',
            message: `求解器初始化中……`,
        })
        console.log('求解器初始化成功')
        await init_solver(props.initStatus!, allowedList)
        msg1.close()
        ElMessage({
            showClose: true,
            duration: 0,
            type: 'success',
            message: `求解器创建成功`,
        })
        console.log('求解过程结束')
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
        <el-button
            class="list-item"
            :disabled="initStatus == undefined"
            @click="createSolver"
        >以当前配方和属性开始求解</el-button>
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
}
</style>