<!-- 
    This file is part of BestCraft.
    Copyright (C) 2024  Tnze

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
import { Ref, ref } from 'vue'
import { ElSpace, ElDialog, ElButton, ElCheckbox, ElLink } from 'element-plus'
import { raphael_solve } from '@/libs/Solver'
import { ChatSquare } from '@element-plus/icons-vue'
import { Actions, Status } from '@/libs/Craft';
import { useFluent } from 'fluent-vue';
import { SequenceSource } from '../types';

const { $t } = useFluent()

const props = defineProps<{
    initStatus: Status,
    recipeName: string,
}>()

const emits = defineEmits<{
    (event: 'runSimpleSolver', solverId: SequenceSource, solvingRunningState: Ref<Boolean>, solver: (initStatus: Status) => Promise<Actions[]>): void
}>()

// UI states
const dialogVisible = ref(false)
const raphaelSolveIsSolving = ref(false)

// Solver options
const backloadProgress = ref(false)
const minimizeSteps = ref(false)

function runRaphaelSolver() {
    emits('runSimpleSolver', SequenceSource.RaphaelSolver, raphaelSolveIsSolving, initStatus => raphael_solve(initStatus, backloadProgress.value, minimizeSteps.value))
}
</script>

<template>
    <el-dialog v-model="dialogVisible" :title="$t('solver-info-title')">
        <i18n path="solver-info" tag="span" class="solver-info">
            <template #origin>
                <el-link href="https://www.raphael-xiv.com/" target="_blank">https://www.raphael-xiv.com/</el-link>
            </template>
            <template #source>
                <el-link href="https://github.com/KonaeAkira/raphael-rs/" target="_blank">
                    https://github.com/KonaeAkira/raphael-rs/
                </el-link>
            </template>
        </i18n>
    </el-dialog>
    <el-space direction="vertical" alignment="normal">
        <el-checkbox v-model="backloadProgress" :label="$t('backload-progress')" />
        <el-checkbox v-model="minimizeSteps" :label="$t('minimize-steps')" />
    </el-space>
    <div style="margin-top: 10px;">
        <el-button @click="runRaphaelSolver" type="primary" :loading="raphaelSolveIsSolving">
            {{ raphaelSolveIsSolving ? $t('simple-solver-solving') : $t('solver-start') }}
        </el-button>
        <el-button :icon="ChatSquare" circle @click="dialogVisible = true" />
    </div>
</template>

<style scoped>
.solver-info {
    white-space: pre-line;
}
</style>

<fluent locale="zh-CN">
solver-start = 开始求解
simple-solver-solving = 正在求解中
error-with = 错误：{ $err }

backload-progress = 后置作业技能
minimize-steps = 最小化步数

solver-info-title = Raphael 求解器
solver-info =
    来源：{ $origin }

    源代码：{ $source }

    许可证：Apache-2.0

    原理：A*搜索 + 帕累托优化 + 动态规划

    特性：
    · 产生最优解，不可能达到比求解器更高的品质
    · 求解时间短（5-20 秒），内存占用合理（300-500 MB）
</fluent>
<fluent locale="en-US">
solver-start = Start
simple-solver-solving = Solving
error-with = Error: { $err }

backload-progress = Backload Progress Actions
minimize-steps = Minimize Steps

solver-info-title = Raphael FFXIV Crafting Solver
solver-info =
    Origin: { $origin }

    Source: { $source }

    License: Apache-2.0

    How it works: A* search + Pareto optimization + Dynamic programming.

    Feature:
    - Produces optimal solutions. Achieving higher quality than the solver is impossible.
    - Short solve time (5-20 seconds) and reasonable memory usage (300-500 MB).
</fluent>