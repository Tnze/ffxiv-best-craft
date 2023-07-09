<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { ElContainer, ElHeader, ElRow, ElCol, ElMain, ElButton, ElInputNumber, ElForm, ElFormItem, ElSwitch, ElCheckboxGroup, ElCheckboxButton, ElText } from 'element-plus'
import { useRouter } from 'vue-router';
import { selectRecipe } from './common';
import { useFluent } from 'fluent-vue';
import { Conditions, Item, Recipe, RecipeInfo, RecipeLevel } from '../../Craft';
import { useSettingsStore } from '../../store';

const router = useRouter()
const { $t } = useFluent()
const settingStore = useSettingsStore()

const autoLoad = ref(true)
const autoLoadLoading = ref(false)

const defaultRecipe = {
    rlv: 0,
    job_level: 0,
    difficulty: 0,
    quality: 0,
    durability: 0,
    conditions_flag: 15
}
const defaultRecipeLevel = {
    class_job_level: 0,
    stars: 0,
    suggested_craftsmanship: 0,
    suggested_control: 0,
    difficulty: 0,
    quality: 0,
    progress_divider: 0,
    quality_divider: 0,
    progress_modifier: 0,
    quality_modifier: 0,
    durability: 0,
    conditions_flag: 0
}

const customRecipe = ref<Recipe>({ ...defaultRecipe })
const recipeLevel = ref<RecipeLevel>({ ...defaultRecipeLevel })
const conditionsFlag = computed({
    get: () => {
        const flag = customRecipe.value.conditions_flag
        return Object.values(Conditions).filter((_cond, i) => (flag & (1 << i)) != 0)
    },
    set: (val) => {
        customRecipe.value.conditions_flag = Object
            .values(Conditions)
            .map((cond, i) => val.indexOf(cond) >= 0 ? 1 << i : 0)
            .reduce((a, b) => a | b)
    }
})

let recipeLevelPromise: Promise<RecipeLevel> | null = null
watch(
    [
        autoLoad,
        () => settingStore.getDataSource,
        () => customRecipe.value.rlv
    ],
    async ([autoLoad, dataSource, rlv]) => {
        if (!autoLoad) return;
        try {
            autoLoadLoading.value = true
            if (recipeLevelPromise != null) await recipeLevelPromise
            recipeLevelPromise = dataSource.recipeLevelTable(rlv ?? 0)
            recipeLevel.value = await recipeLevelPromise
        } catch {
            recipeLevel.value = { ...defaultRecipeLevel }
        } finally {
            autoLoadLoading.value = false
        }
    }
)

function confirm() {
    // selectRecipe(r, recipeLevel, recipeInfo, itemInfo, '', false)
}

</script>

<template>
    <el-container>
        <el-header>
            <h1>{{ $t('custom-recipe') }}</h1>
        </el-header>
        <el-main>
            <el-row>
                <el-col :span="12">
                </el-col>
                <el-col :span="12">
                </el-col>
            </el-row>
            <el-form :inline="true" label-position="right" label-width="100px">
                <el-form-item :label="$t('class-job-level')">
                    <el-input-number v-model="customRecipe.job_level" :min="1"></el-input-number>
                </el-form-item>
                <el-form-item :label="$t('difficulty')">
                    <el-input-number v-model="customRecipe.difficulty" :min="1"></el-input-number>
                </el-form-item>
                <el-form-item :label="$t('quality')">
                    <el-input-number v-model="customRecipe.quality" :min="1"></el-input-number>
                </el-form-item>
                <el-form-item :label="$t('durability')">
                    <el-input-number v-model="customRecipe.durability" :min="1"></el-input-number>
                </el-form-item>
                <el-form-item :label="$t('conditions-flag')">
                    <el-text class="conditions-flag">{{ customRecipe.conditions_flag }}</el-text>
                    <el-checkbox-group v-model="conditionsFlag" size="small">
                        <el-checkbox-button v-for="cond in Conditions" :key="cond" :label="(cond as string)">
                            {{ $t(cond.toLowerCase()) }}
                        </el-checkbox-button>
                    </el-checkbox-group>
                </el-form-item>
                <el-form-item :label="$t('recipe-level')">
                    <el-input-number v-model="customRecipe.rlv"></el-input-number>
                </el-form-item>
                <el-form-item :label="$t('auto-load')">
                    <el-switch v-model="autoLoad" :loading="autoLoadLoading"></el-switch>
                </el-form-item>
                <el-form-item :label="$t('progress-divider')">
                    <el-input-number :disabled="autoLoad" v-model="recipeLevel.progress_divider"></el-input-number>
                </el-form-item>
                <el-form-item :label="$t('progress-modifier')">
                    <el-input-number :disabled="autoLoad" v-model="recipeLevel.progress_modifier"></el-input-number>
                </el-form-item>
                <el-form-item :label="$t('quality-divider')">
                    <el-input-number :disabled="autoLoad" v-model="recipeLevel.quality_divider"></el-input-number>
                </el-form-item>
                <el-form-item :label="$t('quality-modifier')">
                    <el-input-number :disabled="autoLoad" v-model="recipeLevel.quality_modifier"></el-input-number>
                </el-form-item>
            </el-form>
            <span>
                <el-button type="primary" @click="confirm">{{ $t('confirm') }}</el-button>
            </span>
        </el-main>
    </el-container>
</template>

<style scoped>
.conditions-flag {
    margin-right: 5px;
}
</style>

<fluent locale="zh-CN">
back = 返回
custom-recipe = 自定义配方
conditions-flag = 球色标志
progress-divider = 作业难度系数
progress-modifier = 作业压制系数
quality-divider = 加工难度系数
quality-modifier = 加工压制系数
auto-load = 自动填充

cancel = 取消
confirm = 确认
</fluent>

<fluent locale="en-US">
back = Back
custom-recipe = Custom Recipe
conditions-flag = Cond. Flag
auto-load = Auto Fill

cancel = Cancel
confirm = Confirm
</fluent>
