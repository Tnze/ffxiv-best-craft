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

#[derive(Default)]
pub struct Scope {
    craftsmanship_range: Option<(i32, i32)>,
    control_range: Option<(i32, i32)>,
    craft_points: i32,
}

pub fn calc_scope(init_status: Status, actions: &[Actions]) -> Scope {
    let final_status = simulate(init_status.clone(), actions);
    Scope {
        craftsmanship_range: find_craftsmanship_range(&init_status, &final_status, actions),
        control_range: find_control_range(),
        craft_points: init_status.craft_points - final_status.craft_points,
    }
}

fn simulate(mut status: Status, actions: &[Actions]) -> Status {
    actions.iter().for_each(|a| {
        if status.is_action_allowed(*a).is_ok() {
            status.cast_action(*a);
        }
    });
    status
}

fn find_craftsmanship_range(
    init_status: &Status,
    final_status: &Status,
    actions: &[Actions],
) -> Option<(i32, i32)> {
    let init_craftsmanship = init_status.attributes.craftsmanship;
    let mut low = None;
    for cm in (0..init_craftsmanship).rev() {
        let mut status = init_status.clone();
        status.attributes.craftsmanship = cm;
        status = simulate(status, actions);
        if status.progress < status.recipe.difficulty {
            break;
        }
        low = Some(cm);
    }
    let mut high = None;
    for cm in (init_craftsmanship..).take(5000) {
        let mut status = init_status.clone();
        status.attributes.craftsmanship = cm;
        status = simulate(status, actions);
        if status.step != final_status.step {
            break;
        }
        high = Some(cm);
    }
    Some((low?, high?))
}

fn find_control_range() -> Option<(i32, i32)> {
    todo!()
}
