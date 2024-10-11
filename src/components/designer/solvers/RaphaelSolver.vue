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
import { ElSpace, ElDialog, ElButton, ElCheckbox, ElLink, ElTag } from 'element-plus'
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
const useManipulation = ref(false)
const useHeartAndSoul = ref(false)
const useQuickInnovation = ref(false)
const useTrainedEye = ref(true)
const backloadProgress = ref(true)
const unsoundBranchPruning = ref(true)
const adversarial = ref(false)

function runRaphaelSolver() {
    emits('runSimpleSolver', SequenceSource.RaphaelSolver, raphaelSolveIsSolving,
        initStatus => raphael_solve(
            initStatus,
            useManipulation.value,
            useHeartAndSoul.value,
            useQuickInnovation.value,
            useTrainedEye.value,
            backloadProgress.value,
            adversarial.value,
            unsoundBranchPruning.value,
        ),
    )
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
        <el-checkbox v-model="useTrainedEye" :label="$t('enable-action', { action: $t('trained-eye') })" />
        <el-space>
            <el-checkbox v-model="useManipulation" :label="$t('enable-action', { action: $t('manipulation') })" />
            <el-tag v-if="useManipulation" type="warning">{{ $t('need-learn-manipulation') }}</el-tag>
        </el-space>
        <el-space>
            <el-checkbox v-model="useHeartAndSoul" :label="$t('enable-action', { action: $t('heart-and-soul') })" />
            <el-tag v-if="useHeartAndSoul" type="warning">{{ $t('consume-crafters-delineation') }}</el-tag>
        </el-space>
        <el-space>
            <el-checkbox v-model="useQuickInnovation"
                :label="$t('enable-action', { action: $t('quick-innovation') })" />
            <el-tag v-if="useQuickInnovation" type="warning">{{ $t('consume-crafters-delineation') }}</el-tag>
        </el-space>
        <el-space>
            <el-checkbox v-model="backloadProgress" :label="$t('backload-progress')" />
            <el-tag v-if="backloadProgress" type="success">{{ $t('speed-up') }}</el-tag>
            <el-tag v-if="backloadProgress" type="danger">{{ $t('quality-down') }}</el-tag>
            <el-tag v-if="backloadProgress" type="danger">{{ $t('increase-duration') }}</el-tag>
        </el-space>
        <el-space>
            <el-checkbox v-model="unsoundBranchPruning" :label="$t('unsound-branch-pruning')" />
            <el-tag v-if="unsoundBranchPruning" type="success">{{ $t('speed-up') }}</el-tag>
            <el-tag v-if="unsoundBranchPruning" type="danger">{{ $t('increase-duration') }}</el-tag>
        </el-space>
        <el-space>
            <el-checkbox v-model="adversarial" :label="$t('adversarial')" />
            <el-tag v-if="adversarial" type="danger">{{ $t('quality-down') }}</el-tag>
            <el-tag v-if="adversarial" type="danger">{{ $t('increase-duration') }}</el-tag>
        </el-space>
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

enable-action = 使用技能：{ $action }
backload-progress = 后置作业技能（快速求解）
unsound-branch-pruning = 不健全剪枝
adversarial = 确保 100% 可靠（防黑球）

speed-up = 求解速度提高
quality-down = 求解品质下降
increase-duration = 最终步数增加
need-learn-manipulation = 需要学习掌握技能
consume-crafters-delineation = 消耗能工巧匠图纸

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

enable-action = Enable { $action }
backload-progress = Backload progress (Quick solve)
unsound-branch-pruning = Unsound branch pruning
adversarial = Ensure 100% reliability

speed-up = speed up
quality-down = quality decline
increase-duration = increase macro duration
need-learn-manipulation = need manipulation
consume-crafters-delineation = consume crafter's delineation

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