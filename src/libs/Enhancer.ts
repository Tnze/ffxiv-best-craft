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

import { Attributes } from './Craft';

export interface Enhancer {
    name: string;
    level?: number;
    is_hq?: boolean;

    cm?: number;
    cm_max?: number;
    ct?: number;
    ct_max?: number;
    cp?: number;
    cp_max?: number;
}

export function calculateEnhancedAttributs(
    attributes: Attributes,
    ...enhancers: Enhancer[]
): [Attributes, { cm: number; ct: number; cp: number }] {
    const sum = (prev: number, curr: number) => prev + curr;
    const cm = enhancers
        .filter(v => v.cm && v.cm_max)
        .map(v =>
            Math.floor(
                Math.min((attributes.craftsmanship * v.cm!) / 100, v.cm_max!),
            ),
        )
        .reduce(sum, 0);
    const ct = enhancers
        .filter(v => v.ct && v.ct_max)
        .map(v =>
            Math.floor(Math.min((attributes.control * v.ct!) / 100, v.ct_max!)),
        )
        .reduce(sum, 0);
    const cp = enhancers
        .filter(v => v.cp && v.cp_max)
        .map(v =>
            Math.floor(
                Math.min((attributes.craft_points * v.cp!) / 100, v.cp_max!),
            ),
        )
        .reduce(sum, 0);

    return [
        {
            level: attributes.level,
            craftsmanship: attributes.craftsmanship + cm,
            control: attributes.control + ct,
            craft_points: attributes.craft_points + cp,
        },
        { cm, ct, cp },
    ];
}
