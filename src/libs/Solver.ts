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
import { invoke } from "@tauri-apps/api/tauri";

export const create_solver = (
    status: Status,
    useMuscleMemory: boolean,
    useManipulation: boolean,
    useObserve: boolean,
) => {
    return invoke("create_solver", {
        status,
        useMuscleMemory,
        useManipulation,
        useObserve,
    });
};

export const destroy_solver = (status: Status) => {
    return invoke("destroy_solver", { status });
};

export const read_solver = (status: Status): Promise<Actions[]> => {
    return invoke("read_solver", { status });
};

export const rika_solve = (status: Status): Promise<Actions[]> => {
    return invoke("rika_solve", { status })
}

export const rika_solve_tnzever = (status: Status, useManipulation: boolean, useWastNot: number, useObserve: boolean, reduceSteps: boolean): Promise<Actions[]> => {
    return invoke("rika_solve_tnzever", { status, useManipulation, useWastNot, useObserve, reduceSteps })
}

export const dfs_solve = (status: Status, depth: number, specialist: boolean): Promise<Actions[]> => {
    return invoke("dfs_solve", { status, depth, specialist })
}

export const nq_solve = (status: Status): Promise<Actions[]> => {
    return invoke("nq_solve", { status })
}

export const reflect_solve = (status: Status, useManipulation: boolean): Promise<Actions[]> => {
    return invoke("reflect_solve", { status, useManipulation })
}

export const formatDuration = (u: number): string => {
    if (u < 1000) {
        return u + "ms"
    } else {
        const h = Math.floor(u / 1000 / 3600)
        const m = Math.floor(u / 1000 / 60) - h * 60
        const s = (u / 1000 - h * 3600 - m * 60).toFixed(3)
        return (h > 0 ? h + 'h' : '') + (m > 0 ? m + 'm' : '') + (s + 's')
    }
}