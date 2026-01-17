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
import { computed, reactive } from 'vue';
import { NFlex, NCard, NCheckbox, useMessage } from 'naive-ui';
import { Actions, calcWaitTime } from '@/libs/Craft';
import { useFluent } from 'fluent-vue';
import { formatDuration } from '@/libs/Utils';
import { isTauri } from '@/libs/Consts';

const props = defineProps<{
    actions: Actions[];
}>();
const { $t } = useFluent();
const message = useMessage();

const genOptions = reactive({
    hasNotify: true, // 宏执行完成是否提示
    hasLock: false, // 添加锁定宏语句
    avgSize: true, // 让每个宏的长度尽量相同
});

const chunkedActions = computed(() => {
    const macros = [];
    let maxLinesPerChunk = 15;
    if (genOptions.hasNotify) maxLinesPerChunk--;
    if (genOptions.hasLock) maxLinesPerChunk--;
    const minChunks = Math.ceil(props.actions.length / maxLinesPerChunk);
    const size = props.actions.length / minChunks || 14;
    for (let sec = 0; sec < minChunks; sec++) {
        let section: Actions[];
        if (genOptions.avgSize) {
            section = props.actions.slice(
                sec * size,
                Math.min(props.actions.length, (sec + 1) * size),
            );
        } else {
            const start = sec * maxLinesPerChunk;
            section = props.actions.slice(
                start,
                Math.min(props.actions.length, start + maxLinesPerChunk),
            );
        }
        const lines = [];
        let totalWaitTime = 0;
        if (genOptions.hasLock) lines.push(`/mlock`);
        for (let action of section) {
            const waitTime = calcWaitTime(action);
            let actionName = $t(action.replaceAll('_', '-'));
            if (actionName.includes(' ')) {
                actionName = `"${actionName}"`;
            }
            lines.push(`/ac ${actionName} <wait.${waitTime}>`);
            totalWaitTime += waitTime;
        }
        if (genOptions.hasNotify)
            lines.push(`/echo ${$t('macro-finished', { id: sec + 1 })}<se.1>`);
        macros.push({ lines, totalWaitTime });
    }
    return macros;
});

const copyChunk = async (i: number, macro: string[]) => {
    const macroText = macro.join('\r\n').replaceAll(/\u2068|\u2069/g, '');
    if (isTauri) {
        let { writeText } =
            await import('@tauri-apps/plugin-clipboard-manager');
        await writeText(macroText);
    } else {
        let { useClipboard } = await import('@vueuse/core');
        useClipboard().copy(macroText);
    }
    message.success($t('copied-macro', { id: i + 1 }), {
        duration: 2000,
        closable: true,
    });
};
</script>

<template>
    <n-flex vertical>
        <n-flex>
            <n-checkbox
                v-model:checked="genOptions.hasNotify"
                :label="$t('has-notify')"
            />
            <n-checkbox
                v-model:checked="genOptions.hasLock"
                :label="$t('has-lock')"
            />
            <n-checkbox
                v-model:checked="genOptions.avgSize"
                :label="$t('avg-size')"
            />
        </n-flex>
        <template v-for="({ lines, totalWaitTime }, i) in chunkedActions">
            <span
                >{{ $t('macro') }} #{{ i + 1 }} ({{
                    formatDuration(totalWaitTime * 1e3, 0)
                }})</span
            >
            <n-card class="box-card" @click="copyChunk(i, lines)" hoverable>
                <span v-for="line in lines">
                    {{ line }}
                    <br />
                </span>
            </n-card>
        </template>
    </n-flex>
</template>

<style scoped>
.text {
    font-size: 14px;
}

.item {
    margin-bottom: 18px;
}

.box-card {
    cursor: pointer;
    width: auto;
}
</style>

<fluent locale="zh-CN">
macro = 宏指令
has-notify = 添加完成提示
has-lock = 锁定宏指令
avg-size = 长度平均化
copied-macro = 已复制 宏#{ $id } 到系统剪切板
macro-finished = 宏#{ $id } 已完成！
</fluent>

<fluent locale="zh-TW">
macro = 巨集指令
has-notify = 新增完成提示
has-lock = 鎖定巨集指令
avg-size = 長度平均化
copied-macro = 已複製 巨集#{ $id } 到系統剪下板
macro-finished = 巨集#{ $id } 已完成！
</fluent>

<fluent locale="en-US">
macro = Macro
copied-macro = Copied M#{ $id } to system clipboard!
macro-finished = M#{ $id } is finished!
</fluent>
