<!-- 
    This file is part of BestCraft.
    Copyright (C) 2026  Tnze

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
import { Ref, ref } from 'vue';
import {
    ElAlert,
    ElScrollbar,
    ElMessage,
    ElTabs,
    ElTabPane,
} from 'element-plus';
import { Actions, CollectablesShopRefine, Status } from '@/libs/Craft';
import { supported as solverSupported } from '@/libs/Solver';
import { formatDuration } from '@/libs/Utils';
import { useFluent } from 'fluent-vue';
import { Solver } from './DpSolver.vue';
import DpSolver from './DpSolver.vue';
import DfsSolver from './DfsSolver.vue';
import RaphaelSolver from './RaphaelSolver.vue';
import { SequenceSource } from '../types';

const { $t } = useFluent();

const props = defineProps<{
    initStatus: Status;
    currentStatus: Status;
    recipeName: string;
    canHq: boolean;
    collectableShopRefine?: CollectablesShopRefine;
    maxStellarSteadyHand: number;
}>();

const emits = defineEmits<{
    (event: 'solverLoad', solver: Solver): void;
    (
        event: 'solverResult',
        result: Actions[],
        solverName: SequenceSource,
        additional: boolean,
    ): void;
}>();

const activeNames = ref<string>('raphael');

async function runSimpleSolver(
    solverId: SequenceSource,
    solvingRunningState: Ref<boolean>,
    solver: (initStatus: Status) => Promise<Actions[]>,
    fromState: 'initial' | 'current' = 'initial',
) {
    const msg1 = ElMessage({
        showClose: true,
        duration: 0,
        type: 'info',
        message: $t('solving-info', { solverName: $t(solverId) }),
    });
    try {
        solvingRunningState.value = true;
        const startTime = new Date().getTime();
        const initStatus =
            fromState == 'current' ? props.currentStatus : props.initStatus;
        const result = await solver(initStatus);
        const stopTime = new Date().getTime();

        const msgArgs = {
            solveTime: formatDuration(stopTime - startTime),
            solverName: $t(solverId),
        };
        if (result.length > 0) {
            ElMessage({
                type: 'success',
                message: $t('simple-solver-finished', msgArgs),
            });
            emits('solverResult', result, solverId, fromState == 'current');
        } else {
            ElMessage({
                showClose: true,
                duration: 0,
                type: 'error',
                message: $t('simple-solver-finished-no-result', msgArgs),
            });
        }
    } catch (err) {
        ElMessage({
            showClose: true,
            duration: 0,
            type: 'error',
            message: $t('error-with', { err: $t(String(err)) }),
        });
        console.error(err);
    } finally {
        solvingRunningState.value = false;
        msg1.close();
    }
}
</script>

<template>
    <el-scrollbar class="container">
        <template v-if="!solverSupported">
            <el-alert
                type="error"
                :title="$t('web-worker-not-avaliable')"
                show-icon
                :closable="false"
            />
            <br />
        </template>
        <el-tabs v-model="activeNames">
            <el-tab-pane :label="$t('raphael-solver')" name="raphael">
                <RaphaelSolver
                    :init-status="initStatus"
                    :recipe-name="recipeName"
                    @run-simple-solver="runSimpleSolver"
                    :collectable-shop-refine="collectableShopRefine"
                    :maxStellarSteadyHand="maxStellarSteadyHand"
                />
            </el-tab-pane>
            <el-tab-pane :label="$t('dp-solver')" name="dp">
                <DpSolver
                    :init-status="initStatus"
                    :recipe-name="recipeName"
                    @run-simple-solver="runSimpleSolver"
                />
            </el-tab-pane>
            <el-tab-pane
                :label="$t('dfs-solver')"
                name="dfs"
                style="flex: auto"
            >
                <DfsSolver
                    :can-hq="canHq"
                    @run-simple-solver="runSimpleSolver"
                />
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

.el-slider {
    margin-right: 40px;
}

span {
    font-size: var(--el-font-size-base);
    color: var(--el-text-color-regular);
}
</style>

<fluent locale="zh-CN">
web-worker-not-avaliable = 您正在使用的浏览器不支持 Web Worker 功能，无法运行求解器。

do-not-touch = 不推品质
reduce-steps-info = 最少资源方案

solving-info = 「{ $solverName }」求解中，请耐心等待
error-with = 错误：{ $err }

warning = 警告
solver-start = 开始求解
simple-solver-solving = 正在求解中
simple-solver-finished =「{ $solverName }」求解完成({ $solveTime })
simple-solver-finished-no-result = 发动了「{ $solverName }」求解器，没有获得任何结果({ $solveTime })

sum-info = 提示：下面会显示对您没有帮助的碎碎念，使用求解器请直接点击“{solver-start}”按钮。
</fluent>

<fluent locale="zh-TW">
web-worker-not-avaliable = 您正在使用的瀏覽器不支援 Web Worker 功能，無法執行求解器。

do-not-touch = 不推品質
reduce-steps-info = 最少資源方案

solving-info = 「{ $solverName }」求解中，請耐心等待
error-with = 錯誤：{ $err }

warning = 警告
solver-start = 開始求解
simple-solver-solving = 正在求解中
simple-solver-finished =「{ $solverName }」求解完成({ $solveTime })
simple-solver-finished-no-result = 發動了「{ $solverName }」求解器，沒有獲得任何結果({ $solveTime })

sum-info = 提示：下面會顯示對您沒有幫助的碎碎念，使用求解器請直接點選“{solver-start}”按鈕。
</fluent>

<fluent locale="en-US">
web-worker-not-avaliable = Your browser doesn't support Web Worker, which is required to running solvers.

do-not-touch = Do not "touching"
reduce-steps-info = Minimum resource

solving-info = Solving, please wait patiently
error-with = Error: { $err }

warning = Warning
solver-start = Start
simple-solver-solving = Solving
simple-solver-finished = Solver "{ $solverName }" finished. ({ $solveTime })
simple-solver-finished-no-result = "{ $solverName }" is finished. None of result is returned. ({ $solveTime })

sum-info = Warning: The following content contains many fragmented ideas that are not helpful to you. To use the solvers, please click on the '{solver-start}' button directly.
</fluent>
<fluent locale="ja-JP">
</fluent>
