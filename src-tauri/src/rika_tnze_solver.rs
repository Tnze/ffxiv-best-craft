use ffxiv_crafting::{Actions, Status};

use crate::memory_search_solver;

pub fn solve(craft: Status, mn: bool, wn: usize, obz: bool) -> Vec<Actions> {
    let tnzes_quality_solver = memory_search_solver::Solver::new(craft.clone(), mn, wn, obz);
    let phase1_routes = generate_routes_phase1(craft.clone());
    let mut phase2_routes: Vec<(Status, Vec<Actions>)> = Vec::new();
    let prog_120 = craft.calc_synthesis(1.2);
    let prog_180 = craft.calc_synthesis(1.8);
    let prog_200 = craft.calc_synthesis(2.0);
    for (mut s, mut actions) in phase1_routes {
        let (mut final_actions, final_cp, final_du) = match s.recipe.difficulty - s.progress {
            x if x <= prog_120 => (vec![Actions::BasicSynthesis], 0, 1),
            x if x <= prog_180 => (vec![Actions::CarefulSynthesis], 7, 1),
            x if x <= prog_200 => (vec![Actions::Observe, Actions::FocusedSynthesis], 12, 1),
            _ => continue,
        };
        let mut craft_points = s.craft_points - final_cp;
        let mut durability = s.durability - final_du;
        while let Some(next_action) = tnzes_quality_solver
            .next_touch(craft_points, durability, s.buffs.clone())
            .action
        {
            s.cast_action(next_action);
            craft_points = s.craft_points - final_cp;
            durability = s.durability - final_du;
            actions.push(next_action);
        }
        actions.append(&mut final_actions);
        println!("{actions:?} {} {}", s.is_finished(), s.quality);
        phase2_routes.push((s, actions));
    }
    let res = phase2_routes.into_iter().max_by(|a, b| {
        a.0.progress
            .cmp(&b.0.progress)
            .then(a.0.quality.cmp(&b.0.quality))
            .then(a.0.step.cmp(&b.0.step).reverse())
    });
    let Some((_, content)) = res else {
       return vec![]
    };
    content
}

pub fn next_action_picker_1(craft: &Status) -> Vec<Actions> {
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

pub fn generate_routes_phase1(s: Status) -> Vec<(Status, Vec<Actions>)> {
    let prog_200 = s.calc_synthesis(2.0);
    let mut queue = vec![(s, vec![])];
    let mut routes = Vec::new();
    while let Some((status, actions)) = queue.pop() {
        for action in next_action_picker_1(&status) {
            let mut craft = status.clone();
            craft.cast_action(action);
            let remaining_prog = craft.recipe.difficulty - craft.progress;
            let get_actions = || {
                let mut new_actions = Vec::with_capacity(actions.len() + 1);
                new_actions.clone_from(&actions);
                new_actions.push(action);
                new_actions
            };
            if remaining_prog <= prog_200 && remaining_prog > 0 {
                routes.push((craft, get_actions()));
            } else if craft.step < 8 {
                queue.push((craft, get_actions()));
            }
        }
    }
    routes
}

#[cfg(test)]
mod test {
    use ffxiv_crafting::{Actions, Attributes, Recipe, Status};

    use crate::memory_search_solver;

    use super::solve;

    fn init() -> Status {
        let r = Recipe {
            rlv: 620,
            job_level: 90,
            difficulty: 5720,
            quality: 12900,
            durability: 70,
            conditions_flag: 15,
        };
        let a = Attributes {
            level: 90,
            craftsmanship: 4138,
            control: 3938,
            craft_points: 705,
        };
        Status::new(a, r)
    }

    #[test]
    fn test() {
        let init_status = init();
        let result = solve(init_status.clone(), true, 8, true);
        let quality = {
            let mut status = init_status.clone();
            for action in &result {
                status.cast_action(*action);
            }
            status.quality
        };
        println!("{result:?} {quality}");
    }

    #[test]
    fn test_inner() {
        let mut init_status = init();
        let tnzes_quality_solver =
            memory_search_solver::Solver::new(init_status.clone(), true, 8, true);

        let init_actions = [
            Actions::MuscleMemory,
            Actions::Manipulation,
            Actions::Veneration,
            Actions::WasteNot,
            Actions::Groundwork,
            Actions::Groundwork,
            Actions::CarefulSynthesis,
            Actions::BasicSynthesis,
        ];
        for action in init_actions {
            init_status.cast_action(action);
        }
        let result = tnzes_quality_solver.next_touch(440 - 7, 60 - 1, init_status.buffs);
        println!("{result:?}");
        init_status.craft_points -= 7;
        init_status.durability -= 1;
        while let memory_search_solver::Slot {
            action: Some(action),
            ..
        } = tnzes_quality_solver.next_touch(
            init_status.craft_points,
            init_status.durability,
            init_status.buffs,
        ) {
            println!("{action:?}");
            init_status.cast_action(action);
        }
        println!("{init_status:?}");
        // let quality = {
        //     let mut status = init_status.clone();
        //     for action in &result {
        //         status.cast_action(*action);
        //     }
        //     status.quality
        // };
        // println!("{result:?} {quality}");
    }
}
