<script setup lang="ts">
import { useStore } from '../../store';

const store = useStore()
</script>

<template>
    <Suspense>
        <Designer v-if="store.state.designer != null" />
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

<fluent locale="en">
not-selected = Please select recipe first
loading = Loading
</fluent>