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
import { clarityReport } from '@/libs/Utils';
import {
    ElInput,
    ElButton,
    ElMessage,
    ElCheckbox,
    ElSpace,
    ElSwitch,
} from 'element-plus';
import { useFluent } from 'fluent-vue';
import { ref } from 'vue';

const emits = defineEmits<{
    onRecognized: [actions: Actions[]];
}>();
const fluent = useFluent();

// Create a map converts action names to std Action enum
const namesToAction: Map<string, Actions> = new Map();
const actionKeys = Object.values(Actions).filter(v => !v.endsWith('_fail'));
for (const bundle of fluent.bundles.value) {
    for (const action of actionKeys) {
        const msg = fluent.getMessage(bundle, action.replaceAll('_', '-'))!;
        namesToAction.set(msg.value as string, action);
    }
}

// Textarea input
const inputText = ref('');
const strictMode = ref(false);

// Start parcing user input text
function confirm() {
    const input = inputText.value;
    let result: Actions[];

    if (input.trimStart().charAt(0) == '[') {
        try {
            result = parseJson(JSON.parse(input));
            clarityReport('importJsonSuccess');
        } catch (err) {
            clarityReport('importJsonError');
            ElMessage({
                type: 'error',
                showClose: true,
                message: fluent.$t('err-parse-json', { err: String(err) }),
            });
            return;
        }
    } else if (strictMode.value) {
        try {
            result = parseMacroStrict(input);
            clarityReport('importMacroStrictSuccess');
        } catch (err) {
            ElMessage({
                type: 'error',
                showClose: true,
                message: fluent.$t('err-parse-strict', { err: String(err) }),
            });
            return;
        }
    } else {
        result = input
            .split(/\/[^\s]+|<wait\.\d+>|\n/)
            .map(v => v.trim())
            .filter(v => v.length > 0)
            .map(v => namesToAction.get(trimQuotation(v)))
            .filter(v => v != undefined);
        if (result.length == 0) {
            clarityReport('importMacroError');
            ElMessage({
                type: 'warning',
                showClose: true,
                message: fluent.$t('warn-action-not-found'),
            });
            return;
        }
        clarityReport('importMacroSuccess');
    }

    ElMessage({
        type: 'success',
        showClose: true,
        message: fluent.$t('recognize-success', { n: result.length }),
    });
    emits('onRecognized', result);
    inputText.value = '';
}

// Parse input as json
function parseJson(result: any): Actions[] {
    if (!Array.isArray(result)) {
        throw fluent.$t('err-not-an-array');
    }
    const validActions = new Set(Object.values(Actions).map(v => v as string));

    return result.map(v => {
        if (typeof v != 'string') {
            throw fluent.$t('err-not-a-string', { elem: String(v) });
        }
        if (validActions.has(v)) {
            return v as Actions;
        } else {
            throw fluent.$t('err-invalid-action', { action: String(v) });
        }
    });
}

function parseMacroStrict(input: string): Actions[] {
    return input
        .split('\n')
        .flatMap(v => {
            const matchResult = /^\/(?:ac(?:tion)?|技能)\s+(?<body>.*)$/g.exec(
                v,
            );
            return matchResult ? matchResult.groups!['body'] : [];
        })
        .map((v, i) => {
            const cmdBody = v.trim(); // "ACTIONNAME <wait.n>"
            const matchResult =
                /^(?<action>"[^"]+"|\S+)(?:\s+<wait\.\d+>)?$/g.exec(cmdBody);
            if (matchResult == null || matchResult.groups == undefined) {
                throw fluent.$t('err-parse-line-error', { n: i + 1 });
            }
            const actionName = trimQuotation(matchResult.groups['action']);
            const action = namesToAction.get(actionName);
            if (action == undefined) {
                throw fluent.$t('err-invalid-action', { action: actionName });
            }
            return action;
        });
}

// '"Reflect"' => 'Reflect'
function trimQuotation(v: string): string {
    if (v.charAt(0) == '"' && v.charAt(v.length - 1) == '"') {
        return v.substring(1, v.length - 1);
    }
    return v;
}
</script>

<template>
    <el-input
        v-model="inputText"
        type="textarea"
        class="user-input"
        :autosize="{ minRows: 4 }"
        :placeholder="$t('auto-recognize')"
    />
    <el-space>
        <el-button
            type="primary"
            @click="confirm"
            :disabled="inputText.length == 0"
        >
            {{ $t('confirm') }}
        </el-button>
        <el-checkbox v-model="strictMode" :label="$t('strict-mode')" />
        <el-switch v-model="strictMode" />
    </el-space>
</template>

<style scoped>
.user-input {
    margin-bottom: 10px;
}
</style>

<fluent locale="zh-CN">
auto-recognize = 粘贴自动识别
confirm = 确认
strict-mode = 严格模式

err-parse-json = 尝试解析 JSON 失败：{ $err }
err-not-an-array = 输入的 JSON 不是一个数组
err-not-a-string = 元素 { $elem } 不是一个字符串
err-invalid-action = 未知的技能：{ $action }

err-parse-strict = 严格模式导入宏失败：{ $err }
err-parse-line-error = 导入第 { $n } 行失败

warn-action-not-found = 没有识别到技能
recognize-success = 识别成功，一共导入了 { $n } 个技能
</fluent>

<fluent locale="en-US">
auto-recognize = Paste macros or JSON here
confirm = Confirm
strict-mode = Strict mode

err-parse-json = Try parsing JSON failed: { $err }
err-not-an-array = Input JSON is not an array
err-not-a-string = Element { $elem } is not a string
err-invalid-action = Invalid action: { $action }

err-parse-strict = Try parsing in strict mode failed: { $err }
err-parse-line-error = Parsing line { $n } failed

warn-action-not-found = No action is found
recognize-success = Recognize successed, { $n ->
    [one] one action is
    *[other] {$n} actions are
} imported
</fluent>
