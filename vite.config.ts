// This file is part of BestCraft.
// Copyright (C) 2025 Tnze
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

import { PluginOption, defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import ElementPlus from 'unplugin-element-plus/vite';
import {
    ExternalFluentPlugin,
    SFCFluentPlugin,
} from 'unplugin-fluent-vue/vite';
import wasm from 'vite-plugin-wasm';
import { resolve } from 'path';
import oxlintPlugin from 'vite-plugin-oxlint';

const projectRootDir = resolve(__dirname);

const defineTarget = (): PluginOption => {
    let config: { env: any };
    return {
        name: 'define target',
        configResolved(resolvedConfig) {
            config = resolvedConfig;
        },
        transform(code: string, _id: string) {
            const beReplaced = 'import.meta.env.VITE_BESTCRAFT_TARGET';
            if (code.includes(beReplaced)) {
                const target = config.env.VITE_BESTCRAFT_TARGET;
                const replacement = JSON.stringify(target);
                return {
                    code: code.replace(beReplaced, replacement),
                    map: this.getCombinedSourcemap(),
                };
            }
            return undefined;
        },
        enforce: 'pre',
    };
};

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        defineTarget(),
        wasm(),
        vue(),
        ElementPlus({}),
        SFCFluentPlugin(),
        ExternalFluentPlugin({
            locales: ['zh-CN'], // required - list of locales
            baseDir: 'src', // base directory for Vue files
            ftlDir: 'src/assets/locales', // directory with ftl files
        }),
        oxlintPlugin()
    ],
    resolve: {
        alias: [{ find: '@', replacement: resolve(projectRootDir, 'src') }],
    },
    worker: {
        format: 'es',
        plugins: () => [defineTarget(), wasm()],
    },
    build: {
        rollupOptions: {
            input: {
                main: resolve(__dirname, 'index.html'),
                fco: resolve(__dirname, 'fco.html'),
            },
        },
        // Disable sourcemap for Tauri target
        sourcemap: process.env.VITE_BESTCRAFT_TARGET != 'tauri',
        target: 'esnext',
    },
});
