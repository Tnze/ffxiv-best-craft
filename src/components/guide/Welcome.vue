<script setup lang="ts">
import { ElText, ElSelect, ElOption } from 'element-plus';
import { computed, ref } from 'vue';
import { useGuideStore, useSettingsStore } from '../../store';
import { RecipeInfo } from '../../Craft';
import { useRouter } from 'vue-router';

const router = useRouter()
const guideStore = useGuideStore()
const settingStore = useSettingsStore()

const time = computed<'morning' | 'noon' | 'afternoon' | 'evening' | 'night' | 'beforedawn'>(() => {
    const hour = new Date().getHours();
    if (hour >= 4 && hour < 6)
        return 'beforedawn'
    else if (hour >= 6 && hour < 11)
        return 'morning'
    else if (hour >= 11 && hour < 14)
        return 'noon'
    else if (hour >= 14 && hour < 18)
        return 'afternoon'
    else if (hour >= 18 && hour < 21)
        return 'evening'
    else
        return 'night'
})

const recipeSelected = ref<RecipeInfo | undefined>()
const recipeOptions = ref<RecipeInfo[]>([])
const loading = ref(false)
async function searchRecipe(name: string) {
    try {
        loading.value = true
        const { recipes } = await settingStore.getDataSource.recipeTable(1, name)
        recipeOptions.value = recipes
    } catch (e: unknown) {
        recipeOptions.value = []
        console.error(e)
    } finally {
        loading.value = false
    }
}
function recipeChange(val: RecipeInfo) {
    guideStore.setRecipeInfo(val)
    router.push('see-recipe')
}

</script>

<template>
    <div class="container">
        <el-text class="greeting">{{ $t('welcome', { time }) }}</el-text>
        <el-select class="recipe-select" v-model="recipeSelected" value-key="id" size="large" :default-first-option="true"
            :placeholder="$t('input-recipe-name')" :loading-text="$t('loading')" :no-match-text="$t('no-match')"
            :no-data-text="$t('no-data')" :remote-method="searchRecipe" v-on:change="recipeChange" remote remote-show-suffix
            filterable :loading="loading">
            <el-option v-for="r in recipeOptions" :key="r.id" :label="r.item_name" :value="r" />
        </el-select>
    </div>
</template>

<style scoped>
.container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
}

.greeting {
    font-size: 2em;
    margin-bottom: 100px;
}

.confirm-button {
    margin-top: 50px;
}
</style>

<fluent locale="zh-CN">
greeting = 
    { $time ->
        [beforedawn] 日出东方隈，似从地底来
        [morning] 早上好
        [noon] 中午好
        [afternoon] 下午好
        [evening] 晚上好
        [night] 夜深了
        *[other] 很高兴见到你
    }
welcome = { greeting }，现在想搓点什么？
input-recipe-name = 输入配方名称
loading = 加载中
no-match = 没有匹配的配方
no-data = 无配方
</fluent>