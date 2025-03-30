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

export function calculateEnhancedAttributsAddon(
    attributes: Attributes,
    ...enhancers: Enhancer[]
): { cm: number; ct: number; cp: number } {
    let cm = 0,
        ct = 0,
        cp = 0;
    for (const enh of enhancers) {
        if (enh.cm && enh.cm_max) {
            cm += Math.floor(
                Math.min((attributes.craftsmanship * enh.cm) / 100, enh.cm_max),
            );
        }
        if (enh.ct && enh.ct_max) {
            ct += Math.floor(
                Math.min((attributes.control * enh.ct) / 100, enh.ct_max),
            );
        }
        if (enh.cp && enh.cp_max) {
            cp += Math.floor(
                Math.min((attributes.craft_points * enh.cp) / 100, enh.cp_max),
            );
        }
    }
    return { cm, ct, cp };
}

export function calculateEnhancedAttributsAbs(
    attributes: Attributes,
    ...enhancers: Enhancer[]
): Attributes {
    const { cm, ct, cp } = calculateEnhancedAttributsAddon(
        attributes,
        ...enhancers,
    );
    return {
        level: attributes.level,
        craftsmanship: attributes.craftsmanship + cm,
        control: attributes.control + ct,
        craft_points: attributes.craft_points + cp,
    };
}
