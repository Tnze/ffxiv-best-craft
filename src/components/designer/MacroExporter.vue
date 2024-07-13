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
import { computed, reactive } from 'vue';
import { ElSpace, ElCard, ElMessage, ElCheckbox } from 'element-plus'
import { Actions } from '@/libs/Craft';
import { useFluent } from 'fluent-vue';

const props = defineProps<{
    actions: Actions[]
}>()
const { $t } = useFluent()

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
            lines.push(`/ac ${actionName} <wait.${waitTimes.get(action)}>`)
        }
        if (genOptions.hasNotify)
            lines.push(`/echo ${$t('marco-finished', { id: sec + 1 })}<se.1>`)
        macros.push(lines)
    }
    return macros
})

const copyChunk = async (i: number, macro: string[]) => {
    const macroText = macro.join('\r\n').replaceAll(/\u2068|\u2069/g, '')
    if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
        let { writeText } = await import('@tauri-apps/api/clipboard')
        await writeText(macroText)
    } else {
        let { useClipboard } = await import("@vueuse/core")
        useClipboard().copy(macroText)
    }
    ElMessage({
        type: 'success',
        duration: 2000,
        showClose: true,
        message: $t('copied-marco', { id: i + 1 })
    })
}

const waitTimes = new Map([
    [Actions.BasicSynthesis, 3],
    [Actions.BasicTouch, 3],
    [Actions.MastersMend, 3],
    [Actions.HastyTouch, 3],
    [Actions.RapidSynthesis, 3],
    [Actions.Observe, 3],
    [Actions.TricksOfTheTrade, 2],
    [Actions.WasteNot, 2],
    [Actions.Veneration, 2],
    [Actions.StandardTouch, 3],
    [Actions.GreatStrides, 2],
    [Actions.Innovation, 2],
    [Actions.FinalAppraisal, 2],
    [Actions.WasteNotII, 2],
    [Actions.ByregotsBlessing, 3],
    [Actions.PreciseTouch, 3],
    [Actions.MuscleMemory, 3],
    [Actions.CarefulSynthesis, 3],
    [Actions.Manipulation, 2],
    [Actions.PrudentTouch, 3],
    [Actions.Reflect, 3],
    [Actions.PreparatoryTouch, 3],
    [Actions.Groundwork, 3],
    [Actions.DelicateSynthesis, 3],
    [Actions.IntensiveSynthesis, 3],
    [Actions.TrainedEye, 3],
    [Actions.AdvancedTouch, 3],
    [Actions.PrudentSynthesis, 3],
    [Actions.TrainedFinesse, 3],
    [Actions.CarefulObservation, 3],
    [Actions.HeartAndSoul, 3],
    [Actions.RefinedTouch, 3],
    [Actions.DaringTouch, 3],
    [Actions.ImmaculateMend, 3],
    [Actions.QuickInnovation, 2],
    [Actions.TrainedPerfection, 3],
])

</script>

<template>
    <div>
        <el-checkbox v-model="genOptions.hasNotify" :label="$t('has-notify')" />
        <el-checkbox v-model="genOptions.hasLock" :label="$t('has-lock')" />
        <el-checkbox v-model="genOptions.avgSize" :label="$t('avg-size')" />
    </div>
    <el-space wrap alignment="flex-start">
        <el-card v-for="(marco, i) in chunkedActions" class="box-card" shadow="hover" @click="copyChunk(i, marco)">
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
    /* width: 200px; */
    cursor: pointer;
}
</style>

<fluent locale="zh-CN">
has-notify = 添加完成提示
has-lock = 锁定宏指令
avg-size = 长度平均化
copied-marco = 已复制 宏#{ $id } 到系统剪切板
marco-finished = 宏#{ $id } 已完成！
</fluent>

<fluent locale="en-US">
copied-marco = Copied M#{ $id } to system clipboard!
marco-finished = M#{ $id } is finished!
</fluent>
