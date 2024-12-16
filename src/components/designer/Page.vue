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
import { ElEmpty, ElResult, ElButton, ElText, ElSkeleton } from 'element-plus';
import {
    computed,
    defineAsyncComponent,
    onErrorCaptured,
    ref,
    provide,
    onActivated,
} from 'vue';
import useGearsetsStore from '@/stores/gearsets';
import useDesignerStore from '@/stores/designer';
import { useFluent } from 'fluent-vue';
import { displayJobKey } from './injectionkeys';
import { Jobs } from '@/libs/Craft';

const emit = defineEmits<{
    (e: 'setTitle', title: string): void;
}>();

const gearsetsStore = useGearsetsStore();
const designerStore = useDesignerStore();
const { $t } = useFluent();

onActivated(() => emit('setTitle', designerStore.content?.item.name ?? ''));

const Designer = defineAsyncComponent(() => import('./Designer.vue'));
const Simulator = defineAsyncComponent(() => import('./Simulator.vue'));

const isCustomRecipe = computed(() => designerStore.content?.job === undefined);
const attributes = computed(() => {
    const job = designerStore.content?.job;
    return job
        ? (gearsetsStore.special.find(v => v.name == job)?.value ??
              gearsetsStore.default)
        : gearsetsStore.default;
});
const errorMessage = ref<string>();

provide(
    displayJobKey,
    computed(() => designerStore.content?.job ?? Jobs.Culinarian),
);

onErrorCaptured((err: unknown, instance, info) => {
    console.error(err, instance, info);
    try {
        errorMessage.value = $t(String(err));
    } catch {
        errorMessage.value = String(err);
    }
    return false;
});
function reload() {
    window.location.reload();
}
</script>

<template>
    <Suspense :timeout="30">
        <el-result
            v-if="errorMessage"
            icon="error"
            :title="$t('error-happens')"
            :sub-title="errorMessage"
        >
            <template #extra>
                <template v-if="errorMessage.search('WebAssembly') > 0">
                    <el-text type="warning">{{ $t('upgrade-browser') }}</el-text
                    ><br />
                </template>
                <el-button @click="reload">{{ $t('reload') }}</el-button
                ><br />
                <el-text type="info">{{ $t('sorry') }}</el-text
                ><br />
                <el-text type="info">{{ $t('invite') }}</el-text>
            </template>
        </el-result>
        <template v-else-if="designerStore.content != null">
            <Designer
                v-if="!designerStore.content.simulatorMode"
                :item="designerStore.content.item"
                :recipe="designerStore.content.recipe"
                :recipe-id="designerStore.content.recipeId"
                :material-quality-factor="
                    designerStore.content.materialQualityFactor
                "
                :requirements="designerStore.content.requirements"
                :collectability="designerStore.content.collectability"
                :attributes="attributes"
                :is-custom-recipe="isCustomRecipe"
            />
            <Simulator
                v-else
                :item="designerStore.content.item"
                :recipe="designerStore.content.recipe"
                :collectability="designerStore.content.collectability"
                :attributes="attributes"
            />
        </template>
        <el-empty
            v-else
            :description="$t('not-selected')"
            style="height: 100%"
        />

        <template #fallback>
            <el-skeleton :rows="5" animated />
        </template>
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
upgrade-browser = 这个问题似乎是浏览器版本过旧导致的，请尝试更新您的浏览器
reload = 刷新
sorry = 由于技术原因，出现错误后暂时只能通过刷新页面恢复，对此造成的不便我们深表歉意
invite = 如果您有兴趣解决这个问题，欢迎向本项目仓库提交PR
</fluent>

<fluent locale="en-US">
not-selected = Please select recipe first
reload = Reload
upgrade-browser = Please try upgrade your browser
sorry = Due to technical reasons, the error can only be temporarily restored by refreshing the page.
    We apologize for any inconvenience caused.
invite = If you are interested in solving this problem, please submit a Pull Request to the repository
</fluent>

<fluent locale="ja-JP">
reload = 再ロード
sorry = 技術的な理由により、ページを更新することでエラーを一時的に回復するしかありません。
    ご不便をおかけしますのでご了承ください。
invite = この問題の解決に興味がある場合は、リポジトリに「Pull Request」を発行してください
</fluent>
