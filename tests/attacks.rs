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
fn game(attacker_attack: Attack, seed: u64) -> (GameState, sim::ids::CardDefId) {
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
    let defender_ex = db.add(CardDef::Pokemon(Pokemon {
        print_id: "test-defender-ex",
        name: "Defendmon ex",
        hp: 200,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 2,
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
    (state, defender_ex)
}

/// A physical card of `def`, for a definition that was never part of
/// the deck. Placed nowhere; the caller pushes it where the test needs.
fn deal_new_card(state: &mut GameState, player: sim::ids::PlayerId, def: sim::ids::CardDefId) -> sim::ids::CardId {
    let card = sim::ids::CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def, owner: player });
    card
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
    let (mut state, _defender_ex) = game(attack, 3);
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

// --- Ticket 02: damage multiplied by a counted board fact ---

fn multiplier_attack(count: sim::card::Count, per_unit: u32) -> Attack {
    Attack {
        name: "Multiply",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DamagePerCount(count, per_unit)),
    }
}

#[test]
fn damage_per_own_damage_counters() {
    let attack = multiplier_attack(sim::card::Count::OwnDamageCounters, 20);
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    state.pokemon[attacker.index()].damage = 30; // 3 counters

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 60, "3 counters times 20");
}

#[test]
fn damage_per_opponent_basic_energy_in_discard() {
    let attack = multiplier_attack(sim::card::Count::OpponentBasicEnergyInDiscard, 30);
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    for _ in 0..2 {
        let card = *state
            .player(opponent)
            .library
            .iter()
            .find(|c| state.def_of(**c).is_energy())
            .unwrap();
        state.players[opponent.index()].library.retain(|c| *c != card);
        state.players[opponent.index()].discard.push(card);
    }

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 60, "2 Energy in discard times 30");
}

#[test]
fn damage_per_opponent_ex_in_play_is_zero_with_no_ex() {
    let attack = multiplier_attack(sim::card::Count::OpponentPokemonExInPlay, 60);
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);
    assert_eq!(state.pokemon(defender).damage, 0, "no ex in play yet");
}

#[test]
fn damage_per_opponent_ex_in_play_counts_one() {
    let attack = multiplier_attack(sim::card::Count::OpponentPokemonExInPlay, 60);
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let ex_card = deal_new_card(&mut state, opponent, defender_ex);
    state.pokemon[defender.index()].cards = vec![ex_card];

    pay_and_attack(&mut state);
    assert_eq!(state.pokemon(defender).damage, 60, "one opponent ex in play");
}

#[test]
fn damage_per_own_basic_pokemon_in_play() {
    let attack = multiplier_attack(sim::card::Count::OwnBasicPokemonInPlay, 20);
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    // The attacker itself is one Basic in play.

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 20, "the attacker itself, 1 Basic");
}

#[test]
fn damage_per_own_damaged_pokemon_with_name_prefix() {
    // No damage on the attacker yet: the name matches, but nothing counts.
    let attack = multiplier_attack(sim::card::Count::OwnDamagedWithNamePrefix("Attack"), 40);
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    pay_and_attack(&mut state);
    assert_eq!(state.pokemon(defender).damage, 0, "no damage on it yet");

    // With damage on the attacker, it counts.
    let attack = multiplier_attack(sim::card::Count::OwnDamagedWithNamePrefix("Attack"), 40);
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    state.pokemon[attacker.index()].damage = 10;
    pay_and_attack(&mut state);
    assert_eq!(state.pokemon(defender).damage, 40, "one damaged match");
}

#[test]
fn ns_reshiram_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "N's Reshiram")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "N's Reshiram should play");
}

#[test]
fn paldean_tauros_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Paldean Tauros")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Paldean Tauros should play");
}
