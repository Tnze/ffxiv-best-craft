use ffxiv_crafting::{Actions, Buffs, Status};
use std::collections::VecDeque;

macro_rules! action_vec {
    ($($tt:expr),*) => { vec![ $(Some(&$tt),)*]};
}

pub fn next_action_picker_1<'a>(craft: &Status) -> Vec<Option<&'a Actions>> {
    if craft.is_finished() {
        return vec![None];
    }
    let mut available_actions = Vec::<Option<&'a Actions>>::new();
    let mut forbidden_actions = Vec::<Option<&'a Actions>>::new();
    match craft.step {
        0 => return action_vec![Actions::MuscleMemory],
        1 => return action_vec![Actions::Manipulation],
        2 => return action_vec![Actions::Veneration],
        3 => available_actions.append(&mut action_vec![Actions::WasteNotII]),
        _ => {}
    }
    if craft.buffs.wast_not > 0 || craft.buffs.muscle_memory > 0 {
        available_actions.append(&mut action_vec![Actions::Groundwork])
    }
    if craft.buffs.muscle_memory > 0 {
        forbidden_actions.append(&mut action_vec![
            Actions::BasicSynthesis,
            Actions::CarefulSynthesis,
            Actions::PrudentSynthesis,
            Actions::DelicateSynthesis
        ])
    }
    if craft.buffs.wast_not > 0 {
        forbidden_actions.append(&mut action_vec![Actions::PrudentSynthesis])
    }
    available_actions.append(&mut action_vec![
        Actions::BasicSynthesis,
        Actions::CarefulSynthesis,
        Actions::PrudentSynthesis,
        Actions::DelicateSynthesis
    ]);
    let mut result_actions = Vec::<Option<&'a Actions>>::new();
    for action in available_actions {
        if !forbidden_actions.contains(&action)
            && craft.is_action_allowed(*action.unwrap()).is_ok()
            && !result_actions.iter().any(|x| x.unwrap() == action.unwrap())
        {
            result_actions.push(action);
        }
    }
    result_actions
}

pub fn generate_routes_phase1(craft: Status) -> Vec<(Status, Vec<Actions>)> {
    let mut queue = Vec::new();
    let (prog_120, prog_180, prog_200) = {
        let mut craft = craft.clone();
        craft.buffs = Buffs::default();
        (
            craft.calc_synthesis(1.2),
            craft.calc_synthesis(1.8),
            craft.calc_synthesis(2.0),
        )
    };
    queue.push((craft, vec![]));
    let mut routes = Vec::new();
    while !queue.is_empty() {
        let (craft, mut actions) = queue.pop().unwrap();
        for action in next_action_picker_1(&craft) {
            let mut craft = craft.clone();
            craft.cast_action(*action.unwrap());
            actions.push(*action.unwrap());
            let remaining_prog = craft.recipe.difficulty - craft.progress;
            if remaining_prog <= prog_200 {
                craft.craft_points -= match remaining_prog {
                    0 => 0,
                    x if x <= prog_120 => 0,
                    x if x <= prog_180 => 7,
                    x if x <= prog_200 => 12,
                    _ => 0,
                };
                craft.durability -= 10;
                routes.push((craft, actions.clone()));
                continue;
            }

            if craft.step < 8 {
                queue.push((craft, actions.clone()));
            }
            actions.pop();
        }
    }
    routes
}

pub fn next_action_phase_2<'a>(craft: &Status) -> Vec<Option<&'a Actions>> {
    let mut available_actions = vec![
        &Actions::BasicTouch,
        &Actions::PrudentTouch,
        &Actions::PreparatoryTouch,
    ];
    let mut forbidden_actions = Vec::new();
    if craft.is_finished() {
        return vec![None];
    }
    if craft.buffs.innovation > 0 {
        forbidden_actions.push(&Actions::Innovation);
    } else {
        if craft.buffs.inner_quiet >= 2 {
            forbidden_actions.append(&mut vec![
                &Actions::BasicTouch,
                &Actions::StandardTouch,
                &Actions::AdvancedTouch,
                &Actions::TrainedFinesse,
                &Actions::PrudentTouch,
                &Actions::PreparatoryTouch,
                &Actions::ByregotsBlessing,
            ]);
        }
        available_actions.push(&Actions::Innovation);
    }
    if craft.buffs.manipulation == 0
        && craft.buffs.touch_combo_stage == 0
        && craft.buffs.inner_quiet < 8
    {
        available_actions.push(&Actions::Manipulation);
    }
    if craft.buffs.wast_not > 0 {
        available_actions.push(&Actions::PreparatoryTouch);
        forbidden_actions.push(&Actions::PrudentTouch);
    } else {
        available_actions.push(&Actions::PrudentTouch);
        forbidden_actions.push(&Actions::PreparatoryTouch);
    }
    if craft.buffs.touch_combo_stage > 0 {
        available_actions.push(&Actions::StandardTouch);
        forbidden_actions.push(&Actions::BasicTouch);
    }
    if craft.buffs.touch_combo_stage > 1 {
        available_actions.push(&Actions::AdvancedTouch);
        forbidden_actions.push(&Actions::BasicTouch);
    }
    if craft.buffs.inner_quiet >= 10 {
        available_actions.push(&Actions::TrainedFinesse);
        available_actions.push(&Actions::GreatStrides);
    }
    if craft.buffs.great_strides > 0 {
        forbidden_actions.push(&Actions::TrainedFinesse);
        forbidden_actions.push(&Actions::GreatStrides);
        if craft.buffs.innovation > 0 {
            available_actions.push(&Actions::ByregotsBlessing);
        }
    }
    let mut final_actions: Vec<Option<&Actions>> = Vec::new();
    //drop duplicates

    for action in available_actions {
        if !forbidden_actions.contains(&action)
            && !final_actions.iter().any(|a| a.unwrap() == action)
        {
            final_actions.push(Some(action));
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
    while !queue.is_empty() {
        let (_craft, _route) = queue.pop_front().unwrap();
        for action in next_action_phase_2(&_craft) {
            if _craft.is_finished()
                || action.is_none()
                || _craft.is_action_allowed(*action.unwrap()).is_err()
            {
                continue;
            }
            let mut craft = _craft.clone();
            let mut route = _route.clone();
            craft.cast_action(*action.unwrap());
            route.push(*action.unwrap());
            if let &Actions::ByregotsBlessing = action.unwrap() {
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
