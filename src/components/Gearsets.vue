<script setup lang="ts">
import { createDir, readTextFile, writeFile, Dir } from '@tauri-apps/api/fs'
import { ref, onMounted, onUpdated, computed } from 'vue'
import { onBeforeRouteLeave } from 'vue-router';
import { useStore } from '../store'

const store = useStore()

const modelValue = computed({
    get() {
        return store.state.gearsets;
    },
    set(newValue) {
        store.commit('storeGearsets', newValue)
    }
})

async function loadGearsets() {
    try {
        const conf = await readTextFile('gearsets.json', { dir: Dir.App })
        modelValue.value = JSON.parse(conf)
    } catch (err) {
        // may be the file is not exist
        console.log(err)
    }
}

async function storeGearsets() {
    const conf = JSON.stringify(modelValue.value)
    try {
        await writeFile({ contents: conf, path: 'gearsets.json' }, { dir: Dir.App })
    } catch (err) {
        try {
            await createDir('', { dir: Dir.App })
            await writeFile({ contents: conf, path: 'gearsets.json' }, { dir: Dir.App })
        } catch (err) {
            console.error(err)
        }
    }
}

onMounted(loadGearsets)
onUpdated(storeGearsets)
onBeforeRouteLeave((_to, _from) => storeGearsets())

const jobPage = ref('default')

</script>

<template>
    <el-container>
        <el-header>
            <h1>{{ $t('attributes') }}</h1>
        </el-header>
        <el-main>
            <el-tabs v-model="jobPage" tab-position="left">
                <el-tab-pane name="default" :label="$t('default')">
                    <el-form label-position="right" label-width="130px" :model="modelValue.default"
                        style="max-width: 500px">
                        <el-form-item :label="$t('level')">
                            <el-input-number v-model="modelValue.default.level" :min="1" :max="90"></el-input-number>
                        </el-form-item>
                        <el-form-item :label="$t('craftsmanship')">
                            <el-input-number v-model="modelValue.default.craftsmanship" :min="0"></el-input-number>
                        </el-form-item>
                        <el-form-item :label="$t('control')">
                            <el-input-number v-model="modelValue.default.control" :min="0"></el-input-number>
                        </el-form-item>
                        <el-form-item :label="$t('craft-point')">
                            <el-input-number v-model="modelValue.default.craft_points" :min="0"></el-input-number>
                        </el-form-item>
                    </el-form>
                </el-tab-pane>
                <el-tab-pane v-for="v in modelValue.special" :name="v.name" :label="$t(v.name)">
                    <el-form label-position="right" label-width="130px" :model="v.value" style="max-width: 500px">
                        <el-form-item :label="$t('attributes')">
                            <el-switch :model-value="v.value == null" :active-text="$t('inherit-from-default')"
                                @change="v.value = v.value == null ? { ...modelValue.default } : null" />
                        </el-form-item>
                        <el-form-item :label="$t('level')">
                            <el-input-number v-model="(v.value || modelValue.default).level" :disabled="v.value == null"
                                :min="0" :max="90"></el-input-number>
                        </el-form-item>
                        <el-form-item :label="$t('craftsmanship')">
                            <el-input-number v-model="(v.value || modelValue.default).craftsmanship"
                                :disabled="v.value == null" :min="0"></el-input-number>
                        </el-form-item>
                        <el-form-item :label="$t('control')">
                            <el-input-number v-model="(v.value || modelValue.default).control"
                                :disabled="v.value == null" :min="0"></el-input-number>
                        </el-form-item>
                        <el-form-item :label="$t('craft-point')">
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

<fluent locale="zh-CN">
attributes = 装备属性
default = 默认
inherit-from-default = 继承自默认
</fluent>

<fluent locale="en">
attributes = Crafter Attributes
default = Default
inherit-from-default = Inherit from default
</fluent>

<fluent locale="en">
attributes = 属性
default = デフォルト
inherit-from-default = デフォルトから継承
</fluent>