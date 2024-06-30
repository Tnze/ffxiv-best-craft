<!-- 
    This file is part of BestCraft.
    Copyright (C) 2023  Tnze

    BestCraft is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    BestCraft is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
-->

<script setup lang="ts">
import { ElText, ElButton } from 'element-plus';
import { computed } from 'vue';
import useGuideStore from '@/stores/guide';
import { useFluent } from 'fluent-vue';
// import { Recipe, RecipeInfo } from '../../Craft';
// import { useRouter } from 'vue-router';
// import { DataSource } from '../recipe-manager/source';

// const router = useRouter()
const store = useGuideStore()
// const settingStore = useSettingsStore()
const { $t } = useFluent()

store.setCurrentPage('welcome')

// async function recipes(ids: number[], ds: DataSource): Promise<CascaderOption[]> {
//     return await Promise.all(ids.map(async id => {
//         let info = await ds.recipeInfo(id)
//         return { value: id, label: info.item_name }
//     }))
// }

// const recipeSelectorProps: CascaderProps = {
//     lazy: true,
//     lazyLoad(node, resolve) {
//         const { level } = node
//         switch (level) {
//             case 0:
//                 resolve([
//                     { label: "640HQ", value: "640HQ" }
//                 ])
//                 break;
//             case 1:
//                 break;
//             default:
//                 resolve([])
//         }
//     },
// }

// async function selectedRecipe(value: CascaderValue) {
//     while (Array.isArray(value)) // 取数组最后一项
//         value = value.at(-1) ?? 0;
//     readyToConfirm.value = true
//     console.log("selected recipe " + value)
// }

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

</script>

<template>
    <div class="container">
        <div class="greeting-box">
            <el-text class="greeting">
                {{ $t('welcome', { time }) }}
            </el-text>
        </div>
        <!-- <div class="select-box">
            <el-cascader-panel :props="recipeSelectorProps" @change="selectedRecipe" />
        </div> -->
        <div class="confirm-button">
            <!-- <el-button type="primary" :disabled="!readyToConfirm">{{ $t('confirm') }}</el-button> -->
            <el-button type="primary" size="large" @click="$router.push('/recipe')">{{ $t('select-recipe')
                }}</el-button>
        </div>
        <el-text class="info-text" type="info">{{ $t('copyright-notices') }}</el-text>
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
    flex: 2 1 0;
    display: flex;
}

.greeting {
    font-size: 2em;
    align-self: flex-end;
    margin-bottom: 100px;
}

.select-box {
    flex: 4 auto 0;
}

.confirm-button {
    flex: 1 0 0;
    margin-top: 10px;
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

confirm = 确认
select-recipe = 选择配方

guide-mode-info =
    注意：实验性向导模式已删除。如有疑问欢迎反馈。
</fluent>

<fluent locale="en-US">
greeting = 
    { $time ->
        [beforedawn] Good morning
        [morning] Good morning
        [noon] Good afternoon
        [afternoon] Good afternoon
        [evening] Good evening
        [night] It's getting late at night
        *[other] Nice to see you
    }
welcome = { greeting }. What do you want to craft?
input-recipe-name = Input recipe name
loading = Loading
no-match = No match recipe
no-data = No recipe

confirm = Confirm
select-recipe = Select recipe

guide-mode-info =
    Note: The wizard mode as an experimental feature is removed.
    If you have any questions, welcome to feedback in Github.
</fluent>
