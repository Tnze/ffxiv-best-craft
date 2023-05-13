use ffxiv_crafting::Condition::*;
use ffxiv_crafting::{Actions, Status};

pub trait Solver {
    fn run(s: &Status) -> (String, Vec<Actions>);
}

pub struct LycorisSanguinea;

impl Solver for LycorisSanguinea {
    fn run(s: &Status) -> (String, Vec<Actions>) {
        let mut sb = String::new();
        let action = match s {
            s if s.step == 0 => vec![Actions::MuscleMemory],
            s if s.step == 1 => vec![Actions::Veneration],
            s if s.buffs.muscle_memory > 0 => {
                sb.push_str(format!("[Phase 1] 坚信内打高速，").as_str());
                vec![Actions::RapidSynthesis]
            }
            s if s.progress < 7198 => {
                sb.push_str(format!("[Phase 2] 崇敬内打高速，").as_str());
                vec![match s.condition {
                    Centered if s.buffs.veneration > 0 && s.durability > s.calc_durability(10) => {
                        if s.buffs.final_appraisal == 0
                            && s.calc_synthesis(5.0) + s.progress > s.recipe.difficulty
                        {
                            Actions::FinalAppraisal
                        } else {
                            Actions::RapidSynthesis
                        }
                    }
                    Malleable if s.durability > s.calc_durability(10) => {
                        if s.buffs.final_appraisal == 0
                            && s.calc_synthesis(5.0) + s.progress > s.recipe.difficulty
                        {
                            Actions::FinalAppraisal
                        } else {
                            Actions::RapidSynthesis
                        }
                    }
                    Good if s.craft_points + 20 <= s.attributes.craft_points => {
                        Actions::TricksOfTheTrade
                    }
                    Primed if s.buffs.manipulation <= 1 => Actions::Manipulation,
                    Primed if s.buffs.veneration <= 1 => Actions::Veneration,
                    _ if s.buffs.manipulation == 0 => Actions::Manipulation,
                    _ if s.buffs.veneration == 0 => Actions::Veneration,
                    _ if s.durability > s.calc_durability(10) => {
                        if s.buffs.final_appraisal == 0
                            && s.calc_synthesis(5.0) + s.progress > s.recipe.difficulty
                        {
                            Actions::FinalAppraisal
                        } else {
                            Actions::RapidSynthesis
                        }
                    }
                    _ if s.buffs.careful_observation_used < 3 => Actions::CarefulObservation,
                    _ => Actions::Observe,
                }]
            }
            s if s.buffs.inner_quiet < 10
                && s.quality < s.recipe.quality
                && s.craft_points > 100 =>
            {
                sb.push_str(format!("[Phase 3] 堆叠内静，").as_str());
                let ext_du = s.durability as i32 + 5 * s.buffs.manipulation as i32
                    - 5 * (10 - s.buffs.inner_quiet as i32)
                    - 11;
                sb.push_str(format!("额外耐久：{ext_du}，").as_str());

                if s.quality == 0 && s.buffs.manipulation == 0 {
                    sb.push_str(format!("直接使用掌握后进入本阶段，").as_str());
                    vec![Actions::Manipulation]
                } else if s.buffs.innovation == 0 {
                    sb.push_str(format!("无改革buff，").as_str());
                    vec![match s.condition {
                        Good => Actions::TricksOfTheTrade,
                        Sturdy if s.buffs.inner_quiet == 0 => Actions::PrudentTouch,
                        Primed if s.buffs.manipulation <= 1 => Actions::Manipulation,
                        _ => Actions::Innovation,
                    }]
                } else {
                    sb.push_str(format!("有改革buff，").as_str());
                    vec![match s.condition {
                        Good if s.calc_durability(10) < s.durability => Actions::PreciseTouch,
                        Good => Actions::TricksOfTheTrade,

                        Sturdy if s.buffs.inner_quiet == 0 => Actions::PrudentTouch,
                        Sturdy if s.calc_durability(20) < s.durability => Actions::PreparatoryTouch,
                        Sturdy => Actions::BasicTouch,

                        Primed if s.buffs.manipulation <= 1 => Actions::Manipulation,
                        Primed if s.buffs.innovation <= 2 => Actions::Innovation,

                        _ if s.is_action_allowed(Actions::HeartAndSoul).is_ok() => {
                            Actions::HeartAndSoul
                        }
                        _ if s.buffs.manipulation == 0 && s.durability < 15 => {
                            Actions::Manipulation
                        }
                        _ if s.buffs.heart_and_soul > 0 => Actions::PreciseTouch,
                        _ if s.buffs.careful_observation_used < 3 => Actions::CarefulObservation,
                        _ => Actions::PrudentTouch,
                    }]
                }
            }
            s if s.buffs.inner_quiet == 10 => {
                sb.push_str(format!("[Phase 4] 推满加工条，").as_str());
                vec![]
            }
            _ => vec![],
        };
        (sb, action)
    }
}
