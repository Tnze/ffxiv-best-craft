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

import { Jobs, Attributes } from '@/libs/Craft';
import { defineStore } from 'pinia';
import type { TabPaneName } from 'element-plus'
import { fluent } from '../fluent';

export interface OldGearsetsRow {
    name: Jobs;
    value: Attributes | undefined;
}

// 原name拆分为id, ui展示, 所属职业三个功能
export interface GearsetsRow {
    name: string;
    label: string;
    job: Jobs;
    value: Attributes | undefined;
}

type jobUsingSet = {
    [key in Jobs]: string;
}

export const ifBasicGearset = (item: GearsetsRow): boolean => item.name === item.job;
export const labelWrapper = (item: GearsetsRow): string => {
    if (!item) return '';
    if (item.name === DEFAULT_EXTRA_ENTRIES.name) return fluent.format(DEFAULT_EXTRA_ENTRIES.label);
    return ifBasicGearset(item) ? fluent.format(item.label) : item.label
};

const MAX_SET_LENGTH = 20;

const DEFAULT_VAL = {
    level: 100,
    craftsmanship: 4628,
    control: 4221,
    craft_points: 533,
};

const DEFAULT_EXTRA_ENTRIES = {
    name: 'default',
    job: Jobs.Common,
    label: 'default-set-name',
};

export default defineStore('gearsets', {
    state: () => ({
        jobPage: DEFAULT_EXTRA_ENTRIES.name,
        special: [
            {...DEFAULT_EXTRA_ENTRIES, value: DEFAULT_VAL},
            { name: Jobs.Carpenter, job: Jobs.Carpenter, label: Jobs.Carpenter, value: undefined },
            { name: Jobs.Blacksmith, job: Jobs.Blacksmith, label: Jobs.Blacksmith, value: undefined },
            { name: Jobs.Armorer, job: Jobs.Armorer, label: Jobs.Armorer, value: undefined },
            { name: Jobs.Goldsmith, job: Jobs.Goldsmith, label: Jobs.Goldsmith, value: undefined },
            { name: Jobs.Leatherworker, job: Jobs.Leatherworker, label: Jobs.Leatherworker, value: undefined },
            { name: Jobs.Weaver, job: Jobs.Weaver, label: Jobs.Weaver, value: undefined },
            { name: Jobs.Alchemist, job: Jobs.Alchemist, label: Jobs.Alchemist, value: undefined },
            { name: Jobs.Culinarian, job: Jobs.Culinarian, label: Jobs.Culinarian, value: undefined },
        ] as GearsetsRow[],

        // the set name that the job is using
        jobUsingSet: {
            [Jobs.Carpenter]: Jobs.Carpenter,
            [Jobs.Blacksmith]: Jobs.Blacksmith,
            [Jobs.Armorer]: Jobs.Armorer,
            [Jobs.Goldsmith]: Jobs.Goldsmith,
            [Jobs.Leatherworker]: Jobs.Leatherworker,
            [Jobs.Weaver]: Jobs.Weaver,
            [Jobs.Alchemist]: Jobs.Alchemist,
            [Jobs.Culinarian]: Jobs.Culinarian,
            [Jobs.Common]: Jobs.Common
        } as jobUsingSet
    }),
    getters: {
        toJson(): string {
            return JSON.stringify({
                special: this.special,
                jobUsingSet: this.jobUsingSet
            });
        },
        defaultSet(): GearsetsRow { return this.special.find(item => item.name === DEFAULT_EXTRA_ENTRIES.name)!; },
        default(): Attributes { return this.defaultSet.value ?? DEFAULT_VAL; },
        attributes() {
            return (job: Jobs) =>
                this.getUsingSetData(job)?.value ?? this.defaultSet.value;
        },
        getUsingSetData() {
            return (job: Jobs) => this.special.find(v => v.name === this.jobUsingSet[job]);
        },
        getUsebleSets() {
            return (job: Jobs) => this.special.filter(v => v.job === job || v.job === Jobs.Common);
        },
        showAddSetButton(): boolean {
            return this.special.length <= MAX_SET_LENGTH;
        }
    },
    actions: {
        fromJson(json: string) {
            let v = JSON.parse(json);

            // Compatible with older versions
            v.special = v.special.map((singleSet: OldGearsetsRow | GearsetsRow): GearsetsRow => {
                if ('label' in singleSet && 'job' in singleSet) {
                    return singleSet;
                }
                return { ...singleSet, label: singleSet.name, job: singleSet.name };
            });

            if (v.default && !v.special.find((item: OldGearsetsRow | GearsetsRow) => item.name === DEFAULT_EXTRA_ENTRIES.name)) {
                v.special.unshift({ ...DEFAULT_EXTRA_ENTRIES, value: v.default });
            }
            this.special = v.special;
            v.jobUsingSet && (this.jobUsingSet = v.jobUsingSet);
        },
        addSet() {
            let defaultNameNum = 1;
            let nameToCheck = `${fluent.format('new-gearset')}(${defaultNameNum})`;
            const nameExists = (nameToCheck: string) => 
              this.special.find(item => item.name === nameToCheck);
          
            while (nameExists(nameToCheck)) {
                defaultNameNum++;
                nameToCheck = `${fluent.format('new-gearset')}(${defaultNameNum})`;
            }
          
            this.special.push({ name: nameToCheck, label: nameToCheck, job: Jobs.Common, value: undefined });
            this.jobPage = nameToCheck;
        },
        removeSet(targetName: TabPaneName): void {
            const idx = this.special.findIndex(item => item.name === targetName);
            const setToRemove = this.special[idx];
            if (idx < 0) return;
            this.special.splice(idx, 1);
            // when delete the using set
            if (this.jobUsingSet[setToRemove.job] === setToRemove.name) {
                const otherUsableSets = this.getUsebleSets(setToRemove.job);
                this.jobUsingSet[setToRemove.job] = otherUsableSets[0].name;
            }
            this.jobPage = DEFAULT_EXTRA_ENTRIES.name;
        },
        changeJobUsingSet(job: Jobs, name: string) {
            this.jobUsingSet[job] = name;
        }
    },
});
