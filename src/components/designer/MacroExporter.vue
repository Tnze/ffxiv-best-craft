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
import { computed, reactive, ref } from 'vue';
import { ElSpace, ElCard, ElMessage, ElCheckbox } from 'element-plus'
import { Actions, calcWaitTime } from '@/libs/Craft';
import { useFluent } from 'fluent-vue';

const props = defineProps<{
    actions: Actions[]
}>()
const { $t } = useFluent()

const isWebsite = import.meta.env.VITE_BESTCRAFT_TARGET == "web";
const oneclickCopy = ref(true); // 一键复制

const genOptions = reactive({
    hasNotify: true, // 宏执行完成是否提示
    hasLock: false, // 添加锁定宏语句
    avgSize: true, // 让每个宏的长度尽量相同
})

const chunkedActions = computed(() => {
    const macros = []
    let maxLinesPerChunk = 15
    if (genOptions.hasNotify)
        maxLinesPerChunk--
    if (genOptions.hasLock)
        maxLinesPerChunk--
    const minChunks = Math.ceil(props.actions.length / maxLinesPerChunk);
    const size = props.actions.length / minChunks || 14;
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

const copyChunk = async (i: number, macro: string[]) => {
    const macroText = macro.join('\r\n').replaceAll(/\u2068|\u2069/g, '')
    try {
        if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
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
            message: $t('copied-marco', { id: i + 1 })
        })
    } catch (e: any) {
        ElMessage({
            type: 'error',
            duration: 2000,
            showClose: true,
            message: $t('copy-failed', { err: String(e) })
        })
    }
}

</script>

<template>
    <div>
        <el-checkbox v-model="genOptions.hasNotify" :label="$t('has-notify')" />
        <el-checkbox v-model="genOptions.hasLock" :label="$t('has-lock')" />
        <el-checkbox v-model="genOptions.avgSize" :label="$t('avg-size')" />
        <el-checkbox v-if="isWebsite" v-model="oneclickCopy" :label="$t('oneclick-copy')" />
    </div>
    <el-space wrap alignment="flex-start">
        <el-card v-for="(marco, i) in chunkedActions" :class="oneclickCopy ? 'box-card-oneclick' : 'box-card'"
            shadow="hover" @click="oneclickCopy ? copyChunk(i, marco) : undefined">
            <span v-for="line in marco">
                {{ line }}
                <br />
            </span>
        </el-card>
    </el-space>
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
</style>

<fluent locale="zh-CN">
has-notify = 添加完成提示
has-lock = 锁定宏指令
avg-size = 长度平均化
oneclick-copy = 一键复制
copied-marco = 已复制 宏#{ $id } 到系统剪切板
marco-finished = 宏#{ $id } 已完成！
copy-failed = 复制失败：{ $err }
</fluent>

<fluent locale="en-US">
has-notify = Notification
has-lock = Lock Macro
avg-size = Equalize
oneclick-copy = Oneclick Copy
copied-marco = Copied M#{ $id } to system clipboard!
marco-finished = M#{ $id } is finished!
copy-failed = Copy failed: { $err }
</fluent>
