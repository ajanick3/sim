//! Headless self-play, to measure what a masked view costs.
//!
//! ```sh
//! cargo run --release --bin selfplay -- 2000        # bots read the state
//! cargo run --release --bin selfplay -- 2000 views  # bots read a view
//! ```

use std::time::Instant;

use sim::action::{Action, legal_actions, player_to_act};
use sim::cards::{milestone1, starter_decklist};
use sim::engine::apply;
use sim::rng::{Rng, SeededRng};
use sim::state::GameState;
use sim::view::PlayerView;

fn main() {
    let mut args = std::env::args().skip(1);
    let games: u64 = args.next().and_then(|a| a.parse().ok()).unwrap_or(1000);
    let with_views = args.next().is_some_and(|a| a == "views");

    let mut policy = SeededRng::new(0x5EED);
    let mut decisions: u64 = 0;
    let mut finished = 0;
    let started = Instant::now();

    for seed in 0..games {
        let set = milestone1();
        let decklist = starter_decklist(&set);
        let mut state = GameState::new(
            set.db,
            [decklist.clone(), decklist],
            Box::new(SeededRng::new(seed)),
        );

        let mut steps = 0;
        while !state.is_over() && steps < 5_000 {
            let Some(player) = player_to_act(&state) else {
                break;
            };
            // The mask under test: a bot that reads a view builds one per
            // decision, exactly as a real bot would.
            if with_views {
                let view = PlayerView::of(&state, player);
                std::hint::black_box(&view);
            }
            let actions = legal_actions(&state);
            let choice = pick(&mut policy, &actions);
            apply(&mut state, choice).expect("a listed action is legal");
            decisions += 1;
            steps += 1;
        }
        if state.is_over() {
            finished += 1;
        }
    }

    let elapsed = started.elapsed();
    let per_decision = elapsed.as_secs_f64() / decisions as f64;
    println!(
        "{games} games, {finished} finished, {decisions} decisions in {:.3}s",
        elapsed.as_secs_f64()
    );
    println!(
        "{:.0} games/s, {:.2} us/decision, views: {with_views}",
        games as f64 / elapsed.as_secs_f64(),
        per_decision * 1e6
    );
}

/// Attack when you can; otherwise take a random legal action. A game of pure
/// random play rarely ends, and the measurement wants finished games.
fn pick(policy: &mut SeededRng, actions: &[Action]) -> Action {
    if let Some(attack) = actions.iter().find(|a| matches!(a, Action::Attack { .. })) {
        return *attack;
    }
    actions[policy.next_below(actions.len() as u32) as usize]
}
