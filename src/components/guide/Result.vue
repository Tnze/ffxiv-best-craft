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
import { ref, watchEffect } from 'vue';
import { ElAlert, ElStatistic, ElRow, ElCol } from 'element-plus';
import { useRouter } from 'vue-router';
import useGuideStore from '@/stores/guide';
import { Actions, Attributes, Jobs, Recipe, RecipeInfo, RecipeLevel, Status, high_quality_probability, newStatus, simulate } from '@/libs/Craft';
import ActionQueue from '../designer/ActionQueue.vue';
import StatusBar from '../designer/StatusBar.vue';
import { Enhancer } from '../attr-enhancer/Enhancer';

const router = useRouter()
const store = useGuideStore()

store.setCurrentPage('result')

const simulatedResult = ref<{
    attr: Attributes
    enhancers: Enhancer[]
    status: Status
    job: Jobs
    slots: { id: number, action: Actions }[]
    errList: { pos: number, err: string }[]
}>()
const requiredGearsets = ref<{
    craftsmanship: number,
    control: number
}>()
const hqRate = ref(0)

watchEffect(async () => {
    const attr = store.craftTypeAttr
    const finalAttr = store.finalAttr
    const job = store.craftType
    const recipe = store.recipe
    const recipeLevel = store.recipeLevel
    if (store.bestResult == null ||
        attr == null ||
        finalAttr == null ||
        job == null ||
        recipe == null ||
        recipeLevel == null) {
        router.replace('solving')
        return
    }
    const s = await newStatus(finalAttr, recipe, recipeLevel)
    const actions = store.bestResult
    findRequiredAttributes(recipe, recipeLevel, store.recipeInfo!, actions, finalAttr).then(v => requiredGearsets.value = v)
    const slots = actions.map((action, id) => ({ id, action }))
    const { status, errors: errList } = await simulate(s, actions)
    hqRate.value = await high_quality_probability(status) ?? 0

    const enhancers: Enhancer[] = []
    if (store.food) enhancers.push(store.food)
    if (store.potion) enhancers.push(store.potion)

    simulatedResult.value = {
        attr, enhancers, status, job, slots, errList,
    }
})

async function findRequiredAttributes(recipe: Recipe, recipeLevel: RecipeLevel, recipeInfo: RecipeInfo, actions: Actions[], attributes: Attributes) {
    const [craftsmanship, control] = await Promise.all([
        (async () => {
            let minCM = recipeInfo.required_craftsmanship
            let maxCM = attributes.craftsmanship
            while (minCM < maxCM) {
                const midCM = Math.floor(minCM + (maxCM - minCM) / 2)
                const initStatus = await newStatus({ ...attributes, craftsmanship: midCM }, recipe, recipeLevel)
                const { status } = await simulate(initStatus, actions)
                if (status.progress != status.recipe.difficulty) {
                    minCM = midCM + 1
                } else {
                    maxCM = midCM
                }
            }
            return minCM
        })(),
        (async () => {
            let minCT = recipeInfo.required_control
            let maxCT = attributes.control
            while (minCT < maxCT) {
                const midCT = Math.floor(minCT + (maxCT - minCT) / 2)
                const initStatus = await newStatus({ ...attributes, control: midCT }, recipe, recipeLevel)
                const { status } = await simulate(initStatus, actions)
                if (status.quality != status.recipe.quality) {
                    minCT = midCT + 1
                } else {
                    maxCT = midCT
                }
            }
            return minCT
        })()
    ])
    return { craftsmanship, control }
}

</script>

<template>
    <div class="container">
        <el-alert :title="$t('unfinished')" type="warning" show-icon center />
        <template v-if="simulatedResult">
            <StatusBar :status="simulatedResult.status" :attributes="simulatedResult.attr"
                :enhancers="simulatedResult.enhancers" :show-condition="false" />
            <ActionQueue :list="simulatedResult.slots" :job="simulatedResult.job" :err-list="simulatedResult.errList"
                disabled />
            <el-row class="info">
                <el-col :span="6">
                    <el-statistic :value="simulatedResult.status.step">
                        <template #title>{{ $t("steps") }}</template>
                    </el-statistic>
                </el-col>
                <el-col :span="6">
                    <el-statistic :value="requiredGearsets?.craftsmanship ?? 0">
                        <template #title>{{ $t("required-craftsmanship") }}</template>
                    </el-statistic>
                </el-col>
                <el-col :span="6">
                    <el-statistic :value="requiredGearsets?.control ?? 0">
                        <template #title>{{ $t("required-control") }}</template>
                    </el-statistic>
                </el-col>
                <el-col :span="6">
                    <el-statistic :value="hqRate">
                        <template #title>{{ $t("hq-rate") }}</template>
                        <template #suffix>%</template>
                    </el-statistic>
                </el-col>
            </el-row>
        </template>
    </div>
</template>

<style scoped>
.container {
    margin: 10px;
}

.info {
    margin: 15px;
}
</style>

<fluent locale="zh-CN">
required-craftsmanship = 最低{{ craftsmanship }}
required-control = 最低{{ control }}
hq-rate = HQ率

unfinished = 该页面尚未制作完成，请等待软件版本更新。
</fluent>

<fluent locale="en-US">
unfinished = This page has not been developed yet. Please wait for updates.
</fluent>../../stores/store../../libs/Craft