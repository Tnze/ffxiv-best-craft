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
import { NLayout, NLayoutHeader, NLayoutContent, NLayoutFooter, NConfigProvider, darkTheme } from 'naive-ui';
</script>

<template>
    <n-config-provider :theme="darkTheme" abstract>
        <n-layout class="page" style="height: 100%;">
            <n-layout-header style="padding: 20px;">
                Work in Progress
            </n-layout-header>
            <n-layout-content style="margin: 30px 20px;">
                <router-view v-slot="{ Component }">
                    <keep-alive>
                        <component :is="Component" />
                    </keep-alive>
                </router-view>
            </n-layout-content>
            <n-layout-footer style="padding: 20px;">
                {{ $t('copyright-notices') }}
            </n-layout-footer>
        </n-layout>
    </n-config-provider>
</template>

<style>
#app {
    height: 100%;
    margin: 0;
}
</style>

<style scoped></style>
