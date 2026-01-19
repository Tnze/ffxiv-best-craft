<!-- 
    This file is part of BestCraft.
    Copyright (C) 2025  Tnze

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
import {
    useColorMode,
    usePreferredLanguages,
    useCssVar,
    useMediaQuery,
} from '@vueuse/core';
import {
    ElConfigProvider,
    ElIcon,
    ElMessage,
    ElLink,
    ElDialog,
} from 'element-plus';
import { Operation } from '@element-plus/icons-vue';
import { useFluent } from 'fluent-vue';
import { isTauri, isWebsite } from './libs/Consts';
if (isTauri) {
    var pkgTauriFs = import('@tauri-apps/plugin-fs');
    var pkgTauri = import('@tauri-apps/api/core');
    var pkgTauriPath = import('@tauri-apps/api/path');
}

import Menu from '@/components/Menu.vue';
import useSettingsStore from '@/stores/settings';
import useGearsetsStore from '@/stores/gearsets';
import useDesignerStore from '@/stores/designer';
import { elementPlusLang, languages } from './lang';
import { selectLanguage } from './fluent';
import { useRouter } from 'vue-router';
import DesktopEditionDownload from './components/DesktopEditionDownload.vue';

const { $t } = useFluent();
const colorMode = useColorMode();
const settingsStore = useSettingsStore();
const gearsetsStore = useGearsetsStore();
const designerStore = useDesignerStore();
const preferredLang = usePreferredLanguages();
const bgColor = useCssVar('--app-bg-color', ref(null));
const bgMicaColor = useCssVar('--tnze-mica-bg-color', ref(null));

const router = useRouter();
const showDesktopEditionDownload = ref(false);
const showMenu = ref(false);
const topTitle = ref('');
const unfoldSidebar = useMediaQuery('screen and (min-width: 760px)');
watch(router.currentRoute, () => (showMenu.value = false));

const lang = ref('zh-CN');
watchEffect(() => {
    let settingLang: string | null = settingsStore.language;
    if (settingLang == 'system') settingLang = null;
    const systemLang = preferredLang.value.find(v => languages.has(v));
    lang.value = settingLang ?? systemLang ?? 'zh-CN';
    selectLanguage(lang.value);
});

// Check update
if (isTauri) {
    onMounted(() => import('./update').then(x => x.checkUpdate($t, true)));
}

async function loadStorages() {
    let settingsJson: Promise<string> | string | null,
        gearsetsJson: Promise<string> | string | null,
        designerJson: Promise<string> | string | null;
    if (isTauri) {
        const { BaseDirectory, readTextFile } = await pkgTauriFs;
        const options = { baseDir: BaseDirectory.AppData };
        settingsJson = readTextFile('settings.json', options);
        gearsetsJson = readTextFile('gearsets.json', options);
        designerJson = readTextFile('designer.json', options);
    } else {
        settingsJson = window.localStorage.getItem('settings.json');
        gearsetsJson = window.localStorage.getItem('gearsets.json');
        designerJson = window.localStorage.getItem('designer.json');
    }
    for (const v of [
        { dst: settingsStore.fromJson, src: settingsJson },
        { dst: gearsetsStore.fromJson, src: gearsetsJson },
        { dst: designerStore.fromJson, src: designerJson },
    ]) {
        if (v.src === null) continue;
        try {
            v.dst(await v.src);
        } catch (e) {
            console.error(e);
        }
    }
}

async function writeJson(name: string, jsonStr: string) {
    if (isTauri) {
        const { BaseDirectory, writeTextFile, exists, mkdir } =
            await pkgTauriFs;
        const { appDataDir } = await pkgTauriPath;
        try {
            const appDataPath = await appDataDir();
            if (!(await exists(appDataPath))) {
                await mkdir(appDataPath, { recursive: true });
            }
            await writeTextFile(name, jsonStr, {
                baseDir: BaseDirectory.AppData,
            });
        } catch (err) {
            console.error(err);
            ElMessage({
                type: 'error',
                message: $t('error-save-file', {
                    file: name,
                    err: String(err),
                }),
            });
        }
    } else {
        window.localStorage.setItem(name, jsonStr);
    }
}

onMounted(async () => {
    await loadStorages();
    // subscribe only after storage load
    settingsStore.$subscribe(() =>
        writeJson('settings.json', settingsStore.toJson),
    );
    gearsetsStore.$subscribe(() =>
        writeJson('gearsets.json', gearsetsStore.toJson),
    );
    designerStore.$subscribe(() =>
        writeJson('designer.json', designerStore.toJson),
    );
});

watchEffect(async () => {
    let isDark: boolean | null;
    if (colorMode.value == 'dark') isDark = true;
    else if (colorMode.value == 'light') isDark = false;
    else isDark = null;

    let shouldBeTransparent = false;
    if (isTauri) {
        let { invoke } = await pkgTauri;
        // Ask the rust side if the window transparent.
        shouldBeTransparent = await invoke('set_theme', { isDark });
    }
    if (shouldBeTransparent) {
        bgMicaColor.value = 'transparent';
        bgColor.value = isDark ? '#2e2e2e' : '#FFFFFF';
    } else {
        bgMicaColor.value = 'var(--el-bg-color)';
        bgColor.value = isDark ? '#242424' : 'var(--el-bg-color-page)';
    }
});
</script>

<template>
    <el-config-provider :locale="elementPlusLang.get(lang)">
        <el-dialog
            v-if="isWebsite"
            v-model="showDesktopEditionDownload"
            :title="$t('download-desktop-edition')"
        >
            <DesktopEditionDownload />
        </el-dialog>
        <div class="container">
            <Transition>
                <div
                    class="overlay"
                    v-if="showMenu"
                    @click="showMenu = false"
                ></div>
            </Transition>
            <div class="sidebar" v-bind:class="{ 'show-menu': showMenu }">
                <Menu></Menu>
                <div v-if="isWebsite" class="download-desktop-link">
                    <el-link
                        @click="showDesktopEditionDownload = true"
                        type="info"
                    >
                        {{ $t('download-desktop-edition') }}
                    </el-link>
                </div>
            </div>
            <div class="main">
                <div class="topbar">
                    <Transition name="menuicon">
                        <el-icon
                            v-if="!unfoldSidebar"
                            :size="25"
                            @click="showMenu = !showMenu"
                        >
                            <Operation />
                        </el-icon>
                    </Transition>
                    <h3 class="topbar-title">
                        {{ topTitle == '' ? '' : $t(topTitle) }}
                    </h3>
                </div>
                <div style="height: calc(100% - var(--tnze-topbar-height))">
                    <router-view v-slot="{ Component }">
                        <keep-alive>
                            <component
                                :is="Component"
                                @setTitle="(s: string) => (topTitle = s)"
                            />
                        </keep-alive>
                    </router-view>
                </div>
            </div>
        </div>
    </el-config-provider>
</template>

<style>
@font-face {
    font-family: 'xivicon';
    src: url('./assets/fonts/XIV_Icon_Recreations.ttf');
}

#app {
    height: 100%;
    margin: 0;
    background: var(--tnze-mica-bg-color);
}

.el-form-item__label {
    user-select: none;
}

.el-message-box {
    border-radius: var(--el-border-radius-base);
}

.el-dialog {
    --el-dialog-border-radius: var(--el-border-radius-round) !important;
    border: 1px solid var(--el-border-color);
}

.el-card {
    --el-card-border-radius: var(--el-border-radius-base) !important;
}

.el-popper {
    --el-popper-border-radius: 10px;
}

:root {
    font-family:
        Inter, 'Helvetica Neue', Helvetica, 'PingFang SC', 'Hiragino Sans GB',
        'Microsoft YaHei', '微软雅黑', Arial, sans-serif, 'xivicon';
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;

    --el-fill-color-blank: transparent;

    --el-border-radius-base: 10px;
    --el-border-radius-small: 5px;
    --el-border-radius-round: 20px;
    --el-menu-bg-color: var(--app-bg-color);

    --tnze-content-raduis: var(--el-border-radius-round);
    --tnze-sidebar-width: 150px;
    --tnze-topbar-height: 40px;
}

@media screen and (min-width: 760px) {
    .el-menu {
        --el-menu-bg-color: transparent;
    }
}
</style>

<style scoped>
.container {
    height: 100%;
    display: flex;
    flex-direction: column;
}

.overlay {
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    background: rgba(0, 0, 0, 0.6);
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

.menuicon-enter-active {
    transition: transform 0.5s ease;
}

.menuicon-enter-from {
    position: fixed;
    transform: translateX(100%);
}

.sidebar {
    position: fixed;
    top: 0;
    left: 0;
    bottom: 0;
    z-index: 10;

    transform: translateX(-100%);
    transition: transform 0.5s;

    display: flex;
    flex-direction: column;
    width: var(--tnze-sidebar-width);
}

.sidebar.show-menu {
    transform: translateX(0);
}

.download-desktop-link {
    padding: 10px 6px;
    text-align: center;
}

.topbar {
    height: var(--tnze-topbar-height);
    width: 100%;
    box-sizing: border-box;
    flex: none;
    transition: height 0.5s;
    display: flex;
    align-items: center;
    overflow: hidden;
}

.topbar > .el-icon {
    color: var(--el-color-info);
    overflow: hidden;
}

.topbar-title {
    margin-left: 10px;
}

.main {
    height: 100%;
    padding: 0 0 0 10px;
    /* background-color: rgba(246, 246, 246, 0.5); */
    background-color: var(--tnze-mica-bg-color);
    flex: auto;

    transition: margin-left 0.5s;
}

@media screen and (min-width: 760px) {
    .sidebar {
        transform: translateX(0);
    }

    .topbar {
        border-bottom: initial;
    }

    .main {
        margin-left: var(--tnze-sidebar-width);
        border-top-left-radius: var(--tnze-content-raduis);
    }
}
</style>

<fluent locale="zh-CN">
error-save-file = 保存文件 { $file } 失败：{ $err }
download-desktop-edition = 下载桌面客户端
</fluent>

<fluent locale="zh-TW">
error-save-file = 儲存檔案 { $file } 失敗：{ $err }
download-desktop-edition = 下載桌面客戶端
</fluent>

<fluent locale="en-US">
error-save-file = Failed when saving { $file }: { $err }
download-desktop-edition = Download Desktop Edition
</fluent>

<fluent locale="ja-JP">
error-save-file = ファイル { $file } の保存に失敗しました: { $err }
download-desktop-edition = デスクトップ版のダウンロード
</fluent>
