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

import { createRouter, createWebHashHistory } from 'vue-router';

const Welcome = () => import('./pages/Welcome.vue');
const GearsetsVue = () => import('./pages/Gearsets.vue');
const RecipePanelVue = () =>
    import('./components/recipe-manager/RecipePanel.vue');
const CustomizeRecipe = () =>
    import('./components/recipe-manager/CustomizeRecipe.vue');
const FavoritesVue = () => 
    import('./components/recipe-manager/FavoriteRecipePanel.vue');
const DesignerVue = () => import('./components/designer/Page.vue');
const BOMVue = () => import('./pages/Bom.vue');
const SettingsVue = () => import('./pages/Settings.vue');

const router = createRouter({
    history: createWebHashHistory(),
    routes: [
        { path: '/', redirect: '/welcome' },
        { path: '/welcome', component: Welcome },
        { path: '/gearsets', name: 'gearsets', component: GearsetsVue },
        { path: '/recipe', name: 'recipe', component: RecipePanelVue },
        { path: '/favorites', name: 'favorites', component: FavoritesVue },
        {
            path: '/recipe/customize',
            name: 'customize-recipe',
            component: CustomizeRecipe,
        },
        { path: '/designer', name: 'designer', component: DesignerVue },
        { path: '/bom', name: 'bom', component: BOMVue },
        { path: '/settings', name: 'settings', component: SettingsVue },
    ],
});

export { router };
