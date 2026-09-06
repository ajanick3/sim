//! Milestone 4 ticket 01: the action log.

use sim::action::{Action, legal_actions, player_to_act};
use sim::cards::{milestone1, starter_decklist};
use sim::engine::{apply, replay};
use sim::ids::PlayerId;
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

fn deal(seed: u64) -> GameState {
    let set = milestone1();
    let decklist = starter_decklist(&set);
    GameState::new(
        set.db,
        [decklist.clone(), decklist],
        Box::new(SeededRng::new(seed)),
    )
}

/// Everything a replay must reproduce. `GameState` cannot derive `PartialEq`
/// — it holds a generator behind a trait object — so "the same state" is
/// spelled out here rather than assumed.
fn fingerprint(state: &GameState) -> String {
    let mut out = format!(
        "turn={} current={:?} phase={:?} outcome={:?}\n",
        state.turn_number, state.current, state.phase, state.outcome
    );
    for player in [PlayerId::One, PlayerId::Two] {
        let side = state.player(player);
        out += &format!(
            "{player:?} library={:?} hand={:?} discard={:?} prizes={:?} active={:?} bench={:?}\n",
            side.library, side.hand, side.discard, side.prizes, side.active, side.bench
        );
    }
    for pokemon in &state.pokemon {
        out += &format!(
            "  cards={:?} dmg={} attached={:?} conditions={:?} ko={}\n",
            pokemon.cards,
            pokemon.damage,
            pokemon.attached,
            pokemon.conditions,
            pokemon.knocked_out
        );
    }
    out += &state.log.join("\n");
    out
}

/// Attack when you can, otherwise take the first thing offered.
fn choose(state: &GameState) -> Action {
    let actions = legal_actions(state);
    actions
        .iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .copied()
        .unwrap_or(actions[0])
}

#[test]
fn a_fresh_game_has_an_empty_log() {
    let state = deal(9);
    assert!(state.history.is_empty(), "dealing is not an action");
}

#[test]
fn every_applied_action_is_recorded_in_order() {
    let mut state = deal(9);
    let mut expected = Vec::new();
    for _ in 0..6 {
        let action = choose(&state);
        apply(&mut state, action).unwrap();
        expected.push(action);
    }
    assert_eq!(state.history, expected);
}

#[test]
fn a_refused_action_is_not_recorded() {
    let mut state = deal(9);
    // The first turn has no attack step (rule 17), so this is refused.
    let refused = Action::Attack { index: 0 };
    assert!(!legal_actions(&state).contains(&refused));

    let before = state.history.clone();
    assert!(apply(&mut state, refused).is_err());
    assert_eq!(state.history, before, "a refusal changed nothing to record");
}

#[test]
fn a_game_replays_from_its_seed_and_its_log() {
    let mut played = deal(4);
    let mut steps = 0;
    while !played.is_over() {
        assert!(steps < 20_000, "the game should finish");
        let action = choose(&played);
        apply(&mut played, action).unwrap();
        steps += 1;
    }

    let set = milestone1();
    let decklist = starter_decklist(&set);
    let replayed = replay(
        set.db,
        [decklist.clone(), decklist],
        Box::new(SeededRng::new(4)),
        &played.history,
    )
    .expect("a log of applied actions replays");

    assert_eq!(
        fingerprint(&replayed),
        fingerprint(&played),
        "the same seed and the same actions give the same game"
    );
}

#[test]
fn a_replay_stops_at_an_action_that_does_not_fit() {
    let mut state = deal(9);
    while state.phase != Phase::Main {
        let action = choose(&state);
        apply(&mut state, action).unwrap();
    }

    let set = milestone1();
    let decklist = starter_decklist(&set);
    // A log that never belonged to this game: the first setup choice is not
    // legal against a fresh deal made with a different seed's shuffle.
    let nonsense = vec![Action::Attack { index: 0 }];
    let outcome = replay(
        set.db,
        [decklist.clone(), decklist],
        Box::new(SeededRng::new(9)),
        &nonsense,
    );
    assert!(outcome.is_err(), "a log that does not fit is refused");
}

#[test]
fn a_partial_log_replays_to_the_position_it_reached() {
    let mut played = deal(7);
    for _ in 0..12 {
        if played.is_over() {
            break;
        }
        let action = choose(&played);
        apply(&mut played, action).unwrap();
    }

    let set = milestone1();
    let decklist = starter_decklist(&set);
    let replayed = replay(
        set.db,
        [decklist.clone(), decklist],
        Box::new(SeededRng::new(7)),
        &played.history,
    )
    .unwrap();

    assert_eq!(fingerprint(&replayed), fingerprint(&played));
    assert!(player_to_act(&replayed).is_some(), "and it can carry on");
}
