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
import { Actions } from '@/libs/Craft';
import { ElInput, ElButton } from 'element-plus';
import { useFluent } from 'fluent-vue';
import { ref } from 'vue';

const emits = defineEmits<{
    onRecognized: [actions: Actions[]]
}>()
const { $t } = useFluent();

const inputText = ref('');

function confirm() {
    const input = inputText.value;
    if (input.trimStart().charAt(0) == "[") {
        try {
            const result = JSON.parse(input);
            emits('onRecognized', parseJson(result));
        } catch (err) {
            console.error($t('err-parse-json', { err: String(err) }))
        }
    }
}

function parseJson(result: any): Actions[] {
    if (!Array.isArray(result)) {
        throw $t('err-not-an-array');
    }
    const validActions = new Set(Object.values(Actions).map(v => v as string));

    return result.map(v => {
        if (typeof v != 'string') {
            throw $t('err-not-a-string', { elem: String(v) })
        }
        if (validActions.has(v)) {
            return v as Actions
        } else {
            throw $t('err-invalid-action', { action: String(v) })
        }
    })
}

</script>

<template>
    <el-input v-model="inputText" type="textarea" class="user-input" :autosize="{ minRows: 4 }"
        :placeholder="$t('auto-recognize')" />
    <el-button type="primary" @click="confirm">{{ $t('confirm') }}</el-button>
</template>

<style scoped>
.user-input {
    margin-bottom: 10px;
}
</style>

<fluent locale="zh-CN">
auto-recognize = 粘贴自动识别
confirm = 确认

err-parse-json = 尝试解析 JSON 失败：{ $err }
err-not-an-array = 输入的 JSON 不是一个数组
err-not-a-string = 元素 { $elem } 不是一个字符串
err-invalid-action = 未知的技能：{ $action }
</fluent>