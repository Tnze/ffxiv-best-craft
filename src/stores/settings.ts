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

import { defineStore } from 'pinia'
import { CafeMakerApiBase, XivApiRecipeSource, XivapiBase } from '@/components/recipe-manager/remote-source'
import { DataSource } from '@/components/recipe-manager/source'
import { YYYYGamesApiBase, WebSource } from '@/components/recipe-manager/web-source'


export default defineStore('settings', {
    state: () => ({
        language: 'system',
        dataSource: <'local' | "yyyy.games" | 'xivapi' | 'cafe'>(import.meta.env.VITE_BESTCRAFT_TARGET == "tauri" ? 'local' : 'yyyy.games'),
        dataSourceLang: <'en' | 'ja' | 'de' | 'fr' | undefined>undefined
    }),
    getters: {
        toJson(): string {
            return JSON.stringify({
                language: this.language,
            })
        },
        async getDataSource(): Promise<DataSource> {
            let dataSources: Record<string, () => DataSource> = {
                'yyyy.games': () => new WebSource(YYYYGamesApiBase),
                'xivapi': () => new XivApiRecipeSource(XivapiBase, this.dataSourceLang),
                'cafe': () => new XivApiRecipeSource(CafeMakerApiBase),
            }
            let defaultSource: () => DataSource = dataSources['yyyy.games']
            if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
                var { LocalRecipeSource } = await import('../components/recipe-manager/local-source')
                let localSource = () => new LocalRecipeSource()
                dataSources['local'] = localSource
                defaultSource = localSource
            }

            return (dataSources[this.dataSource] ?? defaultSource)()
        }
    },
    actions: {
        loadSettings(localSettings: any) {
            this.$patch(localSettings)
            this.language = localSettings.language
            this.dataSource = localSettings.dataSource
            if (localSettings.dataSourceLang)
                this.dataSourceLang = localSettings.dataSourceLang
            else {
                if (this.language.startsWith('en')) {
                    this.dataSourceLang = 'en'
                } else if (this.language.startsWith('ja')) {
                    this.dataSourceLang = 'ja'
                }
            }
        },
        fromJson(json: string) {
            this.$patch(JSON.parse(json))
            if (this.dataSource === "xivapi") {
                this.dataSource = "yyyy.games"
            }
        },
    }
})
