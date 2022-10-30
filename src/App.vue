<script setup lang="ts">
import { useDark } from '@vueuse/core'
import { ElConfigProvider } from 'element-plus';
import zhCn from 'element-plus/lib/locale/lang/zh-cn'
import en from 'element-plus/lib/locale/lang/en'
import ja from 'element-plus/lib/locale/lang/ja'
import { useStore } from './store';

import Menu from './components/Menu.vue';

useDark()
const store = useStore()

const languages = new Map([
  ["zh-CN", zhCn],
  ["en", en],
  ["ja", ja],
])

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
    <el-config-provider :locale="languages.get(store.state.settings.language)"></el-config-provider>
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
