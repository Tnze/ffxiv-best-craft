import { FluentBundle } from '@fluent/bundle'
import { createFluentVue } from 'fluent-vue';
import zhCNMessages from './assets/locales/zh-CN.ftl'
import enMessages from './assets/locales/en.ftl'

export const zhCNBundle = new FluentBundle('zh-CN')
zhCNBundle.addResource(zhCNMessages)

export const enBundle = new FluentBundle('en')
enBundle.addResource(enMessages)

export const fluent = createFluentVue({ bundles: [zhCNBundle, enBundle] })