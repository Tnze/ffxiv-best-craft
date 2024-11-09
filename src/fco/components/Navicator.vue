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
import { h } from 'vue';
import { RouterLink } from 'vue-router';
import { NGradientText, NMenu } from 'naive-ui';
import type { MenuOption } from 'naive-ui';

const menuOptions: MenuOption[] = [
    {
        key: 'simulator',
        label: () =>
            h(
                RouterLink,
                { to: { name: 'simulator' } },
                { default: () => '模拟器' },
            ),
    },
    {
        key: 'attributes',
        label: () =>
            h(
                RouterLink,
                { to: { name: 'attributes' } },
                { default: () => '状态参数' },
            ),
    },
];
</script>

<template>
    <div class="nav">
        <n-gradient-text class="title" :font-size="20" type="info">
            《最终幻想14》生产模拟器
        </n-gradient-text>
        <n-menu mode="horizontal" :options="menuOptions" />
    </div>
</template>

<style scoped>
.title {
    margin: 0 15px;
}
.nav {
    display: flex;
    align-items: center;
}
</style>
