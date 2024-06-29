// This file is part of BestCraft.
// Copyright (C) 2023 Tnze
//
// BestCraft is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// BestCraft is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

import { Actions, Status } from "./Craft";

export let supported = true;

if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
    // Good, the user is using our Desktop edition. Use the native solvers.
    var pkgTauri = import("@tauri-apps/api/tauri")
} else {
    // They are using the Web edition. Only wasm solvers could be used.
    // Check if the browser supports Web Worker.
    if (!window.Worker) supported = false
    var invokeWasmSolver = (name: string, args: any): Promise<Actions[]> => {
        return new Promise((resolve, reject) => {
            const worker = new Worker(new URL('./SolverWorker.ts', import.meta.url), { type: 'module' })
            worker.onmessage = ev => resolve(ev.data)
            worker.onerror = ev => reject(ev)
            worker.postMessage({ name, args: JSON.stringify(args) })
        })
    }
}

export async function create_solver(
    status: Status,
    useMuscleMemory: boolean,
    useManipulation: boolean,
    useObserve: boolean,
) {
    let { invoke } = await pkgTauri
    return invoke("create_solver", {
        status,
        useMuscleMemory,
        useManipulation,
        useObserve,
    });
};

export async function destroy_solver(status: Status) {
    if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
        let { invoke } = await pkgTauri
        invoke("destroy_solver", { status });
    } else {
        throw "solver-doesn-t-exist"
    }
};

export async function read_solver(status: Status): Promise<Actions[]> {
    if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
        let { invoke } = await pkgTauri
        return invoke("read_solver", { status });
    } else {
        throw "solver-doesn-t-exist"
    }
};

export async function rika_solve(status: Status): Promise<Actions[]> {
    if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
        return (await pkgTauri).invoke("rika_solve", { status })
    } else {
        return invokeWasmSolver("rika_solve", { status })
    }
}

export async function rika_solve_tnzever(status: Status, useManipulation: boolean, useWastNot: number, useObserve: boolean, reduceSteps: boolean): Promise<Actions[]> {
    if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
        let { invoke } = await pkgTauri
        return invoke("rika_solve_tnzever", { status, useManipulation, useWastNot, useObserve, reduceSteps })
    } else {
        throw "solver-doesn-t-exist"
    }
}

export async function dfs_solve(status: Status, depth: number, specialist: boolean): Promise<Actions[]> {
    const args = { status, depth, specialist };
    if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
        return (await pkgTauri).invoke("dfs_solve", args)
    } else {
        return invokeWasmSolver("dfs_solve", args)
    }
}

export async function nq_solve(status: Status, depth: number, specialist: boolean): Promise<Actions[]> {
    const args = { status, depth, specialist };
    if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
        return (await pkgTauri).invoke("nq_solve", args)
    } else {
        return invokeWasmSolver("nq_solve", args)
    }
}

/// 基于DP的闲静手法求解
/// useManipulation: 是否使用掌握
/// useWastNot: 是否使用俭约（0：不使用，4：使用俭约，8：使用俭约和长期俭约）
export async function reflect_solve(status: Status, useManipulation: boolean, useWasteNot: number, useObserve: boolean): Promise<Actions[]> {
    if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
        let { invoke } = await pkgTauri
        return invoke("reflect_solve", { status, useManipulation, useWasteNot, useObserve })
    } else {
        return invokeWasmSolver("reflect_solve", { status, useObserve })
    }
}

export function formatDuration(u: number): string {
    if (u < 1000) {
        return u + "ms"
    } else {
        const h = Math.floor(u / 1000 / 3600)
        const m = Math.floor(u / 1000 / 60) - h * 60
        const s = (u / 1000 - h * 3600 - m * 60).toFixed(3)
        return (h > 0 ? h + 'h' : '') + (m > 0 ? m + 'm' : '') + (s + 's')
    }
}
