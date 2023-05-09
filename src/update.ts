import { checkUpdate as tauriCheckUpdate, installUpdate, onUpdaterEvent, } from '@tauri-apps/api/updater'
import { ElButton, ElMessage, ElMessageBox, ElNotification, ElText } from 'element-plus'
import { FluentVariable } from '@fluent/bundle';
import { h } from 'vue'

import { relaunch } from '@tauri-apps/api/process'

async function updateNow($t: (key: string, value?: Record<string, FluentVariable>) => string) {
    const msg1 = ElMessage({
        showClose: true,
        duration: 0,
        type: 'info',
        message: $t('update-installing'),
    })
    try {
        // Install the update. This will also restart the app on Windows!
        await installUpdate()
        // On macOS and Linux, restart the app manually.
        // (And we add another confirmation dialog)
        await ElMessageBox.confirm($t('update-ask-relaunch'))
        await relaunch()
    } catch (e) {
        // do nothing
        ElMessage({
            type: 'error',
            message: $t('update-error', { error: e as string }),
        })
    } finally {
        msg1.close()
    }
}

export const checkUpdate = async ($t: (key: string, value?: Record<string, FluentVariable>) => string, silent?: boolean) => {
    let privClose = () => { }
    const unlisten = await onUpdaterEvent(({ error, status }) => {
        if (silent) return;
        // This will log all updater events, including status updates and errors.
        privClose()
        switch (status) {
            case 'PENDING':
                privClose = ElMessage({
                    type: 'info',
                    message: $t('update-pending'),
                }).close
                break;
            case 'ERROR':
                privClose = ElMessage({
                    type: 'error',
                    message: $t('update-error', { error: error as string }),
                }).close
                break;
            case 'DONE':
                privClose = ElMessage({
                    type: 'success',
                    message: $t('update-done'),
                }).close
                break;
            case 'UPTODATE':
                privClose = ElMessage({
                    type: 'success',
                    message: $t('update-uptodate'),
                }).close
                break;
        }
    })

    try {
        const { shouldUpdate, manifest } = await tauriCheckUpdate()

        if (shouldUpdate) {
            // Show a dialog asking the user if they want to install the update here.
            let notification = ElNotification({
                type: 'info',
                position: 'bottom-right',
                duration: 0,
                title: $t('update-available', { version: manifest?.version || 'Unknown' }),
                message: h('div', [
                    h(ElText, { style: 'white-space: pre-wrap;' }, () => manifest?.body || ''),
                    h('div', {
                        style: 'margin-top: 5px; text-align: right',
                    }, [
                        h(ElButton, {
                            size: 'small',
                            type: 'primary',
                            onClick: () => {
                                notification.close()
                                updateNow($t)
                            }
                        }, () => $t('update-now')),
                        h(ElButton, {
                            size: 'small',
                            onClick: () => notification.close()
                        }, () => $t('update-not-now'))
                    ]),
                ])
            })

        }
    } catch (error) {
        console.error(error)
    }

    // you need to call unlisten if your handler goes out of scope, for example if the component is unmounted.
    unlisten()
}