<script setup lang="ts">
import { ElEmpty, ElResult, ElButton } from 'element-plus';
import { computed, onErrorCaptured, ref } from 'vue';
import { useGearsetsStore, useDesignerStore } from '../../store';
import Designer from './Designer.vue'
import Simulator from './Simulator.vue';
import { useFluent } from 'fluent-vue';

const gearsetsStore = useGearsetsStore()
const designerStore = useDesignerStore()
const { $t } = useFluent()

const attributes = computed(() => {
    if (designerStore.content == null)
        return gearsetsStore.default
    const special = gearsetsStore.special.find(v => v.name == designerStore.content!.job)
    return special?.value ?? gearsetsStore.default
})
const errorMessage = ref<string>()

onErrorCaptured((err: unknown) => {
    errorMessage.value = String(err)
    return false
})
function reload() {
    window.location.reload()
}
</script>

<template>
    <Suspense>
        <el-result v-if="errorMessage" icon="error" :title="$t('error-happens')" :sub-title="$t(errorMessage)">
            <template #extra>
                <el-button @click="reload">{{ $t('reload') }}</el-button>
            </template>
        </el-result>
        <template v-else-if="designerStore.content != null">
            <Designer v-if="!designerStore.content.simulatorMode" :item="designerStore.content.item"
                :recipe="designerStore.content.recipe" :recipe-info="designerStore.content.recipeInfo"
                :material-quality-factor="designerStore.content.materialQualityFactor"
                :recipe-level="designerStore.content.recipeLevel" :requirements="designerStore.content.requirements"
                :attributes="attributes" :display-job="designerStore.content!.job" />
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

<style scoped>
.el-result {
    margin-top: 20%;
}
</style>

<fluent locale="zh-CN">
error-happens = 加载配方时出现了一些错误
not-selected = 请先选择配方
loading = 加载中
reload = 刷新
</fluent>

<fluent locale="en-US">
not-selected = Please select recipe first
loading = Loading
reload = Reload
</fluent>