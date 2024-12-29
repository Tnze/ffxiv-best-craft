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
import { h, Ref } from 'vue';

type Point = { x: number; y: number };
const props = defineProps<{
    items: [Point, Point][];
}>();

function ingLines(props: { rects: [Point, Point] }) {
    const [p1, p2] = props.rects;
    // return h('path', {
    //     d: `M ${p1.x} ${p1.y} L ${p2.x} ${p2.y}`,
    //     stroke: 'white',
    // });
    const d = (p1.y - p2.y) / 2;
    return h('path', {
        d: `M ${p1.x} ${p1.y} C ${p1.x} ${p1.y - d} ${p2.x} ${p2.y + d} ${p2.x} ${p2.y}`,
        stroke: 'white',
        fill: 'transparent',
    });
}
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
            <ing-lines v-for="v of items" :rects="v" />
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
