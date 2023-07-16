<script setup lang="ts">
import { computed, ref, watchEffect } from 'vue';
import { ElDescriptions, ElDescriptionsItem, ElButton, ElResult, ElSelectV2 } from 'element-plus';
import { useGearsetsStore, useGuideStore, useSettingsStore } from '../../store';
import { newRecipe } from '../../Craft';
import { craftTypeTojobs } from '../recipe-manager/common'
import { useRouter } from 'vue-router';
import meal from '../../assets/data/meal.json'
import potions from '../../assets/data/potions.json'
import { Enhancer } from '../attr-enhancer/Enhancer';

const router = useRouter()
const guideStore = useGuideStore()
const settingStore = useSettingsStore()
const gearsetsStore = useGearsetsStore()

const loading = ref(true)
const errorMessage = ref<string>()
const recipeInfo = computed(() => guideStore.recipeInfo)

guideStore.setCurrentPage('see-recipe')
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
        const dataSource = settingStore.getDataSource
        const [iinfo, rlv] = await Promise.all([
            dataSource.itemInfo(recipeInfo.value.item_id),
            dataSource.recipeLevelTable(recipeInfo.value.rlv),
        ])
        guideStore.setItemInfo(iinfo)
        guideStore.setRecipeLevel(rlv)
        guideStore.setRecipe(await newRecipe(
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
    guideStore.setAttributes(
        special.name,
        special.value ?? gearsetsStore.default
    )
})
const craftingCheckResultIcon = ref<"success" | "warning" | "info" | "error">('success')
const craftingCheckResult = computed(() => {
    craftingCheckResultIcon.value = 'error'
    if (guideStore.recipe == null || guideStore.craftType == null || guideStore.craftTypeAttr == null || recipeInfo.value == null)
        return '';
    if (guideStore.recipe.job_level > (guideStore.craftTypeAttr.level ?? 0) + 5)
        return 'class-job-level-too-low'
    if (recipeInfo.value.required_craftsmanship > guideStore.craftTypeAttr.craftsmanship
        || recipeInfo.value.required_control > guideStore.craftTypeAttr.control)
        return 'class-job-attributes-too-low'

    craftingCheckResultIcon.value = 'success'
    return 'crafting-check-success'
})

const mealSelected = ref<Enhancer>()
const potionSelected = ref<Enhancer>()
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
                :title="$t(craftingCheckResult, { job: $t(guideStore.craftType ?? 'unknown-job') })" :sub-title="$t(craftingCheckResult + '-detail', {
                    job: $t(guideStore.craftType ?? 'unknown-job'),
                    minLevel: (guideStore.recipe?.job_level ?? 0) - 5,
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
            <el-select-v2 class="enhancer-select-box" v-model="mealSelected" clearable :placeholder="$t('select-meal')"
                :options="mealOptions" value-key="value.name" />
            <el-select-v2 class="enhancer-select-box" v-model="potionSelected" clearable :placeholder="$t('select-potions')"
                :options="potionOptions" value-key="value.name" />
        </div>
        <el-descriptions v-if="recipeInfo && guideStore.recipeLevel && guideStore.recipe" :title="$t('recipe-info')"
            size="small" :column="4">
            <el-descriptions-item :label="$t('item-name')" :span="2">{{ recipeInfo.item_name }}</el-descriptions-item>
            <el-descriptions-item :label="$t('recipe-level')">{{ recipeInfo.rlv }}</el-descriptions-item>
            <el-descriptions-item :label="$t('job-level')">{{ guideStore.recipe.job_level }}</el-descriptions-item>
            <el-descriptions-item :label="$t('job-class')">{{ recipeInfo.job }}</el-descriptions-item>
            <el-descriptions-item :label="$t('difficulty')">{{ guideStore.recipe.difficulty }}</el-descriptions-item>
            <el-descriptions-item :label="$t('quality')">{{ guideStore.recipe.quality }}</el-descriptions-item>
            <el-descriptions-item :label="$t('durability')">{{ guideStore.recipe.durability }}</el-descriptions-item>
            <el-descriptions-item :label="$t('suggested-craftsmanship')">
                {{ guideStore.recipeLevel.suggested_craftsmanship }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('suggested-control')">
                {{ guideStore.recipeLevel.suggested_control }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('required-craftsmanship')">
                {{ recipeInfo.required_craftsmanship }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('required-control')">
                {{ recipeInfo.required_control }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('can-hq')">{{ $t(String(recipeInfo.can_hq)) }}</el-descriptions-item>
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
recipe-info = 配方信息
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
select-meal = 选择食物
select-potions = 选择药水

class-job-level-too-low = { $job }等级过低
class-job-level-too-low-detail = 您可能需要将{ $job }升至{ $minLevel }级才能制作该配方
class-job-attributes-too-low = { $job }装备属性过低
class-job-attributes-too-low-detail = 您可能需要考虑更换更好的{ $job }装备，或使用食物和药水，以满足{ craftsmanship } ≥ { $minCraftsmanship }、{ control } ≥ { $minControl }
crafting-check-success = 没有发现任何问题，可以开始制作了！
crafting-check-success-detail = 接下来将运行自动求解算法，帮助您了解制作该配方的手法
</fluent>
