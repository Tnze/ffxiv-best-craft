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
// import * as d3 from "d3";
import { ref } from "vue";

const props = defineProps<{
    initStatus: Status,
    actions: Actions[],
}>();

const simulationResult = ref<Statistics>()

async function runSimulatios(n: number) {
    simulationResult.value = await rand_simulation(props.initStatus, props.actions, n)
    console.table(simulationResult.value)
}

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
</template>

<fluent locale="zh-CN">
run-simulations = 运行模拟
run-n-times = 运行 { $n } 次模拟
</fluent>

<fluent locale="en-US">
run-simulations = Run simulations
</fluent>