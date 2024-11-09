<!-- 
    This file is part of BestCraft.
    Copyright (C) 2024  Tnze

    BestCraft is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    BestCraft is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
-->

<script setup lang="ts">
import { ref, onActivated } from 'vue';
import {
    ElScrollbar,
    ElForm,
    ElFormItem,
    ElSelect,
    ElOption,
    ElButton,
    ElLink,
    ElRadioGroup,
    ElRadioButton,
    ElDialog,
    ElText,
} from 'element-plus';
import { useFluent } from 'fluent-vue';
import useSettingsStore from '@/stores/settings';
import { languages } from '../lang';
import { useColorMode } from '@vueuse/core';
import { isTauri, isWebsite, isYYYYGames } from '@/libs/Consts';
import SupportUs from './SupportUs.vue';

const emit = defineEmits<{
    (e: 'setTitle', title: string): void;
}>();
onActivated(() => emit('setTitle', 'settings'));

const { $t } = useFluent();
const store = useSettingsStore();
const colorMode = useColorMode().store;

const appName = ref('BestCraft');
const version = ref('');
const tauriVersion = ref('');
var checkingUpdate = ref(false);
var onCheckUpdateClick = async () => {};
const licenseDialogVisible = ref(false);
const switchLinesDialogVisible = ref(false);

if (isTauri) {
    import('@tauri-apps/api/app').then(
        ({ getName, getVersion, getTauriVersion }) => {
            getName().then(n => (appName.value = n));
            getVersion().then(v => (version.value = v));
            getTauriVersion().then(t => (tauriVersion.value = t));
        },
    );
    onCheckUpdateClick = async () => {
        let { checkUpdate } = await import('../update');
        checkingUpdate.value = true;
        await checkUpdate($t, false);
        checkingUpdate.value = false;
    };
}
</script>

<template>
    <el-scrollbar>
        <el-form class="setting-page" :model="store" label-width="120px">
            <el-form-item :label="$t('language')">
                <el-select v-model="store.language">
                    <el-option :label="$t('system-lang')" value="system" />
                    <el-option
                        v-for="[v, name] in languages"
                        :label="name"
                        :value="v"
                    />
                </el-select>
            </el-form-item>
            <el-form-item :label="$t('theme')">
                <el-radio-group v-model="colorMode">
                    <el-radio-button value="light">{{
                        $t('light')
                    }}</el-radio-button>
                    <el-radio-button value="dark">{{
                        $t('dark')
                    }}</el-radio-button>
                    <el-radio-button value="auto">{{
                        $t('auto')
                    }}</el-radio-button>
                </el-radio-group>
            </el-form-item>
            <el-form-item :label="$t('data-source')">
                <el-select v-model="store.dataSource">
                    <el-option
                        v-if="isTauri"
                        :label="$t('ds-local')"
                        value="local"
                    />
                    <el-option :label="$t('ds-yyyygames')" value="yyyy.games">
                        <span style="float: left">{{
                            $t('ds-yyyygames')
                        }}</span>
                        <span class="data-source-option-note">{{
                            $t('ds-yyyygames-desc')
                        }}</span>
                    </el-option>
                    <el-option :label="$t('ds-beta-xivapi')" value="xivapi">
                        <span style="float: left">{{
                            $t('ds-beta-xivapi')
                        }}</span>
                        <span class="data-source-option-note"
                            >{{ $t('ds-beta-xivapi-desc') }}
                        </span>
                    </el-option>
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
            <el-form-item v-if="isWebsite" :label="$t('switch-lines')">
                <el-button @click="switchLinesDialogVisible = true">{{
                    $t('detail')
                }}</el-button>
                <el-dialog
                    v-model="switchLinesDialogVisible"
                    :title="$t('switch-lines')"
                >
                    <p>
                        BestCraft
                        是开源软件，可以在多个不同的服务器上部署，以下是目前已知的部署了本软件的网站：
                    </p>
                    <p>
                        <el-link href="https://tnze.yyyy.games/" type="primary"
                            >YYYY.GAMES</el-link
                        >
                        <el-text size="small" type="info"
                            >由 <span>瑤瑤瑤影@神意之地</span> 运营</el-text
                        >
                    </p>
                    <p>
                        <el-link
                            href="https://bestcraft.nbb.fan/"
                            type="primary"
                            >NBB.FAN</el-link
                        >
                        <el-text size="small" type="info"
                            >由 <span>N.B.B</span> 运营</el-text
                        >
                    </p>
                    <p>
                        <el-link
                            href="https://ffxiv-best-craft.pages.dev/"
                            type="primary"
                            >Cloudflare Pages</el-link
                        >
                        <el-text size="small" type="info"
                            >由 <span>Tnze</span> 随意地设置在 Cloudflare
                            上，不适合国内访问</el-text
                        >
                    </p>
                </el-dialog>
            </el-form-item>
            <template v-if="isTauri">
                <el-form-item :label="$t('version-number')">
                    {{ version }}
                </el-form-item>
                <el-form-item :label="$t('tauri')">
                    {{ tauriVersion }}
                </el-form-item>
                <el-form-item>
                    <el-button
                        type="primary"
                        @click="onCheckUpdateClick"
                        :loading="checkingUpdate"
                    >
                        {{
                            checkingUpdate
                                ? $t('checking-update')
                                : $t('check-update')
                        }}
                    </el-button>
                </el-form-item>
            </template>
            <el-form-item :label="$t('developer')"> Tnze </el-form-item>
            <el-form-item :label="$t('feedback')">
                <el-link href="https://pd.qq.com/s/al6b5xo69" target="_blank"
                    >QQ频道</el-link
                >
                <el-link href="https://qm.qq.com/q/YMujBifn6G" target="_blank"
                    >QQ群聊</el-link
                >
            </el-form-item>
            <el-form-item :label="$t('source')">
                <el-link
                    href="https://gitee.com/Tnze/ffxiv-best-craft"
                    target="_blank"
                    >Gitee</el-link
                >
                <el-link
                    href="https://github.com/Tnze/ffxiv-best-craft"
                    target="_blank"
                    >Github</el-link
                >
            </el-form-item>
            <el-form-item :label="$t('license')">
                <el-button @click="licenseDialogVisible = true">AGPL</el-button>
                <el-dialog
                    class="licenses-dialog"
                    v-model="licenseDialogVisible"
                    :title="$t('license')"
                    width="50%"
                >
                    <p>{{ $t('licenses-notices-1') }}</p>
                    <p>{{ $t('licenses-notices-2') }}</p>
                    <p>{{ $t('licenses-notices-3') }}</p>
                </el-dialog>
            </el-form-item>
            <el-form-item v-if="isYYYYGames" :label="$t('donate')">
                <SupportUs />
            </el-form-item>
        </el-form>
    </el-scrollbar>
</template>

<style scoped>
.setting-page {
    margin-top: 20px;
    background-color: transparent !important;
}

.el-link {
    margin-right: 8px;
}

.el-select {
    width: 210px;
}

.data-source-option-note {
    float: right;
    color: var(--el-text-color-secondary);
    font-size: 13px;
}
</style>

<fluent locale="zh-CN">
# language =
theme = 主题
light = 亮
dark = 暗
auto = 自动
data-source = 数据源
ds-local = 本地
# ds-xivapi =
ds-yyyygames-desc = 国服数据
# ds-cafe =
ds-beta-xivapi-desc = 国际服数据
switch-lines = 切换线路
dslang-en = 英语
dslang-ja = 日语
dslang-de = 德语
dslang-fr = 法语
system-lang = 跟随系统
version-number = 版本号
tauri = Tauri
developer = 作者
feedback = 反馈 / 聊天
license = 许可
source = 源代码
donate = 捐赠
detail = 详情

check-update = 检查更新
checking-update = 正在检查更新
</fluent>

<fluent locale="en-US">
language = Language
theme = Theme
light = Light
dark = Dark
auto = Auto
data-source = Data Source
ds-local = Local
ds-yyyygames = YYYY.GAMES
ds-yyyygames-desc = Chinese
ds-xivapi = Xivapi
ds-cafe = Cafe Maker
ds-beta-xivapi = Xivapi (Beta)
ds-beta-xivapi-desc = Latest
switch-lines = Switch Lines
dslang-en = English
dslang-ja = Japanese
dslang-de = German
dslang-fr = French
system-lang = System
version-number = Version
tauri = Tauri
developer = Author
feedback = Feedback
license = License
source = Source
donate = Donate
detail = Detail

check-update = Check Update
checking-update = Checking Update
</fluent>

<fluent locale="ja-JP">
# language =
data-source = データソース
ds-local = ローカル
# ds-xivapi =
# ds-cafe =
switch-lines = サーバの切り替え
dslang-en = 英語
dslang-ja = 日本語
dslang-de = ドイツ語
dslang-fr = フランス語
version-number = バージョン
tauri = Tauri
developer = 作者
feedback = フィードバック
license = ライセンス
source = ソースコード
donate = 寄付する
detail = 詳細

check-update = 更新のチェック
checking-update = 更新をチェックしています
</fluent>
