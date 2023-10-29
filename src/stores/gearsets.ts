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

import { Jobs, Attributes } from "@/libs/Craft"
import { defineStore } from "pinia"

export interface GearsetsRow {
    name: Jobs
    value: Attributes | null
}

export default defineStore('gearsets', {
    state: () => ({
        default: {
            level: 90,
            craftsmanship: 2786,
            control: 2764,
            craft_points: 533,
        },
        special: [
            { name: Jobs.Carpenter, value: null },
            { name: Jobs.Blacksmith, value: null },
            { name: Jobs.Armorer, value: null },
            { name: Jobs.Goldsmith, value: null },
            { name: Jobs.Leatherworker, value: null },
            { name: Jobs.Weaver, value: null },
            { name: Jobs.Alchemist, value: null },
            { name: Jobs.Culinarian, value: null },
        ] as GearsetsRow[]
    }),
    getters: {
        toJson(): string {
            return JSON.stringify({
                default: this.default,
                special: this.special
            })
        }
    },
    actions: {
        fromJson(json: string) {
            let v = JSON.parse(json)
            this.default = v.default
            this.special = v.special
        },
    },
})