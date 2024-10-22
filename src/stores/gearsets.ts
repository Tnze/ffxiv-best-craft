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
    value: Attributes | undefined
}

export default defineStore('gearsets', {
    state: () => ({
        default: {
            level: 100,
            craftsmanship: 4628,
            control: 4221,
            craft_points: 533,
        },
        special: [
            { name: Jobs.Carpenter, value: undefined },
            { name: Jobs.Blacksmith, value: undefined },
            { name: Jobs.Armorer, value: undefined },
            { name: Jobs.Goldsmith, value: undefined },
            { name: Jobs.Leatherworker, value: undefined },
            { name: Jobs.Weaver, value: undefined },
            { name: Jobs.Alchemist, value: undefined },
            { name: Jobs.Culinarian, value: undefined },
        ] as GearsetsRow[]
    }),
    getters: {
        toJson(): string {
            return JSON.stringify({
                default: this.default,
                special: this.special
            })
        },
        attributes() {
            return (job: Jobs) => this.special.find(v => v.name == job)?.value ?? this.default
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