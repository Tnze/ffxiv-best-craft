<script setup lang="ts">
import { defineComponent, ref, shallowRef } from 'vue';
import Recipe from './components/Recipe.vue';
import Gearsets from './components/Gearsets.vue';
import Designer from './components/designer/Designer.vue';
import Menu from './components/Menu.vue';
import { Attributes, Status, Conditions } from './Craft'

const pages = shallowRef([Recipe, Gearsets, Designer])
const pageProps = ref([{}, {}, {
  itemName: "弗里金治愈耳夹",
  attributes: {
    level: 80,
    craftsmanship: 2000,
    control: 3000,
    craftPoints: 400,
  },
  status: {
    condition: Conditions.Excellent
  }
}])
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
