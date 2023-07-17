<script setup lang="ts">
import { ElText, ElSelect, ElOption } from 'element-plus';
import { computed, ref } from 'vue';
import { useGuideStore, useSettingsStore } from '../../store';
import { RecipeInfo } from '../../Craft';
import { useRouter } from 'vue-router';

const router = useRouter()
const store = useGuideStore()
const settingStore = useSettingsStore()

store.setCurrentPage('welcome')

const time = computed<'morning' | 'noon' | 'afternoon' | 'evening' | 'night' | 'beforedawn'>(() => {
    const hour = new Date().getHours();
    if (hour >= 4 && hour < 6)
        return 'beforedawn'
    else if (hour >= 6 && hour < 11)
        return 'morning'
    else if (hour >= 11 && hour < 14)
        return 'noon'
    else if (hour >= 14 && hour < 19)
        return 'afternoon'
    else if (hour >= 19 && hour < 21)
        return 'evening'
    else
        return 'night'
})

const recipeSelected = ref<RecipeInfo>()
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
    store.setRecipeInfo(val)
    router.push('see-recipe')
}

</script>

<template>
    <div class="container">
        <div class="greeting-box">
            <el-text class="greeting">
                {{ $t('welcome', { time }) }}
            </el-text>
        </div>
        <div class="select-box">
            <el-select v-model="recipeSelected" value-key="id" size="large" :default-first-option="true"
                :placeholder="$t('input-recipe-name')" :loading-text="$t('loading')" :no-match-text="$t('no-match')"
                :no-data-text="$t('no-data')" :remote-method="searchRecipe" v-on:change="recipeChange" remote
                remote-show-suffix filterable :loading="loading">
                <el-option v-for="r in recipeOptions" :key="r.id" :label="r.item_name" :value="r" />
            </el-select>
        </div>
        <el-text class="info-text" type="info">{{ $t('guide-mode-info') }}</el-text>
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

.greeting-box {
    flex: 3 1 0;
    display: flex;
}

.greeting {
    font-size: 2em;
    align-self: flex-end;
    margin-bottom: 100px;
}

.select-box {
    flex: 2 1 0;
}

.confirm-button {
    margin-top: 50px;
}

.info-text {
    align-self: center;
    margin-bottom: 5px;
    white-space: pre-line;
    text-align: center;
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

guide-mode-info =
    注意：向导模式是一项实验性功能，旨在帮助新老工匠们快速生成制作手法。
    如有任何问题请至QQ频道反馈交流。
</fluent>