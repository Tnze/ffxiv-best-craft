<script setup lang="ts">
import { ElTable } from 'element-plus';
import { ref, watchEffect } from 'vue';
import { recipesIngredientions, ItemWithAmount } from '../../Craft';
import { useStore } from '../../store';
import ItemVue from './Item.vue';

const store = useStore();

const layers = ref<ItemWithAmount[]>([]);
watchEffect(async () => {
    layers.value = await recipesIngredientions(store.state.checklist)
})

const multipleTableRef = ref<InstanceType<typeof ElTable>>()
const multipleSelection = ref<ItemWithAmount[]>([])
const handleSelectionChange = (val: ItemWithAmount[]) => {
    multipleSelection.value = val
}
const amountChange = (idx: number, val: number) => {
    store.commit("changeChecklist", { idx, amount: val })
}
</script>

<template>
    <suspense>
        <div class="scrollbar-flex-content">
            <el-space alignment="flex-start" :size="20">
                <el-scrollbar height="100%">
                    <el-table ref="multipleTableRef" :data="store.state.checklist" style="width: 500px; height: 100%;"
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
                                    @change="(val: number) => amountChange(scope.$index, val)" size="small" />
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
name = 名称
amount = 数量
</fluent>