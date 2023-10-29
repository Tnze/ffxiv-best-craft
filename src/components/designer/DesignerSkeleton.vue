<script setup lang="ts">
import { ElSkeleton, ElSkeletonItem } from 'element-plus';

// generate ramdom numbers between 30 ~ 100
function* randomSkeletonGenerator(n: number) {
    for (let i = 0; i < n; i++) {
        yield Math.random() * 70 + 30
    }
}
</script>

<template>
    <el-skeleton animated class="main-page">
        <template #template>
            <el-skeleton-item variant="h1" style="width: 200px; margin: 18px;" />
            <div class="status-bar">
                <el-skeleton-item variant="rect" style="width: 100%; height: 100%;" />
            </div>
            <div class="actionpanel-and-savedqueue">
                <el-skeleton-item variant="rect" style="width: 25%; height: 100%; " />
                <div style="flex: auto; margin: 0px 18px; display: flex; flex-direction: column;">
                    <el-skeleton-item variant="text" style="width: 100%; height: 48px; margin-bottom: 18px;" />
                    <el-skeleton-item v-for="w in randomSkeletonGenerator(5)" variant="text"
                        :style="`width: ${w}%; height: 48px; margin-top: 18px;`" />
                </div>
            </div>
        </template>
    </el-skeleton>
</template>

<style scoped>
.main-page {
    height: 100%;
    display: flex;
    flex-direction: column;
}

.status-bar {
    flex: none;
    height: 126px;
    width: 100%;
    padding: 0px 18px;
    box-sizing: border-box;
}

.actionpanel-and-savedqueue {
    border-bottom: 1px solid var(--el-border-color);
    display: flex;
    flex: auto;
    overflow: hidden;
    padding: 18px;
    box-sizing: border-box;
}
</style>