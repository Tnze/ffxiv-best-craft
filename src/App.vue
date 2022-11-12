<script setup lang="ts">
import { ref, watchEffect } from 'vue';
import { useDark, usePreferredLanguages } from '@vueuse/core'
import { ElConfigProvider } from 'element-plus';
import { elementPlusLang, languages } from './lang';
import { selectLanguage } from './fluent'

import Menu from './components/Menu.vue';
import { useStore } from './store';

useDark()
const store = useStore()
const preferredLang = usePreferredLanguages()

const lang = ref('zh-CN')
watchEffect(() => {
  let settingLang: string | null = store.state.settings.language
  if (settingLang == 'system') settingLang = null
  const systemLang = preferredLang.value.find(v => languages.has(v))
  lang.value = settingLang ?? systemLang ?? 'zh-CN'
  selectLanguage(lang.value)
  console.log("language switched to", lang.value)
})

</script>

<template>
  <el-container>
    <el-aside width="64px">
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
}

.el-container {
  height: 100%;
}

.el-main {
  padding: 0;
}
</style>
