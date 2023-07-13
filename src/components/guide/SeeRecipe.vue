<script setup lang="ts">
import { computed, ref } from 'vue';
import { ElDescriptions, ElDescriptionsItem, ElButton, ElResult, ElText } from 'element-plus';
import { useGearsetsStore, useGuideStore, useSettingsStore } from '../../store';
import { Item, Recipe, RecipeInfo, RecipeLevel, newRecipe } from '../../Craft';
import { craftTypeTojobs } from '../recipe-manager/common'
import { useRouter } from 'vue-router';

const router = useRouter()
const guideStore = useGuideStore()
const settingStore = useSettingsStore()
const gearsetsStore = useGearsetsStore()

const loading = ref(true)
const errorMessage = ref<string | null>(null)
const itemInfo = ref<Item | null>(null)
const recipeLevel = ref<RecipeLevel | null>(null)
const recipe = ref<Recipe | null>(null)
const recipeInfo = computed(() => guideStore.recipeInfo)

retry()

function back() {
    router.replace('welcome')
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
        itemInfo.value = iinfo
        recipeLevel.value = rlv
        recipe.value = await newRecipe(
            recipeInfo.value.rlv,
            rlv,
            recipeInfo.value.difficulty_factor,
            recipeInfo.value.quality_factor,
            recipeInfo.value.durability_factor,
        )
    } catch (e: unknown) {
        errorMessage.value = String(e)
    } finally {
        loading.value = false
    }
}

const attributes = computed(() => {
    if (recipeInfo.value == null)
        return null
    const job = craftTypeTojobs(recipeInfo.value.job)
    const special = gearsetsStore.special.find(v => v.name == job)
    return special ?? { name: 'unknown', value: gearsetsStore.default }
})
const craftingCheckResult = computed(() => {
    if (recipe.value == null || attributes.value == null)
        return '';
    if (recipe.value.job_level > (attributes.value.value?.level ?? 0) + 5)
        return 'class-job-level-too-low'
    return 'crafting-check-success'
})

</script>

<template>
    <el-result v-if="errorMessage != null" icon="error" :title="$t('error-happens')" :sub-title="$t(errorMessage)">
        <template #extra>
            <el-button @click="back">{{ $t('back') }}</el-button>
            <el-button @click="retry" type="primary">{{ $t('retry') }}</el-button>
        </template>
    </el-result>
    <div v-else v-tnze-loading="loading" class="container">
        <div class="main-content">
            <el-text>{{ $t(craftingCheckResult, { job: $t(attributes?.name ?? 'unknown') }) }}</el-text>
        </div>
        <el-descriptions v-if="recipeInfo && recipeLevel && recipe" :title="$t('recipe-info')" size="small" :column="4">
            <el-descriptions-item :label="$t('item-name')" :span="2">{{ recipeInfo.item_name }}</el-descriptions-item>
            <el-descriptions-item :label="$t('recipe-level')">{{ recipeInfo.rlv }}</el-descriptions-item>
            <el-descriptions-item :label="$t('job-level')">{{ recipe.job_level }}</el-descriptions-item>
            <el-descriptions-item :label="$t('job-class')">{{ recipeInfo.job }}</el-descriptions-item>
            <el-descriptions-item :label="$t('difficulty')">{{ recipe.difficulty }}</el-descriptions-item>
            <el-descriptions-item :label="$t('quality')">{{ recipe.quality }}</el-descriptions-item>
            <el-descriptions-item :label="$t('durability')">{{ recipe.durability }}</el-descriptions-item>
            <el-descriptions-item :label="$t('suggested-craftsmanship')">
                {{ recipeLevel.suggested_craftsmanship }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('suggested-control')">
                {{ recipeLevel.suggested_control }}
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

.el-text {
    font-size: 1.5em;
    margin-bottom: 100px;
}
</style>

<fluent locale="zh-CN">
recipe-info = 配方信息
item-name = 物品名称
job-class = 职业类型
recipe-level = 配方等级
job-level = 职业等级
can-hq = 是否HQ
suggested-craftsmanship = 建议{ craftsmanship }
suggested-control = 建议{ control }
required-craftsmanship = 最低{ craftsmanship }
required-control = 最低{ control }
true = 是
false = 否

error-happens = 加载配方时出现了一些错误
back = 返回
retry = 重试

class-job-level-too-low = 你的{ $job }等级似乎没有满足该配方的要求
crafting-check-success = 没有发现任何问题！可以开始制作了
</fluent>
