<script setup lang="ts">
import { computed } from 'vue'
import { Conditions, Status } from '../../Craft';
import Condition from './Condition.vue'
import Buffs from './Buffs.vue';

const props = defineProps<{
    status: Status;
}>()

const durability = computed<number>(() => {
    return props.status === undefined
        ? 100
        : props.status.durability / props.status.recipe.durability * 100
})
const progress = computed<number>(() => {
    return props.status === undefined
        ? 100
        : props.status.progress / props.status.recipe.difficulty * 100
})
const quality = computed<number>(() => {
    return props.status === undefined
        ? 100
        : props.status.quality / props.status.recipe.quality * 100
})

const progressColor = computed<string>(() => {
    if (props.status.progress >= props.status.recipe.difficulty)
        return '#13CE66'
    if (props.status.durability <= 0)
        return '#F56C6C'
    return '#409EFF'
})
const qualityColor = computed<string>(() => {
    if (props.status.quality >= props.status.recipe.quality)
        return '#13CE66'
    return '#409EFF'
})

const durabilityColor = [
    { color: '#FF999E', percentage: 20 },
    { color: '#FFD470', percentage: 50 },
    { color: '#62C3FF', percentage: 100 },
]
const condition = computed(() => {
    if (props.status === undefined)
        return Conditions.Normal
    return {
        "Normal": Conditions.Normal,
        "Good": Conditions.Good,
        "Excellent": Conditions.Excellent,
        "Poor": Conditions.Poor,
        "Centered": Conditions.Centered,
        "Sturdy": Conditions.Sturdy,
        "Pliant": Conditions.Pliant,
        "Malleable": Conditions.Malleable,
        "Primed": Conditions.Primed,
    }[props.status!.condition] || Conditions.Normal
})
</script>

<template>
    <div class="conatiner">
        <div id="durability-and-condition">
            <div id="durability">
                耐久{{ status?.durability }} / {{ status?.recipe.durability }}
                <el-progress :stroke-width="14" :show-text="false" :percentage="durability" :color="durabilityColor">
                </el-progress>
            </div>
            <div id="condition">
                <Condition :cond="condition" />
            </div>
        </div>
        <div id="progress-and-buffs">
            进展
            <el-progress :percentage="progress" :color="progressColor">
                {{ status?.progress }} /
                {{ status?.recipe.difficulty }} /
                {{ status?.recipe.difficulty - status?.progress }}
            </el-progress>品质
            <el-progress :percentage="quality" :color="qualityColor">
                {{ status?.quality }} /
                {{ status?.recipe.quality }}
            </el-progress>
            <Buffs id="buffs" :buffs="status.buffs" />
        </div>
        <div id="attributes">
            等级：{{ status?.attributes.level }}
            <br />
            作业精度：{{ status?.attributes.craftsmanship }}
            <br />
            加工精度：{{ status?.attributes.control }}
            <br />
            制作力：{{ status?.craft_points }} / {{ status?.attributes.craft_points }}
            <br />
            <el-progress class="craft-points-progressbar"
                :percentage="status?.craft_points / status?.attributes.craft_points * 100" color="#FF9999"
                :show-text="false" :stroke-width="10" />
        </div>
    </div>
</template>

<style scoped>
.conatiner {
    width: 100%;
    display: flex;
    font-size: 14px;
    color: #606266;
}

#durability-and-condition {
    padding: 3px 10px 5px 5px;
    flex: none;
}

#durability {
    padding: 10px;
}

#condition {
    text-align: center;
}

#progress-and-buffs {
    padding: 7px;
    flex-grow: 5;
}

#buffs {
    margin-top: 7px;
}

#attributes {
    font-size: 14px;
    line-height: 1.5;
    padding: 0px 20px 0px 0px;
    flex-grow: 2;
    text-align: right;
    color: #909399;
}

.craft-points-progressbar {
    width: 120px;
    display: inline-block;
}
</style>