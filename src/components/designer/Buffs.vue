<script setup lang="ts">
import { computed } from 'vue';
import { Buffs } from '../../Craft';

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
                    url: new URL(`../../assets/buffs/${buffName}_${duration as number}.png`, import.meta.url),
                    duration: undefined
                }
            } else {
                return {
                    url: new URL(`../../assets/buffs/${buffName}.png`, import.meta.url),
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