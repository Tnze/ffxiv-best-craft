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

import { check, Update } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'
import { ElButton, ElMessage, ElMessageBox, ElNotification, NotificationHandle, MessageHandler } from 'element-plus'
import { FluentVariable } from '@fluent/bundle';
import { h } from 'vue'

async function updateNow($t: (key: string, value?: Record<string, FluentVariable>) => string, update: Update) {
    let close = () => { }
    try {
        await update.downloadAndInstall((downloadEvent) => {
            close()
            let messageHandler;
            switch (downloadEvent.event) {
                case 'Started':
                    messageHandler = ElMessage({
                        type: 'info',
                        message: $t('update-started', { contentLength: downloadEvent.data.contentLength || '' }),
                    })
                    close = messageHandler.close;
                    break;
                case 'Progress':
                    messageHandler = ElMessage({
                        showClose: true,
                        duration: 0,
                        type: 'info',
                        message: $t('update-progress', { chunkLength: downloadEvent.data.chunkLength }),
                    })
                    close = messageHandler.close;
                    break;
                case 'Finished':
                    messageHandler = ElMessage({
                        type: 'success',
                        message: $t('update-done'),
                    })
                    close = messageHandler.close;
                    break;
            }
        })
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
        close()
    }
}

export async function checkUpdate($t: (key: string, value?: Record<string, FluentVariable>) => string, silent: boolean) {
    try {
        const update = await check()

        if (update?.available) {
            // Show a dialog asking the user if they want to install the update here.
            let notification: NotificationHandle = ElNotification({
                type: 'info',
                position: 'bottom-right',
                duration: 0,
                title: $t('update-available', { version: update.version || 'Unknown' }),
                message: h('div', [
                    h('div', { style: 'white-space: pre-wrap;', innerHTML: update.body || '' }),
                    h('div', {
                        style: 'margin-top: 5px; text-align: right',
                    }, [
                        h(ElButton, {
                            size: 'small',
                            type: 'primary',
                            onClick: () => {
                                notification.close()
                                updateNow($t, update)
                            }
                        }, () => $t('update-now')),
                        h(ElButton, {
                            size: 'small',
                            onClick: () => notification.close()
                        }, () => $t('update-not-now'))
                    ]),
                ])
            })
        } else if (!silent) {
            ElMessage({
                type: 'success',
                message: $t('update-uptodate'),
            })
        }
    } catch (error) {
        console.error(error)
    }
}