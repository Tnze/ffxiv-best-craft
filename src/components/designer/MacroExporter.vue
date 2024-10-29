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
import { computed, reactive, ref, watchEffect } from 'vue';
import { ElSpace, ElCard, ElMessage, ElCheckbox, ElDivider } from 'element-plus'
import { Actions, calcWaitTime } from '@/libs/Craft';
import { useFluent } from 'fluent-vue';
import { isTauri, isWebsite } from '@/libs/Consts';

const props = defineProps<{
    actions: Actions[]
}>()
const { $t } = useFluent()

const oneclickCopy = ref(true); // 一键复制

const genOptions = reactive({
    hasNotify: true, // 宏执行完成是否提示
    hasNotifyIndeterminate: true, // 自动确定是否添加完成提示
    hasLock: false, // 添加锁定宏语句
    avgSize: true, // 让每个宏的长度尽量相同
})

// 自动确认是否添加完成提示
watchEffect(() => {
    if (genOptions.hasNotifyIndeterminate) {
        let maxLinesPerChunk = 15
        if (genOptions.hasLock)
            maxLinesPerChunk--;
        // hasNotify == 0
        const minChunks1 = Math.ceil(props.actions.length / maxLinesPerChunk);
        // hasNotify == 1
        maxLinesPerChunk--;
        const minChunks2 = Math.ceil(props.actions.length / maxLinesPerChunk);
        genOptions.hasNotify = minChunks1 == minChunks2;
    }
});

const chunkedActions = computed(() => {
    const macros = []
    let maxLinesPerChunk = 15
    if (genOptions.hasNotify)
        maxLinesPerChunk--
    if (genOptions.hasLock)
        maxLinesPerChunk--
    const minChunks = Math.ceil(props.actions.length / maxLinesPerChunk);
    const size = Math.ceil(props.actions.length / minChunks);
    for (let sec = 0; sec < minChunks; sec++) {
        let section;
        if (genOptions.avgSize) {
            section = props.actions.slice(sec * size, Math.min(props.actions.length, (sec + 1) * size))
        } else {
            const start = sec * maxLinesPerChunk;
            section = props.actions.slice(start, Math.min(props.actions.length, start + maxLinesPerChunk))
        }
        let lines = [];
        if (genOptions.hasLock)
            lines.push(`/mlock`)
        for (let action of section) {
            // 将冒进替换成仓促
            if (action == Actions.DaringTouch) {
                action = Actions.HastyTouch;
            }
            let actionName = $t(action.replaceAll('_', '-'))
            if (actionName.includes(' ')) {
                actionName = `"${actionName}"`
            }
            lines.push(`/ac ${actionName} <wait.${calcWaitTime(action)}>`)
        }
        if (genOptions.hasNotify)
            lines.push(`/e ${$t('marco-finished', { id: sec + 1 })}<se.1>`)
        macros.push(lines)
    }
    return macros
})

async function copyChunk(i: number, macro: string[]) {
    const macroText = macro.join('\r\n').replaceAll(/\u2068|\u2069/g, '')
    copy(macroText, $t('copied-marco', { id: i + 1 }))
}

async function copy(macroText: string, macroInfo: string) {
    try {
        if (isTauri) {
            let { writeText } = await import('@tauri-apps/plugin-clipboard-manager')
            await writeText(macroText)
        } else {
            let { useClipboard } = await import("@vueuse/core")
            await useClipboard().copy(macroText)
        }
        ElMessage({
            type: 'success',
            duration: 2000,
            showClose: true,
            message: macroInfo,
        })
    } catch (e: any) {
        ElMessage({
            type: 'error',
            duration: 2000,
            showClose: true,
            message: $t('copy-failed', { err: String(e) }),
        })
    }
}
</script>

<template>
    <div style="margin-left: 10px;">
        <div>
            <el-checkbox v-model="genOptions.hasNotify" :indeterminate="genOptions.hasNotifyIndeterminate"
                @change="genOptions.hasNotifyIndeterminate = false" :label="$t('has-notify')" />
            <el-checkbox v-model="genOptions.hasLock" :label="$t('has-lock')" />
            <el-checkbox v-model="genOptions.avgSize" :label="$t('avg-size')" />
            <el-checkbox v-if="isWebsite" v-model="oneclickCopy" :label="$t('oneclick-copy')" />
        </div>
        <el-space wrap alignment="flex-start">
            <el-card v-for="(marco, i) in chunkedActions" :class="oneclickCopy ? 'box-card-oneclick' : 'box-card'"
                shadow="hover" @click="oneclickCopy ? copyChunk(i, marco) : undefined">
                <code class="box-body">
                    {{ marco.join('\n') }}
                </code>
            </el-card>
        </el-space>
        <el-divider />
        <el-card v-if="actions.length > 0" :class="oneclickCopy ? 'box-card-oneclick' : 'box-card'" shadow="hover"
            style="width: 300px;" @click="oneclickCopy ? copy(JSON.stringify(actions), $t('copied-json')) : undefined">
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
</style>

<fluent locale="zh-CN">
has-notify = 添加完成提示
has-lock = 锁定宏指令
avg-size = 长度平均化
oneclick-copy = 一键复制

copied-json = 已复制 JSON 表达式 到系统剪切板
copied-marco = 已复制 宏#{ $id } 到系统剪切板
marco-finished = 宏#{ $id } 已完成！
copy-failed = 复制失败：{ $err }
</fluent>

<fluent locale="en-US">
has-notify = Notification
has-lock = Macro Lock
avg-size = Equalize
oneclick-copy = Oneclick Copy

copied-json = Copied JSON expression to system clipboard!
copied-marco = Copied M#{ $id } to system clipboard!
marco-finished = M#{ $id } is finished!
copy-failed = Copy failed: { $err }
</fluent>
