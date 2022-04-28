<script setup lang="ts">
import { h, ref } from 'vue'
import 'element-plus/es/components/message/style/css'
import { ElMessage } from 'element-plus'
import { Actions, Status } from "../../Craft"

</script>

<template>
    <el-scrollbar class="container">
        网页版模拟器不支持求解功能！
    </el-scrollbar>
</template>

<style scoped>
.container {
    display: flex;
    flex-direction: column;
}

</style>