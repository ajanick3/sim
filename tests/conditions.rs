//! Ticket 03: the Special Conditions and the Pokémon Checkup.

use sim::action::{Action, legal_actions};
use sim::card::{Attack, CardDb, CardDef, Condition, Energy, Pokemon, Type};
use sim::engine::apply;
use sim::rng::{Rng, ScriptedRng, SeededRng};
use sim::state::{GameState, Phase};

/// A card set whose one attack inflicts the condition under test and deals no
/// damage, so a test reads the condition and nothing else.
fn game(inflicts: Condition, rng: Box<dyn Rng>) -> GameState {
    let mut db = CardDb::new();
    let stinger = db.add(CardDef::Pokemon(Pokemon {
        name: "Stinger",
        hp: 100,
        kind: Type::Grass,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        attacks: vec![Attack {
            name: "Sting",
            cost: vec![Type::Colorless],
            base_damage: 0,
            inflicts: Some(inflicts),
        }],
    }));
    let energy = db.add(CardDef::Energy(Energy {
        name: "Grass Energy",
        kind: Type::Grass,
    }));

    let mut decklist = vec![stinger; 6];
    decklist.extend(vec![energy; 54]);
    let mut state = GameState::new(db, [decklist.clone(), decklist], rng);
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    // Rule 17: the first turn has no attack step.
    apply(&mut state, Action::EndTurn).unwrap();
    drive_setup_choices(&mut state);
    state
}

/// Take the first choice through any phase that is not a player's main turn.
fn drive_setup_choices(state: &mut GameState) {
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(state)[0];
        apply(state, first).unwrap();
    }
}

/// Attack with the Active, paying for it first.
fn sting(state: &mut GameState) {
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let energy = *state
        .player(player)
        .hand
        .iter()
        .chain(state.player(player).library.iter())
        .find(|c| state.def_of(**c).is_energy())
        .expect("the deck is mostly Energy");
    state.remove_from_hand(player, energy);
    state.players[player.index()].library.retain(|c| *c != energy);
    state.pokemon[active.index()].attached.push(energy);

    let attack = legal_actions(state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("a paid-for Active can attack");
    apply(state, attack).unwrap();
}

#[test]
fn poison_puts_one_counter_at_the_checkup() {
    let mut state = game(Condition::Poisoned, Box::new(SeededRng::new(9)));
    let poisoned = state.player(state.current.opponent()).active.unwrap();

    sting(&mut state);
    assert!(
        state.has_condition(poisoned, Condition::Poisoned),
        "the attack inflicts Poison"
    );
    assert_eq!(
        state.pokemon(poisoned).damage,
        0,
        "Poison damages at the checkup, not when it lands"
    );

    // The attack ended the turn, so the checkup has run.
    drive_setup_choices(&mut state);
    assert_eq!(
        state.pokemon(poisoned).damage,
        10,
        "rule 54: Poison puts 1 damage counter at the checkup"
    );
    assert!(
        state.has_condition(poisoned, Condition::Poisoned),
        "Poison stays until something removes it"
    );
}

#[test]
fn a_scripted_generator_makes_a_flip_an_assertion() {
    let state = game(Condition::Poisoned, Box::new(ScriptedRng::new(vec![1])));
    assert!(!state.is_over());
}
