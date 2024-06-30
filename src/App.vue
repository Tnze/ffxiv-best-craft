<!-- 
    This file is part of BestCraft.
    Copyright (C) 2023  Tnze

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
import { onMounted, ref, watch, watchEffect } from 'vue';
import { useColorMode, usePreferredLanguages, useCssVar } from '@vueuse/core';
import { ElConfigProvider, ElNotification, ElIcon } from 'element-plus';
import { Operation } from '@element-plus/icons-vue'
import { useFluent } from 'fluent-vue';
if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
    var pkgTauriFs = import('@tauri-apps/api/fs')
    var pkgTauri = import("@tauri-apps/api/tauri")
}

import Menu from '@/components/Menu.vue';
import useSettingsStore from '@/stores/settings';
import useGearsetsStore from '@/stores/gearsets';
import { elementPlusLang, languages } from './lang';
import { selectLanguage } from './fluent'
import { useRouter } from 'vue-router';

const { $t } = useFluent()
const colorMode = useColorMode()
const settingStore = useSettingsStore()
const gearsetsStore = useGearsetsStore()
const preferredLang = usePreferredLanguages()
const bgColor = useCssVar('--app-bg-color', ref(null))
const bgMainColor = useCssVar('--tnze-main-bg-color', ref(null))

const router = useRouter()
const showMenu = ref(false)
watch(router.currentRoute, () => showMenu.value = false)

const lang = ref('zh-CN')
watchEffect(() => {
    let settingLang: string | null = settingStore.language
    if (settingLang == 'system') settingLang = null
    const systemLang = preferredLang.value.find(v => languages.has(v))
    lang.value = settingLang ?? systemLang ?? 'zh-CN'
    selectLanguage(lang.value)
    console.log("language switched to", lang.value)
})

// Check update
if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
    onMounted(() => import('./update').then(x => x.checkUpdate($t, true)))
}

async function loadStorages() {
    if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
        const { Dir, readTextFile } = await pkgTauriFs
        readTextFile("settings.json", { dir: Dir.App }).then(settingStore.fromJson).catch(e => console.error(e))
        readTextFile("gearsets.json", { dir: Dir.App }).then(gearsetsStore.fromJson).catch(e => console.error(e))
    } else {
        const ifNotNullThen = (v: string | null, f: (v: string) => void) => { if (v != null) f(v) }
        ifNotNullThen(window.localStorage.getItem("settings.json"), settingStore.fromJson)
        ifNotNullThen(window.localStorage.getItem("gearsets.json"), gearsetsStore.fromJson)
    }
}
loadStorages()

async function writeJson(name: string, val: any) {
    let jsonStr = JSON.stringify(val)
    if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
        const { Dir, exists, createDir, writeTextFile } = await pkgTauriFs
        try {
            if (!await exists('', { dir: Dir.App }))
                await createDir('', { dir: Dir.App })
            await writeTextFile({ contents: jsonStr, path: name }, { dir: Dir.App })
        } catch (err) {
            console.error(err)
        }
    } else {
        window.localStorage.setItem(name, jsonStr)
    }
}
settingStore.$subscribe((_mutation, state) => writeJson('settings.json', state))
gearsetsStore.$subscribe((_mutation, state) => writeJson('gearsets.json', state))

watchEffect(async () => {
    let isDark: boolean | null;
    if (colorMode.value == 'dark') isDark = true;
    else if (colorMode.value == 'light') isDark = false;
    else isDark = null;

    let shouldBeTransparent = false;
    if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
        let { invoke } = await pkgTauri
        // Ask the rust side if the window transparent.
        shouldBeTransparent = await invoke('set_theme', { isDark })
    }
    bgColor.value = shouldBeTransparent ? 'transparent' : 'var(--el-bg-color)'
    bgMainColor.value = shouldBeTransparent ? '#2e2e2e80' : '#242424'
})


onMounted(() => {
    ElNotification({
        title: $t('try-dawntrain'),
        message: $t('try-dawntrain-desc'),
        duration: 0,
    })
})

</script>

<template>
    <div class="container">
        <Transition>
            <div class="overlay" v-if="showMenu" @click="showMenu = false"></div>
        </Transition>
        <Menu class="sidebar" v-bind:class="{ 'show-menu': showMenu }"></Menu>
        <div class="topbar">
            <el-icon :size="30" @click="showMenu = !showMenu">
                <Operation />
            </el-icon>
        </div>
        <div class="main">
            <router-view v-slot="{ Component }" style="height: 100%;">
                <keep-alive>
                    <component :is="Component" />
                </keep-alive>
            </router-view>
        </div>
        <el-config-provider :locale="elementPlusLang.get(lang)" />
    </div>
</template>

<style>
#app {
    font-family: "Helvetica Neue", Helvetica, "PingFang SC", "Hiragino Sans GB",
        "Microsoft YaHei", "微软雅黑", Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    height: 100%;
    margin: 0;
    background: var(--app-bg-color);
}

.el-menu:not(.el-menu--collapse) {
    width: var(--tnze-sidebar-width);
}

.el-form-item__label {
    user-select: none;
}

.el-message-box {
    border-radius: var(--el-border-radius-base);
}

.el-dialog {
    --el-dialog-border-radius: var(--el-border-radius-round) !important;
    box-shadow: none !important;
    border: 1px solid var(--el-border-color-light);
}

.el-card {
    --el-card-border-radius: var(--el-border-radius-base) !important;
}

:root {
    --el-color-primary: #3fb42d;
    --el-color-primary-light-3: #5ec35d;
    --el-color-primary-light-5: #81c27b;
    --el-color-primary-light-7: #c6ffca;
    --el-color-primary-light-8: #a2c0a2;
    --el-color-primary-light-9: #b0bdae;
    --el-color-primary-dark-2: #288b23;
    --el-fill-color-blank: transparent;

    --el-border-radius-base: 10px;
    --el-border-radius-small: 5px;
    --el-border-radius-round: 20px;

    --tnze-content-raduis: var(--el-border-radius-round);
    --tnze-sidebar-width: 150px;
    --tnze-topbar-height: 50px;
}

:root.dark {
    --el-color-primary: #299929;
    --el-color-primary-light-3: #267921;
    --el-color-primary-light-5: #1f601d;
    --el-color-primary-light-7: #1b461a;
    --el-color-primary-light-8: #16331b;
    --el-color-primary-light-9: #18222c;
    --el-color-primary-dark-2: #5cc952;
    /* --el-bg-color: var(--tnze-main-bg-color) !important; */
}
</style>

<style scoped>
.container {
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: rgba(246, 246, 246, 0.5);
}

:root.dark .container {
    background-color: var(--tnze-main-bg-color);
}

.overlay {
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    background: rgba(0, 0, 0, .6);
    z-index: 9;
}

.v-enter-active,
.v-leave-active {
    transition: opacity 0.5s ease;
}

.v-enter-from,
.v-leave-to {
    opacity: 0;
}
.v-leave-to {
    pointer-events: none;
}

.sidebar {
    position: fixed;
    top: 0;
    left: 0;
    bottom: 0;
    z-index: 10;
    background-color: var(--app-bg-color);

    transform: translateX(-100%);
    transition: transform .5s;
}

.sidebar.show-menu {
    transform: translateX(0);
}

.topbar {
    height: var(--tnze-topbar-height);
    width: 100%;
    border-bottom: solid 2px var(--el-border-color);
    box-sizing: border-box;
    flex: none;
    color: var(--el-color-info);
    transition: height .5s;
    display: flex;
    align-items: center;
    padding-left: 10px;
    overflow: hidden;
}

.main {
    padding: 0;
    background-color: rgba(246, 246, 246, 0.5);
    flex: auto;
    height: calc(100% - var(--tnze-topbar-height));

    transition: margin-left .5s;
}

:root.dark .main {
    background-color: var(--tnze-main-bg-color);
}

@media screen and (min-width: 960px) {
    .sidebar {
        transform: translateX(0);
    }

    .topbar {
        height: 0;
        border-bottom: initial;
    }

    .main {
        margin-left: var(--tnze-sidebar-width);
        border-top-left-radius: var(--tnze-content-raduis);
    }
}
</style>

<fluent locale="zh-CN">
try-dawntrain = FFXIV 7.0 黄金的遗产
try-dawntrain-desc = BestCraft 现已适配 7.0 新技能、新配方。欢迎前往体验！
    https://tnze.yyyy.games/dawntrail/
</fluent>
