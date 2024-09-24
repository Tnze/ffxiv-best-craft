// This file is part of BestCraft.
// Copyright (C) 2024 Tnze
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

use raphael_simulator::{Action, ActionMask, Settings, SimulationState};
use raphael_solvers::MacroSolver;

pub fn solve(
    status: Status,
    use_manipultaion: bool,
    use_heart_and_soul: bool,
    use_quick_innovation: bool,
    backload_progress: bool,
) -> Vec<Actions> {
    let mut allowed_actions = ActionMask::from_level(status.attributes.level);
    if !use_heart_and_soul {
        allowed_actions = allowed_actions.remove(Action::HeartAndSoul)
    }
    if !use_quick_innovation {
        allowed_actions = allowed_actions.remove(Action::QuickInnovation)
    }
    if !use_manipultaion {
        allowed_actions = allowed_actions.remove(Action::Manipulation);
    }
    if status.is_action_allowed(Actions::TrainedEye).is_err() {
        allowed_actions = allowed_actions.remove(Action::TrainedEye);
    }
    let settings = Settings {
        max_cp: status.attributes.craft_points as i16,
        max_durability: status.recipe.durability as i8,
        max_progress: status.recipe.difficulty as u16,
        max_quality: status.recipe.quality as u16 - status.quality as u16,
        base_progress: status.caches.base_synth as u16,
        base_quality: status.caches.base_touch as u16,
        job_level: status.attributes.level,
        allowed_actions,
        adversarial: false,
    };
    let state = SimulationState::new(&settings);
    let mut solver = MacroSolver::new(
        settings,
        backload_progress,
        false,
        Box::new(|_| {}),
        Box::new(|_| {}),
    );
    solver
        .solve(state)
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

        Action::ComboAdvancedTouch => Actions::AdvancedTouch,
        Action::ComboRefinedTouch => Actions::RefinedTouch,
        Action::ComboStandardTouch => Actions::StandardTouch,

        Action::HeartAndSoul => Actions::HeartAndSoul,
        Action::QuickInnovation => Actions::QuickInnovation,
    }
}
