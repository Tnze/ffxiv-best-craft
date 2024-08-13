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
import { NCard, NProgress, NDivider } from 'naive-ui';
</script>

<template>
    <n-card>
        <template #header>
            Lv.100 铸甲匠
        </template>
        <n-progress type="line" :height="24" :percentage="100">6600 / 6600</n-progress>
        <n-progress type="line" :height="24" :percentage="70">7000 / 12000</n-progress>
        <n-progress type="line" :height="24" :percentage="70">563 / 563</n-progress>
        <n-divider />
        <n-flex justify="space-between">
            <span>耐久度 70/70</span>
            <span>优质率 0%</span>
            <span>成功率 0%</span>
        </n-flex>
    </n-card>
</template>
