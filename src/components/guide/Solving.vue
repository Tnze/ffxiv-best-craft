<script setup lang="ts">
import { Component, SetupContext, VNode, h, onMounted, ref } from 'vue';
import { ElText, ElTimeline, ElTimelineItem, ElButton, TimelineItemProps } from 'element-plus';
import { useGuideStore } from '../../store';
import { formatDuration } from "../../Solver";
import { useFluent } from 'fluent-vue'

const guideStore = useGuideStore();
const { $t } = useFluent();

async function selectSolver() {

    const startTime = new Date().getTime()
    const getDuration = () => {
        const currentTime = new Date().getTime()
        return formatDuration(currentTime - startTime)
    }
    if (guideStore.recipe == null ||
        guideStore.craftTypeAttr == null ||
        guideStore.recipeInfo == null) {

        solverLines.value.push({
            content: [h(ElText, { innerHTML: $t('recipe-info-incomplete') })],
            lineprops: { timestamp: getDuration(), type: 'danger' },
        })
        return
    }

    let useDfs = false
    let useRikaBfs = false
    let useTnzeBfs = false
    let useDp = false
    if (guideStore.recipe.conditions_flag == 15) {
        // normal recipes
        if (guideStore.recipeInfo.can_hq) {
            if (guideStore.craftTypeAttr.level >= guideStore.recipe.job_level + 10) {
                solverLines.value.push({
                    content: [h(ElText, { innerHTML: $t('trained-eye-usable-try-dfs') })],
                    lineprops: { timestamp: getDuration() },
                })
                useDfs = true
            }
        }
    } else {
        // hard recipes
    }

    if (useDfs) solverLines.value.push({
        content: await tryUseDFS(),
        lineprops: { timestamp: getDuration() }, // evaluated after running solver
    })

}

guideStore.setCurrentPage('solving')
onMounted(selectSolver)

async function tryUseDFS(): Promise<VNode[]> {
    return [h(ElText, { innerHTML: $t('trained-eye-usable-try-dfs') })]
}

interface SolverLine {
    content: VNode[],
    lineprops: {
        timestamp: string,
        type?: 'primary' | 'success' | 'warning' | 'danger' | 'info'
    },
}

const solverLines = ref<SolverLine[]>([])
function lines(
    props: { lines: SolverLine[] },
    context: SetupContext,
) {
    return h(
        ElTimeline,
        () => props.lines.map(({ lineprops, content }) => h(
            ElTimelineItem,
            lineprops,
            () => content,
        ))
    )
}

</script>

<template>
    <div class="container">
        <lines :lines="solverLines" />
        <el-button @click="$router.replace('welcome')">{{ $t('back') }}</el-button>
    </div>
</template>

<style scoped>
.containter {
    display: flex;
}
</style>

<fluent locale="zh-CN">
dfs-solver = 深度优先搜索

back = 返回

recipe-info-incomplete = 错误：未获取到配方信息
trained-eye-usable-try-dfs = 制作该配方可以使用{ trained-eye }，将尝试使用{ dfs-solver }
</fluent>
