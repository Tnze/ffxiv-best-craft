import { invoke } from "@tauri-apps/api/tauri";

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
    suggested_craftsmanship: number,
    suggested_control: number,
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
    heart_and_soul: number;
    careful_observation_used: number;
    heart_and_soul_used: number;
    touch_combo_stage: number;
    observed: number;
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
    FocusedSynthesis = "focused_synthesis",
    FocusedTouch = "focused_touch",
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
    // fake skills
    RapidSynthesisFail = "rapid_synthesis_fail",
    HastyTouchFail = "hasty_touch_fail",
    FocusedSynthesisFail = "focused_synthesis_fail",
    FocusedTouchFail = "focused_touch_fail",
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

export const newStatus = (
    attrs: Attributes,
    recipe: Recipe,
    recipeLevel: RecipeLevel,
): Promise<Status> => {
    console.debug(recipe)
    return invoke("new_status", { attrs, recipe, recipeLevel })
};

export interface SimulateResult {
    status: Status;
    errors: {
        pos: number;
        err: string;
    }[];
}

export const simulate = (status: Status, actions: Actions[]): Promise<SimulateResult> => {
    return invoke("simulate", { status, actions });
};

export interface SimulateOneStepResult {
    status: Status;
    is_success: boolean;
}

export const simulateOneStep = (status: Status, action: Actions, forceSuccess: boolean): Promise<SimulateOneStepResult> => {
    return invoke("simulate_one_step", { status, action, forceSuccess });
};

export const allowedList = (status: Status, actions: Actions[]): Promise<string[]> => {
    return invoke("allowed_list", { status, skills: actions });
};

export const craftPointsList = (status: Status, actions: Actions[]): Promise<number[]> => {
    return invoke("craftpoints_list", { status, skills: actions })
};

export interface RecipeInfo {
    id: number;
    rlv: number;
    item_id: number;
    item_name: string;
    job: string;

    difficulty_factor: number;
    quality_factor: number;
    durability_factor: number;

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

export const recipesIngredientions = async (checklist: ItemWithAmount[]): Promise<ItemWithAmount[]> => {
    const ings = await invoke("recipes_ingredientions", {
        checklist: checklist.map(x => [x.ingredient_id, x.amount])
    }) as [number, number][];
    return ings.map(x => {
        return { ingredient_id: x[0], amount: x[1] }
    })
}
