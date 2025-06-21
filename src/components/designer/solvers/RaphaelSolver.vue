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
import { computed, Ref, ref } from 'vue';
import {
    ElSpace,
    ElDialog,
    ElButton,
    ElCheckbox,
    ElLink,
    ElTag,
    ElText,
    ElInputNumber,
    ElSegmented,
} from 'element-plus';
import { raphael_solve } from '@/libs/Solver';
import { ChatSquare } from '@element-plus/icons-vue';
import { Actions, CollectablesShopRefine, Status } from '@/libs/Craft';
import { useFluent } from 'fluent-vue';
import { SequenceSource } from '../types';

const { $t } = useFluent();

const props = defineProps<{
    initStatus: Status;
    recipeName: string;
    collectableShopRefine?: CollectablesShopRefine;
}>();

const emits = defineEmits<{
    (
        event: 'runSimpleSolver',
        solverId: SequenceSource,
        solvingRunningState: Ref<boolean>,
        solver: (initStatus: Status) => Promise<Actions[]>,
    ): void;
}>();

// UI states
const dialogVisible = ref(false);
const raphaelSolveIsSolving = ref(false);
const solverTarget = ref<TargetQuality>('full');
const solverTargetSegmented = computed({
    get: () => {
        const v = solverTarget.value;
        return typeof v == 'number' ? 'custom' : v;
    },
    set: v => {
        if (v == 'custom')
            console.warn(
                "cannot set solverTarget to 'custom' by clicking segmented controller",
            );
        else solverTarget.value = v;
    },
});

type TargetQuality = 'full' | '1st' | '2nd' | '3rd' | number;
type TargetQualityOption = {
    label: string;
    value: TargetQuality | 'custom';
    disabled?: boolean;
};
const targetQualityOptions = computed(() => {
    const options: TargetQualityOption[] = [];
    options.push({ label: 'custom', value: 'custom', disabled: true });
    const v = props.collectableShopRefine;
    if (v != undefined) {
        if (v.low_collectability > 0)
            options.push({ label: 'first-stage', value: '1st' });
        if (v.mid_collectability > 0)
            options.push({ label: 'second-stage', value: '2nd' });
        if (v.high_collectability > 0)
            options.push({ label: 'third-stage', value: '3rd' });
    }
    options.push({ label: 'maximum', value: 'full' });
    return options;
});

// Solver options
const targetQuality = computed({
    get: () => {
        const v = solverTarget.value;
        if (props.collectableShopRefine != undefined) {
            const c = props.collectableShopRefine;
            if (v === '1st') return c.low_collectability * 10;
            if (v === '2nd') return c.mid_collectability * 10;
            if (v === '3rd') return c.high_collectability * 10;
        }
        if (typeof v == 'number') return v;
        /* if (v == 'full') */ return props.initStatus.recipe.quality;
    },
    set: (x: number | null) => {
        let v: TargetQuality = x ?? 0;
        if (props.collectableShopRefine != undefined) {
            const c = props.collectableShopRefine;
            if (x == c.low_collectability * 10) v = '1st';
            if (x == c.mid_collectability * 10) v = '2nd';
            if (x == c.high_collectability * 10) v = '3rd';
        }
        if (x == props.initStatus.recipe.quality) v = 'full';
        solverTarget.value = v;
    },
});
const useManipulation = ref(false);
const useHeartAndSoul = ref(false);
const useQuickInnovation = ref(false);
const useTrainedEye = ref(true);
const backloadProgress = ref(false);
const adversarial = ref(false);

function runRaphaelSolver() {
    emits(
        'runSimpleSolver',
        SequenceSource.RaphaelSolver,
        raphaelSolveIsSolving,
        initStatus =>
            raphael_solve(
                initStatus,
                targetQuality.value,
                useManipulation.value,
                useHeartAndSoul.value,
                useQuickInnovation.value,
                useTrainedEye.value,
                backloadProgress.value,
                adversarial.value,
            ).catch(e => {
                const err = String(e);
                if (err == 'RuntimeError: unreachable')
                    throw $t('error-probably-out-of-memory', { err });
                else throw e;
            }),
    );
}
</script>

<template>
    <el-dialog v-model="dialogVisible" :title="$t('solver-info-title')">
        <i18n path="solver-info" tag="span" class="solver-info">
            <template #origin>
                <el-link href="https://www.raphael-xiv.com/" target="_blank"
                    >https://www.raphael-xiv.com/</el-link
                >
            </template>
            <template #source>
                <el-link
                    href="https://github.com/KonaeAkira/raphael-rs/"
                    target="_blank"
                >
                    https://github.com/KonaeAkira/raphael-rs/
                </el-link>
            </template>
        </i18n>
    </el-dialog>
    <el-space direction="vertical" alignment="normal">
        <el-space>
            <el-text style="flex: none">
                {{ $t('target-quality') }}
            </el-text>
            <el-input-number
                v-model="targetQuality"
                :min="0"
                :max="initStatus.recipe.quality"
                :step="1"
                step-strictly
            />
            <el-segmented
                v-model="solverTargetSegmented"
                :options="targetQualityOptions"
            >
                <template #default="scope">
                    {{ $t((scope.item as TargetQualityOption).label) }}
                </template>
            </el-segmented>
        </el-space>
        <el-checkbox
            v-model="useTrainedEye"
            :label="$t('enable-action', { action: $t('trained-eye') })"
        />
        <el-space>
            <el-checkbox
                v-model="useManipulation"
                :label="$t('enable-action', { action: $t('manipulation') })"
            />
            <el-tag v-if="useManipulation" type="warning">
                {{ $t('need-learn-manipulation') }}
            </el-tag>
        </el-space>
        <el-space>
            <el-checkbox
                v-model="useHeartAndSoul"
                :label="$t('enable-action', { action: $t('heart-and-soul') })"
            />
            <el-tag v-if="useHeartAndSoul" type="warning">
                {{ $t('consume-crafters-delineation') }}
            </el-tag>
        </el-space>
        <el-space>
            <el-checkbox
                v-model="useQuickInnovation"
                :label="$t('enable-action', { action: $t('quick-innovation') })"
            />
            <el-tag v-if="useQuickInnovation" type="warning">
                {{ $t('consume-crafters-delineation') }}
            </el-tag>
        </el-space>
        <el-space>
            <el-checkbox
                v-model="backloadProgress"
                :label="$t('backload-progress')"
            />
            <el-tag v-if="backloadProgress" type="success">
                {{ $t('speed-up') }}
            </el-tag>
            <el-tag v-if="backloadProgress" type="danger">
                {{ $t('quality-down') }}
            </el-tag>
            <el-tag v-if="backloadProgress" type="danger">
                {{ $t('increase-duration') }}
            </el-tag>
        </el-space>
        <el-space>
            <el-checkbox v-model="adversarial" :label="$t('adversarial')" />
            <el-tag v-if="adversarial" type="danger">
                {{ $t('quality-down') }}
            </el-tag>
            <el-tag v-if="adversarial" type="danger">
                {{ $t('increase-duration') }}
            </el-tag>
        </el-space>
    </el-space>
    <div style="margin-top: 10px">
        <el-button
            @click="runRaphaelSolver"
            type="primary"
            :loading="raphaelSolveIsSolving"
        >
            {{
                raphaelSolveIsSolving
                    ? $t('simple-solver-solving')
                    : $t('solver-start')
            }}
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
error-probably-out-of-memory = { $err }（可能是内存不足，请尝试使用桌面端）

first-stage = 一档
second-stage = 二档
third-stage = 三档
maximum = 最大
custom = 自定义

target-quality = 目标品质
enable-action = 使用技能：{ $action }
backload-progress = 后置作业技能（快速求解）
minimize-steps = 使步骤最短
adversarial = 确保 100% 可靠（防黑球）

speed-up = 求解速度提高
speed-down = 求解速度降低
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
error-probably-out-of-memory = { $err } (Probably out of memory, please use the desktop edition)

first-stage = 1st
second-stage = 2nd
third-stage = 3rd
maximum = Maximum
custom = Custom

target-quality = Target quality
enable-action = Enable { $action }
backload-progress = Backload progress (Quick solve)
minimize-steps = Minimize Steps
adversarial = Ensure 100% reliability

speed-up = speed up
speed-down = speed down
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
