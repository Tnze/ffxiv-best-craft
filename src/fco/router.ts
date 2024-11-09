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

import { createRouter, createWebHashHistory } from 'vue-router';

const Simulator = () => import('./pages/Simulator.vue');
const CrafterAttributes = () => import('./pages/CrafterAttributes.vue');

const router = createRouter({
    history: createWebHashHistory(),
    routes: [
        { path: '/', redirect: '/simulator' },
        { path: '/simulator', name: 'simulator', component: Simulator },
        {
            path: '/crafter-attributes',
            name: 'attributes',
            component: CrafterAttributes,
        },
    ],
});

export { router };
