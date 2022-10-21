import { createApp } from 'vue'
import App from './App.vue'
import 'element-plus/es/components/message/style/css'
import 'element-plus/es/components/message-box/style/css'
import 'element-plus/theme-chalk/dark/css-vars.css'

import { createRouter, createWebHashHistory } from 'vue-router';
import DesignerVue from './components/designer/Designer.vue';
import GearsetsVue from './components/Gearsets.vue';
import RecipePanelVue from './components/recipe-manager/RecipePanel.vue';
import AutomationVue from './components/automation/Automation.vue'
import SettingsVue from './components/Settings.vue';

const routes = [
    { path: '/gearsets', component: GearsetsVue },
    { path: '/recipe', component: RecipePanelVue },
    { path: '/designer', component: DesignerVue },
    { path: '/automation', component: AutomationVue },
    { path: '/settings', component: SettingsVue },
]

const router = createRouter({
    history: createWebHashHistory(),
    routes
})

createApp(App).use(router).mount('#app')
