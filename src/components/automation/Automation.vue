<script setup lang="ts">
import { onMounted, ref, watchEffect } from 'vue'
import { createDir, readTextFile, writeFile, Dir } from '@tauri-apps/api/fs'
import * as Blockly from 'blockly'
import Theme from '@blockly/theme-modern'
import DarkTheme from '@blockly/theme-dark';
import BlocklyJS from 'blockly/javascript'
import InGameContext from './InGameContext'
import ZhHans from 'blockly/msg/zh-hans'
import BlockDefines from './block_defines.json'
import toolbox from './toolbox.json'
import { Promotion } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

const blocklyDiv = ref<Element | null>(null)
const props = defineProps<{
    isDark?: boolean
}>()

function defineTheme(base: any, name: string) {
    return Blockly.Theme.defineTheme('BestCraftTheme-' + name, {
        'base': base,
        'startHats': false,
        'blockStyles': {
            "crafting_blocks": {
                'colourPrimary': 230,
                'colourSecondary': 230,
                'colourTertiary': 230,
            },
            "craft_simulating_blocks": {
                'colourPrimary': 300,
                'colourSecondary': 300,
                'colourTertiary': 300,
            }
        },
        'categoryStyles': {
            "crafting_category": {
                "colour": 230
            },
            "craft_simulating_category": {
                "colour": 300
            }
        }
    });
}
const theme = defineTheme(Theme, 'light');
const themeDark = defineTheme(DarkTheme, 'dark');
//@ts-ignore
Blockly.setLocale(ZhHans)
Blockly.defineBlocksWithJsonArray(BlockDefines)
//@ts-ignore
BlocklyJS['on_craft_start'] = function (block: Blockly.Block) {
    const code = 'this.onCraftStart(function (attr, recipe) {\n' + BlocklyJS.statementToCode(block, 'STEPS') + '})'
    return code;
};
//@ts-ignore
BlocklyJS['do_action'] = function (block: Blockly.Block) {
    const actionName = BlocklyJS.valueToCode(block, 'ACTION', (BlocklyJS as any).ORDER_FUNCTION_CALL)
    const code = `await this.command('/ac ' + ${actionName})\n`
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
//@ts-ignore
BlocklyJS['procedures_callreturn'] = function (block) {
    // Call a procedure with a return value.
    //@ts-ignore
    var funcName = BlocklyJS.variableDB_.getName(block.getFieldValue('NAME'), Blockly.PROCEDURE_CATEGORY_NAME);
    var args = ['this'];
    var variables = block.getVars();
    for (var i = 0; i < variables.length; i++) {
        args[i] = BlocklyJS.valueToCode(block, 'ARG' + i,
            (BlocklyJS as any).ORDER_NONE) || 'null';
    }
    var code = 'await ' + funcName + '.call(' + args.join(', ') + ')';
    return [code, (BlocklyJS as any).ORDER_AWAIT];
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
    watchEffect(() => {
        console.log("use darkmode: ", props.isDark)
        workspace!.setTheme(props.isDark ? themeDark : theme)
    })
})


async function run() {
    let code = BlocklyJS
        .workspaceToCode(workspace! as Blockly.Workspace)
        .replace(/(?<=^|\n)function \w+\(.*\)/g, 'async $&')
    code = `return (async () => {\n${code}})()`
    console.log(code)
    try {
        const hightlightBlock = (id: string) => workspace!.highlightBlock(id)
        const f = new Function("highlightBlock", code)
        await f.call(
            InGameContext,
            hightlightBlock,
        )
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