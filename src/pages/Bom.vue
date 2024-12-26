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
import { onActivated } from 'vue';
import { ElScrollbar, ElDivider } from 'element-plus';

import Item from '@/components/bom/Item.vue';

const emit = defineEmits<{
    (e: 'setTitle', title: string): void;
}>();
onActivated(() => emit('setTitle', 'bill-of-material'));
</script>

<template>
    <el-scrollbar>
        <div class="page">
            <el-scrollbar>
                <div class="row">
                    <Item class="item" v-for="j in 3" :name="'item' + j" />
                </div>
            </el-scrollbar>
            <el-divider />
        </div>
    </el-scrollbar>
</template>

<style scoped>
.page {
    padding: 10px;
}

.row {
    display: flex;
    padding-bottom: 5px;
}

.item {
    flex: none;
    margin: 5px;
}
</style>
