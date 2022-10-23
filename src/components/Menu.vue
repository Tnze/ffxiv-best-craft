<script setup lang="ts">
import {
    Notebook,
    Suitcase,
    Edit,
    MagicStick,
    Setting,
} from '@element-plus/icons-vue'
import { ref } from 'vue';

const disableAutomation = ref(true)

</script>

<template>
    <el-menu default-active="/gearsets" :router="true" :collapse="true">
        <el-menu-item index="/gearsets">
            <el-icon>
                <suitcase />
            </el-icon>
            <template #title>装备</template>
        </el-menu-item>
        <el-menu-item index="/recipe">
            <el-icon>
                <notebook />
            </el-icon>
            <template #title>配方</template>
        </el-menu-item>
        <el-menu-item index="/designer">
            <el-icon>
                <edit />
            </el-icon>
            <template #title>设计</template>
        </el-menu-item>
        <el-menu-item index="/automation" :disabled="disableAutomation" @click.stop.prevent.right="disableAutomation = false">
            <el-icon>
                <magic-stick />
            </el-icon>
            <template #title>自动化</template>
        </el-menu-item>
        <el-menu-item index="/settings">
            <el-icon>
                <setting />
            </el-icon>
            <template #title>设置</template>
        </el-menu-item>
    </el-menu>
</template>

<style>
.el-menu {
    height: 100%;
}
</style>
