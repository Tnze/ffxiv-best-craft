<script setup lang="ts">
import { useStore } from '../../store';

const store = useStore()
</script>

<template>
    <Suspense>
        <Designer v-if="store.state.designer != null" />
        <el-empty v-else description="请先选择配方" style="height: 100%;" />
        <template #fallback>
            <el-empty description="加载中" style="height: 100%;" />
        </template>
    </Suspense>
</template>