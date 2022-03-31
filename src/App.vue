<script setup lang="ts">
import { reactive, ref, shallowRef, watch } from 'vue';
import RecipePanel from './components/recipe-manager/RecipePanel.vue';
import Gearsets from './components/Gearsets.vue';
import Designer from './components/designer/Designer.vue';
import Settings from './components/Settings.vue';
import Menu from './components/Menu.vue';
import { Attributes, Recipe, Jobs } from './Craft'

const attributes = reactive<Attributes>({
  level: 82,
  craftsmanship: 2786,
  control: 2764,
  craft_points: 533,
})

const recipe = ref<Recipe | null>(null)
const job = ref<Jobs>(Jobs.Culinarian)

const currentPage = ref(0)
const settings = ref({
  language: "zh-CN"
})
const recipeName = ref('')
const onRecipeChange = (j: Jobs, name: string, r: Recipe) => {
  job.value = j
  recipe.value = r
  recipeName.value = name
}

watch(attributes, () => {
  console.log(attributes)
})

</script>

<template>
  <el-container>
    <el-aside width="64px">
      <Menu @select="(page) => currentPage = page"></Menu>
    </el-aside>
    <el-main>
      <Gearsets v-model="attributes" v-if="currentPage == 0" />
      <keep-alive>
        <RecipePanel v-if="currentPage == 1" v-model="recipe" @change="onRecipeChange" />
      </keep-alive>
      <keep-alive>
        <Designer
          v-if="currentPage == 2"
          :item-name="recipeName"
          :attributes="attributes"
          :recipe="recipe"
          :job="job"
        />
      </keep-alive>
      <Settings v-if="currentPage == 3" :settings="settings" />
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
