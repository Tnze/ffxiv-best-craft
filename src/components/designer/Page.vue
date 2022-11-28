<script setup lang="ts">
import { ElEmpty } from 'element-plus';
import { computed } from 'vue';
import { useStore } from '../../store';
import Designer from './Designer.vue'
import Simulator from './Simulator.vue';

const store = useStore()

const displayJob = computed(() => store.state.designer!.job);
const attributes = computed(() => {
    if (store.state.designer == null)
        return store.state.gearsets.default
    const special = store.state.gearsets.special.find(v => v.name == store.state.designer!.job)
    return special?.value ?? store.state.gearsets.default
})
</script>

<template>
    <Suspense>
        <template v-if="store.state.designer != null">
            <Designer v-if="(store.state.designer.recipe.conditions_flag & ~15) == 0" :item="store.state.designer.item"
                :recipe="store.state.designer.recipe" :attributes="attributes" :display-job="displayJob" />
            <Simulator v-else :item="store.state.designer.item" :recipe="store.state.designer.recipe"
                :attributes="attributes" :display-job="displayJob" />
        </template>
        <el-empty v-else :description="$t('not-selected')" style="height: 100%;" />
        <template #fallback>
            <el-empty :description="$t('loading')" style="height: 100%;" />
        </template>
    </Suspense>
</template>

<fluent locale="zh-CN">
not-selected = 请先选择配方
loading = 加载中
</fluent>

<fluent locale="en-US">
not-selected = Please select recipe first
loading = Loading
</fluent>