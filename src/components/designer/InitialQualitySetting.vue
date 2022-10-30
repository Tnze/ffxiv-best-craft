<script setup lang="ts">
import { ArrowUp } from '@element-plus/icons-vue';
import { computed, reactive, ref, watch, watchEffect } from 'vue';
import { Item, itemInfo, ItemWithAmount, Recipe, recipesIngredientions } from '../../Craft';

const props = defineProps<{
    open: boolean
    item: Item
    recipe: Recipe
    modelValue: number
}>()

const emits = defineEmits<{
    (e: 'close'): void
    (e: 'update:modelValue', value: number): void
}>()

const initQuality = computed({
    get() { return props.modelValue },
    set(v: number) { emits('update:modelValue', v) }
})

const calcItems = (ri: ItemWithAmount[]) => Promise.all(ri.map(
    async (item) => {
        return {
            item: await itemInfo(item.ingredient_id),
            amount: item.amount,
            hqAmount: 0,
        }
    }
))
const items = ref<{ item: Item, amount: number, hqAmount: number }[]>([])
watch(() => props.item.id, async (newId) => {
    const ri = props.item.id == -1 ? null : await recipesIngredientions([{ ingredient_id: newId, amount: 1 }])
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
    console.log(hqLvCount, totalLvCount)
    const r = totalLvCount == 0 ? 0 : hqLvCount / totalLvCount
    initQuality.value = Math.floor(props.recipe.quality / 2 * r)
})

</script>

<template>
    <el-dialog v-model="open" @close="$emit('close')">
        <div style="display: flex; flex-direction: column;">
            <el-input-number :disabled="props.item.id != -1" v-model="initQuality" :min="0" :max="recipe.quality"
                class="initial-quality-input" />
            <el-table v-if="props.item.id != -1" :data="items">
                <el-table-column :label="$t('name')" prop="item.name" />
                <el-table-column :label="$t('amount')">
                    <template #default="scope">
                        <el-button-group v-if="scope.row.item.can_be_hq" class="ml-4">
                            <el-button :icon="ArrowUp" size="small" :disabled="scope.row.hqAmount <= 0"
                                @click="scope.row.hqAmount -= 1">
                                NQ {{ scope.row.amount - scope.row.hqAmount }}
                            </el-button>
                            <el-button size="small" :disabled="scope.row.hqAmount >= scope.row.amount"
                                @click="scope.row.hqAmount += 1">
                                HQ {{ scope.row.hqAmount }}
                                <el-icon class="el-icon--right">
                                    <ArrowUp />
                                </el-icon>
                            </el-button>
                        </el-button-group>
                    </template>
                </el-table-column>
            </el-table>
        </div>
    </el-dialog>
</template>

<style scoped>
.initial-quality-input {
    align-self: center;
    margin-bottom: 15px;
}
</style>

<fluent locale="zh-CN">
initial-quality = 初期品质
</fluent>

<fluent locale="en">
initial-quality = Initial Quality
</fluent>