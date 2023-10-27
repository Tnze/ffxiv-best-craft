import { PluginOption, defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import ElementPlus from 'unplugin-element-plus/vite'
import {
  ExternalFluentPlugin,
  SFCFluentPlugin,
} from 'unplugin-fluent-vue/vite'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

const defineTarget = () => {
  let config: { env: any; }
  return <PluginOption>{
    name: 'define target',
    configResolved(resolvedConfig) {
      config = resolvedConfig
    },
    transform(code: string, _id: any) {
      const beReplaced = "import.meta.env.VITE_BESTCRAFT_TARGET"
      if (code.includes(beReplaced)) {
        const target = config.env.VITE_BESTCRAFT_TARGET
        const replacement = JSON.stringify(target)
        return code.replace(beReplaced, replacement);
      }
      return undefined;
    },
    enforce: "pre",
  }
}

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    defineTarget(),
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
  ]
})
