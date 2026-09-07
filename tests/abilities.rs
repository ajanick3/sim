//! Milestone 8: the first Abilities. One fixture, shared across this
//! milestone's tickets, the same shape every earlier milestone's own
//! fixture already took.

use sim::action::{Action, legal_actions};
use sim::card::{Ability, Attack, CardDb, CardDef, Energy, Pokemon, Stage, Type};
use sim::engine::apply;
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

/// A game where the first player's Active carries the Ability under
/// test, and the second player's Active is a plain punching bag.
fn game(ability: Ability, seed: u64) -> GameState {
    let mut db = CardDb::new();
    let carrier = db.add(CardDef::Pokemon(Pokemon {
        print_id: "test-carrier",
        name: "Abilitymon",
        hp: 200,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: Some(ability),
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
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
        ability: None,
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

    let mut carrier_deck = vec![carrier; 4];
    while carrier_deck.len() < 60 {
        carrier_deck.push(energy);
    }
    let mut defender_deck = vec![defender_mon; 4];
    while defender_deck.len() < 60 {
        defender_deck.push(energy);
    }
    let mut state = GameState::new(
        db,
        [carrier_deck, defender_deck],
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

// --- Ticket 01: an Ability a player opts into ---

#[test]
fn draws_once_while_the_carrier_is_active() {
    let ability = Ability {
        name: "Run Errand",
        effect: sim::card::AbilityEffect::OncePerTurnWhileActiveMayDrawCards(2),
    };
    let mut state = game(ability, 3);
    let player = state.current;
    let carrier = state.player(player).active.unwrap();
    let before = state.player(player).hand.len();

    apply(&mut state, Action::UseAbility { pokemon: carrier }).unwrap();

    assert_eq!(state.player(player).hand.len(), before + 2);
    assert_eq!(state.phase, Phase::Main, "no phase, no cost, no target");
}

#[test]
fn cannot_use_the_ability_a_second_time_the_same_turn() {
    let ability = Ability {
        name: "Run Errand",
        effect: sim::card::AbilityEffect::OncePerTurnWhileActiveMayDrawCards(2),
    };
    let mut state = game(ability, 3);
    let player = state.current;
    let carrier = state.player(player).active.unwrap();

    apply(&mut state, Action::UseAbility { pokemon: carrier }).unwrap();
    let result = apply(&mut state, Action::UseAbility { pokemon: carrier });

    assert!(result.is_err(), "Run Errand is once per turn");
}

#[test]
fn cannot_use_the_ability_from_the_bench() {
    let ability = Ability {
        name: "Run Errand",
        effect: sim::card::AbilityEffect::OncePerTurnWhileActiveMayDrawCards(2),
    };
    let mut state = game(ability, 3);
    let player = state.current;
    let active_card = state.pokemon(state.player(player).active.unwrap()).top_card();
    let carrier_def = state.cards[active_card.index()].def;

    // Bench a second copy and try to use its Ability instead.
    let card = sim::ids::CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: carrier_def, owner: player });
    let bench_mon = state.put_into_play(player, card);
    state.players[player.index()].bench.push(bench_mon);

    let result = apply(&mut state, Action::UseAbility { pokemon: bench_mon });
    assert!(result.is_err(), "Run Errand only works from the Active Spot");
}

#[test]
fn cannot_use_the_ability_on_the_opponents_turn() {
    let ability = Ability {
        name: "Run Errand",
        effect: sim::card::AbilityEffect::OncePerTurnWhileActiveMayDrawCards(2),
    };
    let mut state = game(ability, 3);
    let owner = state.current;
    let carrier = state.player(owner).active.unwrap();

    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    assert_eq!(state.current, owner.opponent(), "now the opponent's turn");

    let result = apply(&mut state, Action::UseAbility { pokemon: carrier });
    assert!(result.is_err(), "not this player's turn");
}

#[test]
fn mega_kangaskhan_ex_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Mega Kangaskhan ex" && c.playable.is_some()),
        "at least one Mega Kangaskhan ex print should play"
    );
}
