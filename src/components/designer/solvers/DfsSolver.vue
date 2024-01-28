<!-- 
    This file is part of BestCraft.
    Copyright (C) 2023  Tnze

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
import { Actions, Status } from '@/libs/Craft';
import { ElAlert, ElButton, ElCheckbox, ElLink, ElSlider, ElDialog, ElSpace } from 'element-plus'
import { useFluent } from 'fluent-vue';
import { Ref, ref, watch } from 'vue';
import { nq_solve, dfs_solve } from '@/libs/Solver';
import { ChatSquare } from '@element-plus/icons-vue'

const { $t } = useFluent()

const props = defineProps<{
    canHq: boolean
}>()

const emits = defineEmits<{
    (event: 'runSimpleSolver', solverId: string, solvingRunningState: Ref<Boolean>, solver: (initStatus: Status) => Promise<Actions[]>): void
}>()

// dfs求解器最大深度，设置超过该深度会显示警告
const warningDepth = import.meta.env.VITE_BESTCRAFT_TARGET == "tauri" ? 6 : 4

const dialogVisible = ref(false)
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

function runDfsSolver() {
    emits('runSimpleSolver', "dfs", dfsSolving,
        initStatus => {
            const solver = doNotTouch.value ? nq_solve : dfs_solve
            return solver(initStatus, maxDepth.value, useSpecialist.value)
        }
    )
}
</script>

<template>
    <el-dialog v-model="dialogVisible" :title="$t('dfs-solver-info-title')">
        <i18n path="dfs-solver-info" tag="span" class="solver-info">
            <template #ffxivCraftingAlgo="{ commandLineTool }">
                <el-link type="primary" href="https://github.com/Tnze/ffxiv-crafting-algo" target="_blank">
                    {{ commandLineTool }}
                </el-link>
            </template>
        </i18n>
    </el-dialog>
    <div class="argument-block" style="display: flex;">
        <span class="slider-label">{{ $t('dfs-max-depth') }}</span>
        <el-slider v-model="maxDepth" :min="1" :max="10" :format-tooltip="dfsFormatTooltip" :label="$t('dfs-max-depth')"
            :disabled="dfsSolving" />
    </div>
    <el-alert v-if="maxDepth > warningDepth" type="warning" :title="$t('dfs-too-depth')" show-icon :closable="false" />
    <el-checkbox v-model="doNotTouch" :label="$t('do-not-touch')" :disabled="dfsSolving" />
    <el-checkbox v-model="useSpecialist" :label="$t('specialist')" :disabled="dfsSolving" />
    <div>
        <el-button type="primary" @click="runDfsSolver" :loading="dfsSolving">
            {{ dfsSolving ? $t('simple-solver-solving') : $t('solver-start') }}
        </el-button>
        <el-button :icon="ChatSquare" circle @click="dialogVisible = true" />
    </div>
</template>

<style scoped>
.solver-info {
    white-space: pre-line;
}

.argument-block {
    display: flex;
    align-items: center;
    margin-right: 20px;
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
</style>

<fluent locale="zh-CN">
do-not-touch = 不推品质
dfs-max-depth = 最大深度
solver-start = 开始求解
simple-solver-solving = 正在求解中
dfs-solver-info-title = 深度优先搜索
dfs-solver-info =
    此款求解器源于 Tnze 早期开发的一款{ $ffxivCraftingAlgo }，最初用于搜索最短的巨匠手法。

    该算法采用朴素的暴力搜索，所需时间随搜索深度限制指数级增大。推荐将搜索深度限制为6。
    更新至v2后拥有多线程加速。

    此求解器通常适合低于玩家10级以上的配方。
    .command-line-tool = 命令行工具
dfs-too-depth = 选择的最大深度过大，求解所需时间可能极长
</fluent>
<fluent locale="en-US">
do-not-touch = Do not "touching"
dfs-max-depth = Depth
solver-start = Start
simple-solver-solving = Solving
dfs-solver-info-title = Depth First Search
dfs-solver-info =
    This solver is based on an early development of the { $ffxivCraftingAlgo } by Tnze, originally usedto search for the shortest steps to create the 巨匠药水.

    The algorithm adopts naive search, which increases exponentially in time with the depth of the searching. 
    It is recommended to limit the search depth to 6. 
    After updating to v2, adopt multi threaded acceleration.

    This solver is usually suitable for recipes that are 10-level lower than the player or above.
    .command-line-tool = Command line tool
dfs-too-depth = The depth is too big. Solving time could be very long.
</fluent>