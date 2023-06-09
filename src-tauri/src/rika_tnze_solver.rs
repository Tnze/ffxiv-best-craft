use ffxiv_crafting::{Actions, Status};

use crate::{memoization_solver, solver::Score};

pub fn solve(craft: Status, mn: bool, wn: usize, obz: bool, reduce_steps: bool) -> Vec<Actions> {
    let tnzes_quality_solver = memoization_solver::Solver::new(craft.clone(), mn, wn, obz);
    let phase1_routes = generate_routes_phase1(&craft, mn);
    let mut phase2_routes = Vec::new();
    let prog_120 = craft.calc_synthesis(1.2);
    let prog_180 = craft.calc_synthesis(1.8);
    let prog_200 = craft.calc_synthesis(2.0);
    for (s, actions) in phase1_routes {
        let (final_actions, final_cp, final_du) = match s.recipe.difficulty - s.progress {
            x if x <= prog_120 => (vec![Actions::BasicSynthesis], 0, 1),
            x if x <= prog_180 => (vec![Actions::CarefulSynthesis], 7, 1),
            x if x <= prog_200 => (vec![Actions::Observe, Actions::FocusedSynthesis], 12, 1),
            _ => continue,
        };
        let craft_points = s.craft_points - final_cp;
        let durability = s.durability.saturating_sub(final_du);
        'rs: for mut cp in (0..=craft_points).rev() {
            for mut du in (1..=durability).filter(|x| x % 5 == 0).rev() {
                let mut s = s.clone();
                s.craft_points = cp;
                s.durability = du;
                let mut actions = actions.clone();
                while let Some(next_action) =
                    tnzes_quality_solver.next_touch(cp, du, s.buffs).action
                {
                    if s.is_action_allowed(next_action).is_err() {
                        break;
                    }
                    s.cast_action(next_action);
                    cp = s.craft_points;
                    du = s.durability;
                    actions.push(next_action);
                }
                let r#continue = s.quality >= s.recipe.quality;
                actions.append(&mut final_actions.clone());
                phase2_routes.push((Score::from(&s), actions));
                if !reduce_steps && !r#continue {
                    break 'rs;
                }
            }
        }
    }
    phase2_routes
        .into_iter()
        .max_by(|a, b| a.0.cmp(&b.0))
        .map_or_else(|| Vec::new(), |x| x.1)
}

pub fn next_action_picker_1(
    craft: Status,
    use_manipulation: bool,
) -> Box<dyn Iterator<Item = Actions>> {
    if craft.is_finished() {
        return Box::new(std::iter::empty());
    }
    let mut available_actions = Vec::new();
    if craft.step == 0 {
        return Box::new(std::iter::once(Actions::MuscleMemory));
    }
    if use_manipulation && craft.buffs.manipulation == 0 {
        return Box::new(std::iter::once(Actions::Manipulation));
    }
    if craft.buffs.veneration == 0 {
        // return Box::new(std::iter::once(Actions::Veneration));
        available_actions.push(Actions::Veneration);
    }
    if craft.buffs.wast_not == 0 {
        available_actions.push(Actions::WasteNot);
        available_actions.push(Actions::WasteNotII);
    }

    if craft.buffs.wast_not > 0 || craft.buffs.muscle_memory > 0 {
        available_actions.push(Actions::Groundwork)
    }
    if craft.buffs.wast_not == 0 {
        available_actions.push(Actions::PrudentSynthesis)
    }
    available_actions.append(&mut vec![
        Actions::BasicSynthesis,
        Actions::CarefulSynthesis,
        Actions::DelicateSynthesis,
    ]);
    let allowed = available_actions
        .into_iter()
        .filter(move |x| craft.is_action_allowed(*x).is_ok());
    Box::new(allowed)
}

pub fn generate_routes_phase1(s: &Status, use_manipulation: bool) -> Vec<(Status, Vec<Actions>)> {
    let prog_200 = s.calc_synthesis(2.0);
    let mut queue = vec![(s.clone(), vec![])];
    let mut routes = Vec::new();
    while let Some((status, actions)) = queue.pop() {
        for action in next_action_picker_1(status.clone(), use_manipulation) {
            let mut s = status.clone();
            if s.is_action_allowed(action).is_err() {
                continue;
            }
            s.cast_action(action);
            let remaining_prog = s.recipe.difficulty - s.progress;
            let get_actions = || {
                let mut new_actions = Vec::with_capacity(actions.len() + 1);
                new_actions.clone_from(&actions);
                new_actions.push(action);
                new_actions
            };
            if remaining_prog <= prog_200 && remaining_prog > 0 {
                routes.push((s, get_actions()));
            } else if s.step < 8 {
                queue.push((s, get_actions()));
            }
        }
    }
    routes
}
