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
import {
    ElDialog,
    ElDescriptions,
    ElDescriptionsItem,
    ElButton,
    ElInputNumber,
    ElAlert,
    ElSwitch,
} from 'element-plus';
import {
    CollectablesShopRefine,
    Item,
    newRecipe,
    Recipe,
    RecipeInfo,
} from '@/libs/Craft';
import { selectRecipe } from './common';
import { useMediaQuery } from '@vueuse/core';
import { useRouter } from 'vue-router';
import { computed, onWatcherCleanup, ref, watch } from 'vue';
import useSettingStore from '@/stores/settings';
import { useFluent } from 'fluent-vue';

const props = defineProps<{
    recipeInfo: RecipeInfo;
    itemInfo: Item;
    collectability?: CollectablesShopRefine;
    isNormalRecipe: boolean;
}>();
const router = useRouter();
const { $t } = useFluent();
const settingsStore = useSettingStore();
const visible = defineModel<boolean>({ required: true });
const rawRecipe = defineModel<Recipe>('recipe', { required: true });
const compactLayout = useMediaQuery('screen and (max-width: 500px)');

const dynRecipeLevel = ref<number>();
const isDynRecipe = computed(() => {
    const notebook = props.recipeInfo.recipe_notebook_list;
    return notebook >= 1496 && notebook <= 1503;
});
const enableDynRecipe = ref(false);
const dynRecipeLoading = ref(false);
const dynRecipe = ref<Recipe>();
const recipe = computed(() => dynRecipe.value ?? rawRecipe.value);

async function loadDynRecipe(
    isDynRecipe: boolean,
    enableDynRecipe: boolean,
    dynRecipeLevel: number | undefined,
): Promise<Recipe | undefined> {
    if (!isDynRecipe || !enableDynRecipe || dynRecipeLevel == undefined) {
        return undefined;
    }
    const ds = await settingsStore.getDataSource();
    const recipeLevel = await ds.recipeLevelTablebyJobLevel!(dynRecipeLevel);
    if (recipeLevel == null) {
        return undefined;
    }

    return await newRecipe(
        recipeLevel,
        props.recipeInfo.difficulty_factor,
        props.recipeInfo.quality_factor,
        props.recipeInfo.durability_factor,
    );
}

watch(
    [isDynRecipe, enableDynRecipe, dynRecipeLevel],
    async ([isDynRecipe, enableDynRecipe, dynRecipeLevel]) => {
        try {
            dynRecipeLoading.value = true;

            let cancel = false;
            onWatcherCleanup(() => {
                cancel = true;
            });
            const recipe = await loadDynRecipe(
                isDynRecipe,
                enableDynRecipe,
                dynRecipeLevel,
            );
            if (!cancel) {
                dynRecipe.value = recipe;
            }
        } finally {
            dynRecipeLoading.value = false;
        }
    },
);

async function confirm(mode: 'simulator' | 'designer') {
    const itemInfo = { ...props.itemInfo };
    if (enableDynRecipe.value && dynRecipeLevel.value != undefined) {
        itemInfo.name = $t('sync-level-item-name', {
            itemName: itemInfo.name,
            syncLevel: dynRecipeLevel.value,
        });
    }
    selectRecipe(
        recipe.value,
        props.recipeInfo.id,
        props.recipeInfo.material_quality_factor,
        props.recipeInfo,
        props.collectability,
        itemInfo,
        props.recipeInfo.job,
        mode == 'simulator',
    );
    router.push({ name: 'designer' });
    visible.value = false;
}
</script>

<template>
    <el-dialog
        v-model="visible"
        :title="$t('please-confirm')"
        :align-center="true"
        :width="compactLayout ? '90%' : '50%'"
    >
        <template v-if="isDynRecipe">
            <el-alert
                :title="$t('alert-sync-level')"
                :type="dynRecipe != undefined ? 'success' : 'error'"
                :closable="false"
            />
            <br />
        </template>
        <el-descriptions loading="true">
            <el-descriptions-item
                v-if="isDynRecipe"
                :label="$t('sync-level')"
                :span="3"
            >
                <el-switch
                    v-model="enableDynRecipe"
                    :loading="dynRecipeLoading"
                />
                <el-input-number
                    style="margin-left: 14px"
                    v-model="dynRecipeLevel"
                    :min="1"
                    :disabled="!enableDynRecipe"
                />
            </el-descriptions-item>
            <el-descriptions-item :label="$t('name')" :span="3">
                {{ recipeInfo.item_name }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('recipe-level')">
                {{ recipe.rlv.id }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('type')">
                {{ recipeInfo.job }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('level')">
                {{ recipe.job_level }}
            </el-descriptions-item>

            <el-descriptions-item :label="$t('required-craftsmanship')">
                {{ recipeInfo.required_craftsmanship }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('required-control')">
                {{ recipeInfo.required_control }}
            </el-descriptions-item>
            <el-descriptions-item :label="$t('can-hq')">
                {{ $t(String(recipeInfo.can_hq)) }}
            </el-descriptions-item>
        </el-descriptions>
        <br />
        <span>
            {{
                isNormalRecipe
                    ? $t('confirm-select', {
                          itemName: recipeInfo.item_name || '',
                      })
                    : $t('confirm-select2')
            }}
        </span>
        <template #footer>
            <span>
                <el-button @click="visible = false">
                    {{ $t('cancel') }}
                </el-button>
                <el-button
                    v-if="!isNormalRecipe"
                    type="primary"
                    @click="confirm('simulator')"
                >
                    {{ $t('simulator-mode') }}
                </el-button>
                <el-button
                    type="primary"
                    :loading="dynRecipeLoading"
                    :disabled="enableDynRecipe && dynRecipe == undefined"
                    @click="confirm('designer')"
                >
                    {{ $t(isNormalRecipe ? 'confirm' : 'designer-mode') }}
                </el-button>
            </span>
        </template>
    </el-dialog>
</template>

<fluent locale="zh-CN">
confirm-select = 开始制作“{ $itemName }”吗？
confirm-select2 = 这是一个高难度配方，请选择模式。
alert-sync-level = 这是一个等级同步配方，请输入同步等级
please-confirm = 请确认

cancel = 取消
confirm = 确认
designer-mode = 普通模式
simulator-mode = 高难模式
sync-level-item-name = { $itemName }（等级同步：{ $syncLevel }）

sync-level = 等级同步
type = 类型
level = 等级
name = 名称
true = 是
false = 否
can-hq = 存在HQ
required-craftsmanship = 最低{ craftsmanship }
required-control = 最低{ control }
</fluent>

<fluent locale="en-US">
confirm-select = Start crafting "{ $itemName }"?
confirm-select2 = This is a 高难度配方. Please make a choice.
please-confirm = Please confirm

cancel = Cancel
confirm = Confirm
designer-mode = Normal Mode
simulator-mode = Simulator Mode
sync-level-item-name = { $itemName } (Lv. { $syncLevel })

type = Type
level = Level
name = Name
true = True
false = False
can-hq = Can be HQ
required-craftsmanship = Required { craftsmanship }
required-control = Required { control }
</fluent>
