<script setup lang="ts">
import { ref } from 'vue'
import 'element-plus/es/components/message/style/css'
import { ElMessage } from 'element-plus'
import { Actions, Status } from "../../Craft"
import { create_solver } from '../../Solver'

const props = defineProps<{
    initStatus: Status | undefined,
}>()

const synthList = [
    Actions.BasicSynthesis,
    Actions.Observe,
    Actions.WasteNot,
    Actions.Veneration,
    Actions.WasteNotII,
    Actions.CarefulSynthesis,
    // Actions.Manipulation,
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
    Actions.MuscleMemory,
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

const solvers = ref([])

const createSolver = async () => {
    const msg1 = ElMessage({
        showClose: true,
        duration: 0,
        type: 'info',
        message: `求解器初始化中……`,
    })
    try {
        const start_time = new Date().getTime();
        await create_solver(props.initStatus!, synthList, touchList)
        const stop_time = new Date().getTime();
        ElMessage({
            showClose: true,
            duration: 0,
            type: 'success',
            message: `求解器创建成功(${ (stop_time - start_time) }ms)`,
        })
        console.log('求解过程结束')
    } catch (err) {
        ElMessage({
            type: 'error',
            message: `错误: ${err}`,
        })
        console.error(err)
    } finally {
        msg1.close()
    }
}
</script>e

<template>
    <el-scrollbar class="container">
        <el-button
            class="list-item"
            :disabled="initStatus == undefined"
            @click="createSolver"
        >以当前属性求解当前配方</el-button>
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