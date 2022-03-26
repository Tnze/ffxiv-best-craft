<script setup lang="ts">
import { ref, reactive } from 'vue'

const props = defineProps<{
    settings: {
        language: string
    }
}>()

</script>


<template>
    <el-container>
        <el-header>
            <h1>设置</h1>
        </el-header>
        <el-main>
            <el-form class="setting-page" :model="settings" label-width="120px">
                <el-form-item label="语言">
                    <el-select v-model="settings.language">
                        <el-option label="简体中文" value="zh-CN" />
                    </el-select>
                </el-form-item>
            </el-form>
        </el-main>
    </el-container>
</template>

<style scoped>
.setting-page {
    padding-top: 10px;
}
</style>