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
import { ElEmpty, ElResult, ElButton } from 'element-plus';
import { computed, defineAsyncComponent, onErrorCaptured, ref, provide, onActivated } from 'vue';
import useGearsetsStore from '@/stores/gearsets';
import useDesignerStore from '@/stores/designer';
import { useFluent } from 'fluent-vue';
import { displayJobKey } from "./injectionkeys"

const emit = defineEmits<{
    (e: 'setTitle', title: string): void
}>()

const gearsetsStore = useGearsetsStore()
const designerStore = useDesignerStore()
const { $t } = useFluent()

onActivated(() => emit('setTitle', designerStore.content?.item.name ?? ''))

const Designer = defineAsyncComponent(() => import('./Designer.vue'))
const Simulator = defineAsyncComponent(() => import('./Simulator.vue'))

const attributes = computed(() => {
    if (designerStore.content == null)
        return gearsetsStore.default
    const special = gearsetsStore.special.find(v => v.name == designerStore.content!.job)
    return special?.value ?? gearsetsStore.default
})
const errorMessage = ref<string>()

provide(displayJobKey, computed(() => designerStore.content!.job))

onErrorCaptured((err: unknown, instance, info) => {
    console.error(err, instance, info)
    try {
        errorMessage.value = $t(String(err))
    } catch {
        errorMessage.value = String(err)
    }
    return false
})
function reload() {
    window.location.reload()
}
</script>

<template>
    <Suspense :timeout="30">
        <el-result v-if="errorMessage" icon="error" :title="$t('error-happens')" :sub-title="errorMessage">
            <template #extra>
                <el-button @click="reload">{{ $t('reload') }}</el-button>
            </template>
        </el-result>
        <template v-else-if="designerStore.content != null">
            <Designer v-if="!designerStore.content.simulatorMode" :item="designerStore.content.item"
                :recipe="designerStore.content.recipe" :recipe-id="designerStore.content.recipeId"
                :material-quality-factor="designerStore.content.materialQualityFactor"
                :recipe-level="designerStore.content.recipeLevel" :requirements="designerStore.content.requirements"
                :attributes="attributes" />
            <Simulator v-else :item="designerStore.content.item" :recipe="designerStore.content.recipe"
                :recipe-level="designerStore.content.recipeLevel" :attributes="attributes"
                :display-job="designerStore.content!.job" />
        </template>
        <el-empty v-else :description="$t('not-selected')" style="height: 100%;" />
    </Suspense>
</template>

<style scoped>
.el-result {
    vertical-align: middle;
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
