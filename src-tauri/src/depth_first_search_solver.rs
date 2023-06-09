use std::cmp::Ordering;

use ffxiv_crafting::{Actions, Status};

/// 进行一次深度优先搜索（DFS）
///
/// status为开始制作时的初始状态
/// maximum_depth为限制最深搜索深度
pub fn solve(status: &Status, maximum_depth: u16, specialist: bool) -> Vec<Actions> {
    let mut best_list = Vec::new();
    let mut best_score = Score::from(status);
    search(
        &status,
        0,
        maximum_depth,
        &mut Vec::with_capacity(maximum_depth as usize),
        &mut best_list,
        &mut best_score,
        specialist,
    );
    fn search(
        status: &Status,
        current_depth: u16,
        maximum_depth: u16,
        stack_seq: &mut Vec<Actions>,
        best_seq: &mut Vec<Actions>,
        best_score: &mut Score,
        specialist: bool,
    ) {
        let score = Score::from(status);
        if score > *best_score {
            *best_score = score;
            *best_seq = stack_seq.clone();
        }
        // 简单的剪枝，排除比当前最优解更深的分支。
        let is_best_quality_full = best_score.quality >= status.recipe.quality;
        let is_this_steps_longer = current_depth >= best_score.steps;
        if is_best_quality_full && is_this_steps_longer || current_depth > maximum_depth {
            return;
        }
        for sk in SKILL_LIST {
            if matches!(sk, Actions::FocusedSynthesis | Actions::FocusedTouch if status.buffs.observed > 0)
                || matches!(sk, Actions::HeartAndSoul if !specialist)
            {
                continue;
            }
            let mut new_s = status.clone();
            if new_s.is_action_allowed(sk).is_ok() && new_s.success_rate(sk) == 100 {
                new_s.cast_action(sk);
                stack_seq.push(sk);
                search(
                    &new_s,
                    current_depth + 1,
                    maximum_depth,
                    stack_seq,
                    best_seq,
                    best_score,
                    specialist,
                );
                stack_seq.pop();
            }
        }
    }
    best_list
}

#[derive(PartialEq)]
struct Score {
    quality: u32,
    prgress: u16,
    steps: u16,
}

impl From<&Status> for Score {
    fn from(s: &Status) -> Self {
        Self {
            quality: s.quality,
            prgress: s.progress,
            steps: s.step as u16,
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.prgress
                .cmp(&other.prgress)
                .then_with(|| self.quality.cmp(&other.quality))
                .then_with(|| self.steps.cmp(&other.steps).reverse()),
        )
    }
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
