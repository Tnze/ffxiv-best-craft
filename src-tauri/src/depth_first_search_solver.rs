use crossbeam_channel::{Sender, TrySendError};
use ffxiv_crafting::{Actions, Status};

use crate::solver::Score;

/// 进行一次深度优先搜索（DFS）
///
/// status为开始制作时的初始状态
/// maximum_depth为限制最深搜索深度
pub fn solve(status: &Status, maximum_depth: u16, specialist: bool) -> Vec<Actions> {
    let num = num_cpus::get();
    let (sender, receiver) = crossbeam_channel::bounded::<(Status, Vec<Actions>)>(num);
    let (task_sender, task_receiver) = crossbeam_channel::bounded(num);
    for _ in 0..num {
        let r = receiver.clone();
        let s = sender.clone();
        let ts = task_sender.clone();
        std::thread::spawn(move || {
            while let Ok((status, actions)) = r.recv() {
                ts.send(search(status, actions, &s)).unwrap();
            }
        });
    }
    sender.send((status.clone(), Vec::new())).unwrap();

    let mut best_actions = Vec::new();
    let mut best_score = Score::from(status);
    while let Ok((score, actions)) = task_receiver.recv() {
        if score > best_score {
            best_actions = actions;
            best_score = score;
            println!("{:?}", best_actions);
        }
    }

    best_actions
}

fn search(
    status: Status,
    actions: Vec<Actions>,
    sender: &Sender<(Status, Vec<Actions>)>,
) -> (Score, Vec<Actions>) {
    let mut best_actions = actions.clone();
    let mut best_score = Score::from(&status);
    let specialist = false;

    let mut stack = Vec::new();
    let mut stack_seq = actions;
    stack.push((status.clone(), SKILL_LIST.into_iter()));
    stack_seq.push(Actions::BasicSynthesis);
    while let Some((status, action_iter)) = stack.last_mut() {
        let Some(next_action) = action_iter.next() else {
            stack.pop().unwrap();
            stack_seq.pop().unwrap();
            continue;
        };
        *stack_seq.last_mut().unwrap() = next_action;

        if !matches!(next_action, Actions::FocusedSynthesis | Actions::FocusedTouch if status.buffs.observed == 0)
            && !matches!(next_action, Actions::FinalAppraisal if status.buffs.final_appraisal == 0)
            && !matches!(next_action, Actions::HeartAndSoul if specialist)
            && status.is_action_allowed(next_action).is_ok()
        {
            let mut new_s = status.clone();
            new_s.cast_action(next_action);
            if !new_s.is_finished() {
                if best_score.quality != new_s.recipe.quality
                    || best_score.steps >= new_s.step as u16
                {
                    if !sender.is_full() {
                        let payload = (new_s.clone(), stack_seq.clone());
                        if sender.try_send(payload).is_ok() {
                            continue;
                        }
                    }
                    stack.push((new_s, SKILL_LIST.into_iter()));
                    stack_seq.push(next_action);
                }
            } else {
                let score = Score::from(&new_s);
                if score > best_score {
                    best_score = score;
                    best_actions = stack_seq.clone();
                }
            }
        }
    }
    (best_score, best_actions)
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
