<script setup lang="ts">
import { ElContainer, ElHeader, ElMain, ElTabs, ElTabPane, ElForm, ElFormItem, ElInputNumber, ElSwitch } from 'element-plus'
import { createDir, readTextFile, writeFile, Dir } from '@tauri-apps/api/fs'
import { ref, onMounted, onUpdated, computed } from 'vue'
import { onBeforeRouteLeave } from 'vue-router';
import { useGearsetsStore } from '../store'

const store = useGearsetsStore()

async function loadGearsets() {
    try {
        const conf = await readTextFile('gearsets.json', { dir: Dir.App })
        store.fromJson(conf)
    } catch (err) {
        // may be the file is not exist
        console.log(err)
    }
}

async function storeGearsets() {
    const conf = store.toJson
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
                    <el-form label-position="right" label-width="130px" :model="store.default"
                        style="max-width: 500px">
                        <el-form-item :label="$t('level')">
                            <el-input-number v-model="store.default.level" :min="1" :max="90"></el-input-number>
                        </el-form-item>
                        <el-form-item :label="$t('craftsmanship')">
                            <el-input-number v-model="store.default.craftsmanship" :min="0"></el-input-number>
                        </el-form-item>
                        <el-form-item :label="$t('control')">
                            <el-input-number v-model="store.default.control" :min="0"></el-input-number>
                        </el-form-item>
                        <el-form-item :label="$t('craft-point')">
                            <el-input-number v-model="store.default.craft_points" :min="0"></el-input-number>
                        </el-form-item>
                    </el-form>
                </el-tab-pane>
                <el-tab-pane v-for="v in store.special" :name="v.name" :label="$t(v.name)">
                    <el-form label-position="right" label-width="130px" :model="v" style="max-width: 500px">
                        <el-form-item :label="$t('attributes')">
                            <el-switch :model-value="v.value == null" :active-text="$t('inherit-from-default')"
                                @change="v.value = v.value == null ? { ...store.default } : null" />
                        </el-form-item>
                        <el-form-item :label="$t('level')">
                            <el-input-number v-model="(v.value || store.default).level" :disabled="v.value == null"
                                :min="0" :max="90"></el-input-number>
                        </el-form-item>
                        <el-form-item :label="$t('craftsmanship')">
                            <el-input-number v-model="(v.value || store.default).craftsmanship"
                                :disabled="v.value == null" :min="0"></el-input-number>
                        </el-form-item>
                        <el-form-item :label="$t('control')">
                            <el-input-number v-model="(v.value || store.default).control" :disabled="v.value == null"
                                :min="0"></el-input-number>
                        </el-form-item>
                        <el-form-item :label="$t('craft-point')">
                            <el-input-number v-model="(v.value || store.default).craft_points"
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

<fluent locale="en-US">
attributes = Crafter Attributes
default = Default
inherit-from-default = Inherit from default
</fluent>

<fluent locale="ja-JP">
attributes = 属性
default = デフォルト
inherit-from-default = デフォルトから継承
</fluent>