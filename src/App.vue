<script setup lang="ts">
import { reactive, ref, shallowRef, watch } from 'vue';
import 'element-plus/es/components/message/style/css'
import { ElMessage } from 'element-plus'
import RecipePanel from './components/recipe-manager/RecipePanel.vue';
import Gearsets from './components/Gearsets.vue';
import Designer from './components/designer/Designer.vue';
import Settings from './components/Settings.vue';
import Menu from './components/Menu.vue';
import { Attributes, Recipe, Jobs } from './Craft'
import { computed } from '@vue/reactivity';


interface GearsetsRow {
  name: string
  label: string
  value: Attributes | null
}

const gearsets = ref<{ default: Attributes, special: GearsetsRow[] }>({
  default: {
    level: 82,
    craftsmanship: 2786,
    control: 2764,
    craft_points: 533,
  },
  special: [
    { name: 'carpenter', label: "刻木匠", value: null },
    { name: 'blacksmith', label: "锻铁匠", value: null },
    { name: 'armorer', label: "铸甲匠", value: null },
    { name: 'goldsmith', label: "雕金匠", value: null },
    { name: 'leatherworker', label: "制革匠", value: null },
    { name: 'weaver', label: "裁衣匠", value: null },
    { name: 'alchemist', label: "炼金术士", value: null },
    { name: 'culinarian', label: "烹调师", value: null },
  ]
})

const job = ref<Jobs>(Jobs.Culinarian)
const attributes = computed(() => {
  return gearsets.value.special.find(v => v.name == job.value)?.value || gearsets.value.default
})

const recipe = ref<Recipe | null>(null)

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

</script>

<template>
  <el-container>
    <el-aside width="64px">
      <Menu @select="(page) => currentPage = page"></Menu>
    </el-aside>
    <el-main>
      <Gearsets v-model="gearsets" v-show="currentPage == 0" />
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
