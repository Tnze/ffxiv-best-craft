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
import { ElProgress } from 'element-plus';
import { computed, ref } from 'vue';
import { useElementSize } from '@vueuse/core';
import { Attributes, Collectability, Status } from '@/libs/Craft';
import Buffs from './Buffs.vue';
import Condition from './Condition.vue';
import DurabilityProgressBar from './DurabilityProgressBar.vue';
import CollectabilityRefineMark from './CollectabilityRefineMark.vue';

const props = defineProps<{
    status: Status;
    attributes: Attributes;
    showCondition: boolean;
    collectability?: Collectability;
}>();

const qualityProgressBar = ref();
const { width: qualityProgressBarWidth } = useElementSize(qualityProgressBar);

const progress = computed<number>(() =>
    props.status.recipe.difficulty == 0
        ? 100
        : (props.status.progress / props.status.recipe.difficulty) * 100,
);
const remainingProgress = computed(
    () => props.status.recipe.difficulty - props.status.progress,
);
const quality = computed<number>(() =>
    props.status.recipe.quality == 0
        ? 100
        : (props.status.quality / props.status.recipe.quality) * 100,
);

const progressColor = computed<string>(() => {
    if (props.status.progress >= props.status.recipe.difficulty)
        return '#13CE66';
    if (props.status.durability <= 0) return '#F56C6C';
    return '#409EFF';
});

const qualityColor = computed<string>(() =>
    props.status.quality >= props.status.recipe.quality ? '#13CE66' : '#409EFF',
);

const craftPointPercentage = computed(() =>
    props.status.attributes.craft_points == 0
        ? 100
        : (props.status.craft_points / props.status.attributes.craft_points) *
          100,
);

const collectabilityLevel = computed(() => {
    if (props.collectability == undefined) {
        return undefined;
    }
    const { low_collectability, mid_collectability, high_collectability } =
        props.collectability;
    const collectability = props.status.quality / 10;
    if (collectability < low_collectability) {
        return 0;
    }
    if (collectability < mid_collectability) {
        return 1;
    }
    if (collectability < high_collectability) {
        return 2;
    }
    return 3;
});
</script>

<template>
    <div class="conatiner">
        <div id="durability-and-condition">
            <div id="durability">
                <span class="bar-title">{{ $t('durability') }} &nbsp;</span>
                <span>
                    {{ status.durability }} /
                    {{ status.recipe.durability }}
                </span>
                <DurabilityProgressBar
                    v-model="status.durability"
                    :max="status.recipe.durability"
                />

                <span class="bar-title">{{ $t('craft-point') }} &nbsp;</span>
                <span>
                    {{ status.craft_points }} /
                    {{ status.attributes.craft_points }}
                </span>
                <el-progress
                    :stroke-width="12"
                    :percentage="craftPointPercentage"
                    :show-text="false"
                    color="#FF9999"
                    striped
                />

                <Condition v-if="showCondition" :cond="status.condition" />
            </div>
        </div>
        <div id="progress-and-buffs">
            <span class="bar-title">{{ $t('progress') }} &nbsp;</span>
            <span>
                {{ status.progress }} / {{ status.recipe.difficulty }}
            </span>
            <template v-if="remainingProgress > 0">
                <span class="bar-title">
                    &nbsp; {{ $t('remaining') }} &nbsp;
                </span>
                <span>{{ remainingProgress }}</span>
            </template>
            <el-progress
                :percentage="progress"
                :color="progressColor"
                :show-text="false"
                :stroke-width="10"
            />
            <div style="height: 1em"></div>
            <span class="bar-title">{{ $t('quality') }} &nbsp;</span>
            <span> {{ status.quality }} / {{ status.recipe.quality }} </span>
            <template v-if="collectability != undefined">
                <span class="bar-title">
                    &nbsp; {{ $t('collectability-stage') }} &nbsp;
                </span>
                <span>{{ collectabilityLevel }}</span>
            </template>
            <el-progress
                ref="qualityProgressBar"
                :percentage="quality"
                :color="qualityColor"
                :show-text="false"
                :stroke-width="10"
            />
            <CollectabilityRefineMark
                v-if="collectability != undefined && status.recipe.quality > 0"
                :collectability="collectability"
                :max-collectability="status.recipe.quality / 10"
                :progres-bar-width="qualityProgressBarWidth"
            />
            <Buffs id="buffs" :buffs="status.buffs" />
        </div>
        <div id="attributes">
            <div class="attr-block">
                <span class="attr-label">
                    {{ $t('display-attrs-label', { label: $t('level') }) }}
                </span>
                <span class="attr-value"> {{ status.attributes.level }}</span>
            </div>
            <div class="attr-block">
                <span class="attr-label">
                    {{
                        $t('display-attrs-label', {
                            label: $t('craftsmanship'),
                        })
                    }}
                </span>
                <span class="attr-value">
                    {{ status.attributes.craftsmanship }}
                </span>
            </div>
            <div class="attr-block">
                <span class="attr-label">
                    {{ $t('display-attrs-label', { label: $t('control') }) }}
                </span>
                <span class="attr-value">
                    {{ status.attributes.control }}
                </span>
            </div>
            <div class="attr-block">
                <span class="attr-label">
                    {{
                        $t('display-attrs-label', { label: $t('craft-point') })
                    }}
                </span>
                <span class="attr-value">
                    {{ status.attributes.craft_points }}
                </span>
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
    /* padding: 3px 10px 5px 5px; */
    flex: none;
}

#durability {
    padding: 5px;
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
    padding: 0px 20px 0px 10px;
    /* flex-grow: 2; */
    color: var(--el-text-color-secondary);
}

.attr-block {
    display: flex;
    justify-content: space-between;
}

.attr-label {
    text-align: left;
    display: inline-block;
    margin-right: 10px;
}

.attr-value {
    display: inline-block;
}

.bar-title {
    user-select: none;
}

@media screen and (max-width: 480px) {
    .conatiner {
        flex-wrap: wrap;
    }

    #progress-and-buffs {
        flex: 1 0 100%;
        padding: 5px;
    }

    #durability-and-condition {
        order: -2;
        flex: auto;
    }

    #attributes {
        order: -1;
    }
}
</style>

<fluent locale="zh-CN">
display-attrs-label = { $label }：
remaining = 剩余
collectability-stage = 收藏价值等级
</fluent>

<fluent locale="en-US">
display-attrs-label = { $label }: 
remaining = Remaining
collectability-stage = Collectability Stage
</fluent>
