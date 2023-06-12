use ffxiv_crafting::{Actions, Status};

use crate::solver::Score;

/// 进行一次深度优先搜索（DFS）
///
/// status为开始制作时的初始状态
/// maximum_depth为限制最深搜索深度
pub fn solve(status: &Status, maximum_depth: u16, specialist: bool) -> Vec<Actions> {
    let mut best_list = Vec::new();
    let mut best_score = Score::from(status);

    // let (sender, receiver) = crossbeam_channel::bounded(0);
    // for _ in 0..num_cpus::get() {
    //     let receiver = receiver.clone();
    //     std::thread::spawn(move || while let Ok(status) = receiver.recv() {});
    // }
    // sender.send(status.clone());

    let mut stack = Vec::new();
    let mut stack_seq = Vec::new();
    stack.push((status.clone(), SKILL_LIST.into_iter()));
    while let Some((status, action_iter)) = stack.last_mut() {
        if let Some(next_action) = action_iter.next() {
            let mut new_s = status.clone();
            if !matches!(next_action, Actions::FocusedSynthesis | Actions::FocusedTouch if status.buffs.observed == 0)
                && !matches!(next_action, Actions::FinalAppraisal if status.buffs.final_appraisal == 0)
                && !matches!(next_action, Actions::HeartAndSoul if specialist)
                && new_s.is_action_allowed(next_action).is_ok()
            {
                new_s.cast_action(next_action);
                stack_seq.push(next_action);
                if !new_s.is_finished()
                    && !(best_score.quality == status.recipe.quality
                        && best_score.steps < new_s.step as u16)
                    && new_s.step < maximum_depth as i32
                {
                    stack.push((new_s, SKILL_LIST.into_iter()));
                } else {
                    let score = Score::from(&new_s);
                    if score > best_score {
                        best_score = score;
                        best_list = stack_seq.clone();
                        println!("{:?}", best_list);
                    }
                    stack_seq.pop();
                }
            }
        } else {
            stack_seq.pop();
            stack.pop();
        }
        // println!("{stack_seq:?}");
    }
    best_list
}

/// 搜索的技能列表
const SKILL_LIST: [Actions; 30] = [
    Actions::BasicSynthesis,
    Actions::BasicTouch,
    Actions::MastersMend,
    Actions::Observe,
    Actions::TricksOfTheTrade,
    Actions::WasteNot,
    Actions::Veneration,
    Actions::StandardTouch,
    Actions::GreatStrides,
    Actions::Innovation,
    Actions::FinalAppraisal,
    Actions::WasteNotII,
    Actions::ByregotsBlessing,
    Actions::PreciseTouch,
    Actions::MuscleMemory,
    Actions::CarefulSynthesis,
    Actions::Manipulation,
    Actions::PrudentTouch,
    Actions::FocusedSynthesis,
    Actions::FocusedTouch,
    Actions::Reflect,
    Actions::PreparatoryTouch,
    Actions::Groundwork,
    Actions::DelicateSynthesis,
    Actions::IntensiveSynthesis,
    Actions::TrainedEye,
    Actions::AdvancedTouch,
    Actions::PrudentSynthesis,
    Actions::TrainedFinesse,
    Actions::HeartAndSoul,
];
