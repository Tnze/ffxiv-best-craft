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
import { ElDescriptions, ElDescriptionsItem, ElTag } from 'element-plus';
</script>

<template>
    <el-descriptions :column="1" :border="true">
        <el-descriptions-item label="支持软件开发维护">
            欢迎至项目 Github 仓库贡献
            <el-tag effect="plain" round>PR</el-tag>
            或
            <el-tag effect="plain" round>Star</el-tag>
        </el-descriptions-item>
        <el-descriptions-item label="支持 YYYY.GAMES 服务器运行">
            <a href="https://afdian.com/@ffxivtk" target="_blank">
                <img src="@/assets/coffee.png" alt="Buy Me a Coffee" style="vertical-align: middle;">
            </a>
        </el-descriptions-item>
    </el-descriptions>
</template>
