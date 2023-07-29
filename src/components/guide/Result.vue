<script setup lang="ts">
import { onMounted, ref, watchEffect } from 'vue';
import { ElAlert } from 'element-plus';
import { useRouter } from 'vue-router';
import { useGuideStore } from '../../store';
import { Actions, Attributes, Jobs, Recipe, RecipeInfo, RecipeLevel, Status, newStatus, simulate } from '../../Craft';
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
                <span>{{ requiredGearsets }}</span>
            <ActionQueue :list="simulatedResult.slots" :job="simulatedResult.job" :err-list="simulatedResult.errList" />
        </template>
    </div>
</template>

<style scoped>
.container {
    margin: 10px;
}
</style>

<fluent locale="zh-CN">
unfinished = 该页面尚未制作完成，请等待软件版本更新。
</fluent>

<fluent locale="en-US">
unfinished = This page has not been developed yet. Please wait for updates.
</fluent>