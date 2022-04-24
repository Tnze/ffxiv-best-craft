<script setup lang="ts">
import { ref, computed } from 'vue';
import RecipePanel from './components/recipe-manager/RecipePanel.vue';
import Gearsets from './components/Gearsets.vue';
import Designer from './components/designer/Designer.vue';
import Settings from './components/Settings.vue';
import Menu from './components/Menu.vue';
import { Attributes, Recipe, Jobs } from './Craft'


interface GearsetsRow {
  name: string
  value: Attributes | null
}

const gearsets = ref<{ default: Attributes, special: GearsetsRow[] }>({
  default: {
    level: 90,
    craftsmanship: 2786,
    control: 2764,
    craft_points: 533,
  },
  special: [
    { name: 'carpenter', value: null },
    { name: 'blacksmith', value: null },
    { name: 'armorer', value: null },
    { name: 'goldsmith', value: null },
    { name: 'leatherworker', value: null },
    { name: 'weaver', value: null },
    { name: 'alchemist', value: null },
    { name: 'culinarian', value: null },
  ]
})

const job = ref<Jobs | 'unknown'>('unknown')
const attributes = computed<Attributes>(() => {
  return gearsets.value.special.find(v => v.name == job.value)?.value || gearsets.value.default
})

const recipe = ref<Recipe | null>(null)

const currentPage = ref('0')
const settings = ref({
  language: "zh-CN"
})
const recipeName = ref('')
const onRecipeChange = (j: Jobs | 'unknown', name: string, r: Recipe) => {
  debugger
  job.value = j
  recipe.value = r
  recipeName.value = name
  currentPage.value = '2'
}

</script>

<template>
  <el-container>
    <el-aside width="64px">
      <Menu v-model="currentPage"></Menu>
    </el-aside>
    <el-main>
      <Gearsets v-model="gearsets" v-show="currentPage == 0" />
      <keep-alive>
        <RecipePanel v-if="currentPage == 1" v-model="recipe" @change="onRecipeChange" />
      </keep-alive>
      <keep-alive>
        <Suspense v-if="currentPage == 2">
          <Designer v-if="recipe != null" :item-name="recipeName" :attributes="attributes" :recipe="recipe"
            :job="job" />
          <el-empty v-else description="请先选择配方" style="height: 100%;" />
          <template #fallback>
            <el-empty description="加载中" style="height: 100%;" />
          </template>
        </Suspense>
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
