<!-- 
    This file is part of BestCraft.
    Copyright (C) 2025  Tnze

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
import { Ref, reactive, ref } from 'vue';
import {
    ElAlert,
    ElSpace,
    ElDialog,
    ElCard,
    ElButton,
    ElCheckbox,
    ElTable,
    ElTableColumn,
    ElMessage,
    ElSegmented,
} from 'element-plus';
import { create_solver, destroy_solver, reflect_solve } from '@/libs/Solver';
import { ChatSquare } from '@element-plus/icons-vue';
import { Actions, Status } from '@/libs/Craft';
import { useFluent } from 'fluent-vue';
import { SequenceSource } from '../types';
import { formatDuration } from '@/libs/Utils';
import { isTauri } from '@/libs/Consts';

const { $t } = useFluent();

export interface Solver {
    initStatus: Status;
    name: string;
    status: 'solving' | 'prepared' | 'destroying';
}

const props = defineProps<{
    initStatus: Status;
    recipeName: string;
}>();

const emits = defineEmits<{
    (event: 'solverLoad', solver: Solver): void;
    (
        event: 'runSimpleSolver',
        solverId: SequenceSource,
        solvingRunningState: Ref<boolean>,
        solver: (initStatus: Status) => Promise<Actions[]>,
        fromState: 'initial' | 'current',
    ): void;
}>();

const dialogVisible = ref(false);
const useManipulation = ref(false);
const useWasteNot = ref(false);
const useMuscleMemory = ref(false);
const useObserve = ref(true);
const solvers = ref<Solver[]>([]);
const fromState = ref<'initial' | 'current'>('initial');
const fromStateOptions = ['initial', 'current'];

const reflectSolveIsSolving = ref(false);
function runReflectSolver() {
    emits(
        'runSimpleSolver',
        SequenceSource.DPSolver,
        reflectSolveIsSolving,
        initStatus =>
            reflect_solve(
                initStatus,
                useManipulation.value,
                useWasteNot.value ? 8 : 0,
                useObserve.value,
            ),
        fromState.value,
    );
}

const createSolver = async () => {
    const msg1 = ElMessage({
        showClose: true,
        duration: 0,
        type: 'info',
        message: $t('solving-info', { solverName: $t('dp-solver') }),
    });
    let solver = reactive(<Solver>{
        initStatus: {
            ...props.initStatus!,
            quality: 0, // bypass the solver bug that we can't handle the initial quality
        },
        name: props.recipeName,
        status: 'solving',
    });
    try {
        solvers.value.push(solver);
        const startTime = new Date().getTime();
        await create_solver(
            solver.initStatus,
            useMuscleMemory.value,
            useManipulation.value,
            useObserve.value,
        );
        const stopTime = new Date().getTime();
        ElMessage({
            showClose: true,
            type: 'success',
            message: $t('solver-created', {
                solveTime: formatDuration(stopTime - startTime),
            }),
        });
        solver.status = 'prepared';
        emits('solverLoad', solver);
    } catch (err) {
        solvers.value.splice(solvers.value.indexOf(solver), 1);
        ElMessage({
            type: 'error',
            message: $t('error-with', { err: $t(err as string) }),
        });
        console.error(err);
    } finally {
        msg1.close();
    }
};

const destroySolver = async (s: Solver) => {
    try {
        s.status = 'destroying';
        await destroy_solver(s.initStatus);
        solvers.value.splice(solvers.value.indexOf(s), 1);
    } catch (err) {
        ElMessage({
            type: 'error',
            message: `${err}`,
        });
        console.error(err);
    }
};
</script>

<template>
    <el-dialog v-model="dialogVisible" :title="$t('dp-solver-info-title')">
        <i18n path="dp-solver-info" tag="span" class="solver-info">
            <template #usageBlock="{ muscleMemoryMsg }"> </template>
            <template #infoBlock="{ infoMsg }">
                <el-alert
                    type="info"
                    :title="infoMsg"
                    show-icon
                    :closable="false"
                    style="white-space: normal"
                />
            </template>
            <template #calcCard="{ calcMsg }">
                <el-card shadow="never">{{ calcMsg }}</el-card>
            </template>
        </i18n>
    </el-dialog>
    <el-space direction="vertical" alignment="normal">
        <el-segmented v-model="fromState" :options="fromStateOptions">
            <template #default="scope">
                {{ $t('from-' + scope.item) }}
            </template>
        </el-segmented>
        <el-checkbox
            v-model="useMuscleMemory"
            :label="$t('enable-action', { action: $t('muscle-memory') })"
            :disabled="!isTauri"
        />
        <el-checkbox
            v-model="useManipulation"
            :label="$t('enable-action', { action: $t('manipulation') })"
            :disabled="!isTauri"
        />
        <el-checkbox
            v-model="useWasteNot"
            :label="$t('enable-action', { action: $t('waste-not') })"
            :disabled="!isTauri"
        />
        <el-checkbox
            v-model="useObserve"
            :label="$t('enable-action', { action: $t('observe') })"
        />
    </el-space>
    <el-alert
        v-if="useMuscleMemory"
        type="warning"
        :title="$t('muscle-memory-msg')"
        show-icon
        :closable="false"
    />
    <div style="margin-top: 10px">
        <el-button
            v-if="useMuscleMemory"
            type="primary"
            :disabled="initStatus == undefined"
            @click="createSolver"
        >
            {{ $t('create-solver') }}
        </el-button>
        <el-button
            v-else
            @click="runReflectSolver"
            type="primary"
            :loading="reflectSolveIsSolving"
        >
            {{
                reflectSolveIsSolving
                    ? $t('simple-solver-solving')
                    : $t('solver-start')
            }}
        </el-button>
        <el-button :icon="ChatSquare" circle @click="dialogVisible = true" />
    </div>
    <el-table
        v-if="useMuscleMemory"
        :data="solvers"
        :empty-text="$t('dp-solver-empty-text')"
        style="width: 100%"
    >
        <el-table-column>
            <template #default="scope">
                {{ scope.row.name }}
            </template>
        </el-table-column>
        <el-table-column align="right">
            <template #default="scope">
                <el-button
                    size="small"
                    type="danger"
                    @click="destroySolver(scope.row)"
                    :disabled="scope.row.status != 'prepared'"
                    :loading="scope.row.status != 'prepared'"
                >
                    {{ $t('release-solver') }}
                </el-button>
            </template>
        </el-table-column>
    </el-table>
</template>

<style scoped>
.solver-info {
    white-space: pre-line;
}
</style>

<fluent locale="zh-CN">
from-initial = 整体求解
from-current = 追加求解

solver-start = 开始求解
simple-solver-solving = 正在求解中
create-solver = 创建求解器
solver-created = 求解器创建成功({ $solveTime })
release-solver = 释放
error-with = 错误：{ $err }
enable-action = 使用技能：{ $action }

dp-solver-info-title = 基于记忆化搜索的动态规划算法。
dp-solver-info =
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
       具体定义为可以通过“{basic-synthesis}（效率100）”、“{careful-synthesis}（效率180）”或之一完成的状态。

    3. 当算法识别到可以处理的情况后，计算需要留给最后一步的资源，并基于当前的Buff状态运行推动品质的动态规划。
       这时可以看到工作区出现一个正在转圈的Loading标志。几分钟后，求解结果会显示在用户输入的技能后面。

    4. 此时用户可以调整输入，尝试不同的起手，并实时预览求解结果。调整结果一般可以在不到1秒内运算完成。

    .calc-msg =
        70 × 500 × 6 × 11 × 9 × 9 × 5 × 5 × 4 × 4 × 2
        = 149,688,000,000
        = 146,179,687.5 Ki
        ≈ 142,753.6 Mi
        ≈ 139.4 Gi
muscle-memory-msg = 坚信模式的使用方法与其余求解器略有不同，请摸索或阅读下方的说明后使用。
dp-solver-empty-text = 没有已加载的求解器

</fluent>

<fluent locale="en-US">
from-initial = From initial
from-current = From current

solver-start = Start
simple-solver-solving = Solving
create-solver = Create solver
solver-created = Solver successfully created({ $solveTime })
release-solver = Release
error-with = Error: { $err }
enable-action = Enable { $action }

dp-solver-info-title = Dynamic programming algorithm based on Memoization Search.
dp-solver-info =
    This algorithm can be understood as a carefully optimized exhaustive method.

    It exhausts all states, not all actions. Therefore, the exponential time complexity of DFS has been reduced to polynomial time complexity. Make the previously infeasible things feasible.

    However, even if the polynomial time complexity is much better, there are still many state dimensions in the crafting. If all states are considered, the algorithm will occupy a large amount of memory and still take a long time to solve.

    The state dimensions include：
    · Current {durability}
    · Residue {craft-point}
    · Residue {muscle-memory} (0~5)
    · Current Inner Quiet (0~10)
    · Residue {waste-not} (0~8)
    · Residue {manipulation} (0~8)
    · Residue {veneration} (0~4)
    · Residue {innovation} (0~4)
    · Residue {great-strides} (0~3)
    · Touch Combos State (0~3)
    · Is Observed (0~1)
    And most importantly:
    · Current {progress}
    · Current {quality}
    13 dimensions in total。

    To calculate the size of the complete state space, we multiply the sizes of each dimension.
    Estimated at 70 {durability} and 500 {craft-point}: (We will not consider current {progress} and {quality} for now)
    {$calcCard}
    And record these for each state：
    1. Score of current state 
    2. The best action to next state

    It is not difficult to find that without further optimization, running this algorithm will require space in PB and the cost will be too high. (Don't forget that we haven't considered {progress} and {quality} yet)
    Therefore, it is necessary to make the following two necessary compromises:
    1. Regardless of current {quality} and {progress} in state space
    2. Split quality phase and progress phase into two processes and conduct two dynamic programming.

    (The specific solution is difficult to describe in language, and if you cannot understand it, you can refer to the source code of this software.)

    This results in two benefits:
    1. Do not treat progress as a State, but as a Value. And avoid the polynomial to be multiplied by an exaggerated number of thousands.
    2. By splitting a large DP into two small DPs, the quality related states and the progress related states can be separated, reducing spatial complexity.

    But there are also some minor drawbacks:
    1. There is no need to consider both processing and production interweaving ({delicate-synthesis} has been specially treated), but mathematically, it is no longer guaranteed that the exhaustive result is the optimal solution.
    2. The connection between the two dynamic programming only considers the combination of various {durability} and {craft-point}, and the quality stage does not intentionally leave Buff resources for the progress stage.
    3. Difficulty in handling {muscle-memory}: progress needs to be promoted first, quality needs to be promoted, and finally the progress needs to be promoted again for completing the crafting.

    In addition, in order to reduce spatial complexity, only the next optimal action was recorded, without recording the state score.
    Actual testing shows that there was no significant increase of solving time.

    Due to the algorithm's difficulty in handling {muscle-memory}, which is an absolute advantage in the current version. Therefore, this software provides a last-minute solution:

    The user manually specifies the {muscle-memory} starting action. The specific working method of this plan is as follows:

    1. The user sets all the parameters of the recipe and then clicks the {start-solver} button. Create a solver object for the current recipe and equipment attributes.
       The solver object will allocate memory to store the next optimal action for all states.
    2. The user enters a {muscle-memory} and some other actions in the workspace pushs the progress to a state which left only one step away from completing the crafting,
       which is specifically defined as a state that can be completed through one of "{basic-synthesis} (efficiency 1.0)" or "{delicate-synthesis} (efficiency 1.8)".
    3. After the algorithm recognizes the situation that can be processed, it needs to allocate resources for the final step and run the DP to drive quality based on the current Buffs state.
       At this point, you can see a rotating Loading icon in the workspace. After a few minutes, the solving results will be displayed after the actions inputed by the user.
    4. The user can adjust the inputs, try different starting actions, and preview the solving results in real-time. The adjustment results can generally be completed in less than 1 second.

    .calc-msg =
        70 × 500 × 6 × 11 × 9 × 9 × 5 × 5 × 4 × 4 × 2
        = 149,688,000,000
        = 146,179,687.5 Ki
        ≈ 142,753.6 Mi
        ≈ 139.4 Gi
muscle-memory-msg = 
    The usage for {muscle-memory} mode is a little bit different from other solvers. 
    Please discretionary explore, or read the instructions below before using it.
dp-solver-empty-text = None of solver is loaded

</fluent>
