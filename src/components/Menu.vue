<!-- 
    This file is part of BestCraft.
    Copyright (C) 2024  Tnze

    BestCraft is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    BestCraft is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
-->

<script setup lang="ts">
import { ElMenu, ElMenuItem, ElIcon } from 'element-plus';
import {
    Collection,
    User,
    EditPen,
    Setting,
    Guide,
    Box,
} from '@element-plus/icons-vue';
import { useRouter } from 'vue-router';

const router = useRouter();
</script>

<template>
    <el-menu
        :default-active="router.currentRoute.value.path"
        :router="true"
        :collapse="false"
    >
        <el-menu-item index="/welcome">
            <el-icon>
                <guide />
            </el-icon>
            <template #title>{{ $t('guide') }}</template>
        </el-menu-item>
        <el-menu-item index="/gearsets">
            <el-icon>
                <user />
            </el-icon>
            <template #title>{{ $t('gearsets') }}</template>
        </el-menu-item>
        <el-menu-item index="/recipe">
            <el-icon>
                <collection />
            </el-icon>
            <template #title>{{ $t('recipe') }}</template>
        </el-menu-item>
        <el-menu-item index="/designer">
            <el-icon>
                <edit-pen />
            </el-icon>
            <template #title>{{ $t('simulator') }}</template>
        </el-menu-item>
        <el-menu-item index="/bom">
            <el-icon>
                <box />
            </el-icon>
            <template #title>{{ $t('bom') }}</template>
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
    padding: 5px;
}

.el-menu-item {
    border-radius: var(--tnze-content-raduis);
}
</style>

<fluent locale="zh-CN">
guide = 向导
gearsets = 配装
recipe = 配方
simulator = 模拟
bom = 清单
settings = 设置
</fluent>

<fluent locale="zh-TW">
guide = 嚮導
gearsets = 配裝
recipe = 配方
simulator = 模擬
bom = 清單
settings = 設定
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
