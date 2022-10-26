<script setup lang="ts">
import { ref, watch } from 'vue';
import { recipesIngredientions, ItemWithAmount, Item } from '../Craft';
import { useStore } from '../store';

const store = useStore();
const checklist = ref<{ item: Item, amount: number }[]>([]);
watch(store.state.checklist, async (newValue, _oldValue) => {
    checklist.value = newValue.map(v => {
        return {
            item: new Item(v.ingredient_id),
            amount: v.amount
        }
    })
})
const layers = ref<{ item: Item, amount: number }[]>([]);
watch(store.state.checklist, async (newValue, _oldValue) => {
    layers.value = (await recipesIngredientions(newValue)).map(v => {
        return {
            item: new Item(v.ingredient_id),
            amount: v.amount
        }
    })
})
</script>

<template>
    <el-scrollbar>
        <div class="scrollbar-flex-content">
            <el-card class="box-card">
                <el-scrollbar>
                    <p v-for="item in checklist" :key="item.item.id" class="scrollbar-demo-item">
                        {{ item.item.name }} × {{ item.amount }}
                    </p>
                </el-scrollbar>
            </el-card>

            <el-card class="box-card">
                <el-scrollbar>
                    <p v-for="item in layers" :key="item.item.id" class="scrollbar-demo-item">
                        {{ item.item.name }} × {{ item.amount }}
                    </p>
                </el-scrollbar>
            </el-card>
        </div>
    </el-scrollbar>
</template>

<style scoped>
.scrollbar-flex-content {
    display: flex;
}


.box-card {
    width: 480px;
}
</style>
