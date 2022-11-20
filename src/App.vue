<script setup lang="ts">
import { ref, watchEffect } from 'vue';
import { useDark, usePreferredLanguages, useCssVar } from '@vueuse/core';
import { Dir, readTextFile } from '@tauri-apps/api/fs';
import { platform } from '@tauri-apps/api/os'
import { ElContainer, ElAside, ElMain, ElConfigProvider } from 'element-plus';
import { elementPlusLang, languages } from './lang';
import { selectLanguage } from './fluent'

import Menu from './components/Menu.vue';
import { useStore } from './store';

useDark()
const store = useStore()
const preferredLang = usePreferredLanguages()
const bgColor = useCssVar('--app-bg-color', ref(null))

const lang = ref('zh-CN')
watchEffect(() => {
  let settingLang: string | null = store.state.settings.language
  if (settingLang == 'system') settingLang = null
  const systemLang = preferredLang.value.find(v => languages.has(v))
  lang.value = settingLang ?? systemLang ?? 'zh-CN'
  selectLanguage(lang.value)
  console.log("language switched to", lang.value)
})

readTextFile("settings.json", { dir: Dir.App }).then(str => {
  store.commit('loadSettings', JSON.parse(str))
}).catch(_err => { })

platform().then(pf => {
  console.log("platform:", pf)
  bgColor.value = ['darwin', 'win32'].includes(pf) ? 'transparent' : 'white'
})

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
