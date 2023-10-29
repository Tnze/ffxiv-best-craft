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
import { computed } from 'vue';
import { Buffs } from '@/libs/Craft';

const props = defineProps<{
    buffs: Buffs
}>()

const fakeBuffs = ['careful_observation_used', 'heart_and_soul_used', 'touch_combo_stage', 'observed']

const buffsDisplay = computed<{
    url: URL,
    duration: number | undefined
}[]>(() => {
    return Object.entries(props.buffs)
        .filter(v => !fakeBuffs.includes(v[0]))
        .filter(v => v[1] > 0)
        .map(([buffName, duration]) => {
            if (buffName == 'inner_quiet') {
                return {
                    url: new URL(`@/libs/assets/buffs/${buffName}_${duration as number}.png`, import.meta.url),
                    duration: undefined
                }
            } else {
                return {
                    url: new URL(`@/libs/assets/buffs/${buffName}.png`, import.meta.url),
                    duration: buffName == 'heart_and_soul' ? undefined : duration as number,
                }
            }
        })
})
</script>

<template>
    <div class="container">
        <div class="buff" v-for="buffDisplay in buffsDisplay" :duration="buffDisplay.duration">
            <img class="buff-img" :src="buffDisplay.url.href" />
        </div>
    </div>
</template>

<style scoped>
.container {
    display: flex;
    min-height: 38px;
}

.buff {
    margin-right: 5px;
}

.buff::after {
    content: attr(duration);
    display: block;
    text-align: center;
    margin-top: -8px;
    margin-bottom: -5px;
    font-size: 0.9em;
    color: var(--el-text-color-regular);
}

.buff-img {
    max-width: 24px;
    pointer-events: none;
    user-select: none;
}
</style>