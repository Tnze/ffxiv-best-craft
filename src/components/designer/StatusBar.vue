<script setup lang="ts">
import { computed } from 'vue'
import { Conditions, Status } from '../../Craft';
import Condition from './Condition.vue'

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
                <el-progress
                    :stroke-width="14"
                    :show-text="false"
                    :percentage="durability"
                    :color="durabilityColor"
                ></el-progress>
            </div>
            <div id="condition">
                <Condition :cond="condition" />
            </div>
        </div>
        <div id="progress">
            进展
            <el-progress
                :percentage="progress"
            >{{ status?.progress }} / {{ status?.recipe.difficulty }}</el-progress>品质
            <el-progress :percentage="quality">{{ status?.quality }} / {{ status?.recipe.quality }}</el-progress>
        </div>
        <div id="attributes"></div>
    </div>
    <!-- 
    
    制作力{{ status?.craft_points }}-->
</template>

<style scoped>
.conatiner {
    width: 100%;
    display: flex;
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
#progress {
    padding: 7px;
    flex-grow: 5;
}
#attributes {
    flex-grow: 3;
}
</style>