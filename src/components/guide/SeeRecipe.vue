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
import { computed, ref, watchEffect } from 'vue';
import { ElDescriptions, ElDescriptionsItem, ElButton, ElResult, ElSelectV2, ElCheckbox } from 'element-plus';
import useGearsetsStore from '@/stores/gearsets';
import useGuideStore from '@/stores/guide';
import useSettingsStore from '@/stores/settings';
import { newRecipe } from '@/libs/Craft';
import { craftTypeTojobs } from '../recipe-manager/common'
import { useRouter } from 'vue-router';
import meal from '@/assets/data/meal.json'
import potions from '@/assets/data/potions.json'

const router = useRouter()
const store = useGuideStore()
const settingStore = useSettingsStore()
const gearsetsStore = useGearsetsStore()

const loading = ref(true)
const errorMessage = ref<string>()
const recipeInfo = computed(() => store.recipeInfo)

store.setCurrentPage('see-recipe')
retry()

function back() {
    router.replace('welcome')
}

function start() {
    router.replace('solving')
}

async function retry() {
    if (recipeInfo.value == null) {
        back()
        return
    }
    loading.value = true
    try {
        const dataSource = await settingStore.getDataSource
        const [iinfo, rlv] = await Promise.all([
            dataSource.itemInfo(recipeInfo.value.item_id),
            dataSource.recipeLevelTable(recipeInfo.value.rlv),
        ])
        store.setItemInfo(iinfo)
        store.setRecipeLevel(rlv)
        store.setRecipe(await newRecipe(
            recipeInfo.value.rlv,
            rlv,
            recipeInfo.value.difficulty_factor,
            recipeInfo.value.quality_factor,
            recipeInfo.value.durability_factor,
        ))
    } catch (e: unknown) {
        errorMessage.value = String(e)
    } finally {
        loading.value = false
    }
}

watchEffect(() => {
    if (recipeInfo.value == null)
        return null
    const job = craftTypeTojobs(recipeInfo.value.job)
    const special = gearsetsStore.special.find(v => v.name == job)
    if (special == undefined)
        return null
    store.setAttributes(
        special.name,
        special.value ?? gearsetsStore.default
    )
})
const craftingCheckResultIcon = ref<"success" | "warning" | "info" | "error">('success')
const craftingCheckResult = computed(() => {
    craftingCheckResultIcon.value = 'error'
    if (store.recipe == null || store.craftType == null || store.craftTypeAttr == null || recipeInfo.value == null)
        return '';
    if (store.recipe.job_level > (store.craftTypeAttr.level ?? 0) + 5)
        return 'class-job-level-too-low'
    if (recipeInfo.value.required_craftsmanship > store.craftTypeAttr.craftsmanship
        || recipeInfo.value.required_control > store.craftTypeAttr.control)
        return 'class-job-attributes-too-low'

    craftingCheckResultIcon.value = 'success'
    return 'crafting-check-success'
})

const mealOptions = meal.map(v => ({
    value: v,
    label: v.name
}))
const potionOptions = potions.map(v => ({
    value: v,
    label: v.name
}))
</script>

<template>
    <el-result v-if="errorMessage" icon="error" :title="$t('error-happens')" :sub-title="$t(errorMessage)">
        <template #extra>
            <el-button @click="back">{{ $t('back') }}</el-button>
            <el-button @click="retry" type="primary">{{ $t('retry') }}</el-button>
        </template>
    </el-result>
    <div v-else v-tnze-loading="loading" class="container">
        <div class="main-content">
            <el-result v-if="craftingCheckResult != ''" :icon="craftingCheckResultIcon"
                :title="$t(craftingCheckResult, { job: $t(store.craftType ?? 'unknown-job') })" :sub-title="$t(craftingCheckResult + '-detail', {
                    job: $t(store.craftType ?? 'unknown-job'),
                    minLevel: (store.recipe?.job_level ?? 0) - 5,
                    minCraftsmanship: recipeInfo?.required_craftsmanship ?? 0,
                    minControl: recipeInfo?.required_control ?? 0,
                })">
                <template #extra>
                    <el-button @click="back">{{ $t('back') }}</el-button>
                    <el-button @click="start" type="primary" v-if="craftingCheckResult == 'crafting-check-success'">
                        {{ $t('start') }}
                    </el-button>
                </template>
            </el-result>
        </div>
        <div class="enhancer-selectors">
            <el-select-v2 class="enhancer-select-box" v-model="store.food" clearable :placeholder="$t('select-meals')"
                :options="mealOptions" value-key="value.name" />
            <el-select-v2 class="enhancer-select-box" v-model="store.potion" clearable :placeholder="$t('select-potions')"
                :options="potionOptions" value-key="value.name" />
            <el-checkbox v-model="store.manipulation" :label="$t('manipulation')" />
        </div>
        <el-descriptions v-if="recipeInfo && store.recipeLevel && store.recipe" :title="$t('crafting-info')" size="small"
            :column="4">
            <el-descriptions-item :label="$t('item-name')" :span="2">{{ recipeInfo.item_name }}</el-descriptions-item>
            <el-descriptions-item :label="$t('recipe-level')">{{ recipeInfo.rlv }}</el-descriptions-item>
            <el-descriptions-item :label="$t('job-level')">{{ store.recipe.job_level }}</el-descriptions-item>
            <el-descriptions-item :label="$t('job-class')">{{ recipeInfo.job }}</el-descriptions-item>
            <el-descriptions-item :label="$t('difficulty')">{{ store.recipe.difficulty }}</el-descriptions-item>
            <el-descriptions-item :label="$t('quality')">{{ store.recipe.quality }}</el-descriptions-item>
            <el-descriptions-item :label="$t('durability')">{{ store.recipe.durability }}</el-descriptions-item>
            <el-descriptions-item :label="$t('suggested-craftsmanship')">
                {{ store.recipeLevel.suggested_craftsmanship }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('suggested-control')">
                {{ store.recipeLevel.suggested_control }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('required-craftsmanship')">
                {{ recipeInfo.required_craftsmanship }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('required-control')">
                {{ recipeInfo.required_control }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('can-hq')">{{ $t(String(recipeInfo.can_hq)) }}</el-descriptions-item>
            <template v-if="store.craftTypeAttr">
                <el-descriptions-item :label="$t('craftsmanship')">
                    {{ store.craftTypeAttr.craftsmanship }}
                </el-descriptions-item>
                <el-descriptions-item :label="$t('control')">{{ store.craftTypeAttr.control }}</el-descriptions-item>
                <el-descriptions-item :label="$t('craft-point')">{{ store.craftTypeAttr.craft_points
                }}</el-descriptions-item>
            </template>
        </el-descriptions>
    </div>
</template>

<style scoped>
.container {
    display: flex;
    flex-direction: column;
    height: 100%;
}

.main-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}

.enhancer-selectors {
    display: flex;
    justify-content: center;
    margin-bottom: 15px;
    align-items: center;
}

.enhancer-select-box {
    margin: 7px;
}

.title {
    font-size: 1.5em;
    margin-bottom: 10px;
}

.subtitle {
    font-size: 1em;
    margin-bottom: 90px;
}
</style>

<fluent locale="zh-CN">
crafting-info = 制作信息
item-name = 物品名称
job-class = 职业类型
recipe-level = 配方等级
job-level = 职业等级
can-hq = 存在HQ
suggested-craftsmanship = 建议{ craftsmanship }
suggested-control = 建议{ control }
required-craftsmanship = 最低{ craftsmanship }
required-control = 最低{ control }
true = 是
false = 否

error-happens = 加载配方时出现了一些错误
back = 返回
retry = 重试
start = 开始
unknown-job = 某职业
select-meals = 选择食物
select-potions = 选择药水

class-job-level-too-low = { $job }等级过低
class-job-level-too-low-detail = 您可能需要将{ $job }升至{ $minLevel }级才能制作该配方
class-job-attributes-too-low = { $job }装备属性过低
class-job-attributes-too-low-detail = 您可能需要考虑更换更好的{ $job }装备、使用食物和药水，以满足{ craftsmanship } ≥ { $minCraftsmanship }、{ control } ≥ { $minControl }
crafting-check-success = 可以开始制作了！
crafting-check-success-detail = 点击{ start }运行自动求解算法，并生成游戏宏
</fluent>

<fluent locale="en-US">
crafting-info = Crafting Info
item-name = Item Name
job-class = Job Class
recipe-level = Recipe Level
job-level = Job Level
can-hq = Can be HQ
suggested-craftsmanship = Suggested { craftsmanship }
suggested-control = Suggested { control }
required-craftsmanship = Required { craftsmanship }
required-control = Required { control }
true = True
false = False

error-happens = 加载配方时出现了一些错误
back = Back
retry = Retry
start = Start
unknown-job = Unknown Job
select-meals = Select Meals
select-potions = Select Potions

class-job-level-too-low = { $job } level not enough.
class-job-level-too-low-detail = You may need to raise { $job } to Lv. { $minLevel } to craft this recipe.
class-job-attributes-too-low = { $job } gearsets attributes not enough.
class-job-attributes-too-low-detail = 您可能需要考虑更换更好的{ $job }装备、使用食物和药水，以满足{ craftsmanship } ≥ { $minCraftsmanship }、{ control } ≥ { $minControl }
class-job-attributes-too-low-detail = You may need to consider replacing better { $job } equipment, using meals and potions to meet { craftsmanship } ≥ { $minCraftsmanship } and { control } ≥ { $minControl }.
crafting-check-success = It's all ready to crafting!
crafting-check-success-detail = Click the { start } button to running the solver algorithms and generating macros.
</fluent>
