<script setup lang="ts">
import { ElLink } from 'element-plus';
import { computed } from 'vue';
import { useFluent } from 'fluent-vue';

const props = defineProps<{ version: string }>();
const emits = defineEmits<{ click: [url: string] }>();
const { $t } = useFluent();

const link = computed(() => {
    const details = [
        $t('detail-website', { site: location.href }),
        $t('detail-enviroment', { env: navigator.userAgent }),
    ];
    if (props.version) {
        details.push($t('detail-version', { version: props.version }));
    }
    const body = $t('detail-description') + '\n\n' + details.join('\n');
    return `mailto:bugreport@tnze.tech?subject=${encodeURIComponent($t('subject'))}&body=${encodeURIComponent(body)}`;
});
</script>

<template>
    <el-link @click="emits('click', link)">{{ $t('email') }}</el-link>
</template>

<fluent locale="zh-CN">
email = 电子邮件
subject = BestCraft 问题反馈：<请在此处简要描述遇到的问题>
detail-description = 问题描述：<请在此处详细描述遇到的问题、操作步骤、提供屏幕截图等>
detail-enviroment = 运行环境：{ $env }
detail-website = 当前页面：{ $site }
detail-version = 当前版本：{ $version }
</fluent>

<fluent locale="zh-TW">
email = 電子郵件
subject = BestCraft 問題回報：<請在此處簡要描述遇到的問題>
detail-description = 問題描述：<請在此處詳細描述遇到的問題、操作步驟、提供螢幕截圖等>
detail-enviroment = 執行環境：{ $env }
detail-website = 當前頁面：{ $site }
detail-version = 當前版本：{ $version }
</fluent>

<fluent locale="en-US">
email = E-mail
subject = BestCraft Bug Report: <SHORT PROBLEM DESCRIPTION>
detail-description = Description: <DESCRIPTION, OPERATION STEPS, SCREENSHOTS, etc>
detail-enviroment = Enviroments: { $env }
detail-website = Current Page: { $site }
detail-version = Current Version: { $version }
</fluent>

<fluent locale="ja-JP">
email = メール
subject = BestCraft バグ報告：<問題の簡単な説明をここに記入>
detail-description = 説明：<問題の詳細、操作手順、スクリーンショットなど>
detail-enviroment = 実行環境：{ $env }
detail-website = 現在のページ：{ $site }
detail-version = 現在のバージョン：{ $version }
</fluent>
