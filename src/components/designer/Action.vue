<script setup lang="ts">

import { computed } from 'vue'
import { Jobs } from '../../Craft';

const props = defineProps<{
    scale?: number,
    job: Jobs,
    name: string,
}>();

const iconUrl = computed(() => new URL(`../../assets/icons/${Jobs[props.job].toLowerCase()}/${props.name}.png`, import.meta.url).href)

const onClick = (event: MouseEvent) => {
    (event.target as HTMLElement).classList.add('click-animation')
}
const onAnimationEnd = (event: AnimationEvent) => {
    (event.target as HTMLElement).classList.remove('click-animation')
}

</script>

<template>
    <div class="action" @click="onClick" @animationend="onAnimationEnd($event)"></div>
</template>

<style scoped>
.action {
    display: inline-block;
    width: 48px;
    height: 48px;
    background: url("../../assets/icons/icona_frame_tex.png") no-repeat,
        v-bind("'url('+iconUrl+')'") no-repeat top 3px left 4px;
    transform: scale(v-bind("props.scale || 0.85"));
}
.action:hover:after {
    content: "";
    display: block;
    width: 72px;
    height: 72px;
    top: -12px;
    left: -12px;
    position: absolute;
    background: url("../../assets/icons/icona_frame_tex.png") no-repeat top 0px
        left -240px;
}
.click-animation:before {
    content: "";
    display: block;
    width: 64px;
    height: 64px;
    top: -8px;
    left: -8px;
    position: absolute;
    background: url("../../assets/icons/icona_frame_tex.png") no-repeat top -72px
        left -240px;
    transform: scale(0);
    animation: clickable-wave 0.3s;
}
.click-animation:active:before {
    animation: clickable-wave 0s;
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
</style>