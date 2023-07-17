<script setup lang="ts">
import { ref, computed } from 'vue'
import { ElContainer, ElHeader, ElMain, ElForm, ElFormItem, ElSelect, ElOption, ElButton, ElLink, ElMessage } from 'element-plus'
import { useFluent } from 'fluent-vue'
import { getName, getVersion, getTauriVersion } from '@tauri-apps/api/app'
import { checkUpdate } from '../update'
import { useSettingsStore } from '../store'
import { languages } from '../lang'

const { $t } = useFluent()
const store = useSettingsStore()

const appName = ref('')
const version = ref('')
const tauriVersion = ref('')
getName().then(n => appName.value = n)
getVersion().then(v => version.value = v)
getTauriVersion().then(t => tauriVersion.value = t)

const checkingUpdate = ref(false)
const onCheckUpdateClick = async () => {
    checkingUpdate.value = true
    await checkUpdate($t)
    checkingUpdate.value = false
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
                <el-form-item :label="$t('data-source')">
                    <el-select v-model="store.dataSource">
                        <el-option :label="$t('ds-local')" value="local" />
                        <el-option :label="$t('ds-xivapi')" value="xivapi" />
                        <el-option :label="$t('ds-cafe')" value="cafe" />
                    </el-select>
                </el-form-item>
                <el-form-item v-if="store.dataSource == 'xivapi'">
                    <el-select v-model="store.dataSourceLang">
                        <el-option :label="$t('dslang-en')" value="en" />
                        <el-option :label="$t('dslang-ja')" value="ja" />
                        <el-option :label="$t('dslang-de')" value="de" />
                        <el-option :label="$t('dslang-fr')" value="fr" />
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
                    Tnze
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
data-source = 数据源
ds-local = 本地
# ds-xivapi =
# ds-cafe =
dslang-en = 英语
dslang-ja = 日语
dslang-de = 德语
dslang-fr = 法语
system-lang = 跟随系统
version-number = 版本号
tauri = Tauri
developer = ๐•ᴗ•๐
feedback = 反馈

check-update = 检查更新
checking-update = 正在检查更新
</fluent>

<fluent locale="en-US">
settings = Settings
language = Language
data-source = Data Source
ds-local = Local
ds-xivapi = Xivapi
ds-cafe = Cafe Maker
dslang-en = English
dslang-ja = Japanese
dslang-de = German
dslang-fr = French
system-lang = System
version-number = Version
tauri = Tauri
developer = Developer
feedback = Feedback

check-update = Check Update
checking-update = Checking Update
</fluent>

<fluent locale="ja-JP">
settings = 設定
# language =
data-source = データソース
ds-local = ローカル
# ds-xivapi =
# ds-cafe =
dslang-en = 英語
dslang-ja = 日本語
dslang-de = ドイツ語
dslang-fr = フランス語
version-number = バージョン
tauri = Tauri
developer = 開発者
feedback = フィードバック

check-update = 更新のチェック
checking-update = 更新をチェックしています
</fluent>