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
import { NModal, NCard, NTabs, NTabPane } from 'naive-ui';
</script>

<template>
    <n-modal>
        <n-card style="width: 600px">
            <n-tabs type="bar" animated>
                <n-tab-pane name="dfs-solver" :tab="$t('dfs-solver')">
                    TODO
                </n-tab-pane>
                <n-tab-pane name="dp-solver" :tab="$t('dp-solver')">
                    TODO
                </n-tab-pane>
                <n-tab-pane name="raphael-solver" :tab="$t('raphael-solver')">
                    TODO
                </n-tab-pane>
            </n-tabs>
        </n-card>
    </n-modal>
</template>
