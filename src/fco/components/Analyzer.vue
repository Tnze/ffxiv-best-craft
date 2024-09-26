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
import { ref, computed, watch } from "vue";
import { NSwitch, NDropdown, DropdownOption, NButton, NDivider, NForm, NFormItem } from "naive-ui";
import { Statistics, rand_simulation, calc_attributes_scope, Scope } from "@/libs/Analyzer"
import { Actions, Status } from "@/libs/Craft";
import * as d3 from "d3";
import { useFluent } from "fluent-vue";

const props = defineProps<{
    initStatus: Status,
    actions: Actions[],
}>();
const { $t } = useFluent()
const defaultSimulationCounts = 1000
const maximiumSimulatonPow = 5

const simulationResult = ref<Statistics>()
const simulationButtonDisabled = ref(false)
const ignoreErrors = ref(true)
const attributesScope = ref<Scope>()

const simulationsDropdownOptions: DropdownOption[] = []
for (let i = 0; i < maximiumSimulatonPow; i++) {
    const n = 10 ** i
    simulationsDropdownOptions.push({
        label: $t("run-n-times", { n }),
        key: n
    })
}

async function runSimulatios(n: number) {
    simulationResult.value = undefined
    simulationButtonDisabled.value = true
    try {
        simulationResult.value = await rand_simulation(props.initStatus, props.actions, n, ignoreErrors.value)
    } finally {
        simulationButtonDisabled.value = false
    }
}

async function calcScope() {
    try {
        attributesScope.value = await calc_attributes_scope(props.initStatus, props.actions)
    } catch {
    }
}

let autoRunTimeout: any = null;
watch(
    () => [props.actions, props.initStatus],
    () => {
        if (autoRunTimeout !== null) {
            clearTimeout(autoRunTimeout)
        }
        const thisTimeout = setTimeout(
            async () => {
                await runSimulatios(defaultSimulationCounts)
                await calcScope()
                if (autoRunTimeout == thisTimeout) {
                    autoRunTimeout = null
                }
            },
            200,
        )
        autoRunTimeout = thisTimeout
    },
)

const arc = d3.arc<d3.PieArcDatum<[string, number]>>()
    .innerRadius(50)
    .outerRadius(200 / 2 - 1);

const pie = d3.pie<[string, number]>()
    .sort(null)
    .value(d => d[1]);

const color = d3.scaleOrdinal()
    .domain(["errors", "unfinished", "fails", "normal", "highqual"])
    .range(d3.schemeSpectral[5])
    .unknown("#ccc");

const arcs = computed(() => {
    const statistics = simulationResult.value;
    if (statistics == undefined) return undefined;
    const data: [string, number][] = Object.entries(statistics).filter(d => d[1] > 0);
    return pie(data)
})

const labelRadius = (200 / 2 - 1) * 0.75;
const arcLabel = d3.arc<d3.PieArcDatum<[string, number]>>()
    .innerRadius(labelRadius)
    .outerRadius(labelRadius);
</script>

<template>
    <n-form label-placement="left" label-width="auto">
        <n-form-item :label="$t('ignore-errors')">
            <n-switch v-model:value="ignoreErrors" />
        </n-form-item>
        <n-form-item>
            <n-dropdown :options="simulationsDropdownOptions" @select="runSimulatios">
                <n-button :disabled="simulationButtonDisabled">{{ $t("run-simulations") }}</n-button>
            </n-dropdown>
        </n-form-item>
        <Transition>
            <n-form-item v-if="simulationResult">
                <svg width="200" height="200" viewBox="-100 -100 200 200"
                    style="max-width: 100%; height: auto; font: 10px sans-serif; margin-top: 15px;">
                    <g>
                        <path v-for="d in arcs" :d="arc(d) ?? undefined" :fill="color(d.data[0]) as string"
                            stroke="white">
                            <title>{{ $t(d.data[0]) + "×" + d.data[1] }}</title>
                        </path>
                    </g>
                    <g text-anchor="middle">
                        <text v-for="d in arcs" :transform="`translate(${arcLabel.centroid(d)})`">
                            <tspan font-weight="bold" y="-0.4em">
                                {{ $t(d.data[0]) }}
                            </tspan>
                            <tspan v-if="d.endAngle - d.startAngle > 0.1" x="0" y="0.7em" fill-opacity="0.7">
                                {{ d.data[1] }}
                            </tspan>
                        </text>
                    </g>
                </svg>
            </n-form-item>
        </Transition>
        <n-divider />
        <n-form-item>
            <n-button @click="calcScope">{{ $t('calc-scope') }}</n-button>
        </n-form-item>
        <Transition>
            <n-form-item :label="$t('craftsmanship-range')" v-if="attributesScope?.craftsmanship_range">
                {{ attributesScope.craftsmanship_range[0] ?? "" }} ~ {{ attributesScope.craftsmanship_range[1] ?? "" }}
            </n-form-item>
        </Transition>
        <Transition>
            <n-form-item :label="$t('control-range')" v-if="attributesScope?.control_range">
                {{ attributesScope.control_range }} ~
            </n-form-item>
        </Transition>
    </n-form>
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