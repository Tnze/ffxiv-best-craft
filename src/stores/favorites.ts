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

import { defineStore } from 'pinia';

type FavoritesMap = Record<number, boolean>;

const NUMBER_PER_PAGE = 200;

export default defineStore('favorites', {
    state: () => {
        const isFavoriteMap: FavoritesMap = {};
        return { isFavoriteMap };
    },
    getters: {
        toJson(): string {
            return JSON.stringify(this.isFavoriteMap);
        },
        favoriteIds(): number[] {
            return Object.keys(this.isFavoriteMap).map(id => parseInt(id)).filter(id => this.isFavoriteMap[id]);
        },
        pageTotal(): number {
            return Math.ceil(this.favoriteIds.length / NUMBER_PER_PAGE);
        },
    },
    actions: {
        fromJson(json: string) {
            const v = JSON.parse(json);
            this.isFavoriteMap = v || {};
        },
        clickStar(recipeId: number) {
            if (this.isFavoriteMap[recipeId]) {
                delete this.isFavoriteMap[recipeId];
            } else {
                this.isFavoriteMap[recipeId] = true;
            }
        },
        getFavoriteIdsByPage(page: number): number[] {
            const start = (page - 1) * NUMBER_PER_PAGE;
            return this.favoriteIds.slice(start, start + NUMBER_PER_PAGE);
        }
    },
});
