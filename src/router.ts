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
        { path: '/gearsets', component: GearsetsVue },
        { path: '/recipe', component: RecipePanelVue },
        { path: '/recipe/customize', component: CustomizeRecipe },
        { path: '/designer', name: 'designer', component: DesignerVue },
        { path: '/settings', component: SettingsVue },
    ]
})

export { router }