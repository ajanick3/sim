//! Milestone 12: Special Energy. Each card carries its own effect, unlike
//! a Basic Energy — the same discipline an attack or an Ability already
//! holds (ADR 0080 supersedes ADR 0034, which refused every Special
//! Energy outright).

use sim::card::{CardDb, CardDef, Energy, EnergyEffect, Pokemon, Stage, Type};
use sim::ids::PlayerId;
use sim::rng::SeededRng;
use sim::state::GameState;

/// A minimal one-Pokemon board: just enough to attach a Special Energy
/// under test and read what it does.
fn one_pokemon_game(hp: u32, kind: Type) -> (GameState, sim::ids::PokemonId) {
    let mut db = CardDb::new();
    let mon_def = db.add(CardDef::Pokemon(Pokemon {
        print_id: "test-carrier",
        name: "Energymon",
        hp,
        kind,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![],
    }));
    let energy_def = db.add(CardDef::Energy(Energy {
        print_id: "test-colorless-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    let mut deck = vec![mon_def; 4];
    while deck.len() < 60 {
        deck.push(energy_def);
    }
    let mut state = GameState::new(db, [deck.clone(), deck], Box::new(SeededRng::new(3)));
    let card = sim::ids::CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: mon_def, owner: PlayerId::One });
    let pokemon = state.put_into_play(PlayerId::One, card);
    state.players[PlayerId::One.index()].active = Some(pokemon);
    (state, pokemon)
}

/// Attach a physical card of `def` to `pokemon`, as if it were already
/// on the board — the same shape every other milestone's fixture uses
/// for a card that starts nowhere in the deck.
fn attach(state: &mut GameState, pokemon: sim::ids::PokemonId, def: sim::ids::CardDefId) -> sim::ids::CardId {
    let card = sim::ids::CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def, owner: PlayerId::One });
    state.pokemon[pokemon.index()].attached.push(card);
    card
}

// --- Ticket 01: the Special Energy primitive ---

#[test]
fn increases_the_carriers_hp_while_attached() {
    let (mut state, pokemon) = one_pokemon_game(100, Type::Grass);
    assert_eq!(state.effective_hp(pokemon), 100);

    let energy_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-growing-grass-energy",
        name: "Growing Grass Energy",
        kind: Type::Grass,
        effect: Some(EnergyEffect::IncreasesCarrierHp(20)),
    }));
    attach(&mut state, pokemon, energy_def);

    assert_eq!(state.effective_hp(pokemon), 120);
}

#[test]
fn still_pays_its_own_type_toward_an_attack_cost() {
    let (mut state, pokemon) = one_pokemon_game(100, Type::Grass);
    let energy_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-growing-grass-energy-cost",
        name: "Growing Grass Energy",
        kind: Type::Grass,
        effect: Some(EnergyEffect::IncreasesCarrierHp(20)),
    }));
    attach(&mut state, pokemon, energy_def);

    assert!(state.pays_cost(pokemon, &[Type::Grass]));
}

#[test]
fn a_basic_energy_filter_does_not_match_a_special_energy() {
    let (mut state, _pokemon) = one_pokemon_game(100, Type::Grass);
    let special_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-growing-grass-energy-filter",
        name: "Growing Grass Energy",
        kind: Type::Grass,
        effect: Some(EnergyEffect::IncreasesCarrierHp(20)),
    }));
    let basic_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-grass-energy-filter",
        name: "Grass Energy",
        kind: Type::Grass,
        effect: None,
    }));
    let special = sim::ids::CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: special_def, owner: PlayerId::One });
    let basic = sim::ids::CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: basic_def, owner: PlayerId::One });

    assert!(!state.matches_filter(special, sim::card::CardFilter::BasicEnergy));
    assert!(state.matches_filter(basic, sim::card::CardFilter::BasicEnergy));
    assert!(!state.matches_filter(special, sim::card::CardFilter::BasicEnergyOfType(Type::Grass)));
    assert!(state.matches_filter(basic, sim::card::CardFilter::BasicEnergyOfType(Type::Grass)));
}

#[test]
fn growing_grass_energy_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Growing Grass Energy" && c.playable.is_some()),
        "at least one Growing Grass Energy print should play"
    );
}
