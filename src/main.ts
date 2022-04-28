import { createApp } from 'vue'
import App from './App.vue'
import init from "../src-wasm/pkg/best_craft";

await init()
createApp(App).mount('#app')
