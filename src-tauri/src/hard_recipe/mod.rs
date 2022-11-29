use ffxiv_crafting::Condition::{self, *};
use ffxiv_crafting::{Actions, Status};

use crate::preprogress_solver::PreprogressSolver;

pub trait Solver {
    fn run(&self, s: &Status) -> (String, Vec<Actions>);
}

pub struct LycorisSanguinea<'a> {
    dp_solver: &'a PreprogressSolver<10, 0>,
}

impl<'a> LycorisSanguinea<'a> {
    pub fn new(dp_solver: &'a PreprogressSolver<10, 0>) -> Self {
        Self { dp_solver }
    }
}

impl<'a> Solver for LycorisSanguinea<'a> {
    fn run(&self, s: &Status) -> (String, Vec<Actions>) {
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
            s if s.buffs.inner_quiet < 10 => {
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
                use crate::solver::Solver;
                sb.push_str(format!("[Phase 4] 推满加工条，").as_str());
                sb.push_str(format!("{:?}", self.dp_solver.read_all(s)).as_str());
                let best = [
                    Actions::AdvancedTouch,
                    Actions::StandardTouch,
                    Actions::BasicTouch,
                    Actions::MastersMend,
                    Actions::WasteNot,
                    Actions::Veneration,
                    Actions::GreatStrides,
                    Actions::Innovation,
                    Actions::WasteNotII,
                    Actions::ByregotsBlessing,
                    Actions::CarefulSynthesis,
                    Actions::PrudentTouch,
                    Actions::PreparatoryTouch,
                    Actions::Groundwork,
                    Actions::DelicateSynthesis,
                    Actions::PrudentSynthesis,
                    Actions::TrainedFinesse,
                    Actions::Manipulation,
                ]
                .into_iter()
                .filter(|&action| s.is_action_allowed(action).is_ok())
                .filter_map(|action| {
                    print!("{:?}，", action);
                    let mut new_s = s.clone();
                    new_s.cast_action(action);
                    new_s.condition = Condition::Normal;
                    self.dp_solver.get(&new_s)
                })
                .inspect(|x| print!("{:?}，", x.value))
                .max_by_key(|slot| slot.value);
                let mut result = Vec::with_capacity(1);
                if let Some(slot) = best {
                    if let Some(action) = slot.action {
                        println!("best: {:?}, {}", slot.action, slot.value);
                        result.push(action)
                    }
                }
                result
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
        const TOTAL: i32 = 100;
        let solver: LycorisSanguinea;
        for i in 0..TOTAL {
            println!("running simulation {i}");
            let mut status = Status::new(a, r);
            'solve: while !status.is_finished() {
                print!("{}/{}，", status.progress, status.quality);
                print!("球色：{:?}，", status.condition);
                let (log, next_actions) = solver::run(&status);
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
                    if !matches!(next_action, Actions::FinalAppraisal | Actions::HeartAndSoul) {
                        status.condition = ConditionIterator::new(
                            status.recipe.conditions_flag as i32,
                            status.attributes.level as i32,
                        )
                        .collect::<Vec<_>>()
                        .choose_weighted(&mut rng, |c| c.1)
                        .unwrap()
                        .0;
                    }
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
        println!("avg: {}, full: {full}", sum as f32 / TOTAL as f32);
    }
}
