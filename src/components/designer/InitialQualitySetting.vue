<!-- 
    This file is part of BestCraft.
    Copyright (C) 2024  Tnze

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
import { ElIcon, ElForm, ElFormItem, ElInputNumber, ElButton, ElButtonGroup, ElRadioGroup, ElRadioButton } from 'element-plus';
import { ArrowUp } from '@element-plus/icons-vue';
import { computed, reactive, ref, watchEffect } from 'vue';
import { Item, ItemWithAmount, Recipe } from '@/libs/Craft';
import useSettingsStore from '@/stores/settings';

const settingStore = useSettingsStore()

const props = defineProps<{
    item: Item
    recipe: Recipe
    recipeId?: number
    materialQualityFactor: number
    modelValue: number
}>()

const emits = defineEmits<{
    (e: 'update:modelValue', value: number): void
}>()

const initQuality = computed({
    get() { return props.modelValue },
    set(v: number) { emits('update:modelValue', v ?? 0) }
})

const calcItems = (ri: ItemWithAmount[]) => Promise.all(ri.map(
    async (item) => ({
        item: await (await settingStore.getDataSource).itemInfo(item.ingredient_id),
        amount: item.amount,
        hqAmount: 0,
    })
))
const inputType = ref<'manully' | 'ingredient'>('ingredient')
const manullyInput = computed(() => props.recipeId !== undefined && inputType.value != 'manully')
const items = ref<{ item: Item, amount: number, hqAmount: number }[]>([])

watchEffect(() => {
    if (props.recipeId === undefined) {
        inputType.value = 'manully';
    }
})

watchEffect(async () => {
    const recipeId = props.recipeId;
    const ri = recipeId === undefined ? null : await (await settingStore.getDataSource).recipesIngredients(recipeId)
    items.value = ri == null ? [] : reactive(await calcItems(ri))
})

watchEffect(() => {
    if (items.value == null) return
    const [totalLvCount, hqLvCount] = items.value.
        filter(item => item.item.can_be_hq).
        map(v => [v.amount * v.item.level, v.hqAmount * v.item.level]).
        concat([[0, 0]]).
        reduce(([totalPrev, prevHq], [total, hq]) =>
            [totalPrev + total, prevHq + hq]
        )
    const r = totalLvCount == 0 ? 0 : hqLvCount / totalLvCount
    const maxInitQualityPs = props.materialQualityFactor / 100
    initQuality.value = Math.floor(props.recipe.quality * maxInitQualityPs * r)
})

</script>

<template>
    <div style="display: flex; flex-direction: column;">
        <el-form label-width="auto" @submit.prevent>
            <el-form-item label=" ">
                <el-radio-group v-model="inputType">
                    <el-radio-button value="manully">{{ $t('manully-input') }}</el-radio-button>
                    <el-radio-button :disabled="items.length == 0" value="ingredient">
                        {{ $t('select-hq-ingredients') }}
                    </el-radio-button>
                </el-radio-group>
            </el-form-item>
            <el-form-item :label="$t('initial-quality')">
                <el-input-number v-model="initQuality" :readonly="manullyInput" :controls="!manullyInput" :min="0"
                    :max="recipe.quality" value-on-clear="min" :step-strictly="true" />
            </el-form-item>
            <template v-if="inputType == 'ingredient'">
                <el-form-item v-for="row in items" :label="row.item.name">
                    <el-button-group v-if="row.item.can_be_hq" class="ml-4">
                        <el-button :icon="ArrowUp" size="small" :disabled="row.hqAmount <= 0"
                            @click="row.hqAmount -= 1">
                            {{ $t('nq') }} {{ row.amount - row.hqAmount }}
                        </el-button>
                        <el-button size="small" :disabled="row.hqAmount >= row.amount" @click="row.hqAmount += 1">
                            {{ $t('hq') }} {{ row.hqAmount }}
                            <el-icon class="el-icon--right">
                                <ArrowUp />
                            </el-icon>
                        </el-button>
                    </el-button-group>
                    <template v-else>
                        <el-button :icon="ArrowUp" size="small" disabled>
                            {{ $t('nq') }} {{ row.amount }}
                        </el-button>
                    </template>
                </el-form-item>
            </template>
        </el-form>
    </div>
</template>

<fluent locale="zh-CN">
nq = 普通
hq = 优质

please-input-init-quality = 请输入初期品质
config-init-quality = 设置初期品质
please-input-integers = 请输入整数

select-hq-ingredients = 选择HQ半成品计算
manully-input = 手动输入
</fluent>

<fluent locale="en-US">
nq = NQ
hq = HQ

select-hq-ingredients = Calculate from HQ ingredients
manully-input = Manully input
</fluent>

<fluent locale="ja-JP">
nq = NQ
hq = HQ

please-input-init-quality = Please input initial quality
config-init-quality = Set initial quality
please-input-integers = Please input a integer

select-hq-ingredients = HQ成分による計算
manully-input = 手動入力
</fluent>
