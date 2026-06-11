<!-- 
    This file is part of BestCraft.
    Copyright (C) 2026  Tnze

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
import { ref, watch, onMounted, computed } from 'vue';
import { ElDescriptions, ElDescriptionsItem, ElButton } from 'element-plus';
import { gte } from 'semver';

interface Versions {
    version: string;
    notes: string;
    pub_date: string;
    platforms: Record<
        string,
        {
            signature: string;
            url: string;
        }
    >;
}

const versions = ref<Versions>();

const endpoints = [
    'https://gitee.com/Tnze/ffxiv-best-craft/raw/main/versions.json',
    'https://github.com/Tnze/ffxiv-best-craft/releases/latest/download/latest.json',
    'https://raw.githubusercontent.com/Tnze/ffxiv-best-craft/main/versions.json',
];

async function requestVersions() {
    // 从多个源查询最新版本信息
    const requests = endpoints.map(async endpoint => {
        const resp = await fetch(endpoint, {
            method: 'GET',
            mode: 'cors',
        });
        return <Versions>await resp.json();
    });
    requests.push(import('@/../versions.json'));

    // 以任意顺序接收版本信息响应，找到最新的数据
    requests.map(resp =>
        resp
            .then(async v => {
                if (
                    versions.value == undefined ||
                    gte(v.version, versions.value.version)
                ) {
                    versions.value = v;
                }
            })
            .catch(() => {}),
    );
}

/** 检测用户当前系统信息 */
function detectUserPlatform() {
    const ua = navigator.userAgent;
    let osGuess = 'linux';
    if (ua.includes('Windows') || ua.includes('Win')) {
        osGuess = 'windows';
    } else if (ua.includes('Mac')) {
        osGuess = 'darwin';
    } else if (ua.includes('Linux')) {
        osGuess = 'linux';
    }

    let archGuess = 'x86_64';
    // 通过 navigator 或 UA 检测架构
    const nav = navigator as Navigator & {
        userAgentData?: {
            getHighEntropyValues?: (
                hints: string[],
            ) => Promise<{ architecture?: string }[]>;
        };
    };
    if (nav.userAgentData?.getHighEntropyValues) {
        nav.userAgentData
            .getHighEntropyValues(['architecture'])
            .then(([data]) => {
                if (data?.architecture) {
                    const archMap: Record<string, string> = {
                        arm64: 'aarch64',
                        x86_64: 'x86_64',
                        x86: 'x86_64',
                    };
                    if (archMap[data.architecture]) {
                        archGuess = archMap[data.architecture];
                    }
                    // 只在还没被用户手动改过时才自动设置
                    if (!userChanged.value) {
                        arch.value = archGuess;
                    }
                }
            });
    }
    if (
        ua.includes('aarch64') ||
        ua.includes('arm64') ||
        ua.includes('armv8')
    ) {
        archGuess = 'aarch64';
    }

    return { osGuess, archGuess };
}

onMounted(() => requestVersions());

const os = ref('');
const arch = ref('');
const format = ref('');

/** 固定所有可能的 OS / Arch / Format 选项 */
const ALL_OS = ['windows', 'linux', 'darwin'] as const;
const ALL_ARCH = ['x86_64', 'aarch64'] as const;
const ALL_FORMATS = ['msi', 'nsis', 'appimage', 'deb', 'rpm'] as const;

const platformList = computed(() => {
    if (versions.value === undefined) return [];
    const list = Object.keys(versions.value.platforms)
        .map(v => v.split('-'))
        .filter(v => v.length == 3);
    return list;
});

/** 汇总所有 platform 中出现的取值 */
const availableOS = computed(() => new Set(platformList.value.map(v => v[0])));
const availableArch = computed(() => {
    const set = new Set<string>();
    for (const [o, a] of platformList.value) {
        if (o === os.value) set.add(a);
    }
    return set;
});
const availableFormat = computed(() => {
    const set = new Set<string>();
    for (const [o, a, f] of platformList.value) {
        if (o === os.value && a === arch.value) set.add(f);
    }
    return set;
});

type SegOption = { label: string; value: string; disabled: boolean };

const OS_LABELS: Record<string, string> = {
    windows: 'Windows',
    linux: 'Linux',
    darwin: 'macOS',
};

const osOptions = computed<SegOption[]>(() =>
    ALL_OS.map(v => ({
        label: OS_LABELS[v],
        value: v,
        disabled: !availableOS.value.has(v),
    })),
);

const archOptions = computed<SegOption[]>(() =>
    ALL_ARCH.map(v => ({
        label: v,
        value: v,
        disabled: !availableArch.value.has(v),
    })),
);

const formatOptions = computed<SegOption[]>(() =>
    ALL_FORMATS.map(v => ({
        label: v,
        value: v,
        disabled: !availableFormat.value.has(v),
    })),
);

/** 各组分段器中当前选中项的索引 */
const osSelectedIndex = computed(() =>
    osOptions.value.findIndex(o => o.value === os.value),
);
const archSelectedIndex = computed(() =>
    archOptions.value.findIndex(o => o.value === arch.value),
);
const formatSelectedIndex = computed(() =>
    formatOptions.value.findIndex(o => o.value === format.value),
);

function indicatorStyle(index: number, count: number) {
    const pct = 100 / count;
    return {
        width: index < 0 ? '0' : `${pct}%`,
        transform: index < 0 ? 'translateX(0)' : `translateX(${index * 100}%)`,
    };
}

/** 标记用户是否手动更改过选项（手动改过后不再自动替换） */
const userChanged = ref(false);
function markUserChanged() {
    userChanged.value = true;
}

/** 自动选择未手动更改时的默认值 */
function autoSelectDefaults() {
    if (userChanged.value) return;
    if (platformList.value.length === 0) return;

    const { osGuess, archGuess } = detectUserPlatform();

    // 自动选 OS
    if (!os.value || !availableOS.value.has(os.value)) {
        os.value = availableOS.value.has(osGuess)
            ? osGuess
            : (osOptions.value.find(o => !o.disabled)?.value ?? '');
    }
    // 自动选 Arch
    if (!arch.value || !availableArch.value.has(arch.value)) {
        arch.value = availableArch.value.has(archGuess)
            ? archGuess
            : (archOptions.value.find(o => !o.disabled)?.value ?? '');
    }
    // 自动选 Format：优先 nsis > appimage > msi
    autoSelectFormat();
}

/** 确保当前 format 在可用列表中，否则自动选择最佳 */
function autoSelectFormat() {
    const formatPrefs = ['msi', 'nsis', 'appimage', 'deb', 'rpm'];
    if (!format.value || !availableFormat.value.has(format.value)) {
        format.value =
            formatPrefs.find(f => availableFormat.value.has(f)) ??
            formatOptions.value.find(o => !o.disabled)?.value ??
            '';
    }
}

// 当平台列表就绪时自动选择默认值
watch(platformList, autoSelectDefaults, { immediate: true });

// 切换 OS 时，若当前 Arch/Format 不可用则自动切换
watch(os, () => {
    if (arch.value && !availableArch.value.has(arch.value)) {
        arch.value = archOptions.value.find(o => !o.disabled)?.value ?? '';
    }
    if (format.value && !availableFormat.value.has(format.value)) {
        format.value = '';
        autoSelectFormat();
    }
});

// 切换 Arch 时，若当前 Format 不可用则自动切换
watch(arch, () => {
    if (format.value && !availableFormat.value.has(format.value)) {
        format.value = '';
        autoSelectFormat();
    }
});

const url = computed(() => {
    if (versions.value === undefined) return undefined;
    for (const key in versions.value?.platforms) {
        const v = key.split('-');
        if (v[0] === os.value && v[1] === arch.value && v[2] === format.value) {
            return versions.value?.platforms[key].url;
        }
    }
});
</script>

<template>
    <el-descriptions tnze-loading="versions" :column="1" :border="true">
        <el-descriptions-item :label="$t('version')">
            {{ versions?.version ?? $t('loading') }}
        </el-descriptions-item>
        <el-descriptions-item :label="$t('date')">
            {{
                versions
                    ? new Date(versions?.pub_date).toLocaleString()
                    : $t('loading')
            }}
        </el-descriptions-item>
        <el-descriptions-item :label="$t('download')">
            <div v-if="versions" class="download-selector">
                <span class="segmented-label">{{ $t('os-label') }}</span>
                <div class="custom-segmented">
                    <span
                        class="seg-indicator"
                        :style="
                            indicatorStyle(osSelectedIndex, osOptions.length)
                        "
                    />
                    <button
                        v-for="opt in osOptions"
                        :key="opt.value"
                        :class="{ selected: os === opt.value }"
                        :disabled="opt.disabled"
                        @click="
                            os = opt.value;
                            markUserChanged();
                        "
                    >
                        {{ opt.label }}
                    </button>
                </div>
                <span class="segmented-label">{{ $t('arch-label') }}</span>
                <div class="custom-segmented">
                    <span
                        class="seg-indicator"
                        :style="
                            indicatorStyle(
                                archSelectedIndex,
                                archOptions.length,
                            )
                        "
                    />
                    <button
                        v-for="opt in archOptions"
                        :key="opt.value"
                        :class="{ selected: arch === opt.value }"
                        :disabled="opt.disabled"
                        @click="
                            arch = opt.value;
                            markUserChanged();
                        "
                    >
                        {{ opt.label }}
                    </button>
                </div>
                <span class="segmented-label">{{ $t('format-label') }}</span>
                <div class="custom-segmented">
                    <span
                        class="seg-indicator"
                        :style="
                            indicatorStyle(
                                formatSelectedIndex,
                                formatOptions.length,
                            )
                        "
                    />
                    <button
                        v-for="opt in formatOptions"
                        :key="opt.value"
                        :class="{ selected: format === opt.value }"
                        :disabled="opt.disabled"
                        @click="
                            format = opt.value;
                            markUserChanged();
                        "
                    >
                        {{ opt.label }}
                    </button>
                </div>
                <div class="download-link">
                    <el-button
                        v-if="url !== undefined"
                        tag="a"
                        :href="url"
                        target="_blank"
                        type="primary"
                    >
                        {{ $t('download') }}
                    </el-button>
                    <span v-else class="download-hint">
                        {{ $t('select-platform-hint') }}
                    </span>
                </div>
            </div>
            <template v-else>{{ $t('loading') }}</template>
        </el-descriptions-item>
    </el-descriptions>
</template>

<style scoped>
.download-selector {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 8px 10px;
    align-items: center;
}

.segmented-label {
    font-size: 13px;
    color: var(--el-text-color-secondary);
    white-space: nowrap;
}

.custom-segmented {
    display: flex;
    position: relative;
    background: var(--el-fill-color-light);
    border-radius: 6px;
    padding: 3px;

    .seg-indicator {
        position: absolute;
        top: 3px;
        bottom: 3px;
        left: 3px;
        background: var(--el-color-primary);
        border-radius: 4px;
        transition:
            transform 0.3s cubic-bezier(0.4, 0, 0.2, 1),
            width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        z-index: 0;
    }

    button {
        flex: 1;
        position: relative;
        z-index: 1;
        border: none;
        outline: none;
        background: transparent;
        color: var(--el-text-color-regular);
        font-size: 12px;
        line-height: 1.6;
        padding: 3px 8px;
        border-radius: 4px;
        cursor: pointer;
        transition: color 0.3s;
        white-space: nowrap;

        &:hover:not(:disabled) {
            color: var(--el-text-color-primary);
        }

        &.selected {
            color: #fff;
        }

        &:disabled {
            opacity: 0.4;
            cursor: not-allowed;
        }
    }
}

.download-link {
    grid-column: 1 / -1;
    margin-top: 4px;
    min-height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;

    a,
    a:hover,
    a:visited {
        text-decoration: none;
    }
}

.download-hint {
    color: var(--el-text-color-placeholder);
    font-size: 12px;
    font-style: italic;
}
</style>

<fluent locale="zh-CN">
version = 版本号
date = 发布日期
download = 下载
loading = 加载中
os-label = 系统
arch-label = 架构
format-label = 格式
select-platform-hint = 不支持当前组合
</fluent>

<fluent locale="zh-TW">
version = 版本號
date = 釋出日期
download = 下載
loading = 載入中
os-label = 系統
arch-label = 架構
format-label = 格式
select-platform-hint = 不支援當前組合
</fluent>

<fluent locale="en-US">
version = Version
date = Pub Date
download = Download
loading = Loading
os-label = OS
arch-label = Arch
format-label = Format
select-platform-hint = Unsupported combination
</fluent>

<fluent locale="ja-JP">
version = バージョン
date = 公開日
download = ダウンロード
loading = 読み込み中
os-label = OS
arch-label = アーキテクチャ
format-label = 形式
select-platform-hint = この組み合わせはサポートされていません
</fluent>
