<script setup lang="ts">

import { computed } from 'vue'
import { Jobs, Actions } from '../../Craft';

const props = defineProps<{
    scale?: number,
    job: Jobs,
    action: Actions,
    disabled?: boolean,
}>();

const iconUrl = computed(() => new URL(`../../assets/icons/${Jobs[props.job].toLowerCase()}/${props.action}.png`, import.meta.url).href)

const onClick = (event: MouseEvent) => {
    if (!props.disabled)
        (event.target as HTMLElement).classList.add('click-animation')
}
const onAnimationEnd = (event: AnimationEvent) => {
    (event.target as HTMLElement).classList.remove('click-animation')
}

</script>

<template>
    <div class="container">
        <div class="action" @click="onClick" @animationend="onAnimationEnd($event)"></div>
    </div>
</template>

<style scoped>
.container {
    display: inline-block;
    width: calc(48px * v-bind("props.scale || 0.85"));
    height: calc(48px * v-bind("props.scale || 0.85"));
    transform: scale(v-bind("props.scale || 0.85"));
    transform-origin: 0px 0px;
}
.action {
    display: block;
    width: 48px;
    height: 48px;
    background: url("../../assets/icons/icona_frame_tex.png") no-repeat,
        v-bind("'url('+iconUrl+')'") no-repeat top 3px left 4px;
    transform: scale(1);
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
    cursor: v-bind("disabled?'inherit':'pointer'");
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