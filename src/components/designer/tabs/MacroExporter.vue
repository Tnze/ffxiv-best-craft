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
import { computed, reactive } from 'vue';
import {
    ElSpace,
    ElCard,
    ElMessage,
    ElCheckbox,
    ElDivider,
    ElSelectV2,
    ElInputNumber,
    ElForm,
    ElFormItem,
    ElSegmented,
} from 'element-plus';
import { Actions, calcWaitTime } from '@/libs/Craft';
import { useFluent } from 'fluent-vue';
import { isTauri, isWebsite } from '@/libs/Consts';
import useStore from '@/stores/designer';

const props = defineProps<{
    actions: Actions[];
}>();
const { $t } = useFluent();
const store = useStore();
const genOptions = reactive(store.options.exportOptions);

const addNotification = computed(() => [
    { label: $t('has-notify-auto'), value: 'auto' },
    { label: $t('has-notify-true'), value: true },
    { label: $t('has-notify-false'), value: false },
]);
const sectionOptions = computed(() => [
    { label: $t('avg-section'), value: 'avg' },
    { label: $t('greedy-section'), value: 'greedy' },
    { label: $t('disable-section'), value: 'disable' },
]);
const notifySoundOptions = computed(() => {
    const options = [
        { label: $t('no-sound'), value: '' },
        { label: $t('random-sound'), value: ` <se>` },
    ];
    for (let i = 1; i <= 16; i++) {
        options.push({
            label: ` <se.${i}>`,
            value: ` <se.${i}>`,
        });
    }
    return options;
});

// 自动确认是否添加完成提示
const hasNotify = computed(() => {
    if (genOptions.addNotification != 'auto') {
        return genOptions.addNotification;
    } else {
        let maxLinesPerChunk = 15;
        if (genOptions.hasLock) maxLinesPerChunk--;
        // hasNotify == 0
        const minChunks1 = Math.ceil(props.actions.length / maxLinesPerChunk);
        // hasNotify == 1
        maxLinesPerChunk--;
        const minChunks2 = Math.ceil(props.actions.length / maxLinesPerChunk);
        return minChunks1 == minChunks2;
    }
});

const chunkedActions = computed(() => {
    const macros = [];
    // First, find out how many chunks we need
    let maxLinesPerChunk = 15;
    if (hasNotify.value) maxLinesPerChunk--;
    if (genOptions.hasLock) maxLinesPerChunk--;
    if (genOptions.sectionMethod == 'disable') maxLinesPerChunk = 1e9; // Don't use Infinity here for minChunks calculations below
    let minChunks = Math.ceil(props.actions.length / maxLinesPerChunk);
    // Second, generate every chunk
    const size = Math.ceil(props.actions.length / minChunks);
    for (let sec = 0; sec < minChunks; sec++) {
        let section;
        switch (genOptions.sectionMethod) {
            case 'avg':
                section = props.actions.slice(
                    sec * size,
                    Math.min(props.actions.length, (sec + 1) * size),
                );
                break;
            case 'greedy':
                const start = sec * maxLinesPerChunk;
                section = props.actions.slice(
                    start,
                    Math.min(props.actions.length, start + maxLinesPerChunk),
                );
                break;
            case 'disable':
                section = props.actions.slice();
        }
        let lines = [];
        if (genOptions.hasLock) lines.push(`/mlock`);
        for (let action of section) {
            // 将冒进替换成仓促
            if (action == Actions.DaringTouch) {
                action = Actions.HastyTouch;
            }
            let actionName = $t(action.replaceAll('_', '-'));
            if (actionName.includes(' ')) {
                actionName = `"${actionName}"`;
            }
            lines.push(
                `/ac ${actionName} <wait.${calcWaitTime(action) + genOptions.waitTimeInc}>`,
            );
        }
        if (hasNotify.value) {
            lines.push(
                `/e ${$t('marco-finished', { id: sec + 1 })}${genOptions.notifySound}`,
            );
        }
        macros.push(lines);
    }
    return macros;
});

async function copyChunk(i: number, macro: string[]) {
    const macroText = macro.join('\r\n').replaceAll(/\u2068|\u2069/g, '');
    copy(macroText, $t('copied-marco', { id: i + 1 }));
}

async function copy(macroText: string, macroInfo: string) {
    try {
        if (isTauri) {
            let { writeText } = await import(
                '@tauri-apps/plugin-clipboard-manager'
            );
            await writeText(macroText);
        } else {
            let { useClipboard } = await import('@vueuse/core');
            await useClipboard().copy(macroText);
        }
        ElMessage({
            type: 'success',
            duration: 2000,
            showClose: true,
            message: macroInfo,
        });
    } catch (e: any) {
        ElMessage({
            type: 'error',
            duration: 2000,
            showClose: true,
            message: $t('copy-failed', { err: String(e) }),
        });
    }
}
</script>

<template>
    <div style="margin-left: 10px">
        <div>
            <el-checkbox v-model="genOptions.hasLock" :label="$t('has-lock')" />
            <el-checkbox
                v-if="isWebsite"
                v-model="genOptions.oneclickCopy"
                :label="$t('oneclick-copy')"
            />
        </div>
        <el-form label-width="auto">
            <el-form-item :label="$t('has-notify')">
                <el-segmented
                    v-model="genOptions.addNotification"
                    :options="addNotification"
                />
            </el-form-item>
            <el-form-item :label="$t('section-method')">
                <el-segmented
                    v-model="genOptions.sectionMethod"
                    :options="sectionOptions"
                />
            </el-form-item>
            <el-form-item v-if="hasNotify" :label="$t('notify-sound')">
                <el-select-v2
                    v-model="genOptions.notifySound"
                    :options="notifySoundOptions"
                    style="width: 200px"
                />
            </el-form-item>
            <el-form-item :label="$t('wait-time-inc')">
                <el-input-number
                    v-model="genOptions.waitTimeInc"
                    controls-position="right"
                    :min="0"
                    :step-strictly="true"
                />
            </el-form-item>
        </el-form>
        <el-space wrap alignment="flex-start">
            <el-card
                v-for="(marco, i) in chunkedActions"
                :class="
                    genOptions.oneclickCopy ? 'box-card-oneclick' : 'box-card'
                "
                shadow="hover"
                @click="
                    genOptions.oneclickCopy ? copyChunk(i, marco) : undefined
                "
            >
                <code class="box-body">
                    {{ marco.join('\n') }}
                </code>
            </el-card>
        </el-space>
        <el-divider id="divider" content-position="left">
            {{ $t('export-json') }}
        </el-divider>
        <el-card
            v-if="actions.length > 0"
            :class="genOptions.oneclickCopy ? 'box-card-oneclick' : 'box-card'"
            shadow="hover"
            style="width: 300px"
            @click="
                genOptions.oneclickCopy
                    ? copy(JSON.stringify(actions), $t('copied-json'))
                    : undefined
            "
        >
            <code class="box-body">
                {{ JSON.stringify(actions, undefined, 4) }}
            </code>
        </el-card>
    </div>
</template>

<style scoped>
.text {
    font-size: 14px;
}

.item {
    margin-bottom: 18px;
}

.box-card {
    user-select: text;
}

.box-card-oneclick {
    cursor: copy;
    user-select: none;
}

.box-body {
    white-space: pre-wrap;
}

:deep(.el-divider__text) {
    background-color: var(--tnze-mica-bg-color);
}
</style>

<fluent locale="zh-CN">
has-notify = 添加完成提示
has-notify-auto = 自动确定
has-notify-true = 总是提示
has-notify-false = 不提示

has-lock = 锁定宏指令
oneclick-copy = 一键复制

notify-sound = 提示音
random-sound = 随机提示音
no-sound = 无提示音

section-method = 拆分过长的宏
avg-section = 平均
greedy-section = 贪婪
disable-section = 禁用

wait-time-inc = 增加等待时间

export-json = 导出 JSON
copied-json = 已复制 JSON 表达式 到系统剪切板
copied-marco = 已复制 宏#{ $id } 到系统剪切板
marco-finished = 宏#{ $id } 已完成！
copy-failed = 复制失败：{ $err }
</fluent>

<fluent locale="zh-TW">
has-notify = 新增完成提示
has-notify-auto = 自動確定
has-notify-true = 總是提示
has-notify-false = 不提示

has-lock = 鎖定巨集指令
oneclick-copy = 一鍵複製

notify-sound = 提示音
random-sound = 隨機提示音
no-sound = 無提示音

section-method = 拆分過長的巨集
avg-section = 平均
greedy-section = 貪婪
disable-section = 停用

wait-time-inc = 增加等待時間

export-json = 匯出 JSON
copied-json = 已複製 JSON 表示式 到系統剪下板
copied-marco = 已複製 巨集#{ $id } 到系統剪下板
marco-finished = 巨集#{ $id } 已完成！
copy-failed = 複製失敗：{ $err }
</fluent>

<fluent locale="en-US">
has-notify = Notification
has-notify-auto = Auto
has-notify-true = Always
has-notify-false = Never

has-lock = Macro Lock
oneclick-copy = Oneclick Copy

notify-sound = Beep type
random-sound = Random Sound
no-sound = No Sound

section-method = Section method
avg-section = Average
greedy-section = Greedy
disable-section = Disable

wait-time-inc = Increase waiting time

export-json = Export as JSON
copied-json = Copied JSON expression to system clipboard!
copied-marco = Copied M#{ $id } to system clipboard!
marco-finished = M#{ $id } is finished!
copy-failed = Copy failed: { $err }
</fluent>
