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
import { ElTooltip, ElIcon } from "element-plus";
import { BottomRight, Close, Cpu, View, Upload, DocumentAdd, Folder } from "@element-plus/icons-vue";

const props = defineProps<{
    previewSolver: boolean;
}>();

const isOnTauri = import.meta.env.VITE_BESTCRAFT_TARGET == 'tauri'

const emits = defineEmits<{
    (event: "plus"): void;
    (event: "delete"): void;
    (event: "solver"): void;
    (event: "print"): void;
    (event: "analysis"): void;
    (event: "update:previewSolver", value: boolean): void;

    (event: "saveList"): void;
    (event: "openList"): void;
}>();
</script>
<template>
    <div class="container">
        <el-tooltip :content="$t('save-workspace')" placement="top" :show-after="1000">
            <el-icon class="button" :size="16" @click="emits('plus')">
                <bottom-right />
            </el-icon>
        </el-tooltip>
        <el-tooltip :content="$t('clear-workspace')" placement="top" :show-after="1000">
            <el-icon class="button" :size="16" @click="emits('delete')">
                <close />
            </el-icon>
        </el-tooltip>
        <el-tooltip :content="$t('solver-setting')" placement="top" :show-after="1000">
            <el-icon class="button" :size="16" @click="emits('solver')">
                <cpu />
            </el-icon>
        </el-tooltip>
        <el-tooltip :content="$t('preview-solver')" placement="top" :show-after="1000">
            <el-icon class="button" :size="16" :color="previewSolver ? '#409EFC' : undefined"
                @click="emits('update:previewSolver', !previewSolver)">
                <View />
            </el-icon>
        </el-tooltip>
        <el-tooltip :content="$t('export-macro')" placement="top" :show-after="1000">
            <el-icon class="button" :size="16" @click="emits('print')">
                <upload />
            </el-icon>
        </el-tooltip>
        <div class="end-container">
            <el-tooltip :content="$t('export-saved-to-file')" placement="top" :show-after="1000">
                <el-icon class="button" :size="16" @click="emits('saveList')">
                    <document-add />
                </el-icon>
            </el-tooltip>
            <el-tooltip :content="$t('import-saved-from-file')" placement="top" :show-after="1000">
                <el-icon class="button" :size="16" @click="emits('openList')">
                    <folder />
                </el-icon>
            </el-tooltip>
        </div>
    </div>
</template>

<style scoped>
.container {
    display: flex;
    padding: 0px 0px 0px 3px;
    align-items: center;
}

.end-container {
    display: flex;
    padding: 0px 10px 0px 3px;
    align-items: center;
    justify-content: end;
    flex: auto;
}

.button:hover {
    background-color: var(--el-color-primary-light-9);
    transition: all var(--el-transition-duration);
}

.button {
    padding: 6px 10px;
    cursor: pointer;
}
</style>

<fluent locale="zh-CN">
solver-setting = 求解器设置
export-macro = 导出宏
export-saved-to-file = 导出暂存库到文件
import-saved-from-file = 从文件导入暂存库
</fluent>

<fluent locale="en-US">
solver-setting = Solver
export-macro = Export
export-saved-to-file = Export to file
import-saved-from-file = Import from file
</fluent>
