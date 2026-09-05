//! A text interface for one game.
//!
//! It prints the board, numbers the legal actions, and reads a number. The
//! same list a bot would choose from.

use std::io::{self, BufRead, Write};

use sim::action::{describe, legal_actions, player_to_act};
use sim::cards::{milestone1, starter_decklist};
use sim::engine::apply;
use sim::rng::SeededRng;
use sim::state::GameState;

fn main() {
    let seed = std::env::args()
        .nth(1)
        .and_then(|a| a.parse().ok())
        .unwrap_or(1);

    let set = milestone1();
    let decklist = starter_decklist(&set);
    let mut state = GameState::new(
        set.db,
        [decklist.clone(), decklist],
        Box::new(SeededRng::new(seed)),
    );

    let mut shown = 0;
    let stdin = io::stdin();
    let mut input = stdin.lock();

    while !state.is_over() {
        shown = drain_log(&state, shown);
        print_board(&state);

        let actions = legal_actions(&state);
        let player = player_to_act(&state).expect("an unfinished game waits on someone");
        println!("\n{player:?} to act:");
        for (index, action) in actions.iter().enumerate() {
            println!("  {index}. {}", describe(&state, *action));
        }
        print!("> ");
        io::stdout().flush().expect("stdout accepts a prompt");

        let mut line = String::new();
        if input.read_line(&mut line).unwrap_or(0) == 0 {
            println!("\nStopped.");
            return;
        }
        match line.trim().parse::<usize>() {
            Ok(choice) if choice < actions.len() => {
                apply(&mut state, actions[choice]).expect("a listed action is legal");
            }
            _ => println!("Pick a number from the list."),
        }
    }

    drain_log(&state, shown);
    let outcome = state.outcome.expect("a finished game has an outcome");
    println!("\n{:?} wins — {:?}.", outcome.winner, outcome.reason);
}

fn drain_log(state: &GameState, from: usize) -> usize {
    for line in &state.log[from..] {
        println!("{line}");
    }
    state.log.len()
}

fn print_board(state: &GameState) {
    println!();
    for player in [sim::ids::PlayerId::One, sim::ids::PlayerId::Two] {
        let side = state.player(player);
        let active = match side.active {
            Some(pokemon) => format!(
                "{} {}/{} HP",
                state.pokemon_def(pokemon).name,
                state.remaining_hp(pokemon),
                state.pokemon_def(pokemon).hp
            ),
            None => "none".to_string(),
        };
        let bench: Vec<String> = side
            .bench
            .iter()
            .map(|p| {
                format!(
                    "{} {}/{}",
                    state.pokemon_def(*p).name,
                    state.remaining_hp(*p),
                    state.pokemon_def(*p).hp
                )
            })
            .collect();
        println!(
            "{player:?}: Active {active} | Bench [{}] | hand {} | library {} | prizes {}",
            bench.join(", "),
            side.hand.len(),
            side.library.len(),
            side.prizes.len()
        );
    }
}
