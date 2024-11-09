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
    Actions,
    Attributes,
    Jobs,
    Recipe,
    RecipeInfo,
    RecipeLevel,
} from '@/libs/Craft';
import { defineStore } from 'pinia';
import useGearsetsStore from '@/stores/gearsets';

export default defineStore('fco-simulator', {
    state: () => ({
        job: Jobs.Alchemist,
        recipe: <
            | {
                  recipe: Recipe;
                  recipeLevel: RecipeLevel;
                  recipeInfo: RecipeInfo;
              }
            | undefined
        >undefined,
        attributes: <Attributes | undefined>(
            useGearsetsStore().attributes(Jobs.Alchemist)
        ),
        rotation: <Actions[]>[],
    }),
});
