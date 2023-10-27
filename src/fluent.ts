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

import { FluentBundle } from '@fluent/bundle'
import { createFluentVue } from 'fluent-vue';
import zhCNMessages from './assets/locales/zh-CN.ftl'
import enMessages from './assets/locales/en-US.ftl'
import jaMessages from './assets/locales/ja-JP.ftl'

const zhCNBundle = new FluentBundle('zh-CN')
zhCNBundle.addResource(zhCNMessages)

const enUSBundle = new FluentBundle('en-US')
enUSBundle.addResource(enMessages)

const jaJPBundle = new FluentBundle('ja-JP')
jaJPBundle.addResource(jaMessages)

export const fluent = createFluentVue({ bundles: [zhCNBundle, enUSBundle, jaJPBundle] })

export function selectLanguage(newLang: string) {
    switch (newLang) {
        case 'zh-CN':
            fluent.bundles = [zhCNBundle, enUSBundle, jaJPBundle]
            break;
        case 'en-US':
            fluent.bundles = [enUSBundle, jaJPBundle, zhCNBundle]
            break;
        case 'ja-JP':
            fluent.bundles = [jaJPBundle, enUSBundle, zhCNBundle]
            break;
    }
}