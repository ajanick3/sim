//! Ticket 04: Aquabear, the third Basic.

use sim::action::{Action, legal_actions};
use sim::cards::{Milestone1, milestone1, starter_decklist};
use sim::engine::apply;
use sim::ids::CardDefId;
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

/// A deck of Aquabear and the Energy under test. The engine does not check
/// deck construction, so a test says exactly what it needs.
fn deck(set: &Milestone1, energies: &[CardDefId]) -> Vec<CardDefId> {
    let mut decklist = vec![set.aquabear; 12];
    while decklist.len() < 60 {
        decklist.push(energies[decklist.len() % energies.len()]);
    }
    decklist
}

/// Deal a game and stop where an attack is legal: past setup, and past the
/// first turn, which skips the attack step under rule 17.
fn game(set: &Milestone1, energies: &[CardDefId]) -> GameState {
    let decklist = deck(set, energies);
    let mut state = GameState::new(
        milestone1().db,
        [decklist.clone(), decklist],
        Box::new(SeededRng::new(9)),
    );
    settle(&mut state);
    apply(&mut state, Action::EndTurn).unwrap();
    settle(&mut state);
    state
}

fn settle(state: &mut GameState) {
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(state)[0];
        apply(state, first).unwrap();
    }
}

/// Attach one Energy to the Active, without spending the turn's attachment.
fn attach(state: &mut GameState, energy: CardDefId) {
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let side = state.player(player);
    let card = side
        .hand
        .iter()
        .chain(side.deck.iter())
        .find(|c| state.cards[c.index()].def == energy)
        .copied()
        .expect("the deck holds this Energy");
    state.remove_from_hand(player, card);
    state.players[player.index()].deck.retain(|c| *c != card);
    state.pokemon[active.index()].attached.push(card);
}

fn can_attack(state: &GameState) -> bool {
    legal_actions(state)
        .iter()
        .any(|a| matches!(a, Action::Attack { .. }))
}

#[test]
fn bubblebeam_needs_a_water_energy() {
    let set = milestone1();
    let mut state = game(&set, &[set.fire_energy, set.water_energy]);

    attach(&mut state, set.fire_energy);
    attach(&mut state, set.fire_energy);
    assert!(
        !can_attack(&state),
        "two Fire pay the Colorless entry, and nothing pays the Water one"
    );

    attach(&mut state, set.water_energy);
    assert!(
        can_attack(&state),
        "a Water and a spare Energy pay the cost"
    );
}

#[test]
fn bubblebeam_deals_thirty() {
    let set = milestone1();
    let mut state = game(&set, &[set.water_energy]);
    let defender = state.player(state.current.opponent()).active.unwrap();

    attach(&mut state, set.water_energy);
    attach(&mut state, set.water_energy);
    let attack = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("two Water pay Water and Colorless");
    apply(&mut state, attack).unwrap();

    // Aquabear is weak to Lightning, not to Water, so 30 lands as printed.
    assert_eq!(state.pokemon(defender).damage, 30);
}

#[test]
fn aquabear_and_its_energy_are_in_the_starter_decklist() {
    let set = milestone1();
    let decklist = starter_decklist(&set);

    assert_eq!(decklist.len(), 60, "a deck is exactly 60 cards");
    assert_eq!(
        decklist.iter().filter(|c| **c == set.aquabear).count(),
        4,
        "rule 2: at most 4 copies of a card by name"
    );
    assert!(
        decklist.contains(&set.water_energy),
        "a deck with Aquabear can pay Bubblebeam"
    );
}
