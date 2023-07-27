<script setup lang="ts">
import { VNode, computed, h, onMounted, onUnmounted, ref } from 'vue';
import { ElText, ElTimeline, ElTimelineItem, ElButton, ElScrollbar, ElIcon, ElResult } from 'element-plus';
import { Loading } from "@element-plus/icons-vue";
import { useGuideStore } from '../../store';
import { dfs_solve, formatDuration, reflect_solve, rika_solve, rika_solve_tnzever } from "../../Solver";
import { useFluent } from 'fluent-vue'
import { Actions, Jobs, Status, compareStatus, high_quality_probability, newStatus, simulate } from '../../Craft';
import ActionQueue from '../designer/ActionQueue.vue'
import { useRouter } from 'vue-router';

const store = useGuideStore()
const { $t } = useFluent()
const router = useRouter()

const solverTitle = ref<'solve-finished' | 'no-suitable-solver'>()
const runningSolverName = ref<string>()
const solverLines = ref<SolverLine[]>([])

interface SolverLine {
    title: string,
    actionQueue?: {
        job: Jobs
        slots: {
            id: number
            action: Actions
        }[],
        errList: {
            pos: number;
            err: string;
        }[],
    }
    timestamp?: string,
    type?: 'primary' | 'success' | 'warning' | 'danger' | 'info'
}

async function selectSolver() {
    if (store.recipe == null ||
        store.craftType == null ||
        store.craftTypeAttr == null ||
        store.recipeInfo == null ||
        store.recipeLevel == null) {

        router.replace('see-recipe')
        return
    }

    const attr = store.craftTypeAttr
    const job = store.craftType
    const recipe = store.recipe
    const recipeLevel = store.recipeLevel
    const useManipulation = store.manipulation
    const status = await newStatus(attr, recipe, recipeLevel)

    const solveResults: { status: Status, actions: Actions[] }[] = []
    const runSolver = async (name: string, func: () => Promise<Actions[]>) => {
        runningSolverName.value = $t(name)
        const startTime = new Date().getTime()
        const result = await func()
        const stopTime = new Date().getTime()
        runningSolverName.value = undefined

        const slots = result.map((action, id) => ({ id, action }))
        const { status: fstatus, errors: errList } = await simulate(status, result)

        solveResults.push({ status: fstatus, actions: result })
        solverLines.value.push({
            title: $t('solver-result', {
                solverName: $t(name),
                quality: fstatus.quality,
                highQuality: await high_quality_probability(fstatus) ?? '',
            }),
            actionQueue: { job, slots, errList },
            timestamp: formatDuration(stopTime - startTime),
            type: fstatus.progress < recipe.difficulty ? 'danger' :
                fstatus.quality < recipe.quality ? 'warning' : 'primary'
        })
    }

    if (!store.recipeInfo.can_hq) {
        solverLines.value.push({
            type: 'warning',
            title: $t('not-yet-supported-non-hq-recipes'),
        })
    } else {
        if (store.recipe.conditions_flag == 15) {
            // 普通配方
            // 优先尝试暴力求解
            if (store.craftTypeAttr.level >= store.recipe.job_level + 10) {
                await runSolver('dfs-solver', () => dfs_solve(status, 6, false))
            }
        } else {
            // 高难配方
        }
        // 如果符合条件，运行Rika求解
        if (store.recipe.rlv >= 560 && store.recipe.difficulty >= 70) {
            await runSolver('rika-bfs-solver', () => rika_solve(status))
            await runSolver('tnze-bfs-solver', () => rika_solve_tnzever(status, useManipulation, 8, true, true))
        }
        // 以上均无结果，则运行闲静手法DP兜底
        // 坚信手法推不满的时候也运行闲静手法，有些特殊情况闲静手法品质更优
        solveResults.sort((a, b) => compareStatus(a.status, b.status))
        if (solveResults.length == 0 || solveResults[0].status.quality < recipe.quality) {
            await runSolver('reflect-solver', () => reflect_solve(status, useManipulation))
        }
    }

    if (solveResults.length == 0) {
        solverTitle.value = 'no-suitable-solver';
    } else {
        solverTitle.value = 'solve-finished'

        return () => {
            // sort the results and choice the first one
            solveResults.sort((a, b) => compareStatus(a.status, b.status))
            const { actions } = solveResults[0]
            store.setBestResult(actions)
        }
    }
}

store.setCurrentPage('solving')
let cancelLast = () => { }
onMounted(async () => {
    cancelLast()
    let canceled = false
    cancelLast = () => { canceled = true }
    const writeStore = await selectSolver()
    if (!canceled && writeStore != undefined)
        writeStore()
})
onUnmounted(() => {
    cancelLast()
    cancelLast = () => { }
})

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
                        <el-button @click="$router.push('result')" v-if="solverSuccessed" type="primary">
                            {{ $t('view-result') }}
                        </el-button>
                    </template>
                </el-result>
            </el-text>
        </div>

        <div class="button-box" v-if="runningSolverName">
            <el-button @click="$router.replace('see-recipe')">{{ $t('back') }}</el-button>
        </div>
        <el-scrollbar always>
            <el-timeline>
                <el-timeline-item v-for="v in solverLines" :timestamp="v.timestamp" :type="v.type">
                    <el-text>{{ v.title }}</el-text>
                    <ActionQueue v-if="v.actionQueue" :list="v.actionQueue.slots" :job="v.actionQueue.job"
                        :err-list="v.actionQueue.errList" />
                </el-timeline-item>
            </el-timeline>
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
reflect-solver = 动态规划
rika-bfs-solver = Rika原版BFS
tnze-bfs-solver = Tnze修改版BFS

solving = 正在运行 { $solverName } 求解器
back = 返回

not-yet-supported-non-hq-recipes = 尚未支持对非 HQ 配方的求解
solver-result = { $solverName }求解完成，品质：{ $quality }，HQ率：{ $highQuality }%

solve-finished = 求解已结束
no-suitable-solver = 没有合适的求解器

view-result = 查看结果
</fluent>

<fluent locale="en-US">
dfs-solver = Deep First Search
reflect-solver = Dynamic Programing
rika-bfs-solver = Rika original edition BFS
tnze-bfs-solver = Tnze modified version BFS

solving = Runinng { $solverName } solver
back = Back

not-yet-supported-non-hq-recipes = Not yet supported for solving non HQ recipes
solver-result = { $solverName } solving finished, Quality: { $quality }, HQ rate: { $highQuality }%

solve-finished = Solving finished
no-suitable-solver = No suitable solver found

view-result = View Results
</fluent>
        