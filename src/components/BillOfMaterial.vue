<script setup lang="ts">
import { useStore } from '../store';

const store = useStore();
</script>

<template>
    <el-scrollbar>
        <div class="scrollbar-flex-content">
            <el-card class="box-card">
                <el-scrollbar>
                    <p v-for="item in store.state.checklist" :key="item.ingredient_id" class="scrollbar-demo-item">
                        {{ item.ingredient_id }} * {{ item.amount }}
                    </p>
                </el-scrollbar>
            </el-card>
        </div>
    </el-scrollbar>
</template>

<style scoped>
.scrollbar-flex-content {
    display: flex;
}

.scrollbar-demo-item {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100px;
    height: 100%;
    margin: 10px;
    text-align: center;
    border-radius: 4px;
    background: var(--el-color-danger-light-9);
    color: var(--el-color-danger);
}

.box-card {
    width: 480px;
}
</style>
