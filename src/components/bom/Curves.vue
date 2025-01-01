<!-- 
    This file is part of BestCraft.
    Copyright (C) 2025  Tnze

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
import { useCssVar, UseElementBoundingReturn } from '@vueuse/core';
import { h } from 'vue';

export interface Point {
    x: number;
    y: number;
}

export interface Relation {
    p1: Point;
    p2: Point;
    type?: 'completed' | 'crafted' | 'required' | 'not-required';
}

const props = defineProps<{
    relations: Relation[];
    clipZone: UseElementBoundingReturn;
}>();

const palette = new Map([
    ['required', useCssVar('--el-color-danger')],
    ['crafted', useCssVar('--el-color-warning')],
    ['completed', useCssVar('--el-color-success')],
    ['not-required', useCssVar('--el-color-info')],
]);

function ingLines(props: { rects: Relation }) {
    const { p1, p2, type } = props.rects;
    const d = (p1.y - p2.y) / 2;
    return h('path', {
        d: `M ${p1.x} ${p1.y} C ${p1.x} ${p1.y - d} ${p2.x} ${p2.y + d} ${p2.x} ${p2.y}`,
        stroke: palette.get(type ?? '')?.value ?? 'white',
        fill: 'transparent',
    });
}

defineExpose({});
</script>

<template>
    <div class="canvas">
        <svg
            version="1.1"
            baseProfile="full"
            width="100%"
            height="100%"
            xmlns="http://www.w3.org/2000/svg"
        >
            <defs>
                <clipPath id="cut-off">
                    <rect
                        :x="clipZone.x.value"
                        :y="clipZone.y.value"
                        :width="clipZone.width.value"
                        :height="clipZone.height.value"
                    />
                </clipPath>
            </defs>
            <g clip-path="url(#cut-off)">
                <ing-lines v-for="v of relations" :rects="v" />
            </g>
        </svg>
    </div>
</template>

<style scoped>
.canvas {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    pointer-events: none;

    /* background-color: rgba(255, 100, 255, 0.5); */
    /* border: 1px solid blue; */
    overflow: hidden;
}
</style>
