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
import {
    NLayout,
    NLayoutHeader,
    NLayoutContent,
    NLayoutFooter,
    NConfigProvider,
    NLoadingBarProvider,
    darkTheme,
    zhCN,
} from 'naive-ui';
import Navicator from './components/Navicator.vue';
import { useDark } from '@vueuse/core';

const dark = useDark();

const themeOverrides = {
    common: {
        borderRadius: '14px',
    },
};
</script>

<template>
    <n-config-provider
        :theme="dark ? darkTheme : undefined"
        :theme-overrides="themeOverrides"
        :locale="zhCN"
        abstract
    >
        <n-layout class="page" style="height: 100%">
            <n-layout-header style="padding: 5px">
                <Navicator />
            </n-layout-header>
            <n-layout-content class="content">
                <router-view v-slot="{ Component }">
                    <n-loading-bar-provider>
                        <component :is="Component" />
                    </n-loading-bar-provider>
                </router-view>
            </n-layout-content>
            <n-layout-footer class="info-text">
                {{ $t('copyright-notices') }}
            </n-layout-footer>
        </n-layout>
    </n-config-provider>
</template>

<style scoped>
.content {
    margin: 30px 20px;
}

.info-text {
    padding: 20px;
    align-self: center;
    /* margin-bottom: 5px; */
    white-space: pre-line;
}
</style>

<style>
#app {
    height: 100%;
    margin: 0;
}
</style>
