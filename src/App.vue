<script setup lang="ts">
import { ref, shallowRef } from 'vue';
import RecipePanel from './components/RecipePanel.vue';
import Gearsets from './components/Gearsets.vue';
import Designer from './components/designer/Designer.vue';
import Settings from './components/Settings.vue';
import Menu from './components/Menu.vue';
import { Attributes, Recipe, Jobs, new_recipe } from './Craft'

const pages = shallowRef([Gearsets, RecipePanel, Designer, Settings])
const attributes = ref<Attributes>({
  level: 82,
  craftsmanship: 2786,
  control: 2764,
  craft_points: 533,
})
const recipe = ref<Recipe | undefined>(undefined)
const job = ref<Jobs>(Jobs.Weaver)

new_recipe(535, 100, 100, 100).then((r) => { recipe.value = r })

const pageProps = ref([
  {},
  {},
  {
    itemName: "弗里金治愈耳夹",
    attributes,
    recipe,
    job,
  },
  {
    settings: {
      language: "zh-CN"
    }
  }
])
const currentPage = ref(2)
const handlePageSelect = (page: number) => {
  currentPage.value = page
}

</script>

<template>
  <el-container>
    <el-aside width="64px">
      <Menu @select="handlePageSelect"></Menu>
    </el-aside>
    <el-main>
      <keep-alive>
        <component :is="pages[currentPage]" :="pageProps[currentPage]"></component>
      </keep-alive>
    </el-main>

  </el-container>
</template>

<style>
#app {
  font-family: "Helvetica Neue", Helvetica, "PingFang SC", "Hiragino Sans GB",
    "Microsoft YaHei", "微软雅黑", Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
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
