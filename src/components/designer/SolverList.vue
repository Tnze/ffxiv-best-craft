<script setup lang="ts">
import { reactive, ref } from 'vue'
import { ElText, ElScrollbar, ElCollapse, ElCollapseItem, ElButton, ElCheckbox, ElTable, ElTableColumn, ElLink, ElMessage, ElMessageBox, ElSlider, ElRadioGroup, ElRadio } from 'element-plus'
import { Actions, Status } from "../../Craft"
import { create_solver, destroy_solver, formatDuration, rika_solve, rika_solve_tnzever, dfs_solve } from '../../Solver'
import { useFluent } from 'fluent-vue';

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

const createSolver = async () => {
    const msg1 = ElMessage({
        showClose: true,
        duration: 0,
        type: 'info',
        message: $t('solving-info', { solverName: $t('dp-solver') }),
    })
    let solver = reactive(<Solver>{
        initStatus: {
            ...props.initStatus!,
            quality: 0, // bypass the solver bug that we can't handle the initial quality
        },
        name: props.recipeName,
        status: 'solving'
    })
    try {
        solvers.value.push(solver)
        const startTime = new Date().getTime();
        await create_solver(
            solver.initStatus,
            useMuscleMemory.value,
            useManipulation.value,
            useObserve.value,
        )
        const stopTime = new Date().getTime();
        ElMessage({
            showClose: true,
            type: 'success',
            message: $t('solver-created', { solveTime: formatDuration(stopTime - startTime) }),
        })
        solver.status = 'prepared'
        emits('solverLoad', solver)
    } catch (err) {
        solvers.value.splice(solvers.value.indexOf(solver), 1)
        ElMessage({
            type: 'error',
            message: $t('error-with', { err: $t(err as string) }),
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

const rikaIsSolving = ref(false)

async function runRikaSolver() {
    const msg1 = ElMessage({
        showClose: true,
        duration: 0,
        type: 'info',
        message: $t('solving-info', { solverName: $t('bfs-solver') }),
    })
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
        const startTime = new Date().getTime()
        const result = await rika_solve(props.initStatus)
        const stopTime = new Date().getTime()
        ElMessage({
            type: 'success',
            message: $t('rika-solve-finished', {
                solveTime: formatDuration(stopTime - startTime),
                solverName: $t('bfs-solver'),
            }),
        })
        emits('solverResult', result)
    } catch (err) {
        ElMessage({
            showClose: true,
            duration: 0,
            type: 'error',
            message: $t('error-with', { err: $t(err as string) }),
        })
    } finally {
        rikaIsSolving.value = false
        msg1.close()
    }
}

const tnzeVerRikaIsSolving = ref(false)
const tnzeVerRikaUseManipulation = ref(true)
const tnzeVerRikaUseObserve = ref(true)
const tnzeVerRikaReduceSteps = ref(true)

async function runTnzeVerRikaSolver() {
    const msg1 = ElMessage({
        showClose: true,
        duration: 0,
        type: 'info',
        message: $t('solving-info', { solverName: $t('tnzever-rika-solver') }),
    })
    try {
        tnzeVerRikaIsSolving.value = true
        const startTime = new Date().getTime()
        const result = await rika_solve_tnzever(
            props.initStatus,
            tnzeVerRikaUseManipulation.value,
            8,
            tnzeVerRikaUseObserve.value,
            tnzeVerRikaReduceSteps.value
        )
        const stopTime = new Date().getTime()
        ElMessage({
            type: 'success',
            message: $t('rika-solve-finished', {
                solveTime: formatDuration(stopTime - startTime),
                solverName: $t('tnzever-rika-solver'),
            }),
        })
        emits('solverResult', result)
    } catch (err) {
        ElMessage({
            showClose: true,
            duration: 0,
            type: 'error',
            message: $t('error-with', { err: $t(err as string) }),
        })
    } finally {
        tnzeVerRikaIsSolving.value = false
        msg1.close()
    }
}

const maxDepth = ref(6);
const useSpecialist = ref(false);
const dfsSolving = ref(false);

function dfsFormatTooltip(value: number): string {
    let str = String(value);
    if (value > 6) str = '⚠️' + str
    return str
}

async function runDfsSolver() {
    const msg1 = ElMessage({
        showClose: true,
        duration: 0,
        type: 'info',
        message: $t('solving-info', { solverName: $t('dfs-solver') }),
    })
    try {
        dfsSolving.value = true
        const startTime = new Date().getTime()
        const result = await dfs_solve(props.initStatus, maxDepth.value, useSpecialist.value)
        const stopTime = new Date().getTime()
        ElMessage({
            type: 'success',
            message: $t('rika-solve-finished', {
                solveTime: formatDuration(stopTime - startTime),
                solverName: $t('dfs-solver'),
            }),
        })
        emits('solverResult', result)
    } catch (err) {
        ElMessage({
            showClose: true,
            duration: 0,
            type: 'error',
            message: $t('error-with', { err: $t(err as string) }),
        })
    } finally {
        dfsSolving.value = false
        msg1.close()
    }
}

</script>

<template>
    <el-scrollbar class="container">
        <el-text type="info">{{ $t('sum-info') }}</el-text>
        <el-collapse v-model="activeNames" accordion>
            <el-collapse-item :title="$t('dfs-solver')" name="dfs">
                <i18n path="dfs-solver-info" tag="span" class="solver-info">
                    <template #ffxivCraftingAlgo="{ commandLineTool }">
                        <el-link type="primary" href="https://github.com/Tnze/ffxiv-crafting-algo" target="_blank">
                            {{ commandLineTool }}
                        </el-link>
                    </template>
                    <template #startButton>
                        <div class="argument-block">
                            <span class="slider-label">{{ $t('dfs-max-depth') }}</span>
                            <el-slider v-model="maxDepth" :min="1" :max="10" :format-tooltip="dfsFormatTooltip"
                                :label="$t('dfs-max-depth')" />
                        </div>
                        <el-checkbox v-model="useSpecialist" :label="$t('specialist')" /><br />
                        <el-button type="primary" @click="runDfsSolver" :loading="dfsSolving">
                            {{ dfsSolving ? $t('rika-solving') : $t('solver-start') }}
                        </el-button>
                    </template>
                </i18n>
            </el-collapse-item>
            <el-collapse-item :title="$t('tnzever-rika-solver')" name="bfs-dp">
                <i18n path="tnzever-rika-solver-info" tag="span" class="solver-info">
                    <template #startButton>
                        <el-checkbox v-model="tnzeVerRikaUseManipulation" :label="$t('manipulation')" /><br />
                        <el-checkbox v-model="tnzeVerRikaUseObserve" :label="$t('observe')" /><br />
                        <el-checkbox v-model="tnzeVerRikaReduceSteps" :label="$t('reduce-steps-info')" /><br />
                        <el-button type="primary" @click="runTnzeVerRikaSolver" :loading="tnzeVerRikaIsSolving">
                            {{ tnzeVerRikaIsSolving ? $t('rika-solving') : $t('solver-start') }}
                        </el-button>
                    </template>
                </i18n>
            </el-collapse-item>
            <el-collapse-item :title="$t('dp-solver')" name="dp">
                <el-radio-group v-model="useMuscleMemory">
                    <el-radio :label="false">{{ $t('reflect') }}</el-radio>
                    <el-radio :label="true">{{ $t('muscle-memory') }}</el-radio>
                </el-radio-group><br />
                <el-checkbox v-model="useManipulation" :label="$t('manipulation')" /><br />
                <el-checkbox v-model="useObserve" :label="$t('observe')" /><br />
                <!-- <el-checkbox v-model="useMuscleMemory" :label="$t('muscle-memory')" /><br /> -->
                <el-button type="primary" :disabled="initStatus == undefined" @click="createSolver">
                    {{ $t('start-solver') }}
                </el-button>
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
            <el-collapse-item :title="$t('bfs-solver')" name="bfs">
                <i18n path="rika-solver-info" tag="span" class="solver-info">
                    <template #rikaRepoLink="{ designByRika }">
                        <el-link type="primary" href="https://github.com/RikaKagurasaka/xiv_craft_solver" target="_blank">
                            {{ designByRika }}
                        </el-link>
                    </template>
                    <template #startButton>
                        <el-button type="primary" @click="runRikaSolver" :loading="rikaIsSolving">
                            {{ rikaIsSolving ? $t('rika-solving') : $t('solver-start') }}
                        </el-button>
                    </template>
                    <template #rikaSaidLine="{ rikaSaid }">
                        {{ rikaSaid }}
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

.solver-info {
    white-space: pre-line;
}

.el-button {
    margin-top: 7px;
}

.argument-block {
    display: flex;
    align-items: center;
}

.argument-block .slider-label {
    font-size: 14px;
    color: var(--el-text-color-secondary);
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    margin-bottom: 0;
}

.argument-block .slider-label+.el-slider {
    flex: 0 0 70%;
}

.el-slider {
    margin-right: 40px;
}
</style>

<fluent locale="zh-CN">
dp-solver = 动态规划求解 v2.1
bfs-solver = 广度优先搜索 v1
tnzever-rika-solver = 广度优先搜索 ~ Tnze Impv. ~ v2
dfs-solver = 深度优先搜索 v2 力大砖飞版

reduce-steps-info = 最少资源方案
start-solver = 创建求解器
release-solver = 释放

solving-info = 「{ $solverName }」求解中，请耐心等待
solver-created = 求解器创建成功({ $solveTime })
dp-solver-empty-text = 没有已加载的求解器
error-with = 错误：{ $err }

warning = 警告
solver-start = 开始求解
rika-solver-warning = 当前配方不满足 Rika 求解器的使用条件，是否强制运行？
rika-solving = 正在求解中
rika-solve-finished =「{ $solverName }」求解完成({ $solveTime })

sum-info = 警告：以下内容包含许多对您没有帮助的碎碎念，使用求解器请直接点击“{ solver-start }”按钮。

rika-solver-info =
    由{$rikaRepoLink}，作者同意后移植至本应用。
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

    注：类似于“广度优先搜索”求解器，该版算法可能也只适用于特定版本的配方。
    如您认为遇到了异常情况，请通过 Gitee 或 Github 等渠道提交 issue 反馈。

dfs-max-depth = 最大深度
dfs-solver-info =
    此款求解器源于 Tnze 早期开发的一款{ $ffxivCraftingAlgo }，最初用于搜索最短的巨匠手法。

    该算法采用朴素的暴力搜索，所需时间随搜索深度限制指数级增大。推荐将搜索深度限制为6。
    更新至v2后采用拥有多线程加速。

    此求解器通常适合低于玩家10级以上的配方。
    { $startButton }
    .command-line-tool = 命令行工具
</fluent>

<fluent locale="en-US">
dp-solver = Dynamic Programing v2.1
bfs-solver = Breadth First Search v1
tnzever-rika-solver = Breadth First Search ~ Tnze Impv. ~ v2
dfs-solver = Depth First Search v2

reduce-steps-info = Minimum resource
start-solver = Create solver
release-solver = Release

solving-info = Solving, please wait patiently
solver-created = Solver successfully created({ $solveTime })
dp-solver-empty-text = None of solver is loaded
error-with = Error: { $err }

warning = Warning
solver-start = Start
rika-solver-warning = The current recipe does not meet the usage conditions of the Rika's solver. Do you want to force it to run?
rika-solving = Solving
rika-solve-finished = Solver "{ $solverName }" finished. ({ $solveTime })

sum-info = Warning: The following content contains many fragmented ideas that are not helpful to you. To use the solvers, please click on the '{ solver-start }' button directly.

rika-solver-info =
    {$rikaRepoLink}. Transplant with the consent of the author.
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
    
    Note: Similar to the "Breadth First Search" solver, this version of the algorithm may only be applicable to specific some of recipes.
    If you believe that you have encountered an abnormal situation, please submit an issue on Gitee or Github.

dfs-max-depth = Depth
dfs-solver-info =
    This solver is based on an early development of the { $ffxivCraftingAlgo } by Tnze, originally usedto search for the shortest steps to create the 巨匠药水.

    The algorithm adopts naive search, which increases exponentially in time with the depth of the searching. 
    It is recommended to limit the search depth to 6. 
    After updating to v2, adopt multi threaded acceleration.

    This solver is usually suitable for recipes that are 10-level lower than the player or above.
    { $startButton }
    .command-line-tool = Command line tool
</fluent>