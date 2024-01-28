<!-- 
    This file is part of BestCraft.
    Copyright (C) 2023  Tnze

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
import { ElProgress } from 'element-plus';
import { computed } from 'vue'
import { Attributes, Status } from '@/libs/Craft';
import Buffs from './Buffs.vue';
import { Enhancer } from '../attr-enhancer/Enhancer';
import Condition from './Condition.vue'

const props = defineProps<{
    status: Status;
    attributes: Attributes;
    enhancers: Enhancer[];
    showCondition: boolean;
}>()

const emits = defineEmits<{
    (event: 'click-attributes'): void
    (event: 'click-quality'): void
}>()

const durability = computed<number>(() => {
    return props.status === undefined
        ? 100
        : props.status.durability / props.status.recipe.durability * 100
})
const progress = computed<number>(() => {
    return props.status === undefined
        ? 100
        : props.status.progress / props.status.recipe.difficulty * 100
})
const remainingProgress = computed(() => {
    return props.status.recipe.difficulty - props.status.progress
})
const quality = computed<number>(() => {
    return props.status === undefined
        ? 100
        : props.status.quality / props.status.recipe.quality * 100
})

const progressColor = computed<string>(() => {
    if (props.status.progress >= props.status.recipe.difficulty)
        return '#13CE66'
    if (props.status.durability <= 0)
        return '#F56C6C'
    return '#409EFF'
})
const qualityColor = computed<string>(() => {
    if (props.status.quality >= props.status.recipe.quality)
        return '#13CE66'
    return '#409EFF'
})

const durabilityColor = [
    { color: '#FF999E', percentage: 20 },
    { color: '#FFD470', percentage: 50 },
    { color: '#62C3FF', percentage: 100 },
]

const craftPointPercentage = computed(() => props.status?.craft_points / props.status?.attributes.craft_points * 100)
</script>

<template>
    <div class="conatiner">
        <div id="durability-and-condition">
            <div id="durability">
                <span class="bar-title">{{ $t('durability') }}</span> {{ status?.durability }} / {{
                    status?.recipe.durability }}
                <el-progress :stroke-width="14" :show-text="false" :percentage="durability" :color="durabilityColor"
                    striped />
                <div id="craft-point"></div>
                <span class="bar-title">{{ $t('craft-point') }}</span> {{ status?.craft_points }} / {{
                    status?.attributes.craft_points }}
                <el-progress :stroke-width="12" :percentage="craftPointPercentage" :show-text="false" color="#FF9999"
                    striped />
                <Condition v-if="showCondition" :cond="status.condition" />
            </div>
        </div>
        <div id="progress-and-buffs">
            <span class="bar-title">
                {{ $t('progress') }}
                {{ status?.progress }} / {{ status?.recipe.difficulty }}
                <template v-if="remainingProgress > 0">
                    {{ $t('remaining') }} {{ remainingProgress }}
                </template>
            </span>
            <el-progress :percentage="progress" :color="progressColor" :show-text="false" :stroke-width="10" />
                <br />
            <span class="bar-title">
                {{ $t('quality') }}
                {{ status?.quality }} / {{ status?.recipe.quality }}
            </span>
            <el-progress :percentage="quality" :color="qualityColor" :show-text="false" :stroke-width="10" />
            <Buffs id="buffs" :buffs="status.buffs" />
        </div>
        <div id="attributes">
            <div>
                {{ $t('display-attrs', { what: $t('level'), value: status?.attributes.level }) }}
                <br />
                {{ $t('display-attrs', {
                    what: $t('craftsmanship'),
                    value: attributes.craftsmanship + enhancers
                        .filter(v => v.cm && v.cm_max)
                        .map(v => Math.min(
                            status?.attributes.craftsmanship * v.cm!,
                            v.cm_max!
                        ))
                        .map(cm => ` + ${cm}`).join('')
                })
                }}
                <br />
                {{ $t('display-attrs', {
                    what: $t('control'),
                    value: attributes.control + enhancers
                        .filter(v => v.ct && v.ct_max)
                        .map(v => Math.min(
                            status?.attributes.control * v.ct!,
                            v.ct_max!
                        ))
                        .map(ct => ` + ${ct}`).join('')
                })
                }}
                <br />
                {{ $t('display-attrs', {
                    what: $t('craft-point'),
                    value: attributes.craft_points + enhancers
                        .filter(v => v.cp && v.cp_max)
                        .map(v => Math.min(
                            status?.attributes.craft_points * v.cp!,
                            v.cp_max!
                        ))
                        .map(cp => ` + ${cp}`).join('')
                })
                }}
            </div>
        </div>
    </div>
</template>

<style scoped>
.conatiner {
    width: 100%;
    display: flex;
    font-size: 14px;
    color: var(--el-text-color-regular);
}

#durability-and-condition {
    padding: 3px 10px 5px 5px;
    flex: none;
}

#durability {
    padding: 10px;
}

#craft-point {
    margin-top: 7px;
}

#progress-and-buffs {
    padding: 7px;
    flex-grow: 5;
}

#buffs {
    margin-top: 7px;
}

#attributes {
    font-size: 14px;
    line-height: 1.5;
    padding: 0px 20px 0px 0px;
    flex-grow: 2;
    text-align: right;
    color: var(--el-text-color-secondary);
}

.attributes-link {
    display: inline-block;
}

.craft-points-progressbar {
    width: 120px;
    display: inline-block;
}

.bar-title {
    user-select: none;
}
</style>

<fluent locale="zh-CN">
display-attrs = { $what }：{ $value }
remaining = 剩余
</fluent>

<fluent locale="en-US">
display-attrs = { $what }: { $value }
remaining = Remaining
</fluent>
