<script setup lang="ts">
import { onMounted, ref } from 'vue'
import Blockly, { Workspace } from 'blockly'
import Theme from '@blockly/theme-modern'
import BlocklyJS from 'blockly/javascript'
import Context from './Context'
import ZhHans from 'blockly/msg/zh-hans'
import BlockDefines from './block_defines.json'
import { Promotion } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

const blocklyDiv = ref<Element | null>(null)

var toolbox = {
    "kind": "categoryToolbox",
    "contents": [
        {
            "kind": "category",
            "name": "流程",
            "categorystyle": "loop_category",
            "contents": [
                {
                    "kind": "block",
                    "type": "controls_repeat_ext"
                },
                {
                    "kind": "block",
                    "type": "controls_repeat"
                },
                {
                    "kind": "block",
                    "type": "controls_whileUntil"
                },
                {
                    "kind": "block",
                    "type": "controls_for"
                },
                {
                    "kind": "block",
                    "type": "controls_flow_statements"
                },
            ]
        },
        {
            "kind": "category",
            "name": "逻辑",
            "categorystyle": "logic_category",
            "contents": [
                {
                    "kind": "block",
                    "type": "controls_if"
                },
                {
                    "kind": "block",
                    "type": "logic_compare"
                },
                {
                    "kind": "block",
                    "type": "logic_operation"
                },
                {
                    "kind": "block",
                    "type": "logic_boolean"
                },
            ]
        },
        {
            "kind": "category",
            "name": "数学",
            "categorystyle": "math_category",
            "contents": [
                {
                    "kind": "block",
                    "type": "math_number"
                },
                {
                    "kind": "block",
                    "type": "math_arithmetic"
                },
            ]
        },
        {
            "kind": "sep",
        },
        {
            "kind": "category",
            "name": "变量",
            "categorystyle": "variable_dynamic_category",
            "custom": "VARIABLE"
        },
        {
            "kind": "category",
            "name": "函数",
            "categorystyle": "colour_category",
            "custom": "PROCEDURE"
        },
        {
            "kind": "sep",
        },
        {
            "kind": "category",
            "name": "生产",
            "categorystyle": "procedure_category",
            "contents": [
                {
                    "kind": "block",
                    "type": "on_craft_start"
                },
                {
                    "kind": "block",
                    "type": "do_action",
                    "inputs": {
                        "ACTION": {
                            "shadow": {
                                "type": "text",
                                "fields": {
                                    "TEXT": ""
                                }
                            }
                        }
                    },
                },
                {
                    "kind": "block",
                    "type": "text"
                },
                {
                    "kind": "block",
                    "type": "get_progress"
                },
                {
                    "kind": "block",
                    "type": "get_quality"
                },
                {
                    "kind": "block",
                    "type": "get_craft_point"
                },
                {
                    "kind": "block",
                    "type": "get_durability"
                },
                {
                    "kind": "block",
                    "type": "get_condition"
                }
            ]
        },
    ]
}
//@ts-ignore
Blockly.setLocale(ZhHans)
Blockly.defineBlocksWithJsonArray(BlockDefines)
//@ts-ignore
BlocklyJS['on_craft_start'] = function (block: Blockly.Block) {
    const code = 'this.onCraftStart(async (attr, recipe) => {\n' + BlocklyJS.statementToCode(block, 'STEPS') + '})'
    return code;
};
//@ts-ignore
BlocklyJS['do_action'] = function (block: Blockly.Block) {
    const actionName = BlocklyJS.valueToCode(block, 'ACTION', (BlocklyJS as any).ORDER_FUNCTION_CALL)
    const code = `await this.command('/ac ' + ${actionName})\n`
    return code;
};

let workspace: Workspace | null = null
onMounted(() => {
    workspace = Blockly.inject(blocklyDiv.value as Element, {
        theme: Theme,
        toolbox: toolbox,
        grid: {
            spacing: 25,
            length: 3,
            colour: '#ccc',
            snap: true,
        },
        media: "blockly/media/"
    }) as Workspace
})

function run() {
    const code = BlocklyJS.workspaceToCode(workspace!);
    try {
        new Function(`(async () => {${code}})()`).call(Context)
    } catch (err) {
        ElMessage({
            type: "error",
            showClose: true,
            message: err as string,
        })
    }
}

</script>

<template>
    <el-container>
        <el-header>
            <h1>自动化
                <el-button @click="run" type="success" :icon="Promotion" circle />
            </h1>
        </el-header>
        <el-main>
            <div ref="blocklyDiv" id="blocklyDiv"></div>
        </el-main>
    </el-container>
</template>

<style scoped>
#blocklyDiv {
    height: 100%;
    width: 100%;
}
</style>