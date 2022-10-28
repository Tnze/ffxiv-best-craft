<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { getName, getVersion, getTauriVersion } from '@tauri-apps/api/app'
import { checkUpdate } from '@tauri-apps/api/updater'
import { useStore } from '../store'
import { useFluent } from 'fluent-vue'

const store = useStore()
const { $t } = useFluent()

const appName = ref('')
const version = ref('')
const tauriVersion = ref('')
getName().then(n => appName.value = n)
getVersion().then(v => version.value = v)
getTauriVersion().then(t => tauriVersion.value = t)

const onCheckUpdateClick = () => {
    checkUpdate()
        .then(v => {
            ElMessage({
                type: 'success',
                message: $t('check-update-success'),
            })
        })
        .catch(err => {
            ElMessage({
                type: 'error',
                message: err as string,
            })
        })
}

const languageChanged = (newLang: string) => {
    console.log("language switched to", newLang)
    store.commit('selectLanguage', newLang)
}

</script>


<template>
    <el-container>
        <el-header>
            <h1>{{ $t('settings') }}</h1>
        </el-header>
        <el-main>
            <el-form class="setting-page" :model="store.state.settings" label-width="120px">
                <el-form-item :label="$t('language')">
                    <el-select v-model="store.state.settings.language" @change="languageChanged">
                        <el-option label="简体中文" value="zh-CN" />
                        <el-option label="English" value="en" />
                    </el-select>
                </el-form-item>
                <el-form-item :label="$t('version-number')">
                    {{ version }}
                </el-form-item>
                <el-form-item :label="$t('tauri')">
                    {{ tauriVersion }}
                </el-form-item>
                <el-form-item>
                    <el-button type="primary" @click="onCheckUpdateClick">{{ $t('check-update') }}</el-button>
                </el-form-item>
                <el-form-item :label="$t('developer')">
                    Tnze❀潮风亭
                </el-form-item>
                <el-form-item :label="$t('feedback')">
                    <el-link href="https://pd.qq.com/s/cofwn9zhx">QQ频道</el-link>
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
language = 语言
version-number = 版本号
tauri = Tauri
developer = 开发者
feedback = 反馈

check-update = 检查更新
check-update-success = 检查更新成功
</fluent>

<fluent locale="en">
settings = Settings
language = Language
version-number = Version
tauri = Tauri
developer = Developer
feedback = Feedback

check-update = Check Update
check-update-success = Check update success
</fluent>