<script setup lang="ts">
import { onMounted, ref, watchEffect } from 'vue';
import { useDark, usePreferredLanguages, useCssVar } from '@vueuse/core';
import { ElContainer, ElAside, ElMain, ElConfigProvider } from 'element-plus';
import { useFluent } from 'fluent-vue';

import { Dir, exists, createDir, readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import { invoke } from "@tauri-apps/api/tauri";

import Menu from './components/Menu.vue';
import { useSettingsStore, useGearsetsStore } from './store';
import { elementPlusLang, languages } from './lang';
import { selectLanguage } from './fluent'
import { checkUpdate } from './update'

useDark()
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

// Ask the rust size if the window transparent.
invoke('should_be_transparent').then(v => {
    bgColor.value = v ? 'transparent' : 'var(--el-bg-color)'
});

// Check update
onMounted(() => checkUpdate($t))

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
}

.el-menu:not(.el-menu--collapse) {
    width: 150px;
}

:root {
    --el-color-primary: rgb(11, 91, 11);
    /* --el-bg-color: transparent; */
    --el-fill-color-blank: transparent;
}

:root.dark {
    --el-color-primary: rgb(49, 180, 49);
    /* --el-bg-color: transparent; */
}
</style>
