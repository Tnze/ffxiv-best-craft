<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { createDir, readTextFile, writeFile, Dir } from '@tauri-apps/api/fs'
import * as Blockly from 'blockly'
import Theme from '@blockly/theme-modern'
import BlocklyJS from 'blockly/javascript'
import InGameContext from './InGameContext'
import ZhHans from 'blockly/msg/zh-hans'
import BlockDefines from './block_defines.json'
import { Promotion } from '@element-plus/icons-vue'
import { ElMessage, useTimeout } from 'element-plus'

const blocklyDiv = ref<Element | null>(null)

const toolbox = {
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
                // {
                //     "kind": "block",
                //     "type": "on_craft_start"
                // },
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
const theme = Blockly.Theme.defineTheme('BestCraftTheme', {
    'base': Theme,
    'startHats': false
});
//@ts-ignore
Blockly.setLocale(ZhHans)
Blockly.defineBlocksWithJsonArray(BlockDefines)
//@ts-ignore
BlocklyJS['on_craft_start'] = function (block: Blockly.Block) {
    const code = 'yield this.onCraftStart(function *(attr, recipe) {\n' + BlocklyJS.statementToCode(block, 'STEPS') + '})'
    return code;
};
//@ts-ignore
BlocklyJS['do_action'] = function (block: Blockly.Block) {
    const actionName = BlocklyJS.valueToCode(block, 'ACTION', (BlocklyJS as any).ORDER_FUNCTION_CALL)
    const code = `yield this.command('/ac ' + ${actionName})\n`
    return code;
};
//@ts-ignore
BlocklyJS['get_progress'] = function (block: Blockly.Block) {
    return ["this.get_progress()", (BlocklyJS as any).ORDER_FUNCTION_CALL];
};
//@ts-ignore
BlocklyJS['get_quality'] = function (block: Blockly.Block) {
    return ["this.get_quality()", (BlocklyJS as any).ORDER_FUNCTION_CALL];
};
//@ts-ignore
BlocklyJS['get_durability'] = function (block: Blockly.Block) {
    return ["this.get_durability()", (BlocklyJS as any).ORDER_FUNCTION_CALL];
};
//@ts-ignore
BlocklyJS['get_condition'] = function (block: Blockly.Block) {
    return ["this.get_condition()", (BlocklyJS as any).ORDER_FUNCTION_CALL];
};
BlocklyJS.addReservedWords('highlightBlock')
BlocklyJS.STATEMENT_PREFIX = 'highlightBlock(%1);\n'

let workspace: Blockly.WorkspaceSvg | null = null
onMounted(async () => {
    workspace = Blockly.inject(blocklyDiv.value as Element, {
        theme,
        toolbox: toolbox,
        grid: {
            spacing: 25,
            length: 3,
            colour: '#ccc',
            snap: true,
        },
        media: "blockly/media/"
    })
    // load workspace
    try {
        const ws = await readTextFile('automation.json', { dir: Dir.App })
        //@ts-ignore
        Blockly.serialization.workspaces.load(JSON.parse(ws), workspace)
    } catch (err) {
        // may be the file is not exist
        console.log(err)
    }
    let timer: number | null = null
    workspace.addChangeListener((_event: any) => {
        if (timer == null) {
            timer = window.setTimeout(async () => {
                //@ts-ignore
                const ws = JSON.stringify(Blockly.serialization.workspaces.save(workspace))
                try {
                    await writeFile({ contents: ws, path: 'automation.json' }, { dir: Dir.App })
                } catch (err) {
                    try {
                        await createDir('', { dir: Dir.App })
                        await writeFile({ contents: ws, path: 'automation.json' }, { dir: Dir.App })
                    } catch (err) {
                        console.log(err)
                    }
                }
                timer = null
            }, 3000)
        }
    })
})


async function run() {
    const code = `return (function* () {\n${BlocklyJS.workspaceToCode(workspace! as Blockly.Workspace)}}).call(this)`
    console.log(code)
    try {
        const hightlightBlock = (id: string) => workspace!.highlightBlock(id)
        const f = new Function("highlightBlock", code)
        const g: Generator = f.call(InGameContext, hightlightBlock)
        for (let v of g) {
            console.log(await v)
            // await InGameContext.sleep(1000)
        }
    } catch (err) {
        console.error(err)
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