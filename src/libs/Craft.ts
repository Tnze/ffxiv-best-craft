// This file is part of BestCraft.
// Copyright (C) 2024 Tnze
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

import { isTauri } from "./Consts";

if (import.meta.env.VITE_BESTCRAFT_TARGET == "tauri") {
    var pkgTauri = import("@tauri-apps/api/core")
} else {
    var pkgWasm = import("@/../pkg-wasm/app_wasm")
}

export interface Attributes {
    level: number;
    craftsmanship: number;
    control: number;
    craft_points: number;
}

export interface Item {
    id: number,
    name: string,
    level: number,
    can_be_hq: boolean,
    category_id?: number,
}

export interface Recipe {
    rlv: number;
    job_level: number;
    difficulty: number;
    quality: number;
    durability: number;
    conditions_flag: number;
}

export interface RecipeLevel {
    class_job_level: number,
    stars: number,
    suggested_craftsmanship: number | null,
    suggested_control: number | null,
    difficulty: number,
    quality: number,
    progress_divider: number,
    quality_divider: number,
    progress_modifier: number,
    quality_modifier: number,
    durability: number,
    conditions_flag: number,
}

export interface Buffs {
    muscle_memory: number;
    great_strides: number;
    veneration: number;
    innovation: number;
    inner_quiet: number;
    final_appraisal: number;
    manipulation: number;
    wast_not: number;
    heart_and_soul: LimitedActionState;
    trained_perfection: LimitedActionState;
    careful_observation_used: number;
    quick_innovation_used: number;
    touch_combo_stage: number;
    observed: number;
    expedience: number;
}

export interface Status {
    buffs: Buffs;
    attributes: Attributes;
    recipe: Recipe;
    catches: any;
    durability: number;
    craft_points: number;
    progress: number;
    quality: number;
    step: number;
    condition: Conditions;
}

export enum LimitedActionState {
    Unused = "Unused",
    Active = "Active",
    Used = "Used",
}

export const compareStatus = (s1: Status, s2: Status): number => {
    if (s1.progress != s2.progress)
        return s1.progress - s2.progress;
    if (s1.quality != s2.quality)
        return s1.quality - s2.quality;
    if (s1.step != s2.step)
        return s2.step - s1.step;
    return 0
}

export enum Conditions {
    // 白：通常
    Normal = 'Normal',
    // 红：高品质，加工效率1.5倍
    Good = 'Good',
    // 彩：最高品质
    Excellent = 'Excellent',
    // 黑：低品质
    Poor = 'Poor',

    // 黄：成功率增加 25%
    Centered = 'Centered',
    // 蓝：耐久消耗降低 50%, 效果可与俭约叠加
    Sturdy = 'Sturdy',
    // 绿：CP 消耗减少 50%
    Pliant = 'Pliant',
    // 深蓝：作业效率1.5倍
    Malleable = 'Malleable',
    // 紫：技能效果持续增加两回合
    Primed = 'Primed',
    // 粉：下一回合必定是红球
    GoodOmen = "GoodOmen",
}

export enum Jobs {
    Carpenter = "carpenter",
    Blacksmith = "blacksmith",
    Armorer = "armorer",
    Goldsmith = "goldsmith",
    Leatherworker = "leatherworker",
    Weaver = "weaver",
    Alchemist = "alchemist",
    Culinarian = "culinarian",
}

export enum Actions {
    BasicSynthesis = "basic_synthesis",
    BasicTouch = "basic_touch",
    MastersMend = "masters_mend",
    HastyTouch = "hasty_touch",
    RapidSynthesis = "rapid_synthesis",
    Observe = "observe",
    TricksOfTheTrade = "tricks_of_the_trade",
    WasteNot = "waste_not",
    Veneration = "veneration",
    StandardTouch = "standard_touch",
    GreatStrides = "great_strides",
    Innovation = "innovation",
    FinalAppraisal = "final_appraisal",
    WasteNotII = "waste_not_ii",
    ByregotsBlessing = "byregot_s_blessing",
    PreciseTouch = "precise_touch",
    MuscleMemory = "muscle_memory",
    CarefulSynthesis = "careful_synthesis",
    Manipulation = "manipulation",
    PrudentTouch = "prudent_touch",
    Reflect = "reflect",
    PreparatoryTouch = "preparatory_touch",
    Groundwork = "groundwork",
    DelicateSynthesis = "delicate_synthesis",
    IntensiveSynthesis = "intensive_synthesis",
    TrainedEye = "trained_eye",
    AdvancedTouch = "advanced_touch",
    PrudentSynthesis = "prudent_synthesis",
    TrainedFinesse = "trained_finesse",
    CarefulObservation = "careful_observation",
    HeartAndSoul = "heart_and_soul",
    // 7.0
    RefinedTouch = "refined_touch",
    DaringTouch = "daring_touch",
    ImmaculateMend = "immaculate_mend",
    QuickInnovation = "quick_innovation",
    TrainedPerfection = "trained_perfection",
    // fake skills
    RapidSynthesisFail = "rapid_synthesis_fail",
    HastyTouchFail = "hasty_touch_fail",
    DaringTouchFail = "daring_touch_fail",
}

const waitTimes = new Map([
    [Actions.BasicSynthesis, 3],
    [Actions.BasicTouch, 3],
    [Actions.MastersMend, 3],
    [Actions.HastyTouch, 3],
    [Actions.RapidSynthesis, 3],
    [Actions.Observe, 3],
    [Actions.TricksOfTheTrade, 3],
    [Actions.WasteNot, 2],
    [Actions.Veneration, 2],
    [Actions.StandardTouch, 3],
    [Actions.GreatStrides, 2],
    [Actions.Innovation, 2],
    [Actions.FinalAppraisal, 2],
    [Actions.WasteNotII, 2],
    [Actions.ByregotsBlessing, 3],
    [Actions.PreciseTouch, 3],
    [Actions.MuscleMemory, 3],
    [Actions.CarefulSynthesis, 3],
    [Actions.Manipulation, 2],
    [Actions.PrudentTouch, 3],
    [Actions.Reflect, 3],
    [Actions.PreparatoryTouch, 3],
    [Actions.Groundwork, 3],
    [Actions.DelicateSynthesis, 3],
    [Actions.IntensiveSynthesis, 3],
    [Actions.TrainedEye, 3],
    [Actions.AdvancedTouch, 3],
    [Actions.PrudentSynthesis, 3],
    [Actions.TrainedFinesse, 3],
    [Actions.CarefulObservation, 3],
    [Actions.HeartAndSoul, 3],
    [Actions.RefinedTouch, 3],
    [Actions.DaringTouch, 3],
    [Actions.ImmaculateMend, 3],
    [Actions.QuickInnovation, 3],
    [Actions.TrainedPerfection, 3],
])

const postCastDelay = new Map()
for (const [action, time] of waitTimes.entries()) {
    postCastDelay.set(action, time + 0.5)
}

export const newRecipe = async (
    rlv: number,
    rt: RecipeLevel,
    difficultyFactor: number,
    qualityFactor: number,
    durabilityFactor: number,
): Promise<Recipe> => {
    return {
        rlv,
        job_level: rt.class_job_level,
        difficulty: Math.floor(rt.difficulty * difficultyFactor / 100),
        quality: Math.floor(rt.quality * qualityFactor / 100),
        durability: Math.floor(rt.durability * durabilityFactor / 100),
        conditions_flag: rt.conditions_flag,
    };
};

export async function newStatus(
    attrs: Attributes,
    recipe: Recipe,
    recipeLevel: RecipeLevel,
): Promise<Status> {
    if (isTauri) {
        let { invoke } = await pkgTauri
        return invoke("new_status", { attrs, recipe, recipeLevel })
    } else {
        let { new_status } = await pkgWasm
        return new_status(attrs, recipe, recipeLevel)
    }
};

export interface SimulateResult {
    status: Status;
    errors: {
        pos: number;
        err: string;
    }[];
}

export async function simulate(status: Status, actions: Actions[]): Promise<SimulateResult> {
    if (isTauri) {
        let { invoke } = await pkgTauri
        return invoke("simulate", { status, actions });
    } else {
        let { simulate } = await pkgWasm
        return simulate(status, actions)
    }
};

export interface SimulateOneStepResult {
    status: Status;
    is_success: boolean;
}

export async function simulateOneStep(status: Status, action: Actions, forceSuccess: boolean): Promise<SimulateOneStepResult> {
    if (isTauri) {
        let { invoke } = await pkgTauri
        return invoke("simulate_one_step", { status, action, forceSuccess });
    } else {
        let { simulate_one_step } = await pkgWasm
        return simulate_one_step(status, action, forceSuccess)
    }
};

export async function high_quality_probability(status: Status): Promise<number | null> {
    if (isTauri) {
        let { invoke } = await pkgTauri
        return invoke("high_quality_probability", { status });
    } else {
        let { high_quality_probability } = await pkgWasm
        return high_quality_probability(status)
    }
}

export async function allowedList(status: Status, actions: Actions[]): Promise<string[]> {
    if (isTauri) {
        let { invoke } = await pkgTauri
        return invoke("allowed_list", { status, skills: actions });
    } else {
        let { allowed_list } = await pkgWasm
        return allowed_list(status, actions)
    }
};

export async function craftPointsList(status: Status, actions: Actions[]): Promise<number[]> {
    if (isTauri) {
        let { invoke } = await pkgTauri
        return invoke("craftpoints_list", { status, skills: actions })
    } else {
        let { craftpoints_list } = await pkgWasm
        return craftpoints_list(status, actions)
    }
};

export function calcWaitTime(...actions: Actions[]): number {
    return actions.map(v => waitTimes.get(v) ?? 0)
        .reduce((acc, v) => acc + v, 0)
}

export function calcPostCastTime(...actions: Actions[]): number {
    return actions.map(v => postCastDelay.get(v) ?? 0)
        .reduce((acc, v) => acc + v, 0)
}

export interface RecipeInfo {
    id: number;
    rlv: number;
    item_id: number;
    item_name: string;
    job: string;

    difficulty_factor: number;
    quality_factor: number;
    durability_factor: number;
    material_quality_factor: number;

    required_craftsmanship: number;
    required_control: number;

    can_hq: boolean;
}

export interface RecipeRequirements {
    required_craftsmanship: number,
    required_control: number,
}

export interface ItemWithAmount {
    ingredient_id: number;
    amount: number;
}
