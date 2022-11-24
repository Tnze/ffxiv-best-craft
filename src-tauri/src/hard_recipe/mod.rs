use ffxiv_crafting::Condition::*;
use ffxiv_crafting::{Actions, Status};

pub trait Solver {
    fn run(s: &Status) -> (String, Vec<Actions>);
}

pub struct RedstoneSuan611;

impl Solver for RedstoneSuan611 {
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
                if s.durability < 20 {
                    sb.push_str(format!("耐久 < 20，").as_str());
                    let action = match s.condition {
                        Good => {
                            sb.push_str(format!("红球秘诀，").as_str());
                            Actions::TricksOfTheTrade
                        }
                        Sturdy if s.durability % 5 > 3 => {
                            sb.push_str(format!("蓝球俭约加工把耐久尾数修到2，").as_str());
                            Actions::PrudentSynthesis
                        }
                        _ => {
                            if s.buffs.manipulation <= 1 {
                                Actions::Manipulation
                            } else if s.durability <= s.calc_durability(10) {
                                sb.push_str(format!("耐久不够高速、掌握层数有剩下，").as_str());
                                // match s.is_action_allowed(Actions::CarefulObservation) {
                                //     Ok(_) => Actions::CarefulObservation,
                                //     Err(_) => Actions::Observe,
                                // }
                                Actions::Observe
                            } else if s.progress >= 5710 {
                                Actions::BasicSynthesis
                            } else {
                                Actions::RapidSynthesis
                            }
                        }
                    };
                    vec![action]
                } else {
                    sb.push_str(format!("耐久 >= 20，").as_str());

                    match s.condition {
                        Primed if s.buffs.manipulation <= 1 => vec![Actions::Manipulation],
                        Good => vec![Actions::TricksOfTheTrade],
                        Malleable if s.buffs.veneration > 0 => {
                            vec![Actions::FinalAppraisal, Actions::RapidSynthesis]
                        }
                        Malleable | _ if s.buffs.veneration > 0 => {
                            vec![if s.progress >= 5710 {
                                Actions::BasicSynthesis
                            } else {
                                Actions::RapidSynthesis
                            }]
                        }
                        _ => vec![Actions::Veneration],
                    }
                }
            }
            s if s.buffs.inner_quiet < 8 => {
                sb.push_str(format!("[Phase 3] 堆叠内静，").as_str());
                let ext_du = s.durability as i32 + 5 * s.buffs.manipulation as i32
                    - 5 * (10 - s.buffs.inner_quiet as i32)
                    - 11;
                sb.push_str(format!("额外耐久：{ext_du}，").as_str());

                if s.quality == 0 && s.buffs.manipulation == 0 {
                    sb.push_str(format!("直接使用掌握后进入本阶段，").as_str());
                    vec![Actions::Manipulation]
                } else if ext_du > 0 {
                    sb.push_str(format!("大于0、改革-俭约加工至10内静，").as_str());
                    if s.buffs.innovation > 0 {
                        vec![Actions::PrudentTouch]
                    } else {
                        vec![Actions::Innovation]
                    }
                } else if s.buffs.innovation == 0 {
                    sb.push_str(format!("无改革buff，").as_str());
                    vec![match s.condition {
                        Good => Actions::TricksOfTheTrade,
                        Sturdy if s.buffs.inner_quiet == 0 => Actions::PrudentTouch,
                        _ if s.buffs.manipulation <= 1 => Actions::Manipulation,
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
                        _ if s.buffs.heart_and_soul > 0 => Actions::PreciseTouch,
                        _ => Actions::PrudentTouch,
                    }]
                }
            }
            s if s.buffs.inner_quiet < 10 => match (s.buffs.manipulation % 2, s.durability % 10) {
                (1, 1..=5) => vec![
                    Actions::PrudentTouch,
                    Actions::Observe,
                    Actions::FocusedTouch,
                ],
                (1, _) | (0, 1..=5) => vec![
                    Actions::Observe,
                    Actions::FocusedTouch,
                    Actions::Observe,
                    Actions::FocusedTouch,
                ],
                (0, _) => vec![
                    Actions::PrudentTouch,
                    Actions::Observe,
                    Actions::FocusedTouch,
                ],
                _ => unreachable!(),
            },
            _ if s.buffs.inner_quiet == 10 => {
                sb.push_str(format!("[Phase 4] 推满加工条，").as_str());
                let action = match s.condition {
                    Good if s.buffs.innovation == 0 => Actions::TricksOfTheTrade,
                    _ if s.buffs.innovation == 0 => Actions::Innovation,
                    Good if s.calc_durability(10) < s.durability => Actions::PreciseTouch,
                    Good => Actions::TricksOfTheTrade,
                    Sturdy if s.calc_durability(20) < s.durability => Actions::PreparatoryTouch,
                    Primed if s.buffs.innovation <= 2 => Actions::Innovation,
                    _ => Actions::TrainedFinesse,
                };

                let mut tmp_state = s.clone();
                tmp_state.cast_action(action);
                if tmp_state.durability >= 11
                    && (tmp_state.craft_points >= 74
                        || (tmp_state.craft_points >= 56 && tmp_state.buffs.innovation >= 2))
                {
                    vec![action]
                } else {
                    sb.push_str(format!("收尾，").as_str());
                    if s.craft_points >= 74 {
                        vec![
                            Actions::GreatStrides,
                            Actions::Innovation,
                            Actions::ByregotsBlessing,
                            Actions::BasicSynthesis,
                        ]
                    } else if s.craft_points >= 56 && s.buffs.innovation >= 2 {
                        vec![
                            Actions::GreatStrides,
                            Actions::ByregotsBlessing,
                            Actions::BasicSynthesis,
                        ]
                    } else {
                        vec![]
                    }
                }
            }
            _ => vec![],
        };
        (sb, action)
    }
}

#[cfg(test)]
mod test {
    use super::{RedstoneSuan611, Solver};
    use ffxiv_crafting::{Actions, Attributes, ConditionIterator, Recipe, Status};
    use rand::prelude::*;

    #[test]
    fn simulate() {
        let mut rng = thread_rng();
        let r = Recipe {
            rlv: 611,
            job_level: 90,
            difficulty: 7480,
            quality: 13620,
            durability: 60,
            conditions_flag: 435,
        };
        let a = Attributes {
            level: 90,
            craftsmanship: 4214,
            control: 3528,
            craft_points: 691,
        };
        let conditions =
            ConditionIterator::new(r.conditions_flag as i32, a.level as i32).collect::<Vec<_>>();
        let mut sum = 0;
        let mut full = 0;
        const total: i32 = 100;
        for i in 0..total {
            println!("running simulation {i}");
            let mut status = Status::new(a, r);
            'solve: while !status.is_finished() {
                print!("{}/{}，", status.progress, status.quality);
                print!("球色：{:?}，", status.condition);
                let (log, next_actions) = RedstoneSuan611::run(&status);
                if next_actions.len() == 0 {
                    println!("求解结果为空：{status:?}");
                    break;
                };
                print!("{log}");
                for next_action in next_actions.into_iter() {
                    print!("{next_action:?}");
                    if let Err(e) = status.is_action_allowed(next_action) {
                        println!("\n无法使用{next_action:?}：{e:?}");
                        break 'solve;
                    }
                    if status.success_rate(next_action) as f32 / 100.0 > random() {
                        print!("，");
                        status.cast_action(next_action);
                    } else {
                        print!("失败，");
                        status.cast_action(match next_action {
                            Actions::RapidSynthesis => Actions::RapidSynthesisFail,
                            Actions::HastyTouch => Actions::HastyTouchFail,
                            Actions::FocusedSynthesis => Actions::FocusedSynthesisFail,
                            Actions::FocusedTouch => Actions::FocusedTouchFail,
                            _ => unreachable!(),
                        });
                    }
                    status.condition = conditions.choose_weighted(&mut rng, |c| c.1).unwrap().0;
                }
                println!();
            }
            println!(
                "simulation {i} result: 进展{}/品质{}/耐久{}",
                status.progress, status.quality, status.durability
            );
            if status.progress == r.difficulty {
                sum += status.quality;
                if status.quality == r.quality {
                    full += 1;
                }
            }
        }
        println!("avg: {}, full: {full}", sum as f32 / total as f32);
    }
}
