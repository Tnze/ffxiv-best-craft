<script setup lang="ts">
import { ref } from 'vue'
import { ElScrollbar, ElCollapse, ElCollapseItem, ElButtonGroup, ElButton, ElPopover, ElCheckbox, ElTable, ElTableColumn, ElLink, ElMessage } from 'element-plus'
import { Actions, Status } from "../../Craft"
import { create_solver, destroy_solver, formatDuration, rika_solve } from '../../Solver'
import { useFluent } from 'fluent-vue';
import { ArrowRight } from '@element-plus/icons-vue';

const props = defineProps<{
    initStatus: Status,
    recipeName: string
}>()
const emits = defineEmits<{
    (event: 'solverLoad', solver: Solver): void
    (event: 'solverResult', result: Actions[]): void
}>()
const { $t } = useFluent()

interface Solver {
    initStatus: Status,
    name: string,
    status: 'solving' | 'prepared' | 'destroying'
}
const solvers = ref<Solver[]>([])
const useManipulation = ref(false)
const useMuscleMemory = ref(false)
const useObserve = ref(true)
const activeNames = ref<string[]>([])
const rikaIsSolving = ref(false)

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
            useManipulation.value,
            useObserve.value,
        )
        const stop_time = new Date().getTime();
        ElMessage({
            showClose: true,
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

const destroySolver = async (s: Solver) => {
    try {
        s.status = 'destroying';
        await destroy_solver(s.initStatus)
        solvers.value.splice(solvers.value.indexOf(s), 1)
    } catch (err) {
        ElMessage({
            type: 'error',
            message: `${err}`,
        })
        console.error(err)
    }
}

async function runRikaSolver() {
    try {
        rikaIsSolving.value = true
        const start_time = new Date().getTime()
        const result = await rika_solve(props.initStatus)
        const stop_time = new Date().getTime()
        ElMessage({
            showClose: true,
            duration: 0,
            type: 'success',
            message: $t('solve-finished', { solveTime: formatDuration(stop_time - start_time) }),
        })
        emits('solverResult', result)
    } catch (err) {
        ElMessage({
            showClose: true,
            duration: 0,
            type: 'error',
            message: $t('error-with', { err: err as string }),
        })
    } finally {
        rikaIsSolving.value = false
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
                        <el-checkbox v-model="useObserve" :label="$t('observe-select-info')" />
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
                                :disabled="scope.row.status != 'prepared'" :loading="scope.row.status != 'prepared'">
                                {{ $t('release-solver') }}
                            </el-button>
                        </template>
                    </el-table-column>
                </el-table>
            </el-collapse-item>
            <el-collapse-item :title="$t('bfs-solver')" name="dfs">
                <i18n path="rika-solver-info" tag="span">
                    <template #newLine>
                        <br />
                    </template>
                    <template #rikaRepoLink="{ designByRika }">
                        <el-link type="primary" href="https://github.com/RikaKagurasaka/xiv_craft_solver" target="_blank">
                            {{ designByRika }}
                        </el-link>
                    </template>
                    <template #startButton>
                        <br /> <br />
                        <el-button type="primary" @click="runRikaSolver" :loading="rikaIsSolving">
                            {{ $t('start-solver') }}
                        </el-button>
                    </template>
                    <template #rikaSaidLine="{ rikaSaid }">
                        <br /><br />{{ rikaSaid }}
                    </template>
                </i18n>
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
bfs-solver = 广度优先搜索

manipulation-select-info = { manipulation }（时间&内存×9）
observe-select-info = { observe }（内存×2）
muscle-memory-select-info = { muscle-memory }（内存×2）
start-solver = 启动求解器
release-solver = 释放

solving-info = 正在创建求解器
solve-finished = 求解器创建成功({ $solveTime })
dp-solver-empty-text = 没有已加载的求解器
error-with = 错误：{ $err }

rika-solver-info =
    由{$rikaRepoLink}，作者同意后移植至本应用。
    {$newLine}
    注：该算法通过激进的剪枝策略暴力搜索求解，
    其中剪枝策略由作者根据经验手工指定，适用于特定版本的配方。
    {$startButton}
    {$rikaSaidLine}
    .design-by-rika = Rika设计的算法
    .rika-said =「速度较快但不一定找到最优解，适用范围仅限于560以上70耐久配方」—— Rika
</fluent>

<fluent locale="en-US">
dp-solver = Dynamic Programing
bfs-solver = Breadth First Search

manipulation-select-info = { manipulation }(Time & Memory × 9)
observe-select-info = { observe }(Memory × 2)
muscle-memory-select-info = { muscle-memory }(Memory × 2)
start-solver = Create solver
release-solver = Release

solving-info = Creating solver
solve-finished = Solver successfully created({ $solveTime })
dp-solver-empty-text = None of solver is loaded
error-with = Error: { $err }

rika-solver-info =
    {$rikaRepoLink}. Transplant with the consent of the author.
    {$newLine}
    P.S: The algorithm uses radical pruning strategy for solving,
    The pruning strategy is manually specified by the author based on experience,
    and is applicable to specific versions of the recipe.
    {$startButton}
    {$rikaSaidLine}
    .design-by-rika = Designed by Rika
    .rika-said =「速度较快但不一定找到最优解，适用范围仅限于560以上70耐久配方」—— Rika
</fluent>