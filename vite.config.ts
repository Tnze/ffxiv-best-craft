import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import ElementPlus from 'unplugin-element-plus/vite'
import {
  ExternalFluentPlugin,
  SFCFluentPlugin,
} from 'unplugin-fluent-vue/vite'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait(),
    vue(),
    ElementPlus({}),
    SFCFluentPlugin(),
    ExternalFluentPlugin({
      locales: ['zh-CN'], // required - list of locales
      baseDir: 'src', // base directory for Vue files
      ftlDir: 'src/assets/locales', // directory with ftl files
    }),
  ],
})
