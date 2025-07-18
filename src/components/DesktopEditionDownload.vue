<!-- 
    This file is part of BestCraft.
    Copyright (C) 2025  Tnze

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
import { ref } from 'vue';
import {
    ElDescriptions,
    ElDescriptionsItem,
    ElLink,
    ElSpace,
} from 'element-plus';
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

requestVersions();
</script>

<template>
    <el-descriptions tnze-loading="versions" :column="1" :border="true">
        <el-descriptions-item :label="$t('version')">{{
            versions?.version ?? $t('loading')
        }}</el-descriptions-item>
        <el-descriptions-item :label="$t('date')">
            {{
                versions
                    ? new Date(versions?.pub_date).toLocaleString()
                    : $t('loading')
            }}
        </el-descriptions-item>
        <el-descriptions-item :label="$t('download')">
            <el-space
                v-if="versions"
                direction="vertical"
                alignment="flex-start"
            >
                <el-link
                    v-for="(x, platform) in versions.platforms"
                    :href="x.url"
                    target="_blank"
                    type="primary"
                >
                    {{
                        (
                            platform.substring(0, 1).toUpperCase() +
                            platform.substring(1)
                        ).replaceAll('-', ' ')
                    }}
                </el-link>
            </el-space>
            <template v-else>{{ $t('loading') }}</template>
        </el-descriptions-item>
    </el-descriptions>
</template>

<fluent locale="zh-CN">
version = 版本号
date = 发布日期
download = 下载链接
loading = 加载中
</fluent>
<fluent locale="en-US">
version = Version
date = Pub Date
download = Download
loading = Loading
</fluent>
<fluent locale="ja-JP">
</fluent>
