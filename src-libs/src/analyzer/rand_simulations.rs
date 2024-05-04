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

use ffxiv_crafting::{Actions, CastActionError, Status};

pub fn run(status: Status, actions: &[Actions], n: usize) -> Vec<Result<Status, CastActionError>> {
    std::iter::from_fn(|| {
        let mut s = status.clone();
        Some(simulation(&mut s, actions).map(|()| s))
    })
    .take(n)
    .collect()
}

fn simulation(s: &mut Status, actions: &[Actions]) -> Result<(), CastActionError> {
    for action in actions {
        s.is_action_allowed(*action)?;
        s.cast_action(*action);
    }
    Ok(())
}
