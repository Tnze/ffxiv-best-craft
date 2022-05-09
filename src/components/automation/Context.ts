import { Attributes, Recipe } from "../../Craft"

// 自动化脚本执行上下文
export default {
    // 鲇鱼精邮差URL
    urlPostNamazu: 'http://localhost:2019/command',

    onCraftStart(handler: (attr: Attributes, recipe: Recipe) => void) {
        console.log('craft start event set to\n' + handler)
    },

    async sleep() { new Promise((res, _rej) => setTimeout(res, 1000)) },
    async command(cmd: string): Promise<void> {
        await fetch(this.urlPostNamazu, { body: cmd, mode: 'no-cors', method: 'POST' })
    },
}