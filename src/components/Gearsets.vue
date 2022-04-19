<script setup lang="ts">
import { createDir, readTextFile, writeFile, Dir } from '@tauri-apps/api/fs'
import { ref, onMounted, onUpdated } from 'vue'
import { Attributes } from '../Craft'

interface GearsetsRow {
    name: string
    value: Attributes | null
}

const jobLabels = new Map([
    ['carpenter', "刻木匠"],
    ['blacksmith', "锻铁匠"],
    ['armorer', "铸甲匠"],
    ['goldsmith', "雕金匠"],
    ['leatherworker', "制革匠"],
    ['weaver', "裁衣匠"],
    ['alchemist', "炼金术士"],
    ['culinarian', "烹调师"],
])

const props = defineProps<{
    modelValue: { default: Attributes, special: GearsetsRow[] }
}>()

const emits = defineEmits<{
    (event: 'update:modelValue', attr: Attributes): void
}>()

onMounted(async () => {
    try {
        const conf = await readTextFile('gearsets.json', { dir: Dir.App })
        emits('update:modelValue', JSON.parse(conf) as Attributes)
    } catch (err) {
        // may be the file is not exist
        console.log(err)
    }
})

onUpdated(async () => {
    const conf = JSON.stringify(props.modelValue)
    try {
        await writeFile({ contents: conf, path: 'gearsets.json' }, { dir: Dir.App })
    } catch (err) {
        try {
            await createDir('', { dir: Dir.App })
            await writeFile({ contents: conf, path: 'gearsets.json' }, { dir: Dir.App })
        } catch (err) {
            console.log(err)
        }
    }
})

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
                    <el-form label-position="right" label-width="100px" :model="modelValue.default"
                        style="max-width: 460px">
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
                <el-tab-pane v-for="v in modelValue.special" :name="v.name" :label="jobLabels.get(v.name)">
                    <el-form label-position="right" label-width="100px" :model="v.value" style="max-width: 460px">
                        <el-form-item label="属性">
                            <el-switch :model-value="v.value == null" active-text="继承自默认"
                                @change="v.value = v.value == null ? { ...modelValue.default } : null" />
                        </el-form-item>
                        <el-form-item label="等级">
                            <el-input-number v-model="(v.value || modelValue.default).level" :disabled="v.value == null"
                                :min="0" :max="90"></el-input-number>
                        </el-form-item>
                        <el-form-item label="作业精度">
                            <el-input-number v-model="(v.value || modelValue.default).craftsmanship"
                                :disabled="v.value == null" :min="0"></el-input-number>
                        </el-form-item>
                        <el-form-item label="加工精度">
                            <el-input-number v-model="(v.value || modelValue.default).control"
                                :disabled="v.value == null" :min="0"></el-input-number>
                        </el-form-item>
                        <el-form-item label="制作力">
                            <el-input-number v-model="(v.value || modelValue.default).craft_points"
                                :disabled="v.value == null" :min="0"></el-input-number>
                        </el-form-item>
                    </el-form>
                </el-tab-pane>
            </el-tabs>
        </el-main>
    </el-container>
</template>

<style scoped>
.el-tabs {
    user-select: none;
}
</style>
