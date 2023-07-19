<script setup lang="ts">
import { VNode, computed, h, onMounted, ref } from 'vue';
import { ElText, ElTimeline, ElTimelineItem, ElButton, ElScrollbar, ElIcon, ElResult } from 'element-plus';
import { Loading } from "@element-plus/icons-vue";
import { useGuideStore } from '../../store';
import { dfs_solve, formatDuration, reflect_solve, rika_solve, rika_solve_tnzever } from "../../Solver";
import { useFluent } from 'fluent-vue'
import { Actions, Jobs, Status, compareStatus, newStatus, simulate } from '../../Craft';
import ActionQueue from '../designer/ActionQueue.vue'

const store = useGuideStore();
const { $t } = useFluent();

const solverTitle = ref<'solve-finished' | 'no-suitable-solver'>()
const runningSolverName = ref<string>()
const solverLines = ref<SolverLine[]>([])

async function selectSolver() {
    if (store.recipe == null ||
        store.craftType == null ||
        store.craftTypeAttr == null ||
        store.recipeInfo == null ||
        store.recipeLevel == null) {

        solverLines.value.push({
            content: [h(ElText, { innerHTML: $t('recipe-info-incomplete') })],
            lineprops: { timestamp: '', type: 'danger' },
        })
        return
    }

    const attr = store.craftTypeAttr
    const job = store.craftType
    const recipe = store.recipe
    const recipeLevel = store.recipeLevel
    const status = await newStatus(attr, recipe, recipeLevel)
    const useManipulation = true

    const solveResults: { status: Status, actions: Actions[] }[] = []
    const runSolver = async (name: string, func: () => Promise<Actions[]>) => {
        runningSolverName.value = $t(name)
        const startTime = new Date().getTime()
        const result = await func()
        const stopTime = new Date().getTime()
        runningSolverName.value = undefined

        const list = result.map((action, id) => ({ id, action }))
        const { status: fstatus, errors: errList } = await simulate(status, result)

        solveResults.push({ status: fstatus, actions: result })
        solverLines.value.push({
            content: [
                h(ElText, { innerHTML: $t('solver-result', { solverName: $t(name) }) }),
                h(ActionQueue, { list, job, errList })
            ],
            lineprops: {
                timestamp: formatDuration(stopTime - startTime),
                type: fstatus.progress < recipe.difficulty ? 'danger' :
                    fstatus.quality < recipe.quality ? 'warning' : 'primary'
            }, // evaluated after running solver
        })
    }
    if (store.recipe.conditions_flag == 15) {
        // normal recipes
        if (store.recipeInfo.can_hq) {
            if (store.craftTypeAttr.level >= store.recipe.job_level + 10) {
                solverLines.value.push({
                    content: [h(ElText, { innerHTML: $t('trained-eye-usable-try-dfs') })],
                    lineprops: { timestamp: '', type: 'info' },
                })
                await runSolver('dfs-solver', () => dfs_solve(status, 6, false))
            }
            solveResults.sort((a, b) => compareStatus(a.status, b.status))
            if (solveResults.length == 0 || solveResults[0].status.quality < recipe.quality) {
                await runSolver('reflect-solver', () => reflect_solve(status, useManipulation))
            }
            // if bestResult quality <= maxQuality { runReflectSolver() }
            if (store.recipe.rlv >= 560 && store.recipe.difficulty >= 70) {
                solverLines.value.push({
                    content: [h(ElText, { innerHTML: $t('rika-say-good-try-rika-bfs') })],
                    lineprops: { timestamp: '', type: 'info' },
                })
                await runSolver('rika-bfs-solver', () => rika_solve(status))
                await runSolver('tnze-bfs-solver', () => rika_solve_tnzever(status, useManipulation, 8, true, true))
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

    if (solveResults.length == 0) {
        solverTitle.value = 'no-suitable-solver';
    } else {
        solverTitle.value = 'solve-finished'
    }
}

store.setCurrentPage('solving')
onMounted(selectSolver)

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
                <el-result v-else-if="solverTitle" :title="$t(solverTitle)" :icon="solverSuccessed ? 'success' : 'error'">
                    <template #extra>
                        <el-button @click="$router.replace('see-recipe')">
                            {{ $t('back') }}
                        </el-button>
                        <el-button @click="$router.replace('result')" v-if="solverSuccessed" type="primary">
                            {{ $t('view-result') }}
                        </el-button>
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
reflect-solver = 闲静DP
rika-bfs-solver = Rika原版BFS
tnze-bfs-solver = Tnze修改版BFS

solving = 正在运行 { $solverName } 求解器
back = 返回

recipe-info-incomplete = 错误：未获取到配方信息
trained-eye-usable-try-dfs = 满足{ trained-eye }使用条件，尝试使用{ dfs-solver }
rika-say-good-try-rika-bfs = 满足 Rika 设定的条件，尝试使用广度优先搜索
not-yet-supported-non-hq-recipes = 尚未支持对非 HQ 配方的求解
solver-result = { $solverName }求解完成

solve-finished = 求解已结束
no-suitable-solver = 没有合适的求解器

view-result = 查看结果
</fluent>
