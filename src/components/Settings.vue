<script setup lang="ts">
import { ref, computed } from 'vue'
import { ElContainer, ElHeader, ElMain, ElForm, ElFormItem, ElSelect, ElOption, ElButton, ElLink, ElMessage } from 'element-plus'
import { useFluent } from 'fluent-vue'
import { getName, getVersion, getTauriVersion } from '@tauri-apps/api/app'
import { checkUpdate } from '@tauri-apps/api/updater'
import { useSettingsStore } from '../store'
import { languages } from '../lang'

const store = useSettingsStore()
const { $t } = useFluent()

const appName = ref('')
const version = ref('')
const tauriVersion = ref('')
getName().then(n => appName.value = n)
getVersion().then(v => version.value = v)
getTauriVersion().then(t => tauriVersion.value = t)

const checkingUpdate = ref(false)
const onCheckUpdateClick = async () => {
    try {
        checkingUpdate.value = true
        await checkUpdate()
        ElMessage({
            type: 'success',
            message: $t('check-update-success'),
        })
    } catch (err) {
        ElMessage({
            type: 'error',
            message: err as string,
        })
    } finally {
        checkingUpdate.value = false
    }
}

</script>

<template>
    <el-container>
        <el-header>
            <h1>{{ $t('settings') }}</h1>
        </el-header>
        <el-main>
            <el-form class="setting-page" :model="store" label-width="120px">
                <el-form-item :label="$t('language')">
                    <el-select v-model="store.language">
                        <el-option :label="$t('system-lang')" value="system" />
                        <el-option v-for="[v, name] in languages" :label="name" :value="v" />
                    </el-select>
                </el-form-item>
                <el-form-item :label="$t('version-number')">
                    {{ version }}
                </el-form-item>
                <el-form-item :label="$t('tauri')">
                    {{ tauriVersion }}
                </el-form-item>
                <el-form-item>
                    <el-button type="primary" @click="onCheckUpdateClick" :loading="checkingUpdate">{{
                        checkingUpdate ? $t('checking-update') : $t('check-update')
                    }}</el-button>
                </el-form-item>
                <el-form-item :label="$t('developer')">
                    Tnze❀潮风亭
                </el-form-item>
                <el-form-item :label="$t('feedback')">
                    <el-link href="https://pd.qq.com/s/cofwn9zhx" target="_blank">QQ频道</el-link>
                    <el-link href="https://gitee.com/Tnze/ffxiv-best-craft" target="_blank">Gitee</el-link>
                    <el-link href="https://github.com/Tnze/ffxiv-best-craft" target="_blank">Github</el-link>
                </el-form-item>
            </el-form>
        </el-main>
    </el-container>
</template>

<style scoped>
.setting-page {
    padding-top: 10px;
}

.el-link {
    margin-right: 8px;
}
</style>

<fluent locale="zh-CN">
settings = 设置
# language =
system-lang = 跟随系统
version-number = 版本号
tauri = Tauri
developer = 开发者
feedback = 反馈

check-update = 检查更新
checking-update = 正在检查更新
check-update-success = 检查更新成功
</fluent>

<fluent locale="en-US">
settings = Settings
language = Language
system-lang = System
version-number = Version
tauri = Tauri
developer = Developer
feedback = Feedback

check-update = Check Update
checking-update = Checking Update
check-update-success = Check update success
</fluent>

<fluent locale="ja-JP">
settings = 設定
# language =
# system-lang = 
version-number = バージョン
tauri = Tauri
developer = 開発者
feedback = フィードバック

check-update = 更新のチェック
checking-update = 更新をチェックしています
check-update-success = 更新のチェックに成功しました
</fluent>