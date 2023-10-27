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

use ffxiv_crafting::{Actions, Status};

pub fn solve(status: &Status) -> Vec<Actions> {
    let (_, mut actions) = search(status.clone(), 6, false);
    actions.reverse();
    actions
}

fn search(status: Status, maximum_depth: usize, specialist: bool) -> (i32, Vec<Actions>) {
    if status.progress == status.recipe.difficulty {
        return (status.step, Vec::new());
    } else if status.durability <= 0 || maximum_depth == 0 {
        return (0, Vec::new());
    }
    SKILL_LIST
        .into_iter()
        .filter(|x| status.is_action_allowed(*x).is_ok())
        .filter_map(|x| {
            let mut new_s = status.clone();
            new_s.cast_action(x);
            let (steps, mut actions) = search(new_s, maximum_depth - 1, specialist);
            if steps == 0 {
                None
            } else {
                actions.push(x);
                Some((steps, actions))
            }
        })
        .min_by_key(|x| x.0)
        .unwrap_or((0, Vec::new()))
}

/// 搜索的技能列表
const SKILL_LIST: [Actions; 17] = [
    Actions::BasicSynthesis,
    Actions::MastersMend,
    Actions::Observe,
    Actions::WasteNot,
    Actions::Veneration,
    Actions::WasteNotII,
    Actions::MuscleMemory,
    Actions::CarefulSynthesis,
    Actions::Manipulation,
    Actions::FocusedSynthesis,
    Actions::Groundwork,
    Actions::DelicateSynthesis,
    Actions::IntensiveSynthesis,
    Actions::TrainedEye,
    Actions::PrudentSynthesis,
    Actions::TrainedFinesse,
    Actions::HeartAndSoul,
];
