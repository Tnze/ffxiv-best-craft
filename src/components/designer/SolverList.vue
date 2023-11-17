<!-- 
    This file is part of BestCraft.
    Copyright (C) 2023  <name of author>

    BestCraft is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    BestCraft is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
-->

<script setup lang="ts">
import { Ref, reactive, ref, watch } from 'vue'
import { ElAlert, ElCard, ElDialog, ElScrollbar, ElButton, ElCheckbox, ElTable, ElTableColumn, ElLink, ElMessage, ElMessageBox, ElSlider, ElTabs, ElTabPane } from 'element-plus'
import { Actions, Status } from "@/libs/Craft"
import { supported as solverSupported, formatDuration, rika_solve, rika_solve_tnzever, dfs_solve, nq_solve, reflect_solve } from '@/libs/Solver'
import { useFluent } from 'fluent-vue';
import { Solver } from './solvers/DpSolver.vue'
import DpSolver from './solvers/DpSolver.vue'

const { $t } = useFluent()

const props = defineProps<{
    initStatus: Status,
    recipeName: string,
    canHq: boolean
}>()

const emits = defineEmits<{
    (event: 'solverLoad', solver: Solver): void
    (event: 'solverResult', result: Actions[]): void
}>()

const activeNames = ref<string>("dp")
const platform = import.meta.env.VITE_BESTCRAFT_TARGET


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
        console.error(err)
    } finally {
        solvingRunningState.value = false
        msg1.close()
    }
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

// dfs求解器最大深度，设置超过该深度会显示警告
const warningDepth = platform == "tauri" ? 6 : 4

const maxDepth = ref(warningDepth);
const useSpecialist = ref(false);
const doNotTouch = ref(false);
const dfsSolving = ref(false);

watch(() => props.canHq, v => {
    // single way update
    doNotTouch.value = !v
})

function dfsFormatTooltip(value: number): string {
    let str = String(value);
    if (value > warningDepth) str = '⚠️' + str
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
        <template v-if="platform != 'tauri'">
            <el-alert type="warning" :title="$t('please-use-desktop-solvers')" show-icon :closable="false" />
            <br />
        </template>
        <template v-if="!solverSupported">
            <el-alert type="error" :title="$t('web-worker-not-avaliable')" show-icon :closable="false" />
            <br />
        </template>
        <!-- <el-text type="info" class="sum-info">{{ $t('sum-info') }}</el-text> -->
        <el-tabs v-model="activeNames">
            <el-tab-pane :label="$t('dp-solver')" name="dp">
                <el-alert v-if="platform != 'tauri'" type="error" :title="$t('solver-not-avaliable')" show-icon
                    :closable="false" />
                <DpSolver v-else :init-status="initStatus" :recipe-name="recipeName" @run-simple-solver="runSimpleSolver" />
            </el-tab-pane>
            <el-tab-pane :label="$t('dfs-solver')" name="dfs">
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
                                :label="$t('dfs-max-depth')" :disabled="dfsSolving" />
                        </div>
                        <el-alert v-if="maxDepth > warningDepth" type="warning" :title="$t('dfs-too-depth')" show-icon
                            :closable="false" />
                        <el-checkbox v-model="doNotTouch" :label="$t('do-not-touch')" :disabled="dfsSolving" /><br />
                        <el-checkbox v-model="useSpecialist" :label="$t('specialist')" :disabled="dfsSolving" /><br />
                        <el-button type="primary" @click="runDfsSolver" :loading="dfsSolving">
                            {{ dfsSolving ? $t('simple-solver-solving') : $t('solver-start') }}
                        </el-button>
                    </template>
                </i18n>
            </el-tab-pane>
            <el-tab-pane :label="$t('bfs-solver')" name="bfs">
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
            </el-tab-pane>
            <el-tab-pane :label="$t('tnzever-rika-solver')" name="bfs-dp">
                <el-alert v-if="platform != 'tauri'" type="error" :title="$t('solver-not-avaliable')" show-icon
                    :closable="false" />
                <i18n v-else path="tnzever-rika-solver-info" tag="span" class="solver-info">
                    <template #startButton>
                        <el-checkbox v-model="tnzeVerRikaUseManipulation" :label="$t('manipulation')" /><br />
                        <el-checkbox v-model="tnzeVerRikaUseObserve" :label="$t('observe')" /><br />
                        <el-checkbox v-model="tnzeVerRikaReduceSteps" :label="$t('reduce-steps-info')" /><br />
                        <el-button type="primary" @click="runTnzeVerRikaSolver" :loading="tnzeVerRikaIsSolving">
                            {{ tnzeVerRikaIsSolving ? $t('simple-solver-solving') : $t('solver-start') }}
                        </el-button>
                    </template>
                </i18n>
            </el-tab-pane>
        </el-tabs>
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
please-use-desktop-solvers = 网页版求解器性能较差，请考虑使用桌面版。
    网页版求解器依赖于浏览器对 Wasm、ESM WebWorker 的支持，如遇无法使用的情况，请尝试升级您的浏览器。
solver-not-avaliable = 该求解器尚未适配网页版 BestCraft，如需使用请下载桌面版。
web-worker-not-avaliable = 您正在使用的浏览器不支持 Web Worker 功能，无法运行求解器。

dp-solver = 动态规划求解 v2.2
bfs-solver = 广度优先搜索 v1
tnzever-rika-solver = 广度优先搜索 ~ Tnze Impv. ~ v2
dfs-solver = 深度优先搜索 v2

do-not-touch = 不推品质
reduce-steps-info = 最少资源方案

solving-info = 「{ $solverName }」求解中，请耐心等待
error-with = 错误：{ $err }

warning = 警告
solver-start = 开始求解
rika-solver-warning = 当前配方不满足 Rika 求解器的使用条件，是否强制运行？
simple-solver-solving = 正在求解中
simple-solver-finished =「{ $solverName }」求解完成({ $solveTime })
simple-solver-finished-no-result = 发动了「{ $solverName }」求解器，没有获得任何结果({ $solveTime })

sum-info = 提示：下面会显示对您没有帮助的碎碎念，使用求解器请直接点击“{solver-start}”按钮。

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
    更新至v2后拥有多线程加速。

    此求解器通常适合低于玩家10级以上的配方。
    { $startButton }
    .command-line-tool = 命令行工具
dfs-too-depth = 选择的最大深度过大，求解所需时间可能极长
</fluent>

<fluent locale="en-US">
solver-not-avaliable = The Web edition of BestCraft doesn't support this solver. Please download the Desktop edition if needed.
web-worker-not-avaliable = Your browser doesn't support Web Worker, which is required to running solvers.
    
solver-not-avaliable = Developments of web-based BestCraft haven't done yet. Downloading the Desktop version is required to run these solvers.
dp-solver = Dynamic Programing v2.2
bfs-solver = Breadth First Search v1
tnzever-rika-solver = Breadth First Search ~ Tnze Impv. ~ v2
dfs-solver = Depth First Search v2

do-not-touch = Do not "touching"
reduce-steps-info = Minimum resource

solving-info = Solving, please wait patiently
error-with = Error: { $err }

warning = Warning
solver-start = Start
rika-solver-warning = The current recipe does not meet the usage conditions of the Rika's solver. Do you want to force it to run?
simple-solver-solving = Solving
simple-solver-finished = Solver "{ $solverName }" finished. ({ $solveTime })
simple-solver-finished-no-result = "{ $solverName }" is finished. None of result is returned. ({ $solveTime })

sum-info = Warning: The following content contains many fragmented ideas that are not helpful to you. To use the solvers, please click on the '{solver-start}' button directly.

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
dfs-too-depth = The depth is too big. Solving time could be very long.
</fluent>
<fluent locale="ja-JP">
</fluent>
