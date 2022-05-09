<script setup lang="ts">
import Blockly from 'blockly'
import Theme from '@blockly/theme-modern';
import 'blockly/javascript'
import ZhHans from 'blockly/msg/zh-hans'
import { unescape } from 'lodash'
import { onMounted, ref } from 'vue'
import BlockDefines from './block_defines.json'

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
Blockly.setLocale(ZhHans)
Blockly.defineBlocksWithJsonArray(BlockDefines)
Blockly.JavaScript['on_craft_start'] = function (block: Blockly.Block) {
    const code = 'function() {\n' + Blockly.JavaScript.statementToCode(block, 'STEPS') + '}'
    return code;
};
Blockly.JavaScript['do_action'] = function (block: Blockly.Block) {
    const actionName = unescape(block.getFieldValue('ACTION'));
    const code = 'command("/ac ' + actionName + '")\n'
    return code;
};

const code = ref('')
onMounted(() => {
    const workspace = Blockly.inject(blocklyDiv.value as Element, {
        theme: Theme,
        toolbox: toolbox,
        grid: {
            spacing: 25,
            length: 3,
            colour: '#ccc',
            snap: true,
        },
    })
    workspace.addChangeListener((event: any) => {
        code.value = Blockly.JavaScript.workspaceToCode(workspace as Blockly.Workspace);
        console.log(code.value)
    })
})

</script>

<template>
    <el-container>
        <el-header>
            <h1>自动化</h1>
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