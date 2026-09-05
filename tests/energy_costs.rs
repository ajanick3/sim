//! Ticket 02: an attack cost names Energy types.

use sim::action::{Action, legal_actions};
use sim::card::Type;
use sim::cards::milestone1;
use sim::engine::apply;
use sim::ids::CardDefId;
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

fn game(seed: u64, decklists: [Vec<CardDefId>; 2]) -> GameState {
    let set = milestone1();
    let mut state = GameState::new(set.db, decklists, Box::new(SeededRng::new(seed)));
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    // Rule 17 skips the first attack step, which would hide every result a
    // test here is looking for.
    apply(&mut state, Action::EndTurn).unwrap();
    state
}

/// A deck of one Pokémon and Energy of both types, so a test says exactly
/// what is attached. Deck construction is not checked by the engine yet.
fn deck(pokemon: CardDefId) -> Vec<CardDefId> {
    let set = milestone1();
    let mut decklist = vec![pokemon; 6];
    decklist.extend(vec![set.lightning_energy; 27]);
    decklist.extend(vec![set.fire_energy; 27]);
    decklist
}

/// Attach Energy to the Active without spending the turn's attachment. It
/// takes from hand or library, because a test states what is attached, not
/// where it came from.
fn force_attach(state: &mut GameState, energy: CardDefId, count: usize) {
    let player = state.current;
    let active = state.player(player).active.unwrap();
    for _ in 0..count {
        let side = state.player(player);
        let card = side
            .hand
            .iter()
            .chain(side.library.iter())
            .find(|c| state.cards[c.index()].def == energy)
            .copied()
            .expect("the deck holds Energy of both types");
        state.remove_from_hand(player, card);
        state.players[player.index()].library.retain(|c| *c != card);
        state.pokemon[active.index()].attached.push(card);
    }
}

#[test]
fn an_attack_that_costs_fire_refuses_lightning() {
    let set = milestone1();
    // Cinderpup's Ember costs Fire. Lightning does not pay it.
    let mut state = game(9, [deck(set.cinderpup), deck(set.cinderpup)]);
    force_attach(&mut state, set.lightning_energy, 2);

    assert!(
        !legal_actions(&state)
            .iter()
            .any(|a| matches!(a, Action::Attack { .. })),
        "two Lightning cannot pay a cost of two Fire"
    );

    assert_eq!(
        milestone1().db.get(set.cinderpup).as_pokemon().unwrap().attacks[0].cost,
        vec![Type::Fire, Type::Fire],
        "a cost names types, not a count"
    );
}

#[test]
fn a_colorless_entry_takes_any_energy() {
    let set = milestone1();
    // Sparkmouse's Spark Tackle costs Lightning and Colorless.
    let mut state = game(9, [deck(set.sparkmouse), deck(set.sparkmouse)]);
    force_attach(&mut state, set.lightning_energy, 1);
    force_attach(&mut state, set.fire_energy, 1);

    let attacks: Vec<Action> = legal_actions(&state)
        .into_iter()
        .filter(|a| matches!(a, Action::Attack { .. }))
        .collect();
    assert_eq!(
        attacks.len(),
        2,
        "one Lightning and one Fire pay both Nibble and Spark Tackle"
    );
}

#[test]
fn a_colorless_entry_does_not_eat_the_energy_a_named_entry_needs() {
    let set = milestone1();
    let mut state = game(9, [deck(set.sparkmouse), deck(set.sparkmouse)]);
    // Two Fire pay the Colorless entry twice over, and the Lightning never.
    force_attach(&mut state, set.fire_energy, 2);

    assert!(
        !legal_actions(&state)
            .iter()
            .any(|a| matches!(a, Action::Attack { .. })),
        "Spark Tackle still needs its Lightning"
    );
}
