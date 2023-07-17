<script setup lang="ts">
import { VNode, computed, h, onMounted, ref } from 'vue';
import { ElText, ElTimeline, ElTimelineItem, ElButton, ElScrollbar, ElIcon, ElResult } from 'element-plus';
import { Loading } from "@element-plus/icons-vue";
import { useGuideStore } from '../../store';
import { dfs_solve, formatDuration } from "../../Solver";
import { useFluent } from 'fluent-vue'
import { Jobs, Status, newStatus, simulate } from '../../Craft';
import ActionQueue from '../designer/ActionQueue.vue'

const store = useGuideStore();
const { $t } = useFluent();

const solverTitle = ref<'solve-finished' | 'no-suitable-solver'>()
const runningSolverName = ref<string>()
const solverLines = ref<SolverLine[]>([])

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
                useDfs = true
                solverLines.value.push({
                    content: [h(ElText, { innerHTML: $t('trained-eye-usable-try-dfs') })],
                    lineprops: { timestamp: '', type: 'info' },
                })
            }
            if (store.recipe.rlv >= 560 && store.recipe.difficulty >= 70) {
                useRikaBfs = true
                solverLines.value.push({
                    content: [h(ElText, { innerHTML: $t('rika-say-good-try-rika-bfs') })],
                    lineprops: { timestamp: '', type: 'info' },
                })
            }
        } else {
            solverLines.value.push({
                content: [h(ElText, { innerHTML: $t('not-yet-supported-non-hq-recipes') })],
                lineprops: { timestamp: '', type: 'warning' },
            })
        }
    } else {
        // hard recipes
    }

    if (!useDfs && !useRikaBfs && !useTnzeBfs && !useDp) {
        solverTitle.value = 'no-suitable-solver';
        return
    }
    console.debug(useDfs, useRikaBfs, useTnzeBfs, useDp)

    if (useDfs) solverLines.value.push({
        content: await tryUseDFS(status, store.craftType),
        lineprops: { timestamp: getDuration(), type: 'primary' }, // evaluated after running solver
    })
    solverTitle.value = 'solve-finished'
}

store.setCurrentPage('solving')
onMounted(selectSolver)

async function tryUseDFS(status: Status, job: Jobs): Promise<VNode[]> {
    runningSolverName.value = $t('dfs-solver')
    const result = await dfs_solve(status, 6, false)
    runningSolverName.value = undefined
    const list = result.map((action, id) => ({ id, action }))
    const { errors: errList } = await simulate(status, result)
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

const solverSuccessed = computed(() => solverTitle.value == 'solve-finished')

</script>

<template>
    <div class="container">
        <div class="title-box">
            <el-text class="title">
                <template v-if="runningSolverName">
                    <el-icon class="is-loading">
                        <Loading />
                    </el-icon>
                    {{ $t('solving', { solverName: runningSolverName }) }}
                </template>
                <el-result v-else-if="solverTitle" :title="$t(solverTitle)" :icon="solverSuccessed ? 'success' : 'error'
                    ">
                    <template #extra>
                        <el-button @click="$router.replace('see-recipe')">{{ $t('back') }}</el-button>
                        <el-button @click="$router.replace('result')" v-if="solverSuccessed" type="primary">{{
                            $t('view-result') }}</el-button>
                    </template>
                </el-result>
            </el-text>
        </div>

        <div class="button-box" v-if="runningSolverName">
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
bfs-solver = 广度优先搜索 v1

solving = 正在运行 { $solverName } 求解器
back = 返回

recipe-info-incomplete = 错误：未获取到配方信息
trained-eye-usable-try-dfs = 满足{ trained-eye }使用条件，尝试使用{ dfs-solver }
rika-say-good-try-rika-bfs = 满足 Rika 设定的条件，尝试使用{ bfs-solver }
not-yet-supported-non-hq-recipes = 尚未支持对非 HQ 配方的求解
dfs-solver-result = { dfs-solver }求解完成

solve-finished = 求解已结束
no-suitable-solver = 没有合适的求解器

view-result = 查看结果
</fluent>
