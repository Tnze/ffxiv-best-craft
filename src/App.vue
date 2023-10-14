<script setup lang="ts">
import { onMounted, ref, watchEffect } from 'vue';
import { useColorMode, usePreferredLanguages, useCssVar } from '@vueuse/core';
import { ElContainer, ElAside, ElMain, ElConfigProvider } from 'element-plus';
import { useFluent } from 'fluent-vue';

import { Dir, exists, createDir, readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import { invoke } from "@tauri-apps/api/tauri";

import Menu from './components/Menu.vue';
import { useSettingsStore, useGearsetsStore } from './store';
import { elementPlusLang, languages } from './lang';
import { selectLanguage } from './fluent'
import { checkUpdate } from './update'

const colorMode = useColorMode()
const { $t } = useFluent()
const settingStore = useSettingsStore()
const gearsetsStore = useGearsetsStore()
const preferredLang = usePreferredLanguages()
const bgColor = useCssVar('--app-bg-color', ref(null))

const lang = ref('zh-CN')
watchEffect(() => {
    let settingLang: string | null = settingStore.language
    if (settingLang == 'system') settingLang = null
    const systemLang = preferredLang.value.find(v => languages.has(v))
    lang.value = settingLang ?? systemLang ?? 'zh-CN'
    selectLanguage(lang.value)
    console.log("language switched to", lang.value)
})

readTextFile("settings.json", { dir: Dir.App }).then(settingStore.fromJson).catch(e => console.error(e))
readTextFile("gearsets.json", { dir: Dir.App }).then(gearsetsStore.fromJson).catch(e => console.error(e))

async function writeJson(name: string, val: any) {
    let jsonStr = JSON.stringify(val)
    try {
        if (!await exists('', { dir: Dir.App }))
            await createDir('', { dir: Dir.App })
        await writeTextFile({ contents: jsonStr, path: name }, { dir: Dir.App })
    } catch (err) {
        console.error(err)
    }
}
settingStore.$subscribe((_mutation, state) => writeJson('settings.json', state))
gearsetsStore.$subscribe((_mutation, state) => writeJson('gearsets.json', state))

// Ask the rust side if the window transparent.
watchEffect(() => {
    let isDark: boolean | null;
    if (colorMode.value == 'dark') isDark = true;
    else if (colorMode.value == 'light') isDark = false;
    else isDark = null;
    invoke('set_theme', { isDark }).then(v => {
        bgColor.value = v ? 'transparent' : 'var(--el-bg-color)'
    });
})

// Check update
onMounted(() => checkUpdate($t, true))

</script>

<template>
    <el-container>
        <el-aside width="150px">
            <Menu></Menu>
        </el-aside>
        <el-main>
            <router-view v-slot="{ Component }">
                <keep-alive>
                    <component :is="Component" />
                </keep-alive>
            </router-view>
        </el-main>
        <el-config-provider :locale="elementPlusLang.get(lang)" />
    </el-container>
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

.el-container {
    height: 100%;
}

.el-main {
    padding: 0;
    background-color: rgba(255, 255, 255, 0.5);
    border-top-left-radius: var(--tnze-content-raduis);
}

:root.dark .el-main {
    background-color: rgba(46, 46, 46, 0.5);
}

.el-menu:not(.el-menu--collapse) {
    width: 150px;
}

.el-form-item__label {
    user-select: none;
}

.el-message-box {
    border-radius: var(--el-border-radius-base);
}

.el-dialog {
    --el-dialog-border-radius: var(--el-border-radius-round) !important;
}

.el-drawer {
    box-shadow: var(--el-box-shadow-light) !important;
}

.el-drawer.btt {
    border-top-left-radius: var(--tnze-content-raduis);
    border-top-right-radius: var(--tnze-content-raduis);
}

.el-drawer.rtl {
    border-top-left-radius: var(--tnze-content-raduis);
    border-bottom-left-radius: var(--tnze-content-raduis);
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
}

:root.dark {
    --el-color-primary: #299929;
    --el-color-primary-light-3: #267921;
    --el-color-primary-light-5: #1f601d;
    --el-color-primary-light-7: #1b461a;
    --el-color-primary-light-8: #16331b;
    --el-color-primary-light-9: #18222c;
    --el-color-primary-dark-2: #5cc952;
}
</style>
