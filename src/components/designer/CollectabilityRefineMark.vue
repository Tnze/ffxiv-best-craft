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
import { Collectability } from '@/libs/Craft';

const props = defineProps<{
    collectability: Collectability;
    maxCollectability: number;
    progresBarWidth: number;
}>();

const palette = ['#79c7ec', '#fbc800', '#c0ffc0'];
const marginLeft = 8;

const markWidth = computed(() => {
    const pers =
        props.collectability.low_collectability / props.maxCollectability;
    return props.progresBarWidth * (1 - pers);
});

const lowPos = computed(() => {
    return (
        (props.collectability.low_collectability / props.maxCollectability) *
        props.progresBarWidth
    );
});

function calcMark(collectability: number) {
    if (collectability == 0) return undefined;
    const pers = collectability / props.maxCollectability;
    const absPos = pers * props.progresBarWidth;
    return absPos - lowPos.value;
}

const midMark = computed(() =>
    calcMark(props.collectability.mid_collectability),
);
const highMark = computed(() =>
    calcMark(props.collectability.high_collectability),
);
</script>

<template>
    <svg
        class="mark"
        version="1.1"
        baseProfile="full"
        :width="markWidth + marginLeft"
        height="25"
        xmlns="http://www.w3.org/2000/svg"
    >
        <title>
            {{
                $t('required-collectability') +
                [
                    collectability.low_collectability,
                    collectability.mid_collectability,
                    collectability.high_collectability,
                ]
                    .flatMap((v, i) =>
                        v != 0
                            ? '\n' + $t('collectability-' + (i + 1), { v })
                            : undefined,
                    )
                    .join('')
            }}
        </title>
        <defs>
            <g id="Mark">
                <circle cx="0" cy="7" r="3" />
                <rect
                    width="8"
                    height="8"
                    transform="translate(0,11) rotate(45)"
                />
                <rect
                    width="2.5"
                    height="2.5"
                    transform="translate(0,14) rotate(45)"
                    fill="black"
                />
            </g>
        </defs>
        <g :transform="`translate(${marginLeft},0)`">
            <g stroke="black">
                <rect y="5" :width="midMark" height="4" :fill="palette[0]" />
                <rect
                    v-if="midMark"
                    :x="midMark"
                    y="5"
                    :width="highMark - midMark"
                    height="4"
                    :fill="palette[1]"
                />
                <rect
                    v-if="highMark"
                    :x="highMark"
                    y="5"
                    :width="markWidth - highMark"
                    height="4"
                    :fill="palette[2]"
                />
            </g>
            <g stroke="black">
                <use xlink:href="#Mark" :fill="palette[0]" />
                <use
                    v-if="midMark"
                    :x="midMark"
                    xlink:href="#Mark"
                    :fill="palette[1]"
                />
                <use
                    v-if="highMark"
                    :x="highMark"
                    xlink:href="#Mark"
                    :fill="palette[2]"
                />
            </g>
        </g>
    </svg>
</template>

<style lang="css" scoped>
.mark {
    float: right;
}
</style>

<fluent locale="zh-CN">
required-collectability = 所需收藏价值
collectability-1 = 一档：{ $v } ~
collectability-2 = 二档：{ $v } ~
collectability-3 = 三档：{ $v } ~
</fluent>
