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

onmessage = async (e) => {
    if (import.meta.env.VITE_BESTCRAFT_TARGET == "web") {
        var { rand_simulation, calc_attributes_scope } = await import("../../pkg-wasm/app_wasm")
    } else return
    const { name, args: argsJson } = e.data
    const args = JSON.parse(argsJson)
    try {
        switch (name) {
            case "rand_simulation":
                postMessage(rand_simulation(args.status, args.actions, args.n, args.ignoreErrors))
                break
            case "calc_attributes_scope":
                postMessage(calc_attributes_scope(args.status, args.actions))
        }
    } catch (e) {
        throw e
    }
    close()
};
