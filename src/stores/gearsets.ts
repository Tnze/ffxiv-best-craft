// This file is part of BestCraft.
// Copyright (C) 2025 Tnze
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

import { Jobs } from '@/libs/Craft';
import { DEFAULT_ATTRIBUTS, GearsetsRow } from '@/libs/Gearsets';
import { defineStore } from 'pinia';

export default defineStore('gearsets', {
    state: () => ({ gearsets: <GearsetsRow[]>[] }),
    getters: {
        toJson(): string {
            return JSON.stringify({ gearsets: this.gearsets });
        },
        nextId(): number {
            const maxId = this.gearsets.reduce(
                (acc: number, v: GearsetsRow) => Math.max(acc, v.id),
                0,
            );
            return maxId + 1;
        },
        default(): GearsetsRow {
            return this.gearsets[0];
        },
    },
    actions: {
        fromJson(json: string) {
            this.gearsets.splice(0);
            const v = JSON.parse(json);

            if ('gearsets' in v) {
                this.gearsets = v.gearsets;
            } else {
                // Transport data from older version
                if ('default' in v) {
                    this.gearsets.push({
                        id: 0,
                        value: v.default,
                        compatibleJobs: Object.values(Jobs),
                    });
                }
                if ('special' in v) {
                    for (const s of v.special) {
                        // value == undefined means inherit from default
                        const value = s.value ?? v.default;
                        if (value == undefined) {
                            continue;
                        }

                        this.gearsets.push({
                            id: this.nextId,
                            value,
                            compatibleJobs: [s.name],
                        });
                    }
                }
            }
        },
        addGearset() {
            this.gearsets.push({
                id: this.nextId,
                value: { ...DEFAULT_ATTRIBUTS },
                compatibleJobs: Object.values(Jobs),
            });
        },
        delGearset(id: number) {
            const n = this.gearsets.findIndex(v => v.id == id);
            console.assert(n != -1, 'Removing unknown gearset id =', id);
            this.gearsets.splice(n, 1);
        },
    },
});
