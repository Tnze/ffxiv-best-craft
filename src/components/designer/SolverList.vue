<script setup lang="ts">
import { Ref, reactive, ref, watch } from 'vue'
import { ElAlert, ElCard, ElText, ElScrollbar, ElCollapse, ElCollapseItem, ElButton, ElCheckbox, ElTable, ElTableColumn, ElLink, ElMessage, ElMessageBox, ElSlider } from 'element-plus'
import { Actions, Status } from "../../Craft"
import { create_solver, destroy_solver, formatDuration, rika_solve, rika_solve_tnzever, dfs_solve, nq_solve, reflect_solve } from '../../Solver'
import { useFluent } from 'fluent-vue';

const props = defineProps<{
    initStatus: Status,
    recipeName: string,
    canHq: boolean
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

async function runSimpleSolver(solverId: string, solvingRunningState: Ref<Boolean>, solver: (initStatus: Status) => Promise<Actions[]>) {
    const msg1 = ElMessage({
        showClose: true,
        duration: 0,
        type: 'info',
        message: $t('solving-info', { solverName: $t(solverId + '-solver') }),
    })
    try {
        solvingRunningState.value = true
        const startTime = new Date().getTime()
        const result = await solver(props.initStatus)
        const stopTime = new Date().getTime()
        ElMessage({
            type: result.length > 0 ? 'success' : 'error',
            message: $t(result.length > 0 ? 'simple-solver-finished' : 'simple-solver-finished-no-result', {
                solveTime: formatDuration(stopTime - startTime),
                solverName: $t(solverId + '-solver'),
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
        solvingRunningState.value = false
        msg1.close()
    }
}

const reflectSolveIsSolving = ref(false)
async function runReflectSolver() {
    await runSimpleSolver("dp", reflectSolveIsSolving, initStatus => reflect_solve(initStatus, useManipulation.value))
}

const rikaIsSolving = ref(false)

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
    await runSimpleSolver("bfs", rikaIsSolving, rika_solve)
}

const tnzeVerRikaIsSolving = ref(false)
const tnzeVerRikaUseManipulation = ref(true)
const tnzeVerRikaUseObserve = ref(true)
const tnzeVerRikaReduceSteps = ref(true)

async function runTnzeVerRikaSolver() {
    await runSimpleSolver("tnzever-rika", tnzeVerRikaIsSolving,
        initStatus => rika_solve_tnzever(
            initStatus,
            tnzeVerRikaUseManipulation.value,
            8,
            tnzeVerRikaUseObserve.value,
            tnzeVerRikaReduceSteps.value
        )
    )
}

const maxDepth = ref(6);
const useSpecialist = ref(false);
const doNotTouch = ref(false);
const dfsSolving = ref(false);

watch(() => props.canHq, v => {
    // single way update
    doNotTouch.value = !v
})

function dfsFormatTooltip(value: number): string {
    let str = String(value);
    if (value > 6) str = '⚠️' + str
    return str
}

async function runDfsSolver() {
    await runSimpleSolver("dfs", dfsSolving,
        initStatus => {
            const solver = doNotTouch.value ? nq_solve : dfs_solve
            return solver(initStatus, maxDepth.value, useSpecialist.value)
        }
    )
}

</script>

<template>
    <el-scrollbar class="container">
        <el-text type="info" class="sum-info">{{ $t('sum-info') }}</el-text>
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
                        <el-checkbox v-model="doNotTouch" :label="$t('do-not-touch')" /><br />
                        <el-checkbox v-model="useSpecialist" :label="$t('specialist')" /><br />
                        <el-button type="primary" @click="runDfsSolver" :loading="dfsSolving">
                            {{ dfsSolving ? $t('simple-solver-solving') : $t('solver-start') }}
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
                            {{ tnzeVerRikaIsSolving ? $t('simple-solver-solving') : $t('solver-start') }}
                        </el-button>
                    </template>
                </i18n>
            </el-collapse-item>
            <el-collapse-item :title="$t('dp-solver')" name="dp">
                <i18n path="dp-solver-info" tag="span" class="solver-info">
                    <template #usageBlock="{ muscleMemoryMsg }">
                        <el-checkbox v-model="useMuscleMemory" :label="$t('muscle-memory')" /><br />
                        <el-checkbox v-model="useManipulation" :label="$t('manipulation')" /><br />
                        <el-checkbox v-model="useObserve" :label="$t('observe')" /><br />
                        <el-alert v-if="useMuscleMemory" type="warning" :title="muscleMemoryMsg" show-icon
                            :closable="false" />
                        <el-button v-if="useMuscleMemory" type="primary" :disabled="initStatus == undefined"
                            @click="createSolver">
                            {{ $t('start-solver') }}
                        </el-button>
                        <el-button v-else type="primary" @click="runReflectSolver" :loading="reflectSolveIsSolving">
                            {{ reflectSolveIsSolving ? $t('simple-solver-solving') : $t('solver-start') }}
                        </el-button>
                        <el-table v-if="useMuscleMemory" :data="solvers" :empty-text="$t('dp-solver-empty-text')"
                            style="width: 100%">
                            <el-table-column>
                                <template #default="scope">
                                    {{ scope.row.name }}
                                </template>
                            </el-table-column>
                            <el-table-column align="right">
                                <template #default="scope">
                                    <el-button size="small" type="danger" @click="destroySolver(scope.row)"
                                        :disabled="scope.row.status != 'prepared'"
                                        :loading="scope.row.status != 'prepared'">
                                        {{ $t('release-solver') }}
                                    </el-button>
                                </template>
                            </el-table-column>
                        </el-table>
                    </template>
                    <template #infoBlock="{ infoMsg }">
                        <el-alert type="info" :title="infoMsg" show-icon :closable="false" style="white-space: normal;" />
                    </template>
                    <template #calcCard="{ calcMsg }">
                        <el-card shadow="never">{{ calcMsg }}</el-card>
                    </template>
                </i18n>
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
                            {{ rikaIsSolving ? $t('simple-solver-solving') : $t('solver-start') }}
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
.sum-info {
    display: block;
    margin-bottom: 7px;
}

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

do-not-touch = 不推品质
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
simple-solver-solving = 正在求解中
simple-solver-finished =「{ $solverName }」求解完成({ $solveTime })
simple-solver-finished-no-result = 发动了「{ $solverName }」求解器，没有获得任何结果({ $solveTime })

sum-info = 提示：下面会显示对您没有帮助的碎碎念，使用求解器请直接点击“{ solver-start }”按钮。

dp-solver-info =
    基于记忆化搜索的动态规划算法。
    {$infoBlock}
    {$usageBlock}

    可以将该算法理解为一种精心优化的穷举方法。

    它穷举所有状态，而不是所有手法。因此将DFS的指数时间复杂度，降低到了多项式时间复杂度。使得原本不可行的穷举变为可行。

    但是即使降低到了多项式时间复杂度，生产中的状态维度依然很多。如果考虑所有的状态，算法会占用大量内存，且仍然需要较长的时间才能求解完成。

    生产中的状态包括以下几个维度：
    · 当前耐久值
    · 剩余制作力
    · 坚信剩余步数（0~5）
    · 内静层数（0~10）
    · 俭约剩余次数（0~8）
    · 掌握剩余次数（0~8）
    · 崇敬剩余次数（0~4）
    · 改革剩余次数（0~4）
    · 阔步剩余步数（0~3）
    · 加工连击状态（0~3）
    · 是否已观察（0~1）
    以及最重要的：
    · 当前进展
    · 当前品质。
    共13个维度。

    而计算完整状态空间大小，需要将每个维度的大小相乘。
    以70耐久、500制作力估算：（我们先不考虑当前进展和品质）
    {$calcCard}
    而我们需要为每个状态记录：
    1. 当前状态得分
    2. 下一步最优动作

    不难发现，如果不做进一步优化，运行该算法将需要PB级的空间，成本过高。（别忘了我们还没考虑进展和品质）
    因此有必要做出以下两个必要妥协：
    1. 状态空间下不考虑当前品质和当前进展
    2. 将推品质和推进展拆分为两个过程，进行两次动态规划

    （具体的方案难以用语言描述，如果没能理解可以翻阅本软件的源代码。）

    这样便得到两个好处：
    1. 不把进展当作State，而是当作Value，多项式中可以不乘进去一个夸张的大约几千的数。
    2. 将一个大的多项式拆分为两个小的多项式，推品质相关的状态和推进展相关的状态可以分离，降低了空间复杂度。

    但是也有一些小缺点：
    1. 没有同时考虑加工和制作穿插使用的情况（{delicate-synthesis}做了特殊处理），但数学上无法再保证穷举得到的结果为最优解。
    2. 两次动态规划衔接处只考虑了各种耐久和制作力的组合，品质阶段不会特意为进展阶段留Buff类资源。
    3. 难以处理坚信手法的情况：需要先推进展，再推品质，最后再次推进展完成制作。

    另外，为了降低空间复杂度，只记录了下一步最优动作，而没有记录状态得分。
    经过实际测试，并没有明显的求解耗时增加。

    由于算法难以处理坚信，而当前版本坚信又是绝对的优势手法。因此本软件提供了一个不得已而为之的方案：

    由用户手动指定坚信起手。该方案具体工作方式如下：

    1. 由用户设置好配方的所有参数，然后点击{start-solver}按钮。创建一个针对当前配方和装备属性的求解器对象。
       该求解器对象会分配内存，用以储存所有状态的下一步最优动作。

    2. 用户在工作区输入坚信起手，并且需要将进展推动至“差最后一步制作即可完成”的状态。
       具体定义为可以通过“{ basic-synthesis }（效率100）”、“{ delicate-synthesis }（效率180）”或“{ observe }-{ focused-synthesis }（效率200）”之一完成的状态。

    3. 当算法识别到可以处理的情况后，计算需要留给最后一步的资源，并基于当前的Buff状态运行推动品质的动态规划。
       这时可以看到工作区出现一个正在转圈的Loading标志。几分钟后，求解结果会显示在用户输入的技能后面。

    4. 此时用户可以调整输入，尝试不同的起手，并实时预览求解结果。调整结果一般可以在不到1秒内运算完成。

    .muscle-memory-msg =
        坚信模式的使用方法与其余求解器略有不同，请酌情摸索使用，或阅读下方的说明后使用。
    .info-msg =
        旧版本中的实现是普通的动态规划，在初始化时会计算完全部状态。
        目前实现为记忆化搜索，会实时计算当前及依赖的所有状态，二者体验上会有一点小区别。
    .calc-msg =
        70 × 500 × 6 × 11 × 9 × 9 × 5 × 5 × 4 × 4 × 2
        = 149,688,000,000
        = 146,179,687.5 Ki
        ≈ 142,753.6 Mi
        ≈ 139.4 Gi

rika-solver-info =
    由{$rikaRepoLink}，作者同意后移植至本应用。
    注：该算法通过激进的剪枝策略与穷举法求解，
    其中剪枝策略由作者根据经验手工指定，仅适用于特定版本的配方。

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

do-not-touch = Do not "touching"
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
simple-solver-solving = Solving
simple-solver-finished = Solver "{ $solverName }" finished. ({ $solveTime })
simple-solver-finished-no-result = "{ $solverName }" is finished. None of result is returned. ({ $solveTime })

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