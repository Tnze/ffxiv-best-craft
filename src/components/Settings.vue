<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { getName, getVersion, getTauriVersion } from '@tauri-apps/api/app'
import { checkUpdate } from '@tauri-apps/api/updater'
import { useStore } from "../store"

const store = useStore()

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
                message: '检查更新成功',
            })
        })
        .catch(err => {
            ElMessage({
                type: 'error',
                message: err as string,
            })
        })
}

</script>


<template>
    <el-container>
        <el-header>
            <h1>设置</h1>
        </el-header>
        <el-main>
            <el-form class="setting-page" :model="store.state.settings" label-width="120px">
                <el-form-item label="Language">
                    <el-select v-model="store.state.settings.language">
                        <el-option label="简体中文" value="zh-CN" />
                    </el-select>
                </el-form-item>
                <el-form-item label="Version">
                    {{ version }}
                </el-form-item>
                <el-form-item label="Tauri Version">
                    {{ tauriVersion }}
                </el-form-item>
                <el-form-item>
                    <el-button type="primary" @click="onCheckUpdateClick">检查更新</el-button>
                </el-form-item>
                <el-form-item label="Auther">
                    Tnze❀潮风亭
                </el-form-item>
                <el-form-item label="Links">
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