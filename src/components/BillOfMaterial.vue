<script setup lang="ts">

</script>

<template>
    <el-scrollbar>
        <div class="scrollbar-flex-content">
            <el-card v-for="o in 10" :key="o" class="box-card">
                <el-scrollbar>
                    <p v-for="item in 10" :key="item" class="scrollbar-demo-item">
                        {{ item }}
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
