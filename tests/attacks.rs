//! Milestone 11: Pokémon attacks, including their effects. One fixture,
//! shared across this milestone's tickets, the same shape every
//! Trainer-kind milestone's own fixture already took.

use sim::action::{Action, legal_actions};
use sim::card::{Attack, AttackEffect, CardDb, CardDef, Energy, Pokemon, Stage, Type};
use sim::engine::apply;
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

/// A game where the first player's Active has one attack under test, and
/// the second player's Active is a plain punching bag.
fn game(attacker_attack: Attack, seed: u64) -> GameState {
    let mut db = CardDb::new();
    let attacker_mon = db.add(CardDef::Pokemon(Pokemon {
        print_id: "test-attacker",
        name: "Attackmon",
        hp: 200,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        attacks: vec![attacker_attack],
    }));
    let defender_mon = db.add(CardDef::Pokemon(Pokemon {
        print_id: "test-defender",
        name: "Defendmon",
        hp: 200,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-colorless-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
    }));

    let mut attacker_deck = vec![attacker_mon; 4];
    while attacker_deck.len() < 60 {
        attacker_deck.push(energy);
    }
    let mut defender_deck = vec![defender_mon; 4];
    while defender_deck.len() < 60 {
        defender_deck.push(energy);
    }
    let mut state = GameState::new(
        db,
        [attacker_deck, defender_deck],
        Box::new(SeededRng::new(seed)),
    );
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    state
}

/// Pay for and use the Active's only attack.
fn pay_and_attack(state: &mut GameState) {
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let cost_len = state.pokemon_def(active).attacks[0].cost.len();
    for _ in 0..cost_len {
        let side = state.player(player);
        let card = side
            .hand
            .iter()
            .chain(side.library.iter())
            .find(|c| state.def_of(**c).is_energy())
            .copied()
            .expect("the deck holds Energy");
        state.remove_from_hand(player, card);
        state.players[player.index()].library.retain(|c| *c != card);
        state.pokemon[active.index()].attached.push(card);
    }
    let attack = legal_actions(state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("a paid-for Active can attack");
    apply(state, attack).unwrap();
}

// --- Ticket 01: the mechanism, and Recoil ---

#[test]
fn recoil_damages_the_attacker_alongside_the_defender() {
    let attack = Attack {
        name: "Reckless Charge",
        cost: vec![Type::Colorless],
        base_damage: 30,
        inflicts: None,
        effect: Some(AttackEffect::Recoil(10)),
    };
    let mut state = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(attacker).damage, 10, "the recoil");
    assert_eq!(state.pokemon(defender).damage, 30, "the attack's own damage");
}

#[test]
fn carvanha_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Carvanha")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Carvanha should play");
}
