import { createApp } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router';
import { createPinia } from 'pinia'
import { fluent } from './fluent'
import { ElLoading } from "element-plus";

import 'element-plus/theme-chalk/dark/css-vars.css'
import 'element-plus/es/components/message/style/css'
import 'element-plus/es/components/message-box/style/css'

import App from './App.vue'

const pinia = createPinia()

const GearsetsVue = () => import('./components/Gearsets.vue')
const RecipePanelVue = () => import('./components/recipe-manager/RecipePanel.vue')
const BillOfMaterial = () => import('./components/bill-of-material/BillOfMaterial.vue')
const DesignerVue = () => import('./components/designer/Page.vue')
const SettingsVue = () => import('./components/Settings.vue')

const router = createRouter({
    history: createWebHashHistory(),
    routes: [
        { path: '/', redirect: '/gearsets' },
        { path: '/gearsets', component: GearsetsVue },
        { path: '/recipe', component: RecipePanelVue },
        { path: '/bom', component: BillOfMaterial },
        { path: '/designer', name: 'designer', component: DesignerVue },
        { path: '/settings', component: SettingsVue },
    ]
})

createApp(App).directive('tnze-loading', ElLoading.directive).use(pinia).use(router).use(fluent).mount('#app')
