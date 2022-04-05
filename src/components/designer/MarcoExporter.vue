<script setup lang="ts">
import { computed } from '@vue/reactivity';
import 'element-plus/es/components/message/style/css'
import { ElMessage } from 'element-plus'
import { writeText } from '@tauri-apps/api/clipboard'
import { Actions } from '../../Craft';

const props = defineProps<{
    actions: Actions[]
}>()

const chunkedActions = computed(() => {
    const macros = []
    const minChunks = Math.ceil(props.actions.length / 14);
    const size = props.actions.length / minChunks || 14;
    for (let sec = 0; sec < minChunks; sec++) {
        const section = props.actions.slice(sec * size, Math.min(props.actions.length, (sec + 1) * size))
        let lines = [];
        for (let action of section) {
            lines.push(`/ac ${names.get(action)} <wait.${waitTimes.get(action)}>`)
        }
        lines.push(`/echo 宏#${sec + 1} 已完成！<se.1>`)
        macros.push(lines)
    }
    return macros
})

const copyChunk = async (i: number, macro: string[]) => {
    await writeText(macro.join('\n'))
    ElMessage({
        type: 'success',
        duration: 2000,
        showClose: true,
        message: `已复制 宏#${i + 1} 到系统剪切板`
    })
}

const names = new Map([
    [Actions.BasicSynthesis, "制作"],
    [Actions.BasicTouch, "加工"],
    [Actions.MastersMend, "精修"],
    [Actions.HastyTouch, "仓促"],
    [Actions.RapidSynthesis, "高速制作"],
    [Actions.Observe, "观察"],
    [Actions.TricksOfTheTrade, "秘诀"],
    [Actions.WasteNot, "俭约"],
    [Actions.Veneration, "崇敬"],
    [Actions.StandardTouch, "中级加工"],
    [Actions.GreatStrides, "阔步"],
    [Actions.Innovation, "改革"],
    [Actions.FinalAppraisal, "最终确认"],
    [Actions.WasteNotII, "长期俭约"],
    [Actions.ByregotsBlessing, "比尔格的祝福"],
    [Actions.PreciseTouch, "集中加工"],
    [Actions.MuscleMemory, "坚信"],
    [Actions.CarefulSynthesis, "模范制作"],
    [Actions.Manipulation, "掌握"],
    [Actions.PrudentTouch, "俭约加工"],
    [Actions.FocusedSynthesis, "注视制作"],
    [Actions.FocusedTouch, "注视加工"],
    [Actions.Reflect, "闲静"],
    [Actions.PreparatoryTouch, "坯料加工"],
    [Actions.Groundwork, "坯料制作"],
    [Actions.DelicateSynthesis, "精密制作"],
    [Actions.IntensiveSynthesis, "集中制作"],
    [Actions.TrainedEye, "工匠的神速技巧"],
    [Actions.AdvancedTouch, "上级加工"],
    [Actions.PrudentSynthesis, "俭约制作"],
    [Actions.TrainedFinesse, "工匠的神技"],
    [Actions.CarefulObservation, "设计变动"],
    [Actions.HeartAndSoul, "专心致志"],
])

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
    [Actions.FocusedSynthesis, 3],
    [Actions.FocusedTouch, 3],
    [Actions.Reflect, 3],
    [Actions.PreparatoryTouch, 3],
    [Actions.Groundwork, 3],
    [Actions.DelicateSynthesis, 3],
    [Actions.IntensiveSynthesis, 3],
    [Actions.TrainedEye, 3],
    [Actions.AdvancedTouch, 3],
    [Actions.PrudentSynthesis, 3],
    [Actions.TrainedFinesse, 3],
    [Actions.CarefulObservation, 2],
    [Actions.HeartAndSoul, 2],
])

</script>

<template>
    <el-space wrap alignment="flex-start">
        <el-card
            v-for="(marco, i) in chunkedActions"
            class="box-card"
            shadow="hover"
            @click="copyChunk(i, marco)"
        >
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