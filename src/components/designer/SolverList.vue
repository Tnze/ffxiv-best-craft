<script setup lang="ts">
import { ref } from 'vue'
import { ElScrollbar, ElCollapse, ElCollapseItem, ElButtonGroup, ElButton, ElPopover, ElCheckbox, ElTable, ElTableColumn, ElLink, ElMessage, ElMessageBox } from 'element-plus'
import { Actions, Status } from "../../Craft"
import { create_solver, destroy_solver, formatDuration, rika_solve, rika_solve_tnzever } from '../../Solver'
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

const tnzeVerRikaIsSolving = ref(false)
const tnzeVerRikaUseManipulation = ref(true)
const tnzeVerRikaUseObserve = ref(true)

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
            message: $t('solver-created', { solveTime: formatDuration(stop_time - start_time) }),
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
    if (props.initStatus.recipe.rlv < 560 || props.initStatus.recipe.difficulty < 70) {
        try {
            await ElMessageBox.confirm(
                $t('rika-solver-warning'),
                $t('warning'),
                { type: 'warning' },
            )
        } catch (_err) {
            return
        }
    }
    try {
        rikaIsSolving.value = true
        const start_time = new Date().getTime()
        const result = await rika_solve(props.initStatus)
        const stop_time = new Date().getTime()
        ElMessage({
            showClose: true,
            duration: 0,
            type: 'success',
            message: $t('rika-solve-finished', { solveTime: formatDuration(stop_time - start_time) }),
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

async function runTnzeVerRikaSolver() {
    try {
        tnzeVerRikaIsSolving.value = true
        const start_time = new Date().getTime()
        const result = await rika_solve_tnzever(
            props.initStatus,
            tnzeVerRikaUseManipulation.value,
            8,
            tnzeVerRikaUseObserve.value,
        )
        const stop_time = new Date().getTime()
        ElMessage({
            showClose: true,
            duration: 0,
            type: 'success',
            message: $t('rika-solve-finished', { solveTime: formatDuration(stop_time - start_time) }),
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
        tnzeVerRikaIsSolving.value = false
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
                            {{ rikaIsSolving ? $t('rika-solving') : $t('rika-solver-start') }}
                        </el-button>
                    </template>
                    <template #rikaSaidLine="{ rikaSaid }">
                        <br /><br />{{ rikaSaid }}
                    </template>
                </i18n>
            </el-collapse-item>
            <el-collapse-item :title="$t('tnzever-rika-solver')" name="tnzever-rika">
                <i18n path="tnzever-rika-solver-info" tag="span">
                    <template #newLine>
                        <br />
                    </template>
                    <template #startButton>
                        <br /> <br />
                        <el-button-group>
                            <el-button type="primary" @click="runTnzeVerRikaSolver" :loading="tnzeVerRikaIsSolving">
                                {{ tnzeVerRikaIsSolving ? $t('rika-solving') : $t('rika-solver-start') }}
                            </el-button>
                            <el-popover placement="bottom" width="300px" trigger="click">
                                <template #reference>
                                    <el-button type="primary" :icon="ArrowRight" :disabled="tnzeVerRikaIsSolving" />
                                </template>
                                <el-checkbox v-model="tnzeVerRikaUseManipulation" :label="$t('manipulation-select-info')" />
                                <br />
                                <el-checkbox v-model="tnzeVerRikaUseObserve" :label="$t('observe-select-info')" />
                            </el-popover>
                        </el-button-group>
                        <br /> <br />
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
dp-solver = 动态规划求解 v2
bfs-solver = 广度优先搜索
tnzever-rika-solver = 广度优先搜索 v2 ~ Tnze Ver. ~

manipulation-select-info = { manipulation }（时间&内存×9）
observe-select-info = { observe }（内存×2）
muscle-memory-select-info = { muscle-memory }（内存×2）
start-solver = 创建求解器
release-solver = 释放

solving-info = 正在创建求解器
solver-created = 求解器创建成功({ $solveTime })
dp-solver-empty-text = 没有已加载的求解器
error-with = 错误：{ $err }

warning = 警告
rika-solver-start = 开始求解
rika-solver-warning = 当前配方不满足 Rika 求解器的使用条件，是否强制运行？
rika-solving = 正在求解中
rika-solve-finished = 求解完成({ $solveTime })

rika-solver-info =
    由{$rikaRepoLink}，作者同意后移植至本应用。
    {$newLine}
    注：该算法通过激进的剪枝策略暴力搜索求解，
    其中剪枝策略由作者根据经验手工指定，适用于特定版本的配方。
    {$startButton}
    {$rikaSaidLine}
    .design-by-rika = Rika设计的算法
    .rika-said =「速度较快但不一定找到最优解，适用范围仅限于560以上70耐久配方」—— Rika

tnzever-rika-solver-info =
    此款求解器是 Rika 广度优先搜索算法的 Tnze 改良款。
    {$startButton}
    保留了 Rika 算法的 Phase 1，将 Phase 2 交由 Tnze 精心重制的动规算法实现。
    该方法既能利用动态规划能计算最优解的优秀特性，也能充分利用 Rika 算法能处理“坚信”起手的优点。
    {$newLine}
    {$newLine}
    注：类似于“广度优先搜索”求解器，该版算法可能也只适用于特定版本的配方。
    如您认为遇到了异常情况，请通过 Gitee 或 Github 等渠道提交 issue 反馈。
</fluent>

<fluent locale="en-US">
dp-solver = Dynamic Programing v2
bfs-solver = Breadth First Search
tnzever-rika-solver = Breadth First Search v2 ~ Tnze Ver. ~

manipulation-select-info = { manipulation }(Time & Memory × 9)
observe-select-info = { observe }(Memory × 2)
muscle-memory-select-info = { muscle-memory }(Memory × 2)
start-solver = Create solver
release-solver = Release

solving-info = Creating solver
solver-created = Solver successfully created({ $solveTime })
dp-solver-empty-text = None of solver is loaded
error-with = Error: { $err }

warning = Warning
rika-solver-warning = The current recipe does not meet the usage conditions of the Rika's solver. Do you want to force it to run?
rika-solving = Solving
rika-solve-finished = Solved({ $solveTime })

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

tnzever-rika-solver-info =
    This solver is a Tnze improved version of the Rika's Breadth First Search.
    {$startButton}
    Retained Phase 1 of Rika algorithm and entrusted Phase 2 to Tnze's carefully crafted Dynamic Programing algorithm implementation.
    This method can not only take advantage of the excellent characteristics of dynamic programming that can calculate the optimal solution, 
    but also take full advantage of the advantage of Rika algorithm that can handle the "conviction" starting.
    {$newLine}
    {$newLine}
    Note: Similar to the "Breadth First Search" solver, this version of the algorithm may only be applicable to specific some of recipes.
    If you believe that you have encountered an abnormal situation, please submit an issue on Gitee or Github.
</fluent>