import { invoke } from "@tauri-apps/api/tauri";

interface Attributes {
  level: number;
  craftsmanship: number;
  control: number;
  craft_points: number;
}

interface Recipe {
  rlv: number;
  job_level: number;
  difficulty: number;
  quality: number;
  durability: number;
  conditions_flag: number;

  progress_divider: number;
  quality_divider: number;
  progress_modifier: number;
  quality_modifier: number;
}

interface Buffs {
  muscle_memory: number;
  great_strides: number;
  veneration: number;
  innovation: number;
  inner_quiet: number;
  final_appraisal: number;
  manipulation: number;
  wast_not: number;
  standard_touch_prepared: boolean;
  advanced_touch_prepared: boolean;
  observed: boolean;
}

interface Status {
  buffs: Buffs;
  attributes: Attributes;
  recipe: Recipe;
  catches: any;
  durability: number;
  craft_points: number;
  progress: number;
  quality: number;
  step: number;
  condition: string;
}

enum Conditions {
  // 白：通常
  Normal,
  // 红：高品质，加工效率1.5倍
  Good,
  // 彩：最高品质
  Excellent,
  // 黑：低品质
  Poor,

  // 黄：成功率增加 25%
  Centered,
  // 蓝：耐久消耗降低 50%, 效果可与俭约叠加
  Sturdy,
  // 绿：CP 消耗减少 50%
  Pliant,
  // 深蓝：作业效率1.5倍
  Malleable,
  // 紫：技能效果持续增加两回合
  Primed,
}

enum Jobs {
  Carpenter,
  Blacksmith,
  Armorer,
  Goldsmith,
  Leatherworker,
  Weaver,
  Alchemist,
  Culinarian,
}

enum Actions {
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
}

const new_recipe = async (
  rlv: number,
  difficultyFactor: number,
  qualityFactor: number,
  durabilityFactor: number
): Promise<Recipe> => {
  return await invoke("new_recipe", {
    rlv,
    difficultyFactor,
    qualityFactor,
    durabilityFactor,
  });
};

const new_status = (
  attrs: Attributes,
  recipe: Recipe,
  initQuality: number = 0
): Promise<Status> => invoke("new_status", { attrs, recipe, initQuality });

interface SimulateResult {
  status: Status;
  errors: {
    pos: number;
    err: string;
  }[];
}

const simulate = (s: Status, actions: Actions[]): Promise<SimulateResult> => {
  return invoke("simulate", { status: s, skills: actions });
};

interface RecipeRow {
  id: number;
  rlv: number;
  name: string;
  job: string;
  
  difficulty_factor: number;
  quality_factor: number;
  durability_factor: number;
}

const recipe_table = (): Promise<RecipeRow[]> => {
  return invoke("recipe_table");
};

export {
  Attributes,
  Buffs,
  Conditions,
  Recipe,
  Status,
  Jobs,
  Actions,
  new_recipe,
  new_status,
  simulate,
  RecipeRow,
  recipe_table,
};
