import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import ElementPlus from 'unplugin-element-plus/vite'
import {
  ExternalFluentPlugin,
  SFCFluentPlugin,
} from 'unplugin-fluent-vue/vite'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    ElementPlus(undefined),
    SFCFluentPlugin(),
    ExternalFluentPlugin({
      locales: ['zh-CN'], // required - list of locales
      baseDir: 'src', // base directory for Vue files
      ftlDir: 'src/assets/locales', // directory with ftl files
    }),
  ],
})
