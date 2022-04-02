<script setup lang="ts">
import { ref, computed, reactive, watch, watchEffect } from 'vue'
import { Attributes, Jobs } from '../Craft'

interface GearsetsRow {
    name: string
    label: string
    value: Attributes | null
}

const props = defineProps<{
    modelValue: { default: Attributes, special: GearsetsRow[] }
}>()

const emits = defineEmits<{
    (event: 'update:modelValue', attr: Attributes): void
}>()


const jobPage = ref('default')

</script>

<template>
    <el-container>
        <el-header>
            <h1>装备属性</h1>
        </el-header>
        <el-main>
            <el-tabs v-model="jobPage" tab-position="left">
                <el-tab-pane name="default" label="默认">
                    <el-form
                        label-position="right"
                        label-width="100px"
                        :model="modelValue.default"
                        style="max-width: 460px"
                    >
                        <el-form-item label="等级">
                            <el-input-number v-model="modelValue.default.level" :min="1" :max="90"></el-input-number>
                        </el-form-item>
                        <el-form-item label="作业精度">
                            <el-input-number v-model="modelValue.default.craftsmanship" :min="0"></el-input-number>
                        </el-form-item>
                        <el-form-item label="加工精度">
                            <el-input-number v-model="modelValue.default.control" :min="0"></el-input-number>
                        </el-form-item>
                        <el-form-item label="制作力">
                            <el-input-number v-model="modelValue.default.craft_points" :min="0"></el-input-number>
                        </el-form-item>
                    </el-form>
                </el-tab-pane>
                <el-tab-pane v-for="v in modelValue.special" :name="v.name" :label="v.label">
                    <el-form
                        label-position="right"
                        label-width="100px"
                        :model="v.value"
                        style="max-width: 460px"
                    >
                        <el-form-item label="属性">
                            <el-switch
                                :model-value="v.value == null"
                                active-text="继承自默认"
                                @change="v.value = v.value == null ? { ...modelValue.default } : null"
                            />
                        </el-form-item>
                        <el-form-item label="等级">
                            <el-input-number
                                v-model="(v.value || modelValue.default).level"
                                :disabled="v.value == null"
                                :min="0"
                                :max="90"
                            ></el-input-number>
                        </el-form-item>
                        <el-form-item label="作业精度">
                            <el-input-number
                                v-model="(v.value || modelValue.default).craftsmanship"
                                :disabled="v.value == null"
                                :min="0"
                            ></el-input-number>
                        </el-form-item>
                        <el-form-item label="加工精度">
                            <el-input-number
                                v-model="(v.value || modelValue.default).control"
                                :disabled="v.value == null"
                                :min="0"
                            ></el-input-number>
                        </el-form-item>
                        <el-form-item label="制作力">
                            <el-input-number
                                v-model="(v.value || modelValue.default).craft_points"
                                :disabled="v.value == null"
                                :min="0"
                            ></el-input-number>
                        </el-form-item>
                    </el-form>
                </el-tab-pane>
            </el-tabs>
        </el-main>
    </el-container>
</template>

<style scoped>
</style>
