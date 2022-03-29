<script setup lang="ts">
import { ref, computed, reactive, watch } from 'vue'
import { Attributes } from '../Craft'

const props = defineProps<{
    modelValue: Attributes
}>()

// const emits = defineEmits<{
//     (event: 'update:modelValue', attr: Attributes): void
// }>()

</script>

<template>
    <el-container>
        <el-header>
            <h1>装备属性</h1>
        </el-header>
        <el-main>
            <el-form
                label-position="right"
                label-width="100px"
                :model="modelValue"
                style="max-width: 460px"
            >
                <el-form-item label="等级">
                    <el-input-number v-model="modelValue.level" :min="0" :max="90"></el-input-number>
                </el-form-item>
                <el-form-item label="作业精度">
                    <el-input-number v-model="modelValue.craftsmanship" :min="0"></el-input-number>
                </el-form-item>
                <el-form-item label="加工精度">
                    <el-input-number v-model="modelValue.control" :min="0"></el-input-number>
                </el-form-item>
                <el-form-item label="制作力">
                    <el-input-number v-model="modelValue.craft_points" :min="0"></el-input-number>
                </el-form-item>
            </el-form>
        </el-main>
    </el-container>
</template>

<style scoped>
</style>
