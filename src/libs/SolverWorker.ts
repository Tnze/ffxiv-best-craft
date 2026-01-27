// This file is part of BestCraft.
// Copyright (C) 2026 Tnze
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


onmessage = async e => {
    if (import.meta.env.VITE_BESTCRAFT_TARGET == 'web') {
        var { dfs_solve, nq_solve, reflect_solve, raphael_solve } =
            await import('../../pkg-wasm/app_wasm');
    } else return;
    const { name, args: argsJson } = e.data;
    const args = JSON.parse(argsJson);
    try {
        var result;
        switch (name) {
            case 'dfs_solve':
            case 'nq_solve':
                let solve = name == 'dfs_solve' ? dfs_solve : nq_solve;
                result = solve(args.status, args.depth, args.specialist);
                break;
            case 'reflect_solve':
                result = reflect_solve(args.status, args.useObserve);
                break;
            case 'raphael_solve':
                result = raphael_solve(
                    args.status,
                    args.targetQuality,
                    args.useManipulation,
                    args.useHeartAndSoul,
                    args.useQuickInnovation,
                    args.useTrainedEye,
                    args.backloadProgress,
                    args.adversarial,
                );
        }
        postMessage(result);
    } catch (e: any) {
        postMessage({ error: String(e) });
    } finally {
        close();
    }
};
