import { createApp, ref } from 'vue'
import { createStore } from 'vuex'
import { createRouter, createWebHashHistory } from 'vue-router';

import 'element-plus/es/components/message/style/css'
import 'element-plus/es/components/message-box/style/css'
import 'element-plus/theme-chalk/dark/css-vars.css'

import { store, key } from './store'
import App from './App.vue'

const GearsetsVue = () => import('./components/Gearsets.vue')
const RecipePanelVue = () => import('./components/recipe-manager/RecipePanel.vue')
const BillOfMaterial = () => import('./components/BillOfMaterial.vue')
const DesignerVue = () => import('./components/designer/Page.vue')
const AutomationVue = () => import('./components/automation/Automation.vue')
const SettingsVue = () => import('./components/Settings.vue')

const routes = [
    { path: '/', redirect: '/gearsets' },
    { path: '/gearsets', component: GearsetsVue },
    { path: '/recipe', component: RecipePanelVue },
    { path: '/bom', component: BillOfMaterial },
    { path: '/designer', name: 'designer', component: DesignerVue },
    { path: '/automation', component: AutomationVue },
    { path: '/settings', component: SettingsVue },
]

const router = createRouter({
    history: createWebHashHistory(),
    routes
})

createApp(App).use(store, key).use(router).mount('#app')
