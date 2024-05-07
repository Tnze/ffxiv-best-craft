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
import { ElDropdown, ElDropdownMenu, ElDropdownItem } from "element-plus";
import { Statistics, rand_simulation } from "@/libs/Analyzer"
import { Actions, Status } from "@/libs/Craft";
import * as d3 from "d3";
import { ref, computed } from "vue";

const props = defineProps<{
    initStatus: Status,
    actions: Actions[],
}>();

const simulationResult = ref<Statistics>()

async function runSimulatios(n: number) {
    simulationResult.value = await rand_simulation(props.initStatus, props.actions, n)
    console.table(simulationResult.value)
}

const arc = d3.arc<d3.PieArcDatum<[string, number]>>()
    .innerRadius(0)
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

const labelRadius = (200 / 2 - 1) * 0.7;
const arcLabel = d3.arc<d3.PieArcDatum<[string, number]>>()
    .innerRadius(labelRadius)
    .outerRadius(labelRadius);
</script>

<template>
    <div>
        <el-dropdown split-button type="default" @click="runSimulatios(1000)"
            @command="(n: number) => runSimulatios(n)">
            {{ $t("run-simulations") }}
            <template #dropdown>
                <el-dropdown-menu>
                    <el-dropdown-item v-for="i in 6" :command="10 ** i">
                        {{ $t("run-n-times", { n: 10 ** i }) }}
                    </el-dropdown-item>
                </el-dropdown-menu>
            </template>
        </el-dropdown>
    </div>
    <!-- <p v-for="d in arcs">{{ arc(d) }}</p> -->
    <svg v-if="simulationResult" width="200" height="200" viewBox="-100 -100 200 200"
        style="max-width: 100%; height: auto; font: 10px sans-serif; margin-top: 15px;">
        <g>
            <path v-for="d in arcs" :d="arc(d) ?? undefined" :fill="color(d.data[0]) as string" stroke="white">
                <title>{{ $t(d.data[0]) + "×" + d.data[1] }}</title>
            </path>
        </g>
        <g text-anchor="middle">
            <text v-for="d in arcs" :transform="`translate(${arcLabel.centroid(d)})`">
                <tspan font-weight="bold" y="-0.4em">{{ $t(d.data[0]) }}</tspan>
                <tspan v-if="d.endAngle - d.startAngle > 0.25" x="0" y="0.7em" fill-opacity="0.7">{{ d.data[1] }}
                </tspan>
            </text>
        </g>
    </svg>
</template>

<fluent locale="zh-CN">
run-simulations = 运行模拟
run-n-times = 运行 { $n } 次模拟

errors = 错误
unfinished = 未完成
fails = 失败
normal = 普通品质
highqual = 高品质
</fluent>

<fluent locale="en-US">
run-simulations = Run simulations
run-n-times = Run simulation { $n } times
</fluent>