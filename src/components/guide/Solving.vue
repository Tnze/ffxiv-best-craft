<script setup lang="ts">
import { VNode, h, onMounted, ref } from 'vue';
import { ElText, ElTimeline, ElTimelineItem, ElButton, ElScrollbar } from 'element-plus';
import { useGuideStore } from '../../store';
import { dfs_solve, formatDuration } from "../../Solver";
import { useFluent } from 'fluent-vue'
import { Jobs, Status, newStatus, simulate } from '../../Craft';
import ActionQueue from '../designer/ActionQueue.vue'

const store = useGuideStore();
const { $t } = useFluent();

async function selectSolver() {

    const startTime = new Date().getTime()
    const getDuration = () => {
        const currentTime = new Date().getTime()
        return formatDuration(currentTime - startTime)
    }
    if (store.recipe == null ||
        store.craftType == null ||
        store.craftTypeAttr == null ||
        store.recipeInfo == null ||
        store.recipeLevel == null) {

        solverLines.value.push({
            content: [h(ElText, { innerHTML: $t('recipe-info-incomplete') })],
            lineprops: { timestamp: getDuration(), type: 'danger' },
        })
        return
    }

    const attr = store.craftTypeAttr
    const status = await newStatus(attr, store.recipe, store.recipeLevel)

    let useDfs = false
    let useRikaBfs = false
    let useTnzeBfs = false
    let useDp = false
    if (store.recipe.conditions_flag == 15) {
        // normal recipes
        if (store.recipeInfo.can_hq) {
            if (store.craftTypeAttr.level >= store.recipe.job_level + 10) {
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

    console.debug(useDfs, useRikaBfs, useTnzeBfs, useDp)

    if (useDfs) solverLines.value.push({
        content: await tryUseDFS(status, store.craftType),
        lineprops: { timestamp: getDuration() }, // evaluated after running solver
    })

}

store.setCurrentPage('solving')
onMounted(selectSolver)

async function tryUseDFS(status: Status, job: Jobs): Promise<VNode[]> {
    const result = await dfs_solve(status, 6, false)
    const list = result.map((action, id) => ({ id, action }))
    const { status: finalStatus, errors: errList } = await simulate(status, result)
    return [
        h(ElText, { innerHTML: $t('dfs-solver-result') }),
        h(ActionQueue, { list, job, errList })
    ]
}

interface SolverLine {
    content: VNode[],
    lineprops: {
        timestamp: string,
        type?: 'primary' | 'success' | 'warning' | 'danger' | 'info'
    },
}

const solverLines = ref<SolverLine[]>([])
function lines(props: { lines: SolverLine[] }) {
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
        <div class="title-box">
            <el-text class="title">{{ $t('solving') }}</el-text>
        </div>

        <div class="button-box">
            <el-button @click="$router.replace('see-recipe')">{{ $t('back') }}</el-button>
        </div>
        <el-scrollbar>
            <lines :lines="solverLines" />
        </el-scrollbar>
    </div>
</template>

<style scoped>
.title-box {
    flex: auto;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

.button-box {
    flex: auto;
    display: flex;
    justify-content: center;
}

.title {
    font-size: 1.5em;
}

.el-scrollbar {
    height: fit-content;
}

.container {
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
}
</style>

<fluent locale="zh-CN">
dfs-solver = 深度优先搜索

solving = 正在求解中
back = 返回

recipe-info-incomplete = 错误：未获取到配方信息
trained-eye-usable-try-dfs = 制作该配方可以使用{ trained-eye }，将尝试使用{ dfs-solver }
dfs-solver-result = { dfs-solver }求解完成
</fluent>
