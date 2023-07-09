<script setup lang="ts">
import { ElEmpty } from 'element-plus';
import { computed } from 'vue';
import { useGearsetsStore, useDesignerStore } from '../../store';
import Designer from './Designer.vue'
import Simulator from './Simulator.vue';

const gearsetsStore = useGearsetsStore()
const designerStore = useDesignerStore()

const attributes = computed(() => {
    if (designerStore.content == null)
        return gearsetsStore.default
    const special = gearsetsStore.special.find(v => v.name == designerStore.content!.job)
    return special?.value ?? gearsetsStore.default
})
</script>

<template>
    <Suspense>
        <template v-if="designerStore.content != null">
            <Designer v-if="!designerStore.content.simulatorMode" :item="designerStore.content.item"
                :recipe="designerStore.content.recipe" :recipe-level="designerStore.content.recipeLevel"
                :recipe-info="designerStore.content.recipeInfo" :attributes="attributes"
                :display-job="designerStore.content!.job" />
            <Simulator v-else :item="designerStore.content.item" :recipe="designerStore.content.recipe"
                :recipe-level="designerStore.content.recipeLevel" :attributes="attributes"
                :display-job="designerStore.content!.job" />
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