//! Beyond the field: Tools that reduce the carrier's own attack cost.

use sim::action::{Action, legal_actions};
use sim::card::{Attack, CardDb, CardDef, Energy, Marker, Pokemon, Stage, Trainer, TrainerEffect, TrainerKind, Type};
use sim::engine::apply;
use sim::ids::{CardDefId, CardId, PlayerId};
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

/// A game through setup, its Active a `Testmon` carrying `markers`, with
/// one attack costing two Colorless — enough to show a one-Energy
/// discount making the difference between legal and not.
fn game_with_marked_active(seed: u64, markers: Vec<Marker>) -> (GameState, sim::ids::PokemonId) {
    let mut db = CardDb::new();
    let mon = db.add(CardDef::Pokemon(Pokemon {
        markers,
        print_id: "test-marked-mon",
        name: "Testmon",
        hp: 100,
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
            name: "Slam",
            cost: vec![Type::Colorless, Type::Colorless],
            base_damage: 60,
            inflicts: None,
            effect: None,
        }],
    }));
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-colorless-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    let mut decklist = vec![mon; 4];
    while decklist.len() < 60 {
        decklist.push(energy);
    }
    let mut state = GameState::new(db, [decklist.clone(), decklist], Box::new(SeededRng::new(seed)));
    for _ in 0..2 {
        while state.phase != Phase::Main && !state.is_over() {
            let first = legal_actions(&state)[0];
            apply(&mut state, first).unwrap();
        }
        if state.turn_number > 1 {
            break;
        }
        apply(&mut state, Action::EndTurn).unwrap();
    }
    let pokemon = state.player(state.current).active.unwrap();
    (state, pokemon)
}

fn attach(state: &mut GameState, pokemon: sim::ids::PokemonId, owner: PlayerId, def: CardDefId) -> CardId {
    let card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def, owner });
    state.pokemon[pokemon.index()].attached.push(card);
    card
}

fn attach_one_energy(state: &mut GameState, pokemon: sim::ids::PokemonId, owner: PlayerId, count: u32) {
    let energy = state.db.add(CardDef::Energy(Energy {
        print_id: "test-attached-colorless",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    for _ in 0..count {
        attach(state, pokemon, owner, energy);
    }
}

#[test]
fn sparkling_crystal_lets_a_tera_carrier_attack_with_one_less_energy() {
    let (mut state, pokemon) = game_with_marked_active(1, vec![Marker::Tera]);
    let owner = state.current;
    attach_one_energy(&mut state, pokemon, owner, 1);

    assert!(
        !legal_actions(&state).contains(&Action::Attack { index: 0 }),
        "one Energy alone can't pay a two-Colorless attack"
    );

    let sparkling_crystal = state.db.add(CardDef::Trainer(Trainer {
        print_id: "test-sparkling-crystal",
        name: "Sparkling Crystal",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::ReducesAttackCostByAnyTypeIfCarrierMarked(Marker::Tera, 1),
    }));
    attach(&mut state, pokemon, owner, sparkling_crystal);

    assert!(
        legal_actions(&state).contains(&Action::Attack { index: 0 }),
        "Sparkling Crystal discounts the Tera carrier's attack by one Energy"
    );
}

#[test]
fn sparkling_crystal_does_nothing_off_a_non_tera_carrier() {
    let (mut state, pokemon) = game_with_marked_active(1, vec![]);
    let owner = state.current;
    attach_one_energy(&mut state, pokemon, owner, 1);

    let sparkling_crystal = state.db.add(CardDef::Trainer(Trainer {
        print_id: "test-sparkling-crystal-plain",
        name: "Sparkling Crystal",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::ReducesAttackCostByAnyTypeIfCarrierMarked(Marker::Tera, 1),
    }));
    attach(&mut state, pokemon, owner, sparkling_crystal);

    assert!(
        !legal_actions(&state).contains(&Action::Attack { index: 0 }),
        "only a Tera carrier is discounted"
    );
}

#[test]
fn sparkling_crystal_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Sparkling Crystal" && c.playable.is_some()),
        "Sparkling Crystal should play"
    );
}

#[test]
fn counter_gain_discounts_the_attack_while_its_owner_has_more_prizes_remaining() {
    let (mut state, pokemon) = game_with_marked_active(1, vec![]);
    let owner = state.current;
    attach_one_energy(&mut state, pokemon, owner, 1);

    // Owner starts with 6 prizes, opponent 6 — even, so no discount yet.
    state.players[owner.opponent().index()].prizes.clear();

    let counter_gain = state.db.add(CardDef::Trainer(Trainer {
        print_id: "test-counter-gain",
        name: "Counter Gain",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::ReducesAttackCostIfMorePrizesRemaining(1),
    }));
    attach(&mut state, pokemon, owner, counter_gain);

    assert!(
        legal_actions(&state).contains(&Action::Attack { index: 0 }),
        "Counter Gain discounts the attack while its owner has more Prizes remaining"
    );
}

#[test]
fn counter_gain_does_nothing_without_more_prizes_remaining() {
    let (mut state, pokemon) = game_with_marked_active(1, vec![]);
    let owner = state.current;
    attach_one_energy(&mut state, pokemon, owner, 1);

    let counter_gain = state.db.add(CardDef::Trainer(Trainer {
        print_id: "test-counter-gain-even",
        name: "Counter Gain",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::ReducesAttackCostIfMorePrizesRemaining(1),
    }));
    attach(&mut state, pokemon, owner, counter_gain);

    assert!(
        !legal_actions(&state).contains(&Action::Attack { index: 0 }),
        "an even Prize count gives no discount"
    );
}

#[test]
fn counter_gain_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Counter Gain" && c.playable.is_some()),
        "Counter Gain should play"
    );
}
