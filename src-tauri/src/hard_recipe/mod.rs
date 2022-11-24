use ffxiv_crafting::Condition::*;
use ffxiv_crafting::{Actions, Status};

pub trait Solver {
    fn run(s: &Status) -> (String, Option<Actions>);
}

pub struct RedstoneSuan;

impl Solver for RedstoneSuan {
    fn run(s: &Status) -> (String, Option<Actions>) {
        let mut sb = String::new();
        let action = match s {
            s if s.step == 0 || s.buffs.muscle_memory > 0 => {
                sb.push_str(format!("判断阶段：坚信起手\n").as_str());
                Some(match s.step {
                    0 => Actions::MuscleMemory,
                    1 => Actions::Veneration,
                    _ => Actions::RapidSynthesis,
                })
            }
            s if s.progress > 0 && s.progress < 7198 => {
                sb.push_str(format!("判断阶段：崇敬高速，进展{}\n", s.progress).as_str());
                if s.durability < 20 {
                    sb.push_str(format!("耐久 < 20\n").as_str());
                    Some(match s.condition {
                        Good => {
                            sb.push_str(format!("红球秘诀\n").as_str());
                            Actions::TricksOfTheTrade
                        }
                        Sturdy if s.durability % 5 > 3 => {
                            sb.push_str(format!("蓝球俭约加工，把耐久尾数修到2\n").as_str());
                            Actions::PrudentSynthesis
                        }
                        _ => {
                            if s.buffs.manipulation == 0 {
                                Actions::Manipulation
                            } else if s.durability <= s.calc_durability(10) {
                                sb.push_str(format!("耐久不够高速，掌握层数有剩下\n").as_str());
                                Actions::Observe
                            } else {
                                Actions::RapidSynthesis
                            }
                        }
                    })
                } else {
                    sb.push_str(format!("耐久 >= 20\n").as_str());

                    match s.condition {
                        Primed => Some(Actions::Manipulation),
                        Good => Some(Actions::TricksOfTheTrade),
                        Malleable if s.buffs.veneration > 0 => todo!(),
                        Malleable | _ if s.buffs.veneration > 0 => Some(Actions::RapidSynthesis),
                        _ => Some(Actions::Veneration),
                    }
                }
            }
            s => {
                sb.push_str(format!("判断阶段：堆叠内静\n").as_str());
                let ext_du = s.durability as i32 + 5 * s.buffs.
                manipulation as i32
                    - 5 * (10 - s.buffs.inner_quiet as i32)
                    - 11;
                sb.push_str(format!("额外耐久：{ext_du}\n").as_str());

                if s.quality == 0 && s.buffs.manipulation == 0 {
                    Some(Actions::Manipulation)
                } else {
                    None
                }
            }
            _ => None,
        };
        (sb, action)
    }
}

#[cfg(test)]
mod test {
    use super::{RedstoneSuan, Solver};
    use ffxiv_crafting::{Attributes, Recipe, Status};

    #[test]
    fn simulate() {
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
        for i in 0..1 {
            println!("running simulation {i}");
            let mut status = Status::new(a, r);
            while !status.is_finished() {
                let (log, next_action) = RedstoneSuan::run(&status);
                println!("{next_action:?}: {log}");
                let Some(next_action) = next_action else {
                    break;
                };
                status.is_action_allowed(next_action).unwrap();
                status.cast_action(next_action);
                println!("status: {status:?}");
            }
        }
    }
}
