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

#[cfg(test)]
mod test {
    use super::{LycorisSanguinea, Solver};
    use ffxiv_crafting::{Actions, Attributes, ConditionIterator, Recipe, Status};
    use rand::prelude::*;
    use std::sync::Arc;

    #[test]
    fn simulate() {
        use crate::ordinary_solver::ProgressSolver;
        use crate::solver::Solver as _;
        use crate::PreprogressSolver;

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
        const TOTAL: i32 = 1000;

        let init_status = Status::new(a, r);
        let progress_list = vec![init_status.calc_synthesis(1.2)];
        let mut driver = ProgressSolver::new(init_status.clone());
        driver.init();
        let mut dp_solver =
            PreprogressSolver::<10, 0>::new(init_status, progress_list, Arc::new(driver));
        dp_solver.init();
        let solver = LycorisSanguinea::new(&dp_solver);

        for i in 0..TOTAL {
            println!("running simulation {i}");
            let mut status = Status::new(a, r);
            'solve: while !status.is_finished() {
                print!("{}/{}，", status.progress, status.quality);
                print!("球色：{:?}，", status.condition);
                let (log, next_actions) = solver.run(&status);
                if next_actions.len() == 0 {
                    println!("求解结果为空：{status:?}");
                    break;
                };
                print!("{log}");
                let Some(&next_action) = next_actions.get(0) else {
                    break;
                };
                // for next_action in next_actions.into_iter() {
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
                if !matches!(next_action, Actions::FinalAppraisal | Actions::HeartAndSoul) {
                    status.condition = conditions.choose_weighted(&mut rng, |c| c.1).unwrap().0;
                }
                // }
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
        println!("avg: {}, full: {full}", sum as f32 / TOTAL as f32);
    }
}
