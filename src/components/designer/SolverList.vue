<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { Status } from "../../Craft"
import { create_solver, destroy_solver } from '../../Solver'
import { useFluent } from 'fluent-vue';
import { ArrowRight } from '@element-plus/icons-vue';

const props = defineProps<{
    initStatus: Status,
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
const activeNames = ref<string[]>([])

const createSolver = async () => {
    const msg1 = ElMessage({
        showClose: true,
        duration: 0,
        type: 'info',
        message: $t('solving-info'),
    })
    let solver: Solver = {
        initStatus: {
            ...props.initStatus!,
            quality: 0, // bypass the solver bug that we can't handle the initial quality
        },
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
        <el-collapse v-model="activeNames">
            <el-collapse-item :title="$t('dp-solver')" name="dp">
                <el-button-group>
                    <el-button type="primary" :disabled="initStatus == undefined" @click="createSolver">
                        {{ $t('start-solver') }}
                    </el-button>
                    <el-popover placement="bottom" width="300px" trigger="click">
                        <template #reference>
                            <el-button type="primary" :icon="ArrowRight" />
                        </template>
                        <el-checkbox v-model="useManipulation" :label="$t('manipulation-select-info')" />
                        <br />
                        <el-checkbox v-model="useMuscleMemory" :label="$t('muscle-memory-select-info')" />
                    </el-popover>
                </el-button-group>
                <el-table :data="solvers" :empty-text="$t('dp-solver-empty-text')" style="width: 100%">
                    <el-table-column>
                        <template #default="scope">
                            {{ scope.row.name }}
                        </template>
                    </el-table-column>
                    <el-table-column align="right">
                        <template #default="scope">
                            <el-button size="small" type="danger" @click="destroySolver(scope.row)"
                                :disabled="scope.row.status == 'solving'" :loading="scope.row.status == 'solving'">
                                {{ $t('release-solver') }}
                            </el-button>
                        </template>
                    </el-table-column>
                </el-table>
            </el-collapse-item>
            <el-collapse-item :title="$t('dfs-solver')" name="dfs" disabled>
            </el-collapse-item>
        </el-collapse>
    </el-scrollbar>
</template>

<style scoped>
.container {
    display: flex;
    flex-direction: column;
}
</style>

<fluent locale="zh-CN">
dp-solver = 动态规划求解
dfs-solver = 深度优先搜索（敬请期待）

manipulation-select-info = { manipulation }（时间&内存×9）
muscle-memory-select-info = { muscle-memory }（内存×2）
start-solver = 启动求解器
release-solver = 释放

solving-info = 求解器计算中，可能需要消耗大量内存，请稍等……
solve-finished = 求解器准备已完成({ $solveTime })
dp-solver-empty-text = 无求解器已加载
error-with = 错误：{ $err }
</fluent>

<fluent locale="en">
dp-solver = Dynamic Programing
dfs-solver = Depth First Search (Coming soon)

manipulation-select-info = { manipulation }(Time & Memory × 9)
muscle-memory-select-info = { muscle-memory }(Memory × 2)
start-solver = Create solver
release-solver = Release

solving-info = Solving could occupy lots of memory. Please wait...
solve-finished = Solve finished({ $solveTime })
dp-solver-empty-text = No solver is loaded
error-with = Error: { $err }
</fluent>