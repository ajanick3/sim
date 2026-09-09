//! Play out one game between two real decklists, autonomously — a Strategy
//! per seat, no input during the run, a full transcript printed as it goes.
//!
//! ```sh
//! cargo run --bin play -- deck-a.txt deck-b.txt [seed]
//! ```

use sim::action::{describe, legal_actions, player_to_act};
use sim::decklist::{self, Report};
use sim::engine::apply;
use sim::import;
use sim::rng::SeededRng;
use sim::state::{GameState, WinReason};
use sim::strategy::{HeuristicStrategy, Strategy};
use sim::view::PlayerView;

/// The turn cap `selfplay` already trusts, kept as one backstop so a
/// pathological game can never hang the run.
const STEP_CAP: u32 = 5_000;

fn main() {
    let mut args = std::env::args().skip(1);
    let (Some(deck_a_path), Some(deck_b_path)) = (args.next(), args.next()) else {
        eprintln!("usage: play <deck-a.txt> <deck-b.txt> [seed]");
        std::process::exit(2);
    };
    let seed: u64 = match args.next() {
        Some(raw) => match raw.parse() {
            Ok(seed) => seed,
            Err(_) => {
                eprintln!("seed must be a number, got: {raw}");
                std::process::exit(2);
            }
        },
        None => 0x5EED,
    };

    let artifact = read_or_exit("data/cards.json");
    let mut import = import::load(&artifact).unwrap_or_else(|error| {
        eprintln!("data/cards.json: {error}");
        std::process::exit(1);
    });

    let deck_a_text = read_or_exit(&deck_a_path);
    let deck_b_text = read_or_exit(&deck_b_path);
    let deck_a_list = decklist::parse(&deck_a_text);
    let deck_b_list = decklist::parse(&deck_b_text);
    let deck_a_report = decklist::check(&deck_a_list, &import);
    let deck_b_report = decklist::check(&deck_b_list, &import);

    let mut refused = false;
    refuse_unless_playable(&deck_a_path, &deck_a_report, &mut refused);
    refuse_unless_playable(&deck_b_path, &deck_b_report, &mut refused);
    if refused {
        std::process::exit(1);
    }

    let deck_a = decklist::to_deck(&deck_a_list, &deck_a_report, &mut import);
    let deck_b = decklist::to_deck(&deck_b_list, &deck_b_report, &mut import);

    let mut strategy_a: Box<dyn Strategy> =
        Box::new(HeuristicStrategy::new(Box::new(SeededRng::new(seed))));
    let mut strategy_b: Box<dyn Strategy> =
        Box::new(HeuristicStrategy::new(Box::new(SeededRng::new(seed.wrapping_add(1)))));

    let mut state = GameState::new(
        import.db,
        [deck_a, deck_b],
        Box::new(SeededRng::new(seed)),
    );

    println!("P1: {deck_a_path}");
    println!("P2: {deck_b_path}");
    println!("seed: {seed}");
    println!("P1 Strategy: heuristic");
    println!("P2 Strategy: heuristic");
    println!();

    let mut steps = 0;
    while !state.is_over() && steps < STEP_CAP {
        let Some(player) = player_to_act(&state) else {
            break;
        };
        let legal = legal_actions(&state);
        if legal.is_empty() {
            break;
        }
        let view = PlayerView::of(&state, player);
        let db = &state.db;
        let strategy = if player == sim::ids::PlayerId::One { &mut strategy_a } else { &mut strategy_b };
        let action = strategy.choose(&view, db, &legal);
        println!("{player:?}: {}", describe(&state, action));
        apply(&mut state, action).expect("a Strategy chose from legal_actions itself");
        steps += 1;
    }

    println!();
    match state.outcome {
        Some(outcome) => {
            println!(
                "{:?} wins ({}) in {} turns.",
                outcome.winner,
                reason(outcome.reason),
                state.turn_number
            );
        }
        None => {
            println!("Did not finish: {steps} steps, no winner decided.");
        }
    }
}

fn reason(reason: WinReason) -> &'static str {
    match reason {
        WinReason::AllPrizesTaken => "took every Prize",
        WinReason::NoPokemonInPlay => "opponent had no Pokémon left",
        WinReason::CouldNotDraw => "opponent could not draw",
    }
}

fn refuse_unless_playable(path: &str, report: &Report, refused: &mut bool) {
    if report.is_legal() && report.is_playable() {
        return;
    }
    *refused = true;
    eprintln!("{path}: not fully playable");
    for problem in &report.problems {
        eprintln!("  {problem:?}");
    }
    if !report.is_playable() {
        eprintln!(
            "  only {} of {} cards resolve to something the engine plays",
            report.playable, report.total
        );
    }
}

fn read_or_exit(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| {
        eprintln!("{path}: {error}");
        std::process::exit(1);
    })
}
