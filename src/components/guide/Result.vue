<script setup lang="ts">
import { onMounted, ref, watchEffect } from 'vue';
import { ElAlert } from 'element-plus';
import { useRouter } from 'vue-router';
import { useGuideStore } from '../../store';
import { Actions, Attributes, Jobs, Status, newStatus, simulate } from '../../Craft';
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

watchEffect(async () => {
    const attr = store.craftTypeAttr
    const job = store.craftType
    const recipe = store.recipe
    const recipeLevel = store.recipeLevel
    if (store.bestResult == null ||
        attr == null ||
        job == null ||
        recipe == null ||
        recipeLevel == null) {
        router.replace('solving')
        return
    }
    const s = await newStatus(attr, recipe, recipeLevel)
    const actions = store.bestResult
    const slots = actions.map((action, id) => ({ id, action }))
    const { status, errors: errList } = await simulate(s, actions)

    const enhancers: Enhancer[] = []
    if (store.food) enhancers.push(store.food)
    if (store.potion) enhancers.push(store.potion)

    simulatedResult.value = {
        attr, enhancers, status, job, slots, errList,
    }
})

</script>

<template>
    <div class="container">
        <el-alert :title="$t('unfinished')" type="warning" show-icon center />
        <template v-if="simulatedResult">
            <StatusBar :status="simulatedResult.status" :attributes="simulatedResult.attr"
                :enhancers="simulatedResult.enhancers" :show-condition="false" />
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