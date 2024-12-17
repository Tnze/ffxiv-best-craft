<!-- 
    This file is part of BestCraft.
    Copyright (C) 2024  Tnze

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
import {
    ElDropdown,
    ElDropdownMenu,
    ElDropdownItem,
    ElSwitch,
    ElForm,
    ElFormItem,
    ElButton,
    ElDivider,
    ElDescriptions,
    ElDescriptionsItem,
} from 'element-plus';
import {
    Statistics,
    CollectableStatistics,
    rand_simulation,
    rand_collectables_simulation,
    calc_attributes_scope,
    Scope,
} from '@/libs/Analyzer';
import { Actions, CollectablesShopRefine, Status } from '@/libs/Craft';
import * as d3 from 'd3';
import { ref, computed, watch, reactive } from 'vue';
import useStore from '@/stores/designer';

const props = defineProps<{
    initStatus: Status;
    actions: Actions[];
    collectableShopRefine?: CollectablesShopRefine;
}>();
const store = useStore();
const defaultSimulationCounts = 1000;
const maximiumSimulatonPow = 5;

const simulationResult = ref<Statistics | CollectableStatistics>();
const simulationButtonDisabled = ref(false);
const options = reactive(store.options.analyzerOptions);

const attributesScope = ref<Scope>();

async function runSimulatios(n: number) {
    simulationResult.value = undefined;
    simulationButtonDisabled.value = true;
    try {
        if (props.collectableShopRefine == undefined) {
            simulationResult.value = await rand_simulation(
                props.initStatus,
                props.actions,
                n,
                options.ignoreErrors,
            );
        } else {
            simulationResult.value = await rand_collectables_simulation(
                props.initStatus,
                props.actions,
                n,
                options.ignoreErrors,
                props.collectableShopRefine,
            );
        }
    } finally {
        simulationButtonDisabled.value = false;
    }
}

async function calcScope() {
    try {
        attributesScope.value = await calc_attributes_scope(
            props.initStatus,
            props.actions,
        );
    } catch {}
}

let autoRunTimeout: any = null;
watch(
    () => [props.actions, props.initStatus],
    () => {
        if (autoRunTimeout !== null) {
            clearTimeout(autoRunTimeout);
        }
        const thisTimeout = setTimeout(async () => {
            await runSimulatios(defaultSimulationCounts);
            await calcScope();
            if (autoRunTimeout == thisTimeout) {
                autoRunTimeout = null;
            }
        }, 200);
        autoRunTimeout = thisTimeout;
    },
);

const arc = d3
    .arc<d3.PieArcDatum<[string, number]>>()
    .innerRadius(50)
    .outerRadius(200 / 2 - 1);

const pie = d3
    .pie<[string, number]>()
    .sort(null)
    .value(d => d[1]);

const color = d3
    .scaleOrdinal()
    .domain([
        'errors',
        'unfinished',
        'fails',
        'normal',
        'highqual',
        'no_collectability',
        'low_collectability',
        'middle_collectability',
        'high_collectability',
    ])
    .range(d3.schemeSpectral[9])
    .unknown('#ccc');

const arcs = computed(() => {
    const statistics = simulationResult.value;
    if (statistics == undefined) return undefined;
    const data: [string, number][] = Object.entries(statistics).filter(
        d => d[1] > 0,
    );
    return pie(data);
});

const labelRadius = (200 / 2 - 1) * 0.75;
const arcLabel = d3
    .arc<d3.PieArcDatum<[string, number]>>()
    .innerRadius(labelRadius)
    .outerRadius(labelRadius);
</script>

<template>
    <el-form>
        <el-form-item :label="$t('ignore-errors')">
            <el-switch v-model="options.ignoreErrors" />
        </el-form-item>
        <el-form-item>
            <el-dropdown
                split-button
                type="default"
                @click="runSimulatios(defaultSimulationCounts)"
                @command="(n: number) => runSimulatios(n)"
                :disabled="simulationButtonDisabled"
            >
                {{ $t('run-simulations') }}
                <template #dropdown>
                    <el-dropdown-menu>
                        <el-dropdown-item
                            v-for="i in maximiumSimulatonPow"
                            :command="10 ** i"
                        >
                            {{ $t('run-n-times', { n: 10 ** i }) }}
                        </el-dropdown-item>
                    </el-dropdown-menu>
                </template>
            </el-dropdown>
        </el-form-item>
        <Transition>
            <el-form-item v-if="simulationResult">
                <svg
                    width="200"
                    height="200"
                    viewBox="-100 -100 200 200"
                    style="
                        max-width: 100%;
                        height: auto;
                        font: 10px sans-serif;
                        margin: 15px;
                    "
                >
                    <g>
                        <path
                            v-for="d in arcs"
                            :d="arc(d) ?? undefined"
                            :fill="color(d.data[0]) as string"
                            stroke="white"
                        >
                            <title>{{ $t(d.data[0]) + '×' + d.data[1] }}</title>
                        </path>
                    </g>
                    <g text-anchor="middle">
                        <text
                            v-for="d in arcs"
                            :transform="`translate(${arcLabel.centroid(d)})`"
                        >
                            <tspan font-weight="bold" y="-0.4em">
                                {{ $t(d.data[0]) }}
                            </tspan>
                            <tspan
                                v-if="d.endAngle - d.startAngle > 0.1"
                                x="0"
                                y="0.7em"
                                fill-opacity="0.7"
                            >
                                {{ d.data[1] }}
                            </tspan>
                        </text>
                    </g>
                </svg>
                <el-descriptions v-if="simulationResult" :column="1" border>
                    <el-descriptions-item
                        v-for="[key, val] in Object.entries(
                            simulationResult,
                        ).sort((a, b) => b[1] - a[1])"
                        :label="$t(key)"
                    >
                        {{ val }}
                    </el-descriptions-item>
                </el-descriptions>
            </el-form-item>
        </Transition>
        <el-divider />
        <el-form-item>
            <el-button @click="calcScope">{{ $t('calc-scope') }}</el-button>
        </el-form-item>
        <Transition>
            <el-form-item
                :label="$t('craftsmanship-range')"
                v-if="attributesScope?.craftsmanship_range"
            >
                {{ attributesScope.craftsmanship_range[0] ?? '' }} ~
                {{ attributesScope.craftsmanship_range[1] ?? '' }}
            </el-form-item>
        </Transition>
        <Transition>
            <el-form-item
                :label="$t('control-range')"
                v-if="attributesScope?.control_range"
            >
                {{ attributesScope.control_range }} ~
            </el-form-item>
        </Transition>
    </el-form>
</template>

<style scoped>
.action-queue {
    margin-left: 0;
    line-height: 14px;
}

.v-enter-active,
.v-leave-active {
    transition: all 0.5s ease;
}

.v-enter-from,
.v-leave-to {
    opacity: 0;
}
</style>

<fluent locale="zh-CN">
run-simulations = 运行仿真
run-n-times = 运行 { $n } 次仿真
action-queue = 技能队列
ignore-errors = 忽略错误

empty = 空

errors = 错误
unfinished = 未完成
fails = 失败
normal = 普通品质
highqual = 高品质

no_collectability = 无收藏价值
low_collectability = 收藏价值一档
middle_collectability = 收藏价值二档
high_collectability = 收藏价值三档

calc-scope = 计算装备属性适配范围
craftsmanship-range = { craftsmanship }范围
control-range = { control }范围
</fluent>

<fluent locale="en-US">
run-simulations = Run simulations
run-n-times = Run simulation { $n } times
action-queue = Action Queue
ignore-errors = Ignore errors

empty = Empty

errors = Errors
unfinished = Unfinished
fails = Fails
normal = Normal
highqual = High-quality

calc-scope = Calculate the range of adaptive gearsets
craftsmanship-range = { craftsmanship } range
control-range = { control } range
</fluent>
