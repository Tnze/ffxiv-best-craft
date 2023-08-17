<script setup lang="ts">
import { ElMenu, ElMenuItem, ElIcon } from 'element-plus'
import { Notebook, Suitcase, Edit, Setting, Box, Guide } from '@element-plus/icons-vue'
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useGuideStore } from '../store';

const router = useRouter()
const disableBom = ref(true)
const guideStore = useGuideStore()

</script>

<template>
    <el-menu :default-active="router.currentRoute.value.path" :router="true" :collapse="false">
        <el-menu-item :index="'/guide/' + guideStore.currentPage">
            <el-icon>
                <guide />
            </el-icon>
            <template #title>{{ $t('guide') }}</template>
        </el-menu-item>
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
        <el-menu-item index="/designer">
            <el-icon>
                <edit />
            </el-icon>
            <template #title>{{ $t('simulator') }}</template>
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
    border: none;
    user-select: none;
}
</style>

<fluent locale="zh-CN">
guide = 向导
gearsets = 配装
recipe = 配方
bom = 物料
simulator = 模拟
settings = 设置
</fluent>

<fluent locale="en-US">
guide = Wizard
gearsets = Gearsets
recipe = Recipes
bom = BOM
simulator = Simulator
settings = Settings
</fluent>

<fluent locale="ja-JP">
guide = ガイダンス
gearsets = 装備
recipe = フォーミュラ
bom = 材料リスト
# simulator = デザイン
settings = 設定
</fluent>