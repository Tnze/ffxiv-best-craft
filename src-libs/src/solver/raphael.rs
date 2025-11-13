// This file is part of BestCraft.
// Copyright (C) 2025 Tnze
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

use raphael_simulator::{Action, ActionMask, Settings};
use raphael_solvers::{AtomicFlag, MacroSolver, SolverSettings};

pub fn solve(
    status: Status,
    target_quality: Option<u32>,
    use_manipultaion: bool,
    use_heart_and_soul: bool,
    use_quick_innovation: bool,
    use_trained_eye: bool,
    backload_progress: bool,
    adversarial: bool,
) -> Vec<Actions> {
    let mut allowed_actions = ActionMask::all();
    if !use_heart_and_soul {
        allowed_actions = allowed_actions.remove(Action::HeartAndSoul)
    }
    if !use_quick_innovation {
        allowed_actions = allowed_actions.remove(Action::QuickInnovation)
    }
    if !use_manipultaion {
        allowed_actions = allowed_actions.remove(Action::Manipulation);
    }
    if !use_trained_eye || status.is_action_allowed(Actions::TrainedEye).is_err() {
        allowed_actions = allowed_actions.remove(Action::TrainedEye);
    }
    let target_quality = target_quality.unwrap_or(status.recipe.quality) as u16;
    let simulator_settings = Settings {
        max_cp: status.attributes.craft_points as u16,
        max_durability: status.recipe.durability,
        max_progress: status.recipe.difficulty as u16,
        max_quality: target_quality - status.quality as u16,
        base_progress: status.caches.base_synth as u16,
        base_quality: status.caches.base_touch as u16,
        job_level: status.attributes.level,
        allowed_actions,
        adversarial,
        backload_progress,
    };
    let solver_settings = SolverSettings {
        simulator_settings,
        allow_non_max_quality_solutions: true,
    };
    let mut solver = MacroSolver::new(
        solver_settings,
        Box::new(|_| {}),
        Box::new(|_| {}),
        AtomicFlag::new(),
    );
    solver
        .solve()
        .into_iter()
        .flatten()
        .map(map_action)
        .collect()
}

fn map_action(action: Action) -> Actions {
    match action {
        Action::BasicSynthesis => Actions::BasicSynthesis,
        Action::BasicTouch => Actions::BasicTouch,
        Action::MasterMend => Actions::MastersMend,
        Action::Observe => Actions::Observe,
        Action::WasteNot => Actions::WasteNot,
        Action::Veneration => Actions::Veneration,
        Action::StandardTouch => Actions::StandardTouch,
        Action::GreatStrides => Actions::GreatStrides,
        Action::Innovation => Actions::Innovation,
        Action::WasteNot2 => Actions::WasteNotII,
        Action::ByregotsBlessing => Actions::ByregotsBlessing,
        Action::PreciseTouch => Actions::PreciseTouch,
        Action::MuscleMemory => Actions::MuscleMemory,
        Action::CarefulSynthesis => Actions::CarefulSynthesis,
        Action::Manipulation => Actions::Manipulation,
        Action::PrudentTouch => Actions::PrudentTouch,
        Action::AdvancedTouch => Actions::AdvancedTouch,
        Action::Reflect => Actions::Reflect,
        Action::PreparatoryTouch => Actions::PreparatoryTouch,
        Action::Groundwork => Actions::Groundwork,
        Action::DelicateSynthesis => Actions::DelicateSynthesis,
        Action::IntensiveSynthesis => Actions::IntensiveSynthesis,
        Action::PrudentSynthesis => Actions::PrudentSynthesis,
        Action::TrainedFinesse => Actions::TrainedFinesse,
        Action::ImmaculateMend => Actions::ImmaculateMend,
        Action::TrainedPerfection => Actions::TrainedPerfection,
        Action::TrainedEye => Actions::TrainedEye,

        Action::TricksOfTheTrade => Actions::TricksOfTheTrade,
        Action::RefinedTouch => Actions::RefinedTouch,

        Action::HeartAndSoul => Actions::HeartAndSoul,
        Action::QuickInnovation => Actions::QuickInnovation,
    }
}
