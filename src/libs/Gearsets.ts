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

import { JSONSchemaType } from 'ajv';
import { Attributes, AttributesSchema, Jobs, JobsSchema } from './Craft';
import { fluent } from '@/fluent';

const $t = fluent.format;

export interface GearsetsRow {
    id: number;
    name?: string;
    value: Attributes;
    compatibleJobs: Jobs[];
}

export const GearsetsRowSchema: JSONSchemaType<GearsetsRow> = {
    type: 'object',
    properties: {
        id: { type: 'number' },
        name: { type: 'string', nullable: true },
        value: AttributesSchema,
        compatibleJobs: { type: 'array', items: JobsSchema, uniqueItems: true },
    },
    required: ['id', 'value', 'compatibleJobs'],
};

export const DEFAULT_ATTRIBUTS = {
    level: 100,
    craftsmanship: 4628,
    control: 4221,
    craft_points: 533,
};

const jobList = Object.values(Jobs).map(job => job as string);
export function choiceGearsetDisplayName(row: GearsetsRow): string {
    if (row.id == 0) {
        return $t('default');
    }
    // If this row has a name, just use the name
    if (row.name != undefined && row.name.length > 0) {
        return jobList.includes(row.name) ? $t(row.name) : row.name;
    }
    // Choice name from compatibleJobs
    if (row.compatibleJobs != undefined) {
        switch (row.compatibleJobs.length) {
            case 1:
                return $t(row.compatibleJobs[0]);
            case 8:
                return $t('common');
        }
    }
    return $t('custom', { id: row.id });
}
