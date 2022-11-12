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