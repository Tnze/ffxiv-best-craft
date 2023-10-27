// This file is part of BestCraft.
// Copyright (C) 2023 Tnze
//
// BestCraft is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// BestCraft is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use ffxiv_crafting::{Actions, Status};

use crate::solver::Score;

/// 进行一次深度优先搜索（DFS）
///
/// status为开始制作时的初始状态
/// maximum_depth为限制最深搜索深度
pub fn solve(status: &Status, maximum_depth: usize, specialist: bool) -> Vec<Actions> {
    let num = num_cpus::get();
    let aval_worker_num = Arc::new(AtomicUsize::new(num));
    aval_worker_num.fetch_sub(1, Ordering::Relaxed);
    let (_best_score, best_actions) = search(
        status.clone(),
        Vec::new(),
        aval_worker_num,
        maximum_depth,
        specialist,
    );
    best_actions
}

fn search(
    status: Status,
    actions: Vec<Actions>,
    aval_worker_num: Arc<AtomicUsize>,
    maximum_depth: usize,
    specialist: bool,
) -> (Score, Vec<Actions>) {
    let mut threads = Vec::new();
    let mut best_actions = actions.clone();
    let mut best_score = Score::from(&status);

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
            && (!matches!(next_action, Actions::HeartAndSoul) || specialist)
            && stack_seq.len() <= maximum_depth
            && status.is_action_allowed(next_action).is_ok()
        {
            let mut new_s = status.clone();
            new_s.cast_action(next_action);
            if !new_s.is_finished() {
                if best_score.quality != new_s.recipe.quality
                    || best_score.steps >= new_s.step as u16
                {
                    let num = aval_worker_num.load(Ordering::Relaxed);
                    if num > 0
                        && aval_worker_num
                            .compare_exchange(num, num - 1, Ordering::Release, Ordering::Relaxed)
                            .is_ok()
                    {
                        let (status, actions, aval_worker_num) =
                            (new_s.clone(), stack_seq.clone(), aval_worker_num.clone());
                        threads.push(std::thread::spawn(move || {
                            search(status, actions, aval_worker_num, maximum_depth, specialist)
                        }));
                    } else {
                        stack.push((new_s, SKILL_LIST.into_iter()));
                        stack_seq.push(next_action);
                    }
                }
            } else {
                let score = Score::from((&new_s, stack_seq.len()));
                if score > best_score {
                    best_score = score;
                    best_actions = stack_seq.clone();
                }
            }
        }
    }
    aval_worker_num.fetch_add(1, Ordering::Relaxed);
    for (score, actions) in threads.into_iter().map(|x| x.join().unwrap()) {
        if score > best_score {
            best_score = score;
            best_actions = actions;
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
