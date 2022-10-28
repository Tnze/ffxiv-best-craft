import { createApp } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router';
import { store, key } from './store'
import { FluentBundle } from '@fluent/bundle'
import { createFluentVue } from 'fluent-vue';


import 'element-plus/es/components/message/style/css'
import 'element-plus/es/components/message-box/style/css'
import 'element-plus/theme-chalk/dark/css-vars.css'

import zhCNMessages from './assets/locales/zh-CN.ftl'
import App from './App.vue'

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


const zhCNBundle = new FluentBundle('zh-CN')
zhCNBundle.addResource(zhCNMessages)

const fluent = createFluentVue({ bundles: [zhCNBundle] })

createApp(App).use(store, key).use(router).use(fluent).mount('#app')
