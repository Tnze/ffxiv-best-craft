<script setup lang="ts">
import { ref, shallowRef } from 'vue';
import RecipePanel from './components/recipe-manager/RecipePanel.vue';
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
const job = ref<Jobs>(Jobs.Culinarian)

new_recipe(535, 100, 100, 100).then((r) => { recipe.value = r })

const currentPage = ref(2)
const settings = ref({
  language: "zh-CN"
})
const recipeName = ref("弗里金治愈耳夹")
const onRecipeChange = (j: Jobs, name: string) => {
  job.value = j
  recipeName.value = name
}

</script>

<template>
  <el-container>
    <el-aside width="64px">
      <Menu @select="(page) => currentPage = page"></Menu>
    </el-aside>
    <el-main>
      <keep-alive>
        <Gearsets v-model:attr="attributes" v-if="currentPage == 0" />
        <RecipePanel v-else-if="currentPage == 1" @change="onRecipeChange" />
        <Designer
          v-else-if="currentPage == 2"
          :item-name="recipeName"
          :attributes="attributes"
          :recipe="recipe"
          :job="job"
        />
        <Settings v-else-if="currentPage == 3" :settings="settings" />
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
