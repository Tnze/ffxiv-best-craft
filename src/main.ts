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

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { fluent } from './fluent'
import { ElLoading } from "element-plus";
import { router } from './router'

import 'element-plus/theme-chalk/dark/css-vars.css'
import 'element-plus/es/components/message/style/css'
import 'element-plus/es/components/message-box/style/css'

import App from './App.vue'

document.body.addEventListener('touchmove', event => event.preventDefault(), { passive: false })

const pinia = createPinia()

createApp(App)
    .directive('tnze-loading', ElLoading.directive)
    .use(pinia)
    .use(router)
    .use(fluent)
    .mount('#app')
