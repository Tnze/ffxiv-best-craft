use ffxiv_crafting::{Actions, Buffs, Status};
use std::collections::VecDeque;

pub fn solve(craft: Status) -> Vec<Actions> {
    let phase1_routes = generate_routes_phase1(craft);
    let mut phase2_routes = Vec::new();
    for route in phase1_routes {
        if let Some(route) = generate_routes_phase2(route) {
            phase2_routes.push(route);
        }
    }
    let res = phase2_routes.iter().max_by(|a, b| {
        a.0.progress
            .cmp(&b.0.progress)
            .then(a.0.quality.cmp(&b.0.quality))
            .then(a.0.step.cmp(&b.0.step).reverse())
    });
    let (ref craft, ref content) = match res {
        None => return vec![],
        Some(x) => x,
    };
    let mut content = content.clone();
    let prog_120 = craft.calc_synthesis(1.2);
    let prog_180 = craft.calc_synthesis(1.8);
    let prog_200 = craft.calc_synthesis(2.0);
    content.append(&mut match craft.recipe.difficulty - craft.progress {
        x if x <= prog_120 => vec![Actions::BasicSynthesis],
        x if x <= prog_180 => vec![Actions::CarefulSynthesis],
        x if x <= prog_200 => vec![Actions::Observe, Actions::FocusedSynthesis],
        _ => vec![],
    });
    content
}

pub fn next_action_picker_1<'a>(craft: &Status) -> Vec<Actions> {
    if craft.is_finished() {
        return vec![];
    }
    let mut available_actions = Vec::new();
    let mut forbidden_actions = Vec::new();
    match craft.step {
        0 => return vec![Actions::MuscleMemory],
        1 => return vec![Actions::Manipulation],
        2 => return vec![Actions::Veneration],
        3 => available_actions.push(Actions::WasteNotII),
        _ => {}
    }
    if craft.buffs.wast_not > 0 || craft.buffs.muscle_memory > 0 {
        available_actions.push(Actions::Groundwork)
    }
    if craft.buffs.muscle_memory > 0 {
        forbidden_actions.append(&mut vec![
            Actions::BasicSynthesis,
            Actions::CarefulSynthesis,
            Actions::PrudentSynthesis,
            Actions::DelicateSynthesis,
        ])
    }
    if craft.buffs.wast_not > 0 {
        forbidden_actions.push(Actions::PrudentSynthesis)
    }
    available_actions.append(&mut vec![
        Actions::BasicSynthesis,
        Actions::CarefulSynthesis,
        Actions::PrudentSynthesis,
        Actions::DelicateSynthesis,
    ]);
    let mut result_actions = Vec::new();
    for action in available_actions {
        if !forbidden_actions.contains(&action)
            && craft.is_action_allowed(action).is_ok()
            && !result_actions.contains(&action)
        {
            result_actions.push(action);
        }
    }
    result_actions
}

pub fn generate_routes_phase1(craft: Status) -> Vec<(Status, Vec<Actions>)> {
    let (prog_120, prog_180, prog_200) = {
        let mut craft = craft.clone();
        craft.buffs = Buffs::default();
        (
            craft.calc_synthesis(1.2),
            craft.calc_synthesis(1.8),
            craft.calc_synthesis(2.0),
        )
    };
    let mut queue = vec![(craft, vec![])];
    let mut routes = Vec::new();
    while let Some((craft, actions)) = queue.pop() {
        for action in next_action_picker_1(&craft) {
            let mut craft = craft.clone();
            craft.cast_action(action);
            let remaining_prog = craft.recipe.difficulty - craft.progress;
            let get_actions = || {
                let mut new_actions = Vec::with_capacity(actions.len() + 1);
                new_actions.clone_from(&actions);
                new_actions.push(action);
                new_actions
            };
            if remaining_prog <= prog_200 {
                craft.craft_points -= match remaining_prog {
                    0 => 0,
                    x if x <= prog_120 => 0,
                    x if x <= prog_180 => 7,
                    x if x <= prog_200 => 12,
                    _ => 0,
                };
                craft.durability -= 10;
                routes.push((craft, get_actions()));
            } else if craft.step < 8 {
                queue.push((craft, get_actions()));
            }
        }
    }
    routes
}

pub fn next_action_phase_2(craft: &Status) -> Vec<Actions> {
    let mut available_actions = vec![
        Actions::BasicTouch,
        Actions::PrudentTouch,
        Actions::PreparatoryTouch,
    ];
    let mut forbidden_actions = Vec::new();
    if craft.is_finished() {
        return vec![];
    }
    if craft.buffs.innovation > 0 {
        forbidden_actions.push(Actions::Innovation);
    } else {
        if craft.buffs.inner_quiet >= 2 {
            forbidden_actions.append(&mut vec![
                Actions::BasicTouch,
                Actions::StandardTouch,
                Actions::AdvancedTouch,
                Actions::TrainedFinesse,
                Actions::PrudentTouch,
                Actions::PreparatoryTouch,
                Actions::ByregotsBlessing,
            ]);
        }
        available_actions.push(Actions::Innovation);
    }
    if craft.buffs.manipulation == 0
        && craft.buffs.touch_combo_stage == 0
        && craft.buffs.inner_quiet < 8
    {
        available_actions.push(Actions::Manipulation);
    }
    if craft.buffs.wast_not > 0 {
        available_actions.push(Actions::PreparatoryTouch);
        forbidden_actions.push(Actions::PrudentTouch);
    } else {
        available_actions.push(Actions::PrudentTouch);
        forbidden_actions.push(Actions::PreparatoryTouch);
    }
    if craft.buffs.touch_combo_stage > 0 {
        available_actions.push(Actions::StandardTouch);
        forbidden_actions.push(Actions::BasicTouch);
    }
    if craft.buffs.touch_combo_stage > 1 {
        available_actions.push(Actions::AdvancedTouch);
        forbidden_actions.push(Actions::BasicTouch);
    }
    if craft.buffs.inner_quiet >= 10 {
        available_actions.push(Actions::TrainedFinesse);
        available_actions.push(Actions::GreatStrides);
    }
    if craft.buffs.great_strides > 0 {
        forbidden_actions.push(Actions::TrainedFinesse);
        forbidden_actions.push(Actions::GreatStrides);
        if craft.buffs.innovation > 0 {
            available_actions.push(Actions::ByregotsBlessing);
        }
    }
    let mut final_actions: Vec<Actions> = Vec::new();
    //drop duplicates

    for action in available_actions {
        if !forbidden_actions.contains(&action) && !final_actions.iter().any(|a| *a == action) {
            final_actions.push(action);
        }
    }
    final_actions
}

pub fn generate_routes_phase2(
    (craft, route): (Status, Vec<Actions>),
) -> Option<(Status, Vec<Actions>)> {
    let mut queue = VecDeque::new();
    queue.push_back((craft, route));
    let mut top_route: Option<(Status, Vec<Actions>)> = None;
    while let Some((_craft, _route)) = queue.pop_front() {
        for action in next_action_phase_2(&_craft) {
            if _craft.is_finished() || _craft.is_action_allowed(action).is_err() {
                continue;
            }
            let mut craft = _craft.clone();
            let mut route = _route.clone();
            craft.cast_action(action);
            route.push(action);
            if let Actions::ByregotsBlessing = action {
                if let Some(top_route) = &mut top_route {
                    if top_route.0.quality < craft.quality {
                        *top_route = (craft, route);
                    }
                } else {
                    top_route = Some((craft, route));
                }
            } else {
                queue.push_back((craft, route));
            }
        }
    }
    top_route
}
