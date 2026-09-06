//! Milestone 4 ticket 04: one shape for the once-per-turn limits.

use sim::action::{Action, legal_actions};
use sim::cards::{milestone1, starter_decklist};
use sim::engine::apply;
use sim::state::{GameState, Limit, Phase};

fn game(seed: u64) -> GameState {
    let set = milestone1();
    let decklist = starter_decklist(&set);
    let mut state = GameState::new(
        set.db,
        [decklist.clone(), decklist],
        Box::new(sim::rng::SeededRng::new(seed)),
    );
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    state
}

#[test]
fn a_limit_starts_unspent_and_can_be_spent() {
    let mut state = game(9);
    let player = state.current;
    let limit = Limit::SupporterPlayed(player);

    assert!(!state.is_spent(limit));
    state.spend(limit);
    assert!(state.is_spent(limit));
}

#[test]
fn spending_one_limit_leaves_the_others_alone() {
    let mut state = game(9);
    let player = state.current;
    state.spend(Limit::Retreated(player));

    assert!(state.is_spent(Limit::Retreated(player)));
    assert!(!state.is_spent(Limit::EnergyAttached(player)));
    assert!(
        !state.is_spent(Limit::Retreated(player.opponent())),
        "a limit belongs to whoever it names"
    );
}

#[test]
fn attaching_an_energy_spends_that_limit_and_offers_no_second() {
    let mut state = game(9);
    let player = state.current;
    let attach = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::AttachEnergy { .. }))
        .expect("an Energy in hand can be attached");

    apply(&mut state, attach).unwrap();

    assert!(state.is_spent(Limit::EnergyAttached(player)));
    assert!(
        !legal_actions(&state)
            .iter()
            .any(|a| matches!(a, Action::AttachEnergy { .. })),
        "rule 13: one Energy a turn"
    );
}

#[test]
fn a_new_turn_clears_the_turn_limits() {
    let mut state = game(9);
    let player = state.current;
    let attach = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::AttachEnergy { .. }))
        .expect("an Energy in hand can be attached");
    apply(&mut state, attach).unwrap();
    assert!(state.is_spent(Limit::EnergyAttached(player)));

    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }

    assert!(
        !state.is_spent(Limit::EnergyAttached(state.current)),
        "the new turn's player has their attachment back"
    );
}

#[test]
fn a_pokemons_evolution_limit_is_its_own_not_its_players() {
    // Setup fills a Bench only if the opening hand held a second Basic, so
    // the seed is searched rather than assumed.
    let mut state = (1..200)
        .map(game)
        .find(|s| s.player(s.current).in_play().len() >= 2)
        .expect("some seed deals a Bench");
    let player = state.current;
    let in_play = state.player(player).in_play();
    state.spend(Limit::Evolved(in_play[0]));

    assert!(state.is_spent(Limit::Evolved(in_play[0])));
    assert!(
        !state.is_spent(Limit::Evolved(in_play[1])),
        "rule 20 is per Pokémon, not per player"
    );
}
