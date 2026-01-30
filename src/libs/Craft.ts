// This file is part of BestCraft.
// Copyright (C) 2025 Tnze
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

import { JSONSchemaType } from 'ajv';
import { isTauri } from './Consts';

if (import.meta.env.VITE_BESTCRAFT_TARGET == 'tauri') {
    var pkgTauri = import('@tauri-apps/api/core');
} else {
    var pkgWasm = import('@/../pkg-wasm/app_wasm');
}

export interface Attributes {
    level: number;
    craftsmanship: number;
    control: number;
    craft_points: number;
}

export const AttributesSchema: JSONSchemaType<Attributes> = {
    type: 'object',
    properties: {
        level: { type: 'number' },
        craftsmanship: { type: 'number' },
        control: { type: 'number' },
        craft_points: { type: 'number' },
    },
    required: ['level', 'craftsmanship', 'control', 'craft_points'],
    additionalProperties: false,
};

export interface Item {
    id: number;
    name: string;
    level: number;
    can_be_hq: boolean;
    category_id?: number;
}

export interface Recipe {
    rlv: RecipeLevel;
    job_level: number;
    difficulty: number;
    quality: number;
    durability: number;
    conditions_flag: number;
}

export interface RecipeLevel {
    id: number;
    class_job_level: number;
    stars: number;
    suggested_craftsmanship: number;
    suggested_control: number | null;
    difficulty: number;
    quality: number;
    progress_divider: number;
    quality_divider: number;
    progress_modifier: number;
    quality_modifier: number;
    durability: number;
    conditions_flag: number;
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
    Unused = 'Unused',
    Active = 'Active',
    Used = 'Used',
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
    GoodOmen = 'GoodOmen',
    // 强韧：耐久消耗减半，下个工次必定出现结实状态。
    Robust = 'Robust',
}

export enum Jobs {
    Carpenter = 'carpenter',
    Blacksmith = 'blacksmith',
    Armorer = 'armorer',
    Goldsmith = 'goldsmith',
    Leatherworker = 'leatherworker',
    Weaver = 'weaver',
    Alchemist = 'alchemist',
    Culinarian = 'culinarian',
}

export const JobsSchema: JSONSchemaType<Jobs> = {
    type: 'string',
    enum: Object.values(Jobs),
};

export enum Actions {
    BasicSynthesis = 'basic_synthesis',
    BasicTouch = 'basic_touch',
    MastersMend = 'masters_mend',
    HastyTouch = 'hasty_touch',
    RapidSynthesis = 'rapid_synthesis',
    Observe = 'observe',
    TricksOfTheTrade = 'tricks_of_the_trade',
    WasteNot = 'waste_not',
    Veneration = 'veneration',
    StandardTouch = 'standard_touch',
    GreatStrides = 'great_strides',
    Innovation = 'innovation',
    FinalAppraisal = 'final_appraisal',
    WasteNotII = 'waste_not_ii',
    ByregotsBlessing = 'byregot_s_blessing',
    PreciseTouch = 'precise_touch',
    MuscleMemory = 'muscle_memory',
    CarefulSynthesis = 'careful_synthesis',
    Manipulation = 'manipulation',
    PrudentTouch = 'prudent_touch',
    Reflect = 'reflect',
    PreparatoryTouch = 'preparatory_touch',
    Groundwork = 'groundwork',
    DelicateSynthesis = 'delicate_synthesis',
    IntensiveSynthesis = 'intensive_synthesis',
    TrainedEye = 'trained_eye',
    AdvancedTouch = 'advanced_touch',
    PrudentSynthesis = 'prudent_synthesis',
    TrainedFinesse = 'trained_finesse',
    CarefulObservation = 'careful_observation',
    HeartAndSoul = 'heart_and_soul',
    // 7.0
    RefinedTouch = 'refined_touch',
    DaringTouch = 'daring_touch',
    ImmaculateMend = 'immaculate_mend',
    QuickInnovation = 'quick_innovation',
    TrainedPerfection = 'trained_perfection',
    // fake skills
    RapidSynthesisFail = 'rapid_synthesis_fail',
    HastyTouchFail = 'hasty_touch_fail',
    DaringTouchFail = 'daring_touch_fail',
}

export const ActionsSchema: JSONSchemaType<Actions> = {
    type: 'string',
    enum: Object.values(Actions),
};

const waitTimes = new Map([
    [Actions.BasicSynthesis, 2.17],
    [Actions.BasicTouch, 2.17],
    [Actions.MastersMend, 2.17],
    [Actions.HastyTouch, 2.17],
    [Actions.RapidSynthesis, 2.17],
    [Actions.Observe, 2.17],
    [Actions.TricksOfTheTrade, 2.17],
    [Actions.WasteNot, 1.07],
    [Actions.Veneration, 1.07],
    [Actions.StandardTouch, 2.17],
    [Actions.GreatStrides, 1.07],
    [Actions.Innovation, 1.07],
    [Actions.FinalAppraisal, 1.07],
    [Actions.WasteNotII, 1.07],
    [Actions.ByregotsBlessing, 2.17],
    [Actions.PreciseTouch, 2.17],
    [Actions.MuscleMemory, 2.17],
    [Actions.CarefulSynthesis, 2.17],
    [Actions.Manipulation, 1.07],
    [Actions.PrudentTouch, 2.17],
    [Actions.Reflect, 2.17],
    [Actions.PreparatoryTouch, 2.17],
    [Actions.Groundwork, 2.17],
    [Actions.DelicateSynthesis, 2.17],
    [Actions.IntensiveSynthesis, 2.17],
    [Actions.TrainedEye, 2.17],
    [Actions.AdvancedTouch, 2.17],
    [Actions.PrudentSynthesis, 2.17],
    [Actions.TrainedFinesse, 2.17],
    [Actions.CarefulObservation, 2.17],
    [Actions.HeartAndSoul, 2.17],
    [Actions.RefinedTouch, 2.17],
    [Actions.DaringTouch, 2.17],
    [Actions.ImmaculateMend, 2.17],
    [Actions.QuickInnovation, 2.17],
    [Actions.TrainedPerfection, 2.17],
]);

export const newRecipe = async (
    rlv: RecipeLevel,
    difficultyFactor: number,
    qualityFactor: number,
    durabilityFactor: number,
): Promise<Recipe> => {
    return {
        rlv,
        job_level: rlv.class_job_level,
        difficulty: Math.floor((rlv.difficulty * difficultyFactor) / 100),
        quality: Math.floor((rlv.quality * qualityFactor) / 100),
        durability: Math.floor((rlv.durability * durabilityFactor) / 100),
        conditions_flag: rlv.conditions_flag,
    };
};

export async function newStatus(
    attrs: Attributes,
    recipe: Recipe,
): Promise<Status> {
    if (isTauri) {
        let { invoke } = await pkgTauri;
        return invoke('new_status', { attrs, recipe });
    } else {
        let { new_status } = await pkgWasm;
        return new_status(attrs, recipe);
    }
}

export interface SimulateResult {
    status: Status;
    errors: {
        pos: number;
        err: string;
    }[];
}

export async function simulate(
    status: Status,
    actions: Actions[],
): Promise<SimulateResult> {
    if (isTauri) {
        let { invoke } = await pkgTauri;
        return invoke('simulate', { status, actions });
    } else {
        let { simulate } = await pkgWasm;
        return simulate(status, actions);
    }
}

export async function simulateDetail(
    status: Status,
    actions: Actions[],
): Promise<any> {
    if (isTauri) {
        let { invoke } = await pkgTauri;
        return invoke('simulate_detail', { status, actions });
    } else {
        let { simulate_detail } = await pkgWasm;
        return simulate_detail(status, actions);
    }
}

export interface SimulateOneStepResult {
    status: Status;
    is_success: boolean;
}

export async function simulateOneStep(
    status: Status,
    action: Actions,
    forceSuccess: boolean,
): Promise<SimulateOneStepResult> {
    if (isTauri) {
        let { invoke } = await pkgTauri;
        return invoke('simulate_one_step', { status, action, forceSuccess });
    } else {
        let { simulate_one_step } = await pkgWasm;
        return simulate_one_step(status, action, forceSuccess);
    }
}

export async function highQualityProbability(
    status: Status,
): Promise<number | null> {
    if (isTauri) {
        let { invoke } = await pkgTauri;
        return invoke('high_quality_probability', { status });
    } else {
        let { high_quality_probability } = await pkgWasm;
        return high_quality_probability(status);
    }
}

export async function allowedList(
    status: Status,
    actions: Actions[],
): Promise<string[]> {
    if (isTauri) {
        let { invoke } = await pkgTauri;
        return invoke('allowed_list', { status, skills: actions });
    } else {
        let { allowed_list } = await pkgWasm;
        return allowed_list(status, actions);
    }
}

export async function craftPointsList(
    status: Status,
    actions: Actions[],
): Promise<number[]> {
    if (isTauri) {
        let { invoke } = await pkgTauri;
        return invoke('craftpoints_list', { status, skills: actions });
    } else {
        let { craftpoints_list } = await pkgWasm;
        return craftpoints_list(status, actions);
    }
}

export function calcWaitTime(...actions: Actions[]): number {
    return actions
        .map(v => Math.ceil(waitTimes.get(v) ?? 0))
        .reduce((acc, v) => acc + v, 0);
}

export function calcPostCastTime(...actions: Actions[]): number {
    return actions
        .map(v => waitTimes.get(v) ?? 0)
        .reduce((acc, v) => acc + v, 0);
}

export interface RecipeInfo {
    id: number;
    rlv: number;
    item_id: number;
    item_name: string;
    item_amount?: number;
    job: string;

    difficulty_factor: number;
    quality_factor: number;
    durability_factor: number;
    material_quality_factor: number;

    required_craftsmanship: number;
    required_control: number;

    can_hq: boolean;
    recipe_notebook_list: number;
}

export interface RecipeRequirements {
    required_craftsmanship: number;
    required_control: number;
}

export interface ItemWithAmount {
    ingredient_id: number;
    amount: number;
}

export interface CollectablesShopRefine {
    id: number;
    low_collectability: number;
    mid_collectability: number;
    high_collectability: number;
}
