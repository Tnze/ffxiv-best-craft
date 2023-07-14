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

const Guide = () => import('./components/guide/Guide.vue')
const Welcome = () => import('./components/guide/Welcome.vue')
const SeeRecipe = () => import('./components/guide/SeeRecipe.vue')
const Solving = () => import('./components/guide/Solving.vue')
const GearsetsVue = () => import('./components/Gearsets.vue')
const RecipePanelVue = () => import('./components/recipe-manager/RecipePanel.vue')
const CustomizeRecipe = () => import('./components/recipe-manager/CustomizeRecipe.vue')
const BillOfMaterial = () => import('./components/bill-of-material/BillOfMaterial.vue')
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
                { path: 'solving', name: 'solving', component: Solving },
            ]
        },
        { path: '/gearsets', component: GearsetsVue },
        { path: '/recipe', component: RecipePanelVue },
        { path: '/recipe/customize', component: CustomizeRecipe },
        { path: '/bom', component: BillOfMaterial },
        { path: '/designer', name: 'designer', component: DesignerVue },
        { path: '/settings', component: SettingsVue },
    ]
})

createApp(App).directive('tnze-loading', ElLoading.directive).use(pinia).use(router).use(fluent).mount('#app')
