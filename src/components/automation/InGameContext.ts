import { Actions, Attributes, Conditions, Recipe } from "../../Craft"
import { startHttpServer } from '../../Automation'
import { Event, listen } from '@tauri-apps/api/event'

startHttpServer("127.0.0.1:2020")

interface ActionStep {
    action: Actions
    progress: number,
    quality: number,
    durability: number,
    condition: Conditions,
}

let waiters: ((value: ActionStep) => void)[] = []
listen('action-step', (event: Event<ActionStep>) => {
    for (let waiter of waiters)
        waiter(event.payload)
    waiters = []
})

let conditionStrings = new Map([
    [Conditions.Normal, "通常"],
    [Conditions.Good, "高品质"],
    [Conditions.Excellent, "最高品质"],
    [Conditions.Poor, "低品质"],
    [Conditions.Centered, "安定"],
    [Conditions.Sturdy, "结实"],
    [Conditions.Pliant, "高效"],
    [Conditions.Malleable, "高进展"],
    [Conditions.Primed, "长持续"],
])

// 自动化脚本执行上下文
export default {
    // 鲇鱼精邮差URL
    urlPostNamazu: 'http://localhost:2019/command',
    // 最后一次技能状态
    currentStatus: <ActionStep | null>null,

    onCraftStart(handler: Generator) {
        console.log('craft start event set to\n' + handler)
    },
    get_progress(): number { return this.currentStatus!.progress },
    get_quality(): number { return this.currentStatus!.quality },
    get_durability(): number { return this.currentStatus!.durability },
    get_condition(): string { return conditionStrings.get(this.currentStatus!.condition) || "" },

    async sleep(t: number) { return new Promise((res, _rej) => setTimeout(res, t)) },
    async command(cmd: string): Promise<void> {
        const req = fetch(this.urlPostNamazu, { body: cmd, mode: 'no-cors', method: 'POST' })
        this.currentStatus = await new Promise<ActionStep>((res, rej) => {
            const t = setTimeout(() => {
                waiters.splice(waiters.indexOf(f), 1)
                rej('do action time out')
            }, 5000)
            const f = async (e: ActionStep) => {
                clearTimeout(t)
                res(e)
            }
            waiters.push(f)
        }) // wait for the skill
        await req
        await this.sleep(500)
    },
}