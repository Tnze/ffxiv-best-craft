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

const Guide = () => import('./components/guide/Guide.vue')
const Welcome = () => import('./components/guide/Welcome.vue')
const SeeRecipe = () => import('./components/guide/SeeRecipe.vue')

const GearsetsVue = () => import('./components/Gearsets.vue')
const RecipePanelVue = () => import('./components/recipe-manager/RecipePanel.vue')
const CustomizeRecipe = () => import('./components/recipe-manager/CustomizeRecipe.vue')
const DesignerVue = () => import('./components/designer/Page.vue')
const SettingsVue = () => import('./components/Settings.vue')

const router = createRouter({
    history: createWebHashHistory(),
    routes: [
        { path: '/', redirect: '/guide' },
        {
            path: '/guide', component: Guide, children: [
                { path: '', redirect: '/guide/welcome' },
                { path: 'welcome', name: 'welcome', component: Welcome },
                { path: 'see-recipe', name: 'see-recipe', component: SeeRecipe },
            ]
        },
        { path: '/gearsets', name: 'gearsets', component: GearsetsVue },
        { path: '/recipe', name: 'recipe', component: RecipePanelVue },
        { path: '/recipe/customize', name: 'customize-recipe', component: CustomizeRecipe },
        { path: '/designer', name: 'designer', component: DesignerVue },
        { path: '/settings', name: 'settings', component: SettingsVue },
    ]
})

export { router }