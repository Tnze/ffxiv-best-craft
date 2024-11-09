// This file is part of BestCraft.
// Copyright (C) 2024 Tnze
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

import { isTauri } from './Consts';
import { Actions, Status } from './Craft';

if (isTauri) {
    // Good, the user is using our Desktop edition. Use the native solvers.
    var pkgTauri = import('@tauri-apps/api/core');
}

export interface Statistics {
    // 发生技能错误的模拟频数
    errors: number;
    // 技能模拟完成后仍处于制作状态的模拟频数
    unfinished: number;
    // 进展未推满的模拟频数
    fails: number;
    // 进展推满，HQ率未达到100%的模拟频数
    normal: number;
    // 进展推满，品质也推满的模拟频数
    highqual: number;
}

export async function rand_simulation(
    status: Status,
    actions: Actions[],
    n: number,
    ignoreErrors: boolean,
): Promise<Statistics> {
    const args = { status, actions, n, ignoreErrors };
    if (isTauri) {
        let { invoke } = await pkgTauri;
        return invoke('rand_simulation', args);
    } else {
        return new Promise((resolve, reject) => {
            const worker = new Worker(
                new URL('./AnalyzerWorker.ts', import.meta.url),
                { type: 'module' },
            );
            worker.onmessage = ev => resolve(ev.data);
            worker.onerror = ev => reject(ev);
            worker.postMessage({
                name: 'rand_simulation',
                args: JSON.stringify(args),
            });
        });
    }
}

export interface Scope {
    craftsmanship_range: [number?, number?];
    control_range: number | null;
    craft_points: number;
}

export async function calc_attributes_scope(
    status: Status,
    actions: Actions[],
): Promise<Scope> {
    const args = { status, actions };
    if (isTauri) {
        let { invoke } = await pkgTauri;
        return invoke('calc_attributes_scope', args);
    } else {
        return new Promise((resolve, reject) => {
            const worker = new Worker(
                new URL('./AnalyzerWorker.ts', import.meta.url),
                { type: 'module' },
            );
            worker.onmessage = ev => resolve(ev.data);
            worker.onerror = ev => reject(ev);
            worker.postMessage({
                name: 'calc_attributes_scope',
                args: JSON.stringify(args),
            });
        });
    }
}
