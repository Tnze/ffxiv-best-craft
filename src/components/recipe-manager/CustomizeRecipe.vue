<!-- 
    This file is part of BestCraft.
    Copyright (C) 2025  Tnze

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
import { computed, onActivated, ref, watch } from 'vue';
import {
    ElButton,
    ElInputNumber,
    ElForm,
    ElFormItem,
    ElSwitch,
    ElCheckboxGroup,
    ElCheckboxButton,
    ElScrollbar,
} from 'element-plus';
import { useRouter } from 'vue-router';
import { selectRecipe } from './common';
import { useFluent } from 'fluent-vue';
import {
    Conditions,
    Item,
    Recipe,
    RecipeLevel,
    RecipeRequirements,
} from '@/libs/Craft';
import useSettingsStore from '@/stores/settings';

const router = useRouter();
const { $t } = useFluent();
const settingStore = useSettingsStore();

const emit = defineEmits<{
    (e: 'setTitle', title: string): void;
}>();
onActivated(() => emit('setTitle', 'custom-recipe'));

const autoLoad = ref(false);
const autoLoadLoading = ref(false);

const defaultRecipeLevel: RecipeLevel = {
    id: 720,
    class_job_level: 100,
    stars: 0,
    suggested_craftsmanship: 4780,
    suggested_control: null,
    difficulty: 8050,
    quality: 17600,
    progress_divider: 170,
    quality_divider: 150,
    progress_modifier: 90,
    quality_modifier: 75,
    durability: 70,
    conditions_flag: 15,
};
const defaultRecipe: Recipe = {
    rlv: defaultRecipeLevel,
    job_level: 100,
    difficulty: 8050,
    quality: 17600,
    durability: 70,
    conditions_flag: 15,
};

const rlv = ref(defaultRecipeLevel.id);
const customRecipe = ref<Recipe>({
    ...defaultRecipe,
    rlv: { ...defaultRecipeLevel },
});
const conditionsFlag = computed<Conditions[]>({
    get: () => {
        const flag = customRecipe.value.conditions_flag;
        return Object.values(Conditions).filter(
            (_cond, i) => (flag & (1 << i)) != 0,
        );
    },
    set: val => {
        const flag = Object.values(Conditions)
            .map((cond, i) => (val.indexOf(cond) >= 0 ? 1 << i : 0))
            .reduce((a, b) => a | b, 0);
        customRecipe.value.conditions_flag = flag;
    },
});

let recipeLevelPromise: Promise<RecipeLevel> | null = null;
watch(
    [autoLoad, settingStore.getDataSource, rlv],
    async ([autoLoadValue, dataSource, rlv]) => {
        if (!autoLoadValue) return;
        try {
            autoLoadLoading.value = true;
            if (recipeLevelPromise != null) await recipeLevelPromise;
            const ds = await dataSource;
            recipeLevelPromise = ds.recipeLevelTable(rlv);
            customRecipe.value.rlv = await recipeLevelPromise;
            recipeLevelPromise = null;
        } catch {
            autoLoad.value = false;
        } finally {
            autoLoadLoading.value = false;
        }
    },
);

function confirm(simulatorMode: boolean) {
    const itemInfo: Item = {
        id: -1,
        name: $t('custom-recipe'),
        level: customRecipe.value.job_level,
        can_be_hq: true,
    };
    const requirements: RecipeRequirements = {
        required_craftsmanship: 0,
        required_control: 0,
    };
    selectRecipe(
        customRecipe.value,
        undefined,
        0,
        requirements,
        undefined,
        itemInfo,
        '',
        simulatorMode,
    );
    router.push({ name: 'designer' });
}
</script>

<template>
    <el-scrollbar class="container">
        <el-form :inline="true" label-position="right" label-width="100px">
            <el-form-item :label="$t('class-job-level')">
                <el-input-number
                    v-model="customRecipe.job_level"
                    :min="1"
                ></el-input-number>
            </el-form-item>
            <el-form-item :label="$t('difficulty')">
                <el-input-number
                    v-model="customRecipe.difficulty"
                    :min="1"
                ></el-input-number>
            </el-form-item>
            <el-form-item :label="$t('quality')">
                <el-input-number
                    v-model="customRecipe.quality"
                    :min="1"
                ></el-input-number>
            </el-form-item>
            <el-form-item :label="$t('durability')">
                <el-input-number
                    v-model="customRecipe.durability"
                    :min="1"
                ></el-input-number>
            </el-form-item>
            <el-form-item :label="$t('conditions')">
                <el-checkbox-group v-model="conditionsFlag" size="small">
                    <el-checkbox-button
                        v-for="cond in Conditions"
                        :key="cond"
                        :value="cond as string"
                    >
                        {{ $t(cond.toLowerCase()) }}
                    </el-checkbox-button>
                </el-checkbox-group>
            </el-form-item>
        </el-form>
        <el-form :inline="true" label-position="right" label-width="100px">
            <el-form-item :label="$t('recipe-level')">
                <el-input-number v-model="rlv" :min="1"></el-input-number>
            </el-form-item>
            <el-form-item :label="$t('auto-load')">
                <el-switch v-model="autoLoad" :loading="autoLoadLoading" />
            </el-form-item>
        </el-form>
        <el-form :inline="true" label-position="right" label-width="100px">
            <el-form-item :label="$t('progress-divider')">
                <el-input-number
                    :disabled="autoLoad"
                    v-model="customRecipe.rlv.progress_divider"
                />
            </el-form-item>
            <el-form-item :label="$t('progress-modifier')">
                <el-input-number
                    :disabled="autoLoad"
                    v-model="customRecipe.rlv.progress_modifier"
                />
            </el-form-item>
            <el-form-item :label="$t('quality-divider')">
                <el-input-number
                    :disabled="autoLoad"
                    v-model="customRecipe.rlv.quality_divider"
                />
            </el-form-item>
            <el-form-item :label="$t('quality-modifier')">
                <el-input-number
                    :disabled="autoLoad"
                    v-model="customRecipe.rlv.quality_modifier"
                />
            </el-form-item>
        </el-form>
        <span>
            <el-button type="default" @click="router.back()">
                {{ $t('back') }}
            </el-button>
            <el-button type="primary" @click="confirm(false)">
                {{ $t('confirm') }}
            </el-button>
            <el-button
                type="primary"
                v-if="customRecipe.conditions_flag != 15"
                @click="confirm(true)"
            >
                {{ $t('simulator-mode') }}
            </el-button>
        </span>
    </el-scrollbar>
</template>

<style scoped>
.container {
    display: flex;
    flex-direction: column;
    align-items: center;
    background-color: transparent !important;
}
</style>

<fluent locale="zh-CN">
back = 返回
custom-recipe = 自定义配方
auto-load = 自动填充

cancel = 取消
confirm = 确认
simulator-mode = 模拟器模式
</fluent>

<fluent locale="en-US">
back = Back
custom-recipe = Custom Recipe
auto-load = Auto Fill

cancel = Cancel
confirm = Confirm
simulator-mode = Simulator Mode
</fluent>
