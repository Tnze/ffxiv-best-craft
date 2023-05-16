<script setup lang="ts">
import { ElProgress, ElLink } from 'element-plus';
import { computed } from 'vue'
import { Attributes, Conditions, Status } from '../../Craft';
import Condition from './Condition.vue'
import Buffs from './Buffs.vue';
import { Enhancer } from '../attr-enhancer/Enhancer';
import { Setting } from '@element-plus/icons-vue';

const props = defineProps<{
    status: Status;
    attributes: Attributes;
    enhancers: Enhancer[];
    disabledInitQuality?: boolean;
    disabledEnhancer?: boolean;
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
const condition = computed(() => {
    return props.status?.condition || Conditions.Normal
})
</script>

<template>
    <div class="conatiner">
        <div id="durability-and-condition">
            <div id="durability">
                {{ $t('display-durability', {
                    current: status?.durability,
                    total: status?.recipe.durability
                })
                }}
                <el-progress :stroke-width="14" :show-text="false" :percentage="durability" :color="durabilityColor">
                </el-progress>
            </div>
            <div id="condition">
                <Condition :cond="condition" />
            </div>
        </div>
        <div id="progress-and-buffs">
            {{ $t('progress') }}
            <el-progress :percentage="progress" :color="progressColor">
                {{ status?.progress }} /
                {{ status?.recipe.difficulty }} /
                {{ status?.recipe.difficulty - status?.progress }}
            </el-progress>
            {{ $t('quality') }}
            <el-progress :percentage="quality" :color="qualityColor">
                <template v-if="disabledInitQuality">
                    {{ status?.quality }} /
                    {{ status?.recipe.quality }}
                </template>
                <el-link v-else @click="emits('click-quality')" :icon="Setting">
                    {{ status?.quality }} /
                    {{ status?.recipe.quality }}
                </el-link>
            </el-progress>
            <Buffs id="buffs" :buffs="status.buffs" />
        </div>
        <div id="attributes">
            <el-link class="attributes-link" @click="emits('click-attributes')" :icon="Setting"
                :disabled="disabledEnhancer">
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
                    {{ $t('display-craft-point', {
                        current: status?.craft_points,
                        total: attributes.craft_points + enhancers
                            .filter(v => v.cp && v.cp_max)
                            .map(v => Math.min(
                                status?.attributes.craft_points * v.cp!,
                                v.cp_max!
                            ))
                            .map(cp => ` + ${cp}`).join('')
                    })
                    }}
                </div>
            </el-link>
            <br />
            <el-progress class="craft-points-progressbar"
                :percentage="status?.craft_points / status?.attributes.craft_points * 100" color="#FF9999"
                :show-text="false" :stroke-width="10" />
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

#condition {
    text-align: center;
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
</style>

<fluent locale="zh-CN">
display-durability = { durability } { $current } / { $total }
display-attrs = { $what }：{ $value }
display-craft-point = { craft-point }：{ $current } / { $total }
</fluent>

<fluent locale="en-US">
display-durability = { durability } { $current } / { $total }
display-attrs = { $what }: { $value }
display-craft-point = { craft-point }: { $current } / { $total }
</fluent>

<fluent locale="en-US">
display-durability = { durability } { $current } / { $total }
display-attrs = { $what }: { $value }
display-craft-point = { craft-point }: { $current } / { $total }
</fluent>