<script setup lang="ts">
import { ElSpace, ElScrollbar, ElTable, ElTableColumn, ElInputNumber } from 'element-plus';
import { ref, watchEffect } from 'vue';
import { recipesIngredientions, ItemWithAmount } from '../../Craft';
import { useChecklistStore } from '../../store';
import ItemVue from './Item.vue';

const store = useChecklistStore();

const layers = ref<ItemWithAmount[]>([]);
watchEffect(async () => {
    layers.value = await recipesIngredientions(store.items)
})

const multipleTableRef = ref<InstanceType<typeof ElTable>>()
const multipleSelection = ref<ItemWithAmount[]>([])
const handleSelectionChange = (val: ItemWithAmount[]) => {
    multipleSelection.value = val
}
const amountChange = (idx: number, val: number) => {
    store.changeChecklist({ idx, amount: val })
}
</script>

<template>
    <suspense>
        <div class="scrollbar-flex-content">
            <el-space alignment="flex-start" :size="20">
                <el-scrollbar height="100%">
                    <el-table ref="multipleTableRef" :data="store.items" style="width: 500px; height: 100%;"
                        @selection-change="handleSelectionChange">
                        <el-table-column type="selection" width="55" />
                        <el-table-column :label="$t('name')">
                            <template #default="scope">
                                <item-vue :item_id="scope.row.ingredient_id" />
                            </template>
                        </el-table-column>
                        <el-table-column :label="$t('amount')">
                            <template #default="scope">
                                <el-input-number v-model="scope.row.amount"
                                    @change="(cur: number | undefined) => amountChange(scope.$index, cur || 0)" size="small" />
                            </template>
                        </el-table-column>
                    </el-table>
                </el-scrollbar>
                <el-scrollbar height="100%">
                    <el-table ref="multipleTableRef" :data="layers" height="100%" style="width: 500px; height: 100%;"
                        @selection-change="handleSelectionChange">
                        <el-table-column type="selection" width="55" />
                        <el-table-column :label="$t('name')">
                            <template #default="scope">
                                <item-vue :item_id="scope.row.ingredient_id" />
                            </template>
                        </el-table-column>
                        <el-table-column :label="$t('amount')">
                            <template #default="scope">
                                <el-input-number disabled v-model="scope.row.amount" size="small" />
                            </template>
                        </el-table-column>
                    </el-table>
                </el-scrollbar>
            </el-space>
        </div>
    </suspense>
</template>

<style scoped>
.scrollbar-flex-content {
    display: flex;
}
</style>

<fluent locale="zh-CN">
bom = 材料表
</fluent>

<fluent locale="en-US">
bom = Bill of Material
</fluent>

<fluent locale="ja-JP">
# bom = 
</fluent>