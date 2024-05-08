
import { Actions, Status } from "./Craft";

if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
    // Good, the user is using our Desktop edition. Use the native solvers.
    var pkgTauri = import("@tauri-apps/api/tauri")
}

export interface Statistics {
    // 发生技能错误的模拟频数
    errors: number,
    // 技能模拟完成后仍处于制作状态的模拟频数
    unfinished: number,
    // 进展未推满的模拟频数
    fails: number,
    // 进展推满，HQ率未达到100%的模拟频数
    normal: number,
    // 进展推满，品质也推满的模拟频数
    highqual: number,
}

export async function rand_simulation(status: Status, actions: Actions[], n: number): Promise<Statistics> {
    if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
        let { invoke } = await pkgTauri
        return invoke("rand_simulation", { status, actions, n });
    } else {
        return new Promise((resolve, reject) => {
            const worker = new Worker(new URL('./AnalyzerWorker.ts', import.meta.url), { type: 'module' })
            worker.onmessage = ev => resolve(ev.data)
            worker.onerror = ev => reject(ev)
            worker.postMessage({ name: "rand_simulation", args: JSON.stringify({ status, actions, n }) })
        })
    }
}