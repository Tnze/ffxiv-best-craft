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
from-initial = 整體求解
from-current = 追加求解

solver-start = 開始求解
simple-solver-solving = 正在求解中
create-solver = 建立求解器
solver-created = 求解器建立成功({ $solveTime })
release-solver = 釋放
error-with = 錯誤：{ $err }
enable-action = 使用技能：{ $action }

dp-solver-info-title = 基於記憶化搜尋的動態規劃演算法。
dp-solver-info =
    可以將該算法理解為一種精心最佳化的窮舉方法。

    它窮舉所有狀態，而不是所有手法。因此將DFS的指數時間複雜度，降低到了多項式時間複雜度。使得原本不可行的窮舉變為可行。

    但是即使降低到了多項式時間複雜度，生產中的狀態維度依然很多。如果考慮所有的狀態，演算法會佔用大量記憶體，且仍然需要較長的時間才能求解完成。

    生產中的狀態包括以下幾個維度：
    · 當前耐久值
    · 剩餘CP
    · 堅信剩餘步數（0~5）
    · 內靜層數（0~10）
    · 儉約剩餘次數（0~8）
    · 掌握剩餘次數（0~8）
    · 崇敬剩餘次數（0~4）
    · 改革剩餘次數（0~4）
    · 闊步剩餘步數（0~3）
    · 加工連擊狀態（0~3）
    · 是否已觀察（0~1）
    以及最重要的：
    · 當前進展
    · 當前品質。
    共13個維度。

    而計算完整狀態空間大小，需要將每個維度的大小相乘。
    以70耐久、500CP估算：（我們先不考慮當前進展和品質）
    {$calcCard}
    而我們需要為每個狀態記錄：
    1. 當前狀態得分
    2. 下一步最優動作

    不難發現，如果不做進一步最佳化，執行該演算法將需要PB級的空間，成本過高。（別忘了我們還沒考慮進展和品質）
    因此有必要做出以下兩個必要妥協：
    1. 狀態空間下不考慮當前品質和當前進展
    2. 將推品質和推進展拆分為兩個過程，進行兩次動態規劃

    （具體的方案難以用語言描述，如果沒能理解可以翻閱本軟體的原始碼。）

    這樣便得到兩個好處：
    1. 不把進展當作State，而是當作Value，多項式中可以不乘進去一個誇張的大約幾千的數。
    2. 將一個大的多項式拆分為兩個小的多項式，推品質相關的狀態和推進展相關的狀態可以分離，降低了空間複雜度。

    但是也有一些小缺點：
    1. 沒有同時考慮加工和製作穿插使用的情況（{delicate-synthesis}做了特殊處理），但數學上無法再保證窮舉得到的結果為最優解。
    2. 兩次動態規劃銜接處只考慮了各種耐久和CP的組合，品質階段不會特意為進展階段留Buff類資源。
    3. 難以處理堅信手法的情況：需要先推進展，再推品質，最後再次推進展完成製作。

    另外，為了降低空間複雜度，只記錄了下一步最優動作，而沒有記錄狀態得分。
    經過實際測試，並沒有明顯的求解耗時增加。

    由於演算法難以處理堅信，而當前版本堅信又是絕對的優勢手法。因此本軟體提供了一個不得已而為之的方案：

    由使用者手動指定堅信起手。該方案具體工作方式如下：

    1. 由使用者設定好配方的所有引數，然後點選{start-solver}按鈕。建立一個針對當前配方和裝備屬性的求解器物件。
       該求解器物件會分配記憶體，用以儲存所有狀態的下一步最優動作。

    2. 使用者在工作區輸入堅信起手，並且需要將進展推動至“差最後一步製作即可完成”的狀態。
       具體定義為可以透過“{basic-synthesis}（效率100）”、“{careful-synthesis}（效率180）”或之一完成的狀態。

    3. 當演算法識別到可以處理的情況後，計算需要留給最後一步的資源，並基於當前的Buff狀態執行推動品質的動態規劃。
       這時可以看到工作區出現一個正在轉圈的Loading標誌。幾分鐘後，求解結果會顯示在使用者輸入的技能後面。

    4. 此時使用者可以調整輸入，嘗試不同的起手，並即時預覽求解結果。調整結果一般可以在不到1秒內運算完成。

    .calc-msg =
        70 × 500 × 6 × 11 × 9 × 9 × 5 × 5 × 4 × 4 × 2
        = 149,688,000,000
        = 146,179,687.5 Ki
        ≈ 142,753.6 Mi
        ≈ 139.4 Gi
muscle-memory-msg = 堅信模式的使用方法與其餘求解器略有不同，請摸索或閱讀下方的說明後使用。
dp-solver-empty-text = 沒有已載入的求解器

</fluent>

<fluent locale="zh-TW">
from-initial = 整體求解
from-current = 追加求解

solver-start = 開始求解
simple-solver-solving = 正在求解中
create-solver = 建立求解器
solver-created = 求解器建立成功({ $solveTime })
release-solver = 釋放
error-with = 錯誤：{ $err }
enable-action = 使用技能：{ $action }

dp-solver-info-title = 基於記憶化搜尋的動態規劃演演算法。
dp-solver-info =
    可以將該算法理解為一種精心最佳化的窮舉方法。

    它窮舉所有狀態，而不是所有手法。因此將DFS的指數時間複雜度，降低到了多項式時間複雜度。使得原本不可行的窮舉變為可行。

    但是即使降低到了多項式時間複雜度，生產中的狀態維度依然很多。如果考慮所有的狀態，演演算法會佔用大量記憶體，且仍然需要較長的時間才能求解完成。

    生產中的狀態包括以下幾個維度：
    · 當前耐久值
    · 剩餘CP
    · 堅信剩餘步數（0~5）
    · 內靜層數（0~10）
    · 儉約剩餘次數（0~8）
    · 掌握剩餘次數（0~8）
    · 崇敬剩餘次數（0~4）
    · 改革剩餘次數（0~4）
    · 闊步剩餘步數（0~3）
    · 加工連擊狀態（0~3）
    · 是否已觀察（0~1）
    以及最重要的：
    · 當前進展
    · 當前品質。
    共13個維度。

    而計算完整狀態空間大小，需要將每個維度的大小相乘。
    以70耐久、500CP估算：（我們先不考慮當前進展和品質）
    {$calcCard}
    而我們需要為每個狀態記錄：
    1. 當前狀態得分
    2. 下一步最優動作

    不難發現，如果不做進一步最佳化，執行該演演算法將需要PB級的空間，成本過高。（別忘了我們還沒考慮進展和品質）
    因此有必要做出以下兩個必要妥協：
    1. 狀態空間下不考慮當前品質和當前進展
    2. 將推品質和推進展拆分為兩個過程，進行兩次動態規劃

    （具體的方案難以用語言描述，如果沒能理解可以翻閱本軟體的原始碼。）

    這樣便得到兩個好處：
    1. 不把進展當作State，而是當作Value，多項式中可以不乘進去一個誇張的大約幾千的數。
    2. 將一個大的多項式拆分為兩個小的多項式，推品質相關的狀態和推進展相關的狀態可以分離，降低了空間複雜度。

    但是也有一些小缺點：
    1. 沒有同時考慮加工和製作穿插使用的情況（{delicate-synthesis}做了特殊處理），但數學上無法再保證窮舉得到的結果為最優解。
    2. 兩次動態規劃銜接處只考慮了各種耐久和CP的組合，品質階段不會特意為進展階段留Buff類資源。
    3. 難以處理堅信手法的情況：需要先推進展，再推品質，最後再次推進展完成製作。

    另外，為了降低空間複雜度，只記錄了下一步最優動作，而沒有記錄狀態得分。
    經過實際測試，並沒有明顯的求解耗時增加。

    由於演演算法難以處理堅信，而當前版本堅信又是絕對的優勢手法。因此本軟體提供了一個不得已而為之的方案：

    由使用者手動指定堅信起手。該方案具體工作方式如下：

    1. 由使用者設定好配方的所有引數，然後點選{start-solver}按鈕。建立一個針對當前配方和裝備屬性的求解器物件。
       該求解器物件會分配記憶體，用以儲存所有狀態的下一步最優動作。

    2. 使用者在工作區輸入堅信起手，並且需要將進展推動至“差最後一步製作即可完成”的狀態。
       具體定義為可以透過“{basic-synthesis}（效率100）”、“{careful-synthesis}（效率180）”或之一完成的狀態。

    3. 當演演算法識別到可以處理的情況後，計算需要留給最後一步的資源，並基於當前的Buff狀態執行推動品質的動態規劃。
       這時可以看到工作區出現一個正在轉圈的Loading標誌。幾分鐘後，求解結果會顯示在使用者輸入的技能後面。

    4. 此時使用者可以調整輸入，嘗試不同的起手，並即時預覽求解結果。調整結果一般可以在不到1秒內運算完成。

    .calc-msg =
        70 × 500 × 6 × 11 × 9 × 9 × 5 × 5 × 4 × 4 × 2
        = 149,688,000,000
        = 146,179,687.5 Ki
        ≈ 142,753.6 Mi
        ≈ 139.4 Gi
muscle-memory-msg = 堅信模式的使用方法與其餘求解器略有不同，請摸索或閱讀下方的說明後使用。
dp-solver-empty-text = 沒有已載入的求解器

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
