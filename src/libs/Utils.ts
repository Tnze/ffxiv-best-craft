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

export function formatDuration(u: number, fixed?: number): string {
    if (u < 1000) {
        return u + 'ms';
    } else {
        const h = Math.floor(u / 1000 / 3600);
        const m = Math.floor(u / 1000 / 60) - h * 60;
        const s = (u / 1000 - h * 3600 - m * 60).toFixed(fixed ?? 3);
        return (h > 0 ? h + 'h' : '') + (m > 0 ? m + 'm' : '') + (s + 's');
    }
}

declare const window: { clarity: any } & Window & typeof globalThis;
export function clarityReport(event: string) {
    if (window.clarity) {
        window.clarity('event', event);
    }
}

export async function openUrl(url: string) {
    if (import.meta.env.VITE_BESTCRAFT_TARGET == 'tauri') {
        const { openUrl } = await import('@tauri-apps/plugin-opener');
        await openUrl(url);
    } else {
        window.open(url, '_blank');
    }
}
