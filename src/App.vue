<script setup lang="ts">
import { ref, computed } from 'vue';
import { useDark, useToggle } from '@vueuse/core'
import Menu from './components/Menu.vue';
import { Attributes, Recipe, Jobs } from './Craft'
import { useStore } from './store';

const isDark = useDark()

interface GearsetsRow {
  name: string
  value: Attributes | null
}

const store = useStore()

const job = ref<Jobs | 'unknown'>('unknown')

const recipe = ref<Recipe | null>(null)

const recipeName = ref('')
const onRecipeChange = (j: Jobs | 'unknown', name: string, r: Recipe) => {
  job.value = j
  recipe.value = r
  recipeName.value = name
  console.log("recipe changed!", r)
  // currentPage.value = '2'
}

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
