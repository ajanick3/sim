//! Milestone 12: Special Energy. Each card carries its own effect, unlike
//! a Basic Energy — the same discipline an attack or an Ability already
//! holds (ADR 0080 supersedes ADR 0034, which refused every Special
//! Energy outright).

use sim::action::{Action, legal_actions};
use sim::card::{CardDb, CardDef, Energy, EnergyEffect, Pokemon, Stage, Type};
use sim::engine::apply;
use sim::ids::PlayerId;
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

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

// --- Ticket 02: an attach-from-hand trigger ---

/// A game already through setup, so `Action::AttachEnergy` is legal —
/// the same shape `tests/limits.rs`'s own fixture already takes,
/// built from a custom `CardDb` instead of the synthetic set.
fn game_through_setup(seed: u64) -> GameState {
    game_through_setup_of_type(seed, Type::Colorless)
}

fn game_through_setup_of_type(seed: u64, kind: Type) -> GameState {
    let mut db = CardDb::new();
    let mon_def = db.add(CardDef::Pokemon(Pokemon {
        print_id: "test-carrier",
        name: "Energymon",
        hp: 100,
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
        print_id: "test-colorless-energy-setup",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    let mut deck = vec![mon_def; 4];
    while deck.len() < 60 {
        deck.push(energy_def);
    }
    let mut state = GameState::new(db, [deck.clone(), deck], Box::new(SeededRng::new(seed)));
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    state
}

#[test]
fn drawing_from_attaching_from_hand() {
    let mut state = game_through_setup(3);
    let player = state.current;
    let target = state.player(player).active.unwrap();

    let enriching_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-enriching-energy",
        name: "Enriching Energy",
        kind: Type::Colorless,
        effect: Some(EnergyEffect::DrawCardsOnAttachFromHand(4)),
    }));
    let enriching = sim::ids::CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: enriching_def, owner: player });
    state.players[player.index()].hand.push(enriching);
    let hand_before = state.player(player).hand.len();

    apply(&mut state, Action::AttachEnergy { card: enriching, target }).unwrap();

    assert!(state.pokemon(target).attached.contains(&enriching));
    // The card itself leaves hand to attach, then 4 more replace it.
    assert_eq!(state.player(player).hand.len(), hand_before - 1 + 4);
}

#[test]
fn does_not_draw_when_it_arrives_any_other_way() {
    let (mut state, pokemon) = one_pokemon_game(100, Type::Colorless);
    let player = state.current;
    let hand_before = state.player(player).hand.len();

    let enriching_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-enriching-energy-dealt",
        name: "Enriching Energy",
        kind: Type::Colorless,
        effect: Some(EnergyEffect::DrawCardsOnAttachFromHand(4)),
    }));
    attach(&mut state, pokemon, enriching_def);

    assert_eq!(state.player(player).hand.len(), hand_before, "dealt directly, never through the hand");
}

#[test]
fn enriching_energy_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Enriching Energy" && c.playable.is_some()),
        "at least one Enriching Energy print should play"
    );
}

// --- Ticket 03: a search to the Bench from an attach ---

#[test]
fn searches_up_to_the_limit_of_basic_pokemon_of_type_to_the_bench() {
    let mut state = game_through_setup_of_type(3, Type::Psychic);
    let player = state.current;
    let target = state.player(player).active.unwrap();

    let psychic_basic_def = state.db.add(CardDef::Pokemon(Pokemon {
        print_id: "test-psychic-basic",
        name: "Psychicmon",
        hp: 60,
        kind: Type::Psychic,
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
    let first = sim::ids::CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: psychic_basic_def, owner: player });
    state.players[player.index()].library.push(first);
    let second = sim::ids::CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: psychic_basic_def, owner: player });
    state.players[player.index()].library.push(second);
    let library_len_before = state.player(player).library.len();

    let telepathic_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-telepathic-psychic-energy",
        name: "Telepathic Psychic Energy",
        kind: Type::Psychic,
        effect: Some(EnergyEffect::WhenAttachedToTypeSearchesBasicPokemonOfTypeToBench(
            Type::Psychic,
            Type::Psychic,
            2,
        )),
    }));
    let telepathic = sim::ids::CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: telepathic_def, owner: player });
    state.players[player.index()].hand.push(telepathic);

    apply(&mut state, Action::AttachEnergy { card: telepathic, target }).unwrap();
    assert!(matches!(state.phase, Phase::SearchingLibraryForBasicsOfType { .. }));

    apply(&mut state, Action::TakeBasicPokemonOfTypeForEnergyAttach { card: first }).unwrap();
    assert!(matches!(state.phase, Phase::SearchingLibraryForBasicsOfType { .. }), "one more to take");
    apply(&mut state, Action::TakeBasicPokemonOfTypeForEnergyAttach { card: second }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.player(player).bench.len(), 2);
    assert_eq!(state.player(player).library.len(), library_len_before - 2);
    assert!(state.pokemon(target).attached.contains(&telepathic), "the Energy still attaches");
}

#[test]
fn does_not_search_when_attached_to_the_wrong_type() {
    let mut state = game_through_setup_of_type(3, Type::Colorless);
    let player = state.current;
    let target = state.player(player).active.unwrap();

    let telepathic_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-telepathic-psychic-energy-wrong-type",
        name: "Telepathic Psychic Energy",
        kind: Type::Psychic,
        effect: Some(EnergyEffect::WhenAttachedToTypeSearchesBasicPokemonOfTypeToBench(
            Type::Psychic,
            Type::Psychic,
            2,
        )),
    }));
    let telepathic = sim::ids::CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: telepathic_def, owner: player });
    state.players[player.index()].hand.push(telepathic);

    apply(&mut state, Action::AttachEnergy { card: telepathic, target }).unwrap();

    assert_eq!(state.phase, Phase::Main, "the carrier is not Psychic");
    assert!(state.pokemon(target).attached.contains(&telepathic));
}

#[test]
fn no_qualifying_basic_still_attaches_the_energy() {
    let mut state = game_through_setup_of_type(3, Type::Psychic);
    let player = state.current;
    let target = state.player(player).active.unwrap();

    let telepathic_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-telepathic-psychic-energy-no-basic",
        name: "Telepathic Psychic Energy",
        kind: Type::Psychic,
        effect: Some(EnergyEffect::WhenAttachedToTypeSearchesBasicPokemonOfTypeToBench(
            Type::Psychic,
            Type::Psychic,
            2,
        )),
    }));
    let telepathic = sim::ids::CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: telepathic_def, owner: player });
    state.players[player.index()].hand.push(telepathic);
    state.players[player.index()].library.clear();

    apply(&mut state, Action::AttachEnergy { card: telepathic, target }).unwrap();

    assert_eq!(state.phase, Phase::Main, "no Basic Psychic Pokemon anywhere in the library");
    assert!(state.pokemon(target).attached.contains(&telepathic));
}

#[test]
fn telepathic_psychic_energy_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Telepathic Psychic Energy" && c.playable.is_some()),
        "at least one Telepathic Psychic Energy print should play"
    );
}

// --- Ticket 04: counter-damage from a hit taken ---

/// A game where the first player's Active can attack, and the second
/// player's Active is a plain punching bag — the same shape
/// `tests/attacks.rs`'s own fixture already takes.
fn attacker_and_defender_game(seed: u64) -> (GameState, sim::ids::PokemonId, sim::ids::PokemonId) {
    let mut db = CardDb::new();
    let attacker_def = db.add(CardDef::Pokemon(Pokemon {
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
        ability: None,
        attacks: vec![sim::card::Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let defender_def = db.add(CardDef::Pokemon(Pokemon {
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
        attacks: vec![],
    }));
    let energy_def = db.add(CardDef::Energy(Energy {
        print_id: "test-colorless-energy-attack",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    let mut attacker_deck = vec![attacker_def; 4];
    while attacker_deck.len() < 60 {
        attacker_deck.push(energy_def);
    }
    let mut defender_deck = vec![defender_def; 4];
    while defender_deck.len() < 60 {
        defender_deck.push(energy_def);
    }
    let mut state = GameState::new(db, [attacker_deck, defender_deck], Box::new(SeededRng::new(seed)));
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    let attacker = state.player(state.current).active.unwrap();
    let defender = state.player(state.current.opponent()).active.unwrap();
    (state, attacker, defender)
}

fn pay_and_attack(state: &mut GameState, attacker: sim::ids::PokemonId) {
    let player = state.current;
    let cost_len = state.pokemon_def(attacker).attacks[0].cost.len();
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
        state.pokemon[attacker.index()].attached.push(card);
    }
    let attack = legal_actions(state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("a paid-for Active can attack");
    apply(state, attack).unwrap();
}

#[test]
fn counters_the_attacker_when_the_carrier_takes_damage_while_active() {
    let (mut state, attacker, defender) = attacker_and_defender_game(3);

    let spiky_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-spiky-energy",
        name: "Spiky Energy",
        kind: Type::Colorless,
        effect: Some(EnergyEffect::CountersAttackerOnDamageTakenWhileActive(20)),
    }));
    attach(&mut state, defender, spiky_def);

    pay_and_attack(&mut state, attacker);

    assert_eq!(state.pokemon(defender).damage, 10, "the attack's own damage still lands");
    assert_eq!(state.pokemon(attacker).damage, 20, "the attacker takes the counter-damage back");
}

#[test]
fn still_counters_the_attacker_on_a_knockout_hit() {
    let (mut state, attacker, defender) = attacker_and_defender_game(3);
    state.pokemon[defender.index()].damage = 195;

    let spiky_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-spiky-energy-ko",
        name: "Spiky Energy",
        kind: Type::Colorless,
        effect: Some(EnergyEffect::CountersAttackerOnDamageTakenWhileActive(20)),
    }));
    attach(&mut state, defender, spiky_def);

    pay_and_attack(&mut state, attacker);

    assert_eq!(state.pokemon(attacker).damage, 20, "still counters even on a Knockout");
}

#[test]
fn spiky_energy_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Spiky Energy" && c.playable.is_some()),
        "at least one Spiky Energy print should play"
    );
}
