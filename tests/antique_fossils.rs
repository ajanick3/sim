//! Beyond the field: the "Antique … Fossil" Items, admitted as a
//! synthetic Basic Pokémon rather than a Trainer. ADR 0105.

use sim::action::{Action, legal_actions};
use sim::card::{Ability, AbilityEffect, Attack, CardDb, CardDef, Condition, Energy, Pokemon, Stage, Type};
use sim::engine::{apply, damage_dealt};
use sim::ids::{CardDefId, CardId, PlayerId, PokemonId};
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

fn fossil_def(db: &mut CardDb, print_id: &'static str, name: &'static str, ability: Ability) -> CardDefId {
    db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id,
        name,
        hp: 60,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: u8::MAX,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: Some(ability),
        attacks: Vec::new(),
    }))
}

fn attacker_def(db: &mut CardDb, print_id: &'static str, inflicts: Option<Condition>) -> CardDefId {
    db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id,
        name: "Attackmon",
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
            name: "Hit",
            cost: vec![Type::Colorless],
            base_damage: 50,
            inflicts,
            effect: None,
        }],
    }))
}

/// A minimal one-attacker-one-defender board, the same shape
/// `special_energy.rs`'s own `one_pokemon_game` already takes, doubled
/// so both an attacker and a fossil can sit in play at once.
fn two_pokemon_game(db: CardDb, attacker: CardDefId, defender: CardDefId) -> (GameState, PokemonId, PokemonId) {
    let deck_a = vec![attacker; 60];
    let mut state = GameState::new(db, [deck_a.clone(), deck_a.clone()], Box::new(SeededRng::new(3)));
    let attacker_card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: attacker, owner: PlayerId::One });
    let attacker_id = state.put_into_play(PlayerId::One, attacker_card);
    state.players[PlayerId::One.index()].active = Some(attacker_id);

    let defender_card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: defender, owner: PlayerId::Two });
    let defender_id = state.put_into_play(PlayerId::Two, defender_card);
    state.players[PlayerId::Two.index()].active = Some(defender_id);

    (state, attacker_id, defender_id)
}

#[test]
fn six_fossils_are_admitted_and_sail_is_not() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    for name in [
        "Antique Jaw Fossil",
        "Antique Armor Fossil",
        "Antique Skull Fossil",
        "Antique Cover Fossil",
        "Antique Root Fossil",
        "Antique Plume Fossil",
    ] {
        assert!(
            import.cards.iter().any(|c| c.name == name && c.playable.is_some()),
            "{name} should play"
        );
    }
    assert!(
        import.cards.iter().all(|c| c.name != "Antique Sail Fossil" || c.playable.is_none()),
        "Antique Sail Fossil has no single read site for its Ability and stays refused"
    );
}

#[test]
fn a_fossil_plays_from_hand_as_a_basic_pokemon() {
    let mut db = CardDb::new();
    let fossil = fossil_def(
        &mut db,
        "test-jaw-fossil",
        "Antique Jaw Fossil",
        Ability { name: "Intimidating Jaw", effect: AbilityEffect::PassiveWhileActiveTakesLessDamage(30) },
    );
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-colorless-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    let mut deck = vec![fossil; 4];
    while deck.len() < 60 {
        deck.push(energy);
    }
    let mut state = GameState::new(db, [deck.clone(), deck], Box::new(SeededRng::new(3)));
    while !matches!(state.phase, Phase::PlacingActive { .. }) {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    let Phase::PlacingActive { player } = state.phase else { panic!("expected PlacingActive") };
    let card = state.player(player).hand.iter().copied().find(|c| state.def_of(*c).name() == "Antique Jaw Fossil").expect("a fossil in hand");
    apply(&mut state, Action::PlaceActive { card }).unwrap();
    let active = state.player(player).active.expect("the fossil is now Active");
    assert_eq!(state.pokemon_def(active).name, "Antique Jaw Fossil");
    assert_eq!(state.effective_hp(active), 60);
}

#[test]
fn a_fossil_cannot_retreat() {
    let mut db = CardDb::new();
    let fossil = fossil_def(
        &mut db,
        "test-jaw-fossil-retreat",
        "Antique Jaw Fossil",
        Ability { name: "Intimidating Jaw", effect: AbilityEffect::PassiveWhileActiveTakesLessDamage(30) },
    );
    let bench_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-benchmon",
        name: "Benchmon",
        hp: 100,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 0,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![],
    }));
    let mut state = GameState::new(
        db,
        [vec![fossil; 60], vec![fossil; 60]],
        Box::new(SeededRng::new(3)),
    );
    let fossil_card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: fossil, owner: PlayerId::One });
    let active = state.put_into_play(PlayerId::One, fossil_card);
    state.players[PlayerId::One.index()].active = Some(active);
    let bench_card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: bench_mon, owner: PlayerId::One });
    let bench = state.put_into_play(PlayerId::One, bench_card);
    state.players[PlayerId::One.index()].bench.push(bench);
    state.current = PlayerId::One;
    state.phase = Phase::Main;

    assert!(
        !legal_actions(&state).contains(&Action::Retreat { to: bench }),
        "a Retreat Cost of u8::MAX is never payable"
    );
}

#[test]
fn a_fossil_is_immune_to_every_special_condition() {
    let mut db = CardDb::new();
    let fossil = fossil_def(
        &mut db,
        "test-jaw-fossil-immune",
        "Antique Jaw Fossil",
        Ability { name: "Intimidating Jaw", effect: AbilityEffect::PassiveWhileActiveTakesLessDamage(30) },
    );
    let mut state = GameState::new(db, [vec![fossil; 60], vec![fossil; 60]], Box::new(SeededRng::new(3)));
    let card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: fossil, owner: PlayerId::One });
    let pokemon = state.put_into_play(PlayerId::One, card);

    state.inflict(pokemon, Condition::Poisoned);

    assert!(!state.has_condition(pokemon, Condition::Poisoned));
}

#[test]
fn discarding_an_active_fossil_promotes_from_bench_with_no_prize() {
    let mut db = CardDb::new();
    let fossil = fossil_def(
        &mut db,
        "test-jaw-fossil-discard",
        "Antique Jaw Fossil",
        Ability { name: "Intimidating Jaw", effect: AbilityEffect::PassiveWhileActiveTakesLessDamage(30) },
    );
    let bench_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-benchmon-2",
        name: "Benchmon",
        hp: 100,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 0,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![],
    }));
    let mut state = GameState::new(db, [vec![fossil; 60], vec![fossil; 60]], Box::new(SeededRng::new(3)));
    let fossil_card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: fossil, owner: PlayerId::One });
    let active = state.put_into_play(PlayerId::One, fossil_card);
    state.players[PlayerId::One.index()].active = Some(active);
    let bench_card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: bench_mon, owner: PlayerId::One });
    let bench = state.put_into_play(PlayerId::One, bench_card);
    state.players[PlayerId::One.index()].bench.push(bench);
    state.current = PlayerId::One;
    state.phase = Phase::Main;
    let opponent_prizes_before = state.player(PlayerId::Two).prizes.len();

    assert!(legal_actions(&state).contains(&Action::DiscardOwnPokemonFromPlay { pokemon: active }));
    apply(&mut state, Action::DiscardOwnPokemonFromPlay { pokemon: active }).unwrap();
    // Rule 40: the player whose Active left chooses the next one, even
    // with only one Benched Pokémon to choose from.
    assert!(matches!(state.phase, Phase::Promoting { .. }));
    apply(&mut state, Action::Promote { pokemon: bench }).unwrap();

    assert_eq!(
        state.player(PlayerId::Two).prizes.len(),
        opponent_prizes_before,
        "a voluntary discard is not a Knockout"
    );
    assert!(state.players[PlayerId::One.index()].discard.contains(&fossil_card));
    assert_eq!(state.player(PlayerId::One).active, Some(bench), "the Bench promotes");
}

#[test]
fn discarding_a_benched_fossil_just_removes_it() {
    let mut db = CardDb::new();
    let fossil = fossil_def(
        &mut db,
        "test-jaw-fossil-bench-discard",
        "Antique Jaw Fossil",
        Ability { name: "Intimidating Jaw", effect: AbilityEffect::PassiveWhileActiveTakesLessDamage(30) },
    );
    let active_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-activemon",
        name: "Activemon",
        hp: 100,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 0,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![],
    }));
    let mut state = GameState::new(db, [vec![fossil; 60], vec![fossil; 60]], Box::new(SeededRng::new(3)));
    let active_card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: active_mon, owner: PlayerId::One });
    let active = state.put_into_play(PlayerId::One, active_card);
    state.players[PlayerId::One.index()].active = Some(active);
    let fossil_card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: fossil, owner: PlayerId::One });
    let bench = state.put_into_play(PlayerId::One, fossil_card);
    state.players[PlayerId::One.index()].bench.push(bench);
    state.current = PlayerId::One;
    state.phase = Phase::Main;

    apply(&mut state, Action::DiscardOwnPokemonFromPlay { pokemon: bench }).unwrap();

    assert_eq!(state.player(PlayerId::One).active, Some(active), "the Active is untouched");
    assert!(state.player(PlayerId::One).bench.is_empty());
    assert!(state.players[PlayerId::One.index()].discard.contains(&fossil_card));
}

#[test]
fn intimidating_jaw_reduces_damage_to_the_active_jaw_fossil() {
    let mut db = CardDb::new();
    let attacker = attacker_def(&mut db, "test-jaw-attacker", None);
    let jaw = fossil_def(
        &mut db,
        "test-jaw-fossil-damage",
        "Antique Jaw Fossil",
        Ability { name: "Intimidating Jaw", effect: AbilityEffect::PassiveWhileActiveTakesLessDamage(30) },
    );
    let (state, attacker_id, defender_id) = two_pokemon_game(db, attacker, jaw);

    assert_eq!(damage_dealt(&state, attacker_id, defender_id, 50), 20);
}

#[test]
fn protective_armor_reduces_damage_while_it_is_the_active_shielding_itself() {
    let mut db = CardDb::new();
    let attacker = attacker_def(&mut db, "test-armor-attacker", None);
    let armor = fossil_def(
        &mut db,
        "test-armor-fossil",
        "Antique Armor Fossil",
        Ability { name: "Protective Armor", effect: AbilityEffect::PassiveWhileActiveReducesDamageToOwnSide(10) },
    );
    let (state, attacker_id, defender_id) = two_pokemon_game(db, attacker, armor);

    assert_eq!(damage_dealt(&state, attacker_id, defender_id, 50), 40);
}

#[test]
fn spiny_skull_counters_the_attacker_even_on_a_knockout() {
    let mut db = CardDb::new();
    let attacker = attacker_def(&mut db, "test-skull-attacker", None);
    let skull = fossil_def(
        &mut db,
        "test-skull-fossil",
        "Antique Skull Fossil",
        Ability { name: "Spiny Skull", effect: AbilityEffect::PassiveWhileActiveCountersAttackerOnDamageTaken(30) },
    );
    let mut state = GameState::new(
        db,
        [vec![attacker; 60], vec![skull; 60]],
        Box::new(SeededRng::new(3)),
    );
    let attacker_card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: attacker, owner: PlayerId::One });
    let attacker_id = state.put_into_play(PlayerId::One, attacker_card);
    state.players[PlayerId::One.index()].active = Some(attacker_id);
    let energy = state.db.add(CardDef::Energy(Energy {
        print_id: "test-attack-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    let energy_card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: energy, owner: PlayerId::One });
    state.pokemon[attacker_id.index()].attached.push(energy_card);

    let defender_card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: skull, owner: PlayerId::Two });
    let defender_id = state.put_into_play(PlayerId::Two, defender_card);
    state.players[PlayerId::Two.index()].active = Some(defender_id);
    state.current = PlayerId::One;
    state.turn_number = 1;
    state.phase = Phase::Main;

    apply(&mut state, Action::Attack { index: 0 }).unwrap();

    assert!(state.pokemon(attacker_id).damage >= 30, "the attacker takes 3 damage counters back");
}

#[test]
fn protective_cover_prevents_the_condition_but_not_the_damage() {
    let mut db = CardDb::new();
    let attacker = attacker_def(&mut db, "test-cover-attacker", Some(Condition::Poisoned));
    let cover = fossil_def(
        &mut db,
        "test-cover-fossil",
        "Antique Cover Fossil",
        Ability { name: "Protective Cover", effect: AbilityEffect::PassivePreventsAttackEffectsOnSelf },
    );
    let mut state = GameState::new(
        db,
        [vec![attacker; 60], vec![cover; 60]],
        Box::new(SeededRng::new(3)),
    );
    let attacker_card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: attacker, owner: PlayerId::One });
    let attacker_id = state.put_into_play(PlayerId::One, attacker_card);
    state.players[PlayerId::One.index()].active = Some(attacker_id);
    let energy = state.db.add(CardDef::Energy(Energy {
        print_id: "test-attack-energy-cover",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    let energy_card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: energy, owner: PlayerId::One });
    state.pokemon[attacker_id.index()].attached.push(energy_card);

    let defender_card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: cover, owner: PlayerId::Two });
    let defender_id = state.put_into_play(PlayerId::Two, defender_card);
    state.players[PlayerId::Two.index()].active = Some(defender_id);
    state.current = PlayerId::One;
    state.turn_number = 1;
    state.phase = Phase::Main;

    apply(&mut state, Action::Attack { index: 0 }).unwrap();

    assert!(state.pokemon(defender_id).damage > 0, "damage is not an effect, and still lands");
    assert!(
        !state.has_condition(defender_id, Condition::Poisoned),
        "the condition is an effect, and is prevented"
    );
}

#[test]
fn plume_protection_shields_only_while_benched() {
    let mut db = CardDb::new();
    let plume = fossil_def(
        &mut db,
        "test-plume-fossil",
        "Antique Plume Fossil",
        Ability { name: "Plume Protection", effect: AbilityEffect::PassiveWhileBenchedPreventsAllDamage },
    );
    let mut state = GameState::new(db, [vec![plume; 60], vec![plume; 60]], Box::new(SeededRng::new(3)));
    let card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: plume, owner: PlayerId::One });
    let benched = state.put_into_play(PlayerId::One, card);
    state.players[PlayerId::One.index()].bench.push(benched);

    assert!(state.bench_attack_damage_blocked(PlayerId::Two, benched));
}

#[test]
fn primal_root_taxes_an_opponent_basics_attack_while_active() {
    let mut db = CardDb::new();
    let root = fossil_def(
        &mut db,
        "test-root-fossil",
        "Antique Root Fossil",
        Ability { name: "Primal Root", effect: AbilityEffect::PassiveWhileActiveOpponentBasicAttacksCostMore },
    );
    let basic_attacker = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-root-basic-attacker",
        name: "Basicmon",
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
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let mut state = GameState::new(
        db,
        [vec![basic_attacker; 60], vec![root; 60]],
        Box::new(SeededRng::new(3)),
    );
    let attacker_card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: basic_attacker, owner: PlayerId::One });
    let attacker_id = state.put_into_play(PlayerId::One, attacker_card);
    state.players[PlayerId::One.index()].active = Some(attacker_id);
    let energy = state.db.add(CardDef::Energy(Energy {
        print_id: "test-root-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    let energy_card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: energy, owner: PlayerId::One });
    state.pokemon[attacker_id.index()].attached.push(energy_card);

    let root_card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: root, owner: PlayerId::Two });
    let root_id = state.put_into_play(PlayerId::Two, root_card);
    state.players[PlayerId::Two.index()].active = Some(root_id);
    state.current = PlayerId::One;
    state.turn_number = 1;
    state.phase = Phase::Main;

    assert!(
        !legal_actions(&state).contains(&Action::Attack { index: 0 }),
        "one Colorless Energy no longer covers the surcharged cost"
    );
}
