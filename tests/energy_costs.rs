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
    state
}

/// A deck of one Pokémon and one kind of Energy, so a test says exactly what
/// is attached. Deck construction is not checked by the engine yet.
fn deck(pokemon: CardDefId, energy: CardDefId) -> Vec<CardDefId> {
    let mut decklist = vec![pokemon; 6];
    decklist.extend(vec![energy; 54]);
    decklist
}

/// Attach Energy to the Active without spending the turn's attachment.
fn force_attach(state: &mut GameState, energy: CardDefId, count: usize) {
    let player = state.current;
    let active = state.player(player).active.unwrap();
    for _ in 0..count {
        let card = *state
            .player(player)
            .hand
            .iter()
            .find(|c| state.cards[c.index()].def == energy)
            .expect("the deck is mostly Energy");
        state.remove_from_hand(player, card);
        state.pokemon[active.index()].attached.push(card);
    }
}

#[test]
fn an_attack_that_costs_fire_refuses_lightning() {
    let set = milestone1();
    // Cinderpup's Ember costs Fire. Lightning does not pay it.
    let mut state = game(9, [
        deck(set.cinderpup, set.lightning_energy),
        deck(set.cinderpup, set.lightning_energy),
    ]);
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
