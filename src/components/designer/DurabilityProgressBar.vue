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
import { computed } from 'vue';

const props = defineProps<{
    modelValue: number;
    max: number;
}>();

const blockWidth = 12;
const blockMargin = 2;
const blockTotal = blockWidth + blockMargin;

const blockHeight = 13;

const durabilityColor = [
    { color: '#FF5757', colorMid: '#FF7C7C', percentage: 20 },
    { color: '#FFE957', colorMid: '#FFF67C', percentage: 50 },
    { color: '#57bbff', colorMid: '#7ce0ff', percentage: 100 },
];

const color = computed(() => {
    const percentage = Math.floor((props.modelValue / props.max) * 100);
    return durabilityColor.find(p => p.percentage >= percentage)!;
});
</script>

<template>
    <div class="progress-bar">
        <svg
            version="1.1"
            baseProfile="full"
            :width="blockTotal * Math.ceil(max / 10) + 3"
            :height="blockHeight + 3"
            xmlns="http://www.w3.org/2000/svg"
        >
            <defs>
                <linearGradient id="Gradient1" x1="0" x2="0" y1="0" y2="1">
                    <stop offset="0%" :stop-color="color?.color" />
                    <stop offset="50%" :stop-color="color.colorMid" />
                    <stop offset="100%" :stop-color="color?.color" />
                </linearGradient>
                <clipPath id="cut-off">
                    <rect
                        :width="
                            (modelValue / 10) * blockTotal - blockMargin / 2
                        "
                        :height="blockHeight"
                        style="transition: 0.1s width ease"
                    />
                </clipPath>
            </defs>
            <g
                stroke="black"
                stroke-linecap="round"
                transform="translate(1.5, 1.5)"
            >
                <rect
                    v-for="i in Math.ceil(max / 10)"
                    :x="(i - 1) * blockTotal"
                    :width="blockWidth"
                    :height="blockHeight"
                    fill="#424242"
                />
                <rect
                    v-for="i in Math.ceil(max / 10)"
                    :x="(i - 1) * blockTotal"
                    :width="blockWidth"
                    :height="blockHeight"
                    fill="url(#Gradient1)"
                    clip-path="url(#cut-off)"
                />
            </g>
        </svg>
    </div>
</template>

<style lang="css" scoped>
.progress-bar {
    padding: 5px 0;
}
</style>
