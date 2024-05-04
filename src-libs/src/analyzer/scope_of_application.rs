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

pub struct Scope {
    min_craftsmanship: i32,
    max_craftsmanship: i32,

    min_control: i32,
    max_control: i32,

    craft_points: i32,
}

pub fn calc_scope(init_status: Status, actions: &[Actions]) -> Scope {
    let mut final_status = init_status.clone();
    actions.iter().for_each(|a| final_status.cast_action(*a));
    todo!()
}
