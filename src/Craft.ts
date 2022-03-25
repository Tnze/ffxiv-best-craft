import { invoke } from "@tauri-apps/api/tauri";

interface Attributes {
  level: number;
  craftsmanship: number;
  control: number;
  craftPoints: number;
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

interface Status {
  condition: Conditions;
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

const simulate = async (actions: Actions[]): Promise<string> => {
  return await invoke("simulate", { skills: actions });
};

export {
  Attributes,
  Recipe,
  Status,
  Conditions,
  Jobs,
  Actions,
  new_recipe,
  simulate,
};
