<!-- 
    This file is part of BestCraft.
    Copyright (C) 2023  <name of author>

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

import { computed } from 'vue'
import hoverUrl from '@/assets/icons/icona_frame_tex.png'
import { Jobs, Actions } from '@/libs/Craft';

const props = defineProps<{
    job: Jobs,
    action: Actions,
    disabled?: boolean,
    active?: boolean,
    noHover?: boolean,
    effect?: 'normal' | 'red-cross' | 'blue-cross' | 'black' | 'sunken',
    cp?: number,
    opacity?: number,
}>();

const iconUrl = computed(() => {
    let actionName: string = props.action;
    if (actionName.endsWith('_fail'))
        actionName = actionName.slice(0, actionName.length - '_fail'.length)
    return new URL(`../../assets/icons/${props.job.toLowerCase()}/${actionName}.png`, import.meta.url).href
})

const hoverLayerOffset = computed(() => {
    switch (props.effect) {
        case 'sunken':
            return 'top -96px left 0px'
        case 'black':
            return 'top 0px left -48px'
        case 'blue-cross':
            return 'top -48px left 0px'
        case 'red-cross':
            return 'top -48px left -48px'
        case undefined:
        case 'normal':
        default:
            return 'top 0px left 0px'
    }
})

const onClick = (event: MouseEvent) => {
    if (!props.disabled)
        (event.target as HTMLElement).firstElementChild!.classList.add('click-animation')
}
const onAnimationEnd = (event: AnimationEvent) => {
    (event.target as HTMLElement).classList.remove('click-animation')
}

</script>

<template>
    <div class="action" @click="onClick" v-bind:class="noHover ? '' : 'action-hover'" v-bind:style="{ opacity: opacity ?? 1 }">
        <div @animationend="onAnimationEnd($event)"></div>
        <div v-if="active" class="active-mask"></div>
        <div v-if="cp != undefined" class="craft-point">{{ cp }}</div>
    </div>
</template>

<style scoped>
.action {
    position: relative;
    display: inline-block;
    width: 48px;
    height: 48px;
    background: v-bind("'url(' + hoverUrl + ') no-repeat ' + hoverLayerOffset + ', url(' + iconUrl + ') no-repeat top 3px left 4px'");
}

.action-hover:hover::after {
    content: "";
    display: block;
    width: 72px;
    height: 72px;
    top: -12px;
    left: -12px;
    position: absolute;
    pointer-events: none;
    background: url("../../assets/icons/icona_frame_tex.png") no-repeat top 0px left -240px;
    cursor: v-bind("disabled ? 'inherit' : 'pointer'");
}

.click-animation {
    content: "";
    display: block;
    width: 64px;
    height: 64px;
    top: -8px;
    left: -8px;
    position: absolute;
    background: url("../../assets/icons/icona_frame_tex.png") no-repeat top -72px left -240px;
    animation: clickable-wave 0.3s;
}

.active-mask::after {
    content: "";
    width: 48px;
    height: 48px;
    position: absolute;
    pointer-events: none;
    background-image: url("../../assets/icons/icona_frame_tex.png");
    animation: active 0.25s steps(1, start) infinite;
}

@keyframes clickable-wave {
    from {
        transform: scale(0.3);
        opacity: 1;
    }

    to {
        transform: scale(1);
        opacity: 0;
    }
}

@keyframes active {
    0% {
        background-position: top 0px left -96px;
    }

    12.5% {
        background-position: top 0px left -144px;
    }

    25% {
        background-position: top 0px left -192px;
    }

    37.5% {
        background-position: top -48px left -96px;
    }

    50% {
        background-position: top -48px left -144px;
    }

    62.5% {
        background-position: top -48px left -192px;
    }

    75% {
        background-position: top -96px left -96px;
    }

    87.5% {
        background-position: top -96px left -144px;
    }

    100% {
        background-position: top -96px left -144px;
    }
}

.craft-point {
    position: absolute;
    color: #fff;
    font-size: 13px;
    top: 34px;
    left: 3px;
    user-select: none;
    text-shadow:
        #000 0px 0px 2px,
        #000 0px 0px 3px,
        #000 0px 0px 4px,
        #fff 0px 0px 5px,
        #fff 0px 0px 6px;
}
</style>
