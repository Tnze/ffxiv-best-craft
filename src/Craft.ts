interface Attributes {
  level: number;
  craftsmanship: number;
  control: number;
  craftPoints: number;
}

interface Recipe {
  rlv: number;
  jobLevel: number;
  difficulty: number;
  quality: number;
  durability: number;
  conditionsFlag: number;

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

export { Attributes, Recipe, Status, Conditions, Jobs };
