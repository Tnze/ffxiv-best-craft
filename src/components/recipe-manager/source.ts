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

import { Item, ItemWithAmount, RecipeInfo, RecipeLevel } from "@/libs/Craft";

export interface DataSource {
    sourceType: DataSourceType
    recipeTable(page: number, searchName?: string, rlv?: number, craftTypeId?: number): Promise<RecipesSourceResult>
    recipesIngredients(recipeId: number): Promise<ItemWithAmount[]>
    recipeLevelTable(rlv: number): Promise<RecipeLevel>
    recipeInfo?(recipeId: number): Promise<RecipeInfo>
    itemInfo(id: number): Promise<Item>
    craftTypeList(): Promise<CraftType[]>
}

export interface CraftType {
    id: number,
    name: string,
}

export enum DataSourceType {
    Realtime, RemoteRealtime, SingleShot
}

export interface RecipesSourceResult {
    recipes: RecipeInfo[],
    totalPages: number
}
