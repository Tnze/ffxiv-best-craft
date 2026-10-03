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

import {
    CollectablesShopRefine,
    Item,
    Jobs,
    Recipe,
    RecipeRequirements,
} from '@/libs/Craft';
import useDesignerStore from '@/stores/designer';

const designerStore = useDesignerStore();

export const craftTypeIdToJobMap: Record<number, Jobs> = {
    0: Jobs.Carpenter,
    1: Jobs.Blacksmith,
    2: Jobs.Armorer,
    3: Jobs.Goldsmith,
    4: Jobs.Leatherworker,
    5: Jobs.Weaver,
    6: Jobs.Alchemist,
    7: Jobs.Culinarian,
};

export function craftTypeIdToJob(
    craftTypeId: number | undefined,
): Jobs | undefined {
    return craftTypeId == undefined
        ? undefined
        : craftTypeIdToJobMap[craftTypeId];
}

export const selectRecipe = (
    recipe: Recipe,
    recipeId: number | undefined,
    materialQualityFactor: number,
    requirements: RecipeRequirements,
    collectability: CollectablesShopRefine | undefined,
    item: Item,
    craftTypeId: number | undefined,
    simulatorMode: boolean,
    stellarSteadyHandCount: number,
) => {
    designerStore.selectRecipe({
        job: craftTypeIdToJob(craftTypeId),
        item,
        recipe,
        recipeId,
        materialQualityFactor,
        requirements,
        collectability,
        simulatorMode,
        stellarSteadyHandCount,
    });
};
