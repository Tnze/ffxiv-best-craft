import E1 from './e1';
import E2 from './e2';
import { computed } from 'vue';

export interface EasterEgg {
    c(): boolean;
    t0: string;
    t1: string;
    t3?: string;
}

const allEggs: EasterEgg[] = [E1, E2];

export const activeEggs = computed(() => allEggs.filter(e => e.c()));

export const developerText = computed(() => {
    for (const egg of allEggs) {
        if (egg.c() && egg.t3) return egg.t3;
    }
    return 'Tnze';
});
