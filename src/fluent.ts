import { FluentBundle } from '@fluent/bundle'
import { createFluentVue } from 'fluent-vue';
import zhCNMessages from './assets/locales/zh-CN.ftl'
import enMessages from './assets/locales/en.ftl'
import jaMessages from './assets/locales/ja.ftl'

const zhCNBundle = new FluentBundle('zh-CN')
zhCNBundle.addResource(zhCNMessages)

const enBundle = new FluentBundle('en')
enBundle.addResource(enMessages)

const jaBundle = new FluentBundle('ja')
jaBundle.addResource(jaMessages)

export const fluent = createFluentVue({ bundles: [zhCNBundle, enBundle, jaBundle] })

export function selectLanguage(newLang: string) {
    switch (newLang) {
        case 'zh-CN':
            fluent.bundles = [zhCNBundle, enBundle, jaBundle]
            break;
        case 'en':
            fluent.bundles = [enBundle, jaBundle, zhCNBundle]
            break;
        case 'ja':
            fluent.bundles = [jaBundle, enBundle, zhCNBundle]
            break;
    }
}