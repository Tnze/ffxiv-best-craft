// This file is part of BestCraft.
// Copyright (C) 2023 Tnze
//
// BestCraft is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// BestCraft is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

import { PluginOption, defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import ElementPlus from 'unplugin-element-plus/vite'
import {
  ExternalFluentPlugin,
  SFCFluentPlugin,
} from 'unplugin-fluent-vue/vite'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { resolve } from 'path';

const projectRootDir = resolve(__dirname);

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
  ],
  resolve: {
    alias: [
      { find: "@", replacement: resolve(projectRootDir, 'src') }
    ]
  },
  worker: {
    plugins: [
      wasm(),
      topLevelAwait(),
    ]
  }
})
