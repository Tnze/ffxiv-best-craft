<script setup lang="ts">
import {
    Notebook,
    Suitcase,
    Edit,
    MagicStick,
    Setting,
    Box,
} from '@element-plus/icons-vue'
import { ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter()
const disableAutomation = ref(true)

</script>

<template>
    <el-menu :default-active="router.currentRoute.value.path" :router="true" :collapse="true">
        <el-menu-item index="/gearsets">
            <el-icon>
                <suitcase />
            </el-icon>
            <template #title>{{ $t('gearsets') }}</template>
        </el-menu-item>
        <el-menu-item index="/recipe">
            <el-icon>
                <notebook />
            </el-icon>
            <template #title>{{ $t('recipe') }}</template>
        </el-menu-item>
        <el-menu-item index="/bom" :disabled="disableAutomation" @click.stop.prevent.right="disableAutomation = false">
            <el-icon>
                <box />
            </el-icon>
            <template #title>{{ $t('bom') }}</template>
        </el-menu-item>
        <el-menu-item index="/designer">
            <el-icon>
                <edit />
            </el-icon>
            <template #title>{{ $t('designer') }}</template>
        </el-menu-item>
        <el-menu-item index="/settings">
            <el-icon>
                <setting />
            </el-icon>
            <template #title>{{ $t('settings') }}</template>
        </el-menu-item>
    </el-menu>
</template>

<style>
.el-menu {
    height: 100%;
}
</style>

<fluent locale="zh-CN">
gearsets = 配装
recipe = 配方
bom = 物料
designer = 设计
settings = 设置
</fluent>

<fluent locale="en">
gearsets = Gearsets
recipe = Recipe
bom = BOM
designer = Designer
settings = Settings
</fluent>

<fluent locale="ja">
gearsets = 装備
recipe = フォーミュラ
bom = 材料リスト
designer = デザイナ
settings = 設定
</fluent>