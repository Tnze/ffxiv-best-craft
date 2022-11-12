use ffxiv_crafting::{Actions, Status};

mod solver;
use solver::*;

pub fn solve(craft: Status) -> Vec<Actions> {
    let phase1_routes = generate_routes_phase1(craft);
    let mut phase2_routes = Vec::new();
    for route in phase1_routes {
        if let Some(route) = generate_routes_phase2(route) {
            phase2_routes.push(route);
        }
    }
    let (ref craft, ref content) = phase2_routes
        .iter()
        .max_by_key(|route| route.0.quality)
        .unwrap();
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
