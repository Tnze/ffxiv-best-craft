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
import { CollectablesShopRefine } from '@/libs/Craft';
import { collectabilityPalette as palette } from './consts';

const props = defineProps<{
    collectableShopRefine: CollectablesShopRefine;
    maxCollectability: number;
    progresBarWidth: number;
}>();

const marginLeft = 8;

const markWidth = computed(() => {
    const pers =
        props.collectableShopRefine.low_collectability /
        props.maxCollectability;
    return props.progresBarWidth * (1 - pers);
});

const lowPos = computed(() => {
    return (
        (props.collectableShopRefine.low_collectability /
            props.maxCollectability) *
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
    calcMark(props.collectableShopRefine.mid_collectability),
);
const highMark = computed(() =>
    calcMark(props.collectableShopRefine.high_collectability),
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
                '\n' +
                [
                    collectableShopRefine.low_collectability,
                    collectableShopRefine.mid_collectability,
                    collectableShopRefine.high_collectability,
                ]
                    .flatMap((v, i) => {
                        if (v == 0) return undefined;
                        return $t('collectability-stage', { v, lv: i + 1 });
                    })
                    .join('\n')
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
                    :width="highMark ?? markWidth - midMark"
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
collectability-stage =
    { $lv ->
        [1] 一档：{ $v } ~
        [2] 二档：{ $v } ~
        [3] 三档：{ $v } ~
       *[other] 未知
    }
</fluent>

<fluent locale="en-US">
required-collectability = Required Collectability
collectability-stage =
    { $lv ->
        [1] First: { $v } ~
        [2] Second: { $v } ~
        [3] Third: { $v } ~
       *[other] Unknown
    }
</fluent>
