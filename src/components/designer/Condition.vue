<script setup lang="ts">
import { computed } from 'vue'
import { Conditions } from '../../Craft'

const props = defineProps<{
    cond: Conditions
}>();


</script>

<template>
    <span id="icon" :class="Conditions[props.cond].toLowerCase()">{{ cond == Conditions.Normal ? "○" : "●" }}</span>
    <br />
    <span id="text">{{ Conditions[cond] }}</span>
</template>

<style>
#icon {
    font-size: x-large;
    vertical-align: middle;
}
#text {
    font-size: small;
    vertical-align: middle;
}
.normal {
    color: black;
}
.good {
    color: red;
}
.excellent {
    color: red;
    animation: excellent-color 1s infinite;
}
@keyframes excellent-color {
    0% {
        color: black;
    }
    12% {
        color: red;
    }
    24% {
        color: black;
    }
    36% {
        color: yellow;
    }
    48% {
        color: blue;
    }
    60% {
        color: green;
    }
    72% {
        color: darkblue;
    }
    100% {
        color: purple;
    }
}
.poor {
    color: black;
}
.centered {
    color: yellow;
}
.sturdy {
    color: blue;
}
.pliant {
    color: green;
}
.malleable {
    color: darkblue;
}
.primed {
    color: purple;
}
</style>