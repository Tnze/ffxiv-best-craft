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
import { useDark } from '@vueuse/core';
import { Conditions } from '../../Craft'

const props = defineProps<{
    cond: Conditions
}>();

const dark = useDark()

</script>

<template>
    <span id="icon" :class="props.cond.toLowerCase()">
        {{ cond == Conditions.Normal && !dark ? "○" : "●" }}
        <span id="text">{{ $t(cond.toLowerCase()) }}</span>
    </span>
</template>

<style>
#icon {
    font-size: x-large;
    vertical-align: middle;
    /* vertical-align: baseline; */
}

#text {
    font-size: small;
    vertical-align: middle;
    /* vertical-align: baseline; */
}

/* .normal {
    color: var(--el-text-color-primary);
} */

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


<fluent locale="zh-CN">
condition = 制作状态
</fluent>

<fluent locale="en-US">
condition = Condition
</fluent>