//! Beyond the field: Super Potion (heal-then-discard) and Misty's
//! Vitality (a search that always ends the turn, moved or not).

use sim::action::{Action, legal_actions};
use sim::card::{CardDb, CardDef, Energy, Pokemon, Stage, Trainer, TrainerEffect, TrainerKind, Type};
use sim::engine::apply;
use sim::ids::{CardDefId, CardId, PlayerId};
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

fn mon(db: &mut CardDb, print_id: &'static str, name: &'static str, kind: Type) -> CardDefId {
    db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id,
        name,
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
    }))
}

fn game_through_setup(seed: u64) -> (GameState, CardDb) {
    let mut db = CardDb::new();
    let colorless = mon(&mut db, "test-mon", "Testmon", Type::Colorless);
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-colorless-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    let water_energy = db.add(CardDef::Energy(Energy {
        print_id: "test-water-energy",
        name: "Water Energy",
        kind: Type::Water,
        effect: None,
    }));
    let mut decklist = vec![colorless; 4];
    decklist.extend([water_energy; 8]);
    while decklist.len() < 60 {
        decklist.push(energy);
    }
    let mut state = GameState::new(db.clone(), [decklist.clone(), decklist], Box::new(SeededRng::new(seed)));
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
    let db = state.db.clone();
    (state, db)
}

fn attach(state: &mut GameState, pokemon: sim::ids::PokemonId, owner: PlayerId, def: CardDefId) -> CardId {
    let card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def, owner });
    state.pokemon[pokemon.index()].attached.push(card);
    card
}

#[test]
fn super_potion_heals_sixty_then_asks_which_energy_to_discard() {
    let (mut state, _db) = game_through_setup(1);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    state.pokemon[active.index()].damage = 80;
    let energy_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-attached-colorless",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    let energy = attach(&mut state, active, player, energy_def);

    let super_potion = state.db.add(CardDef::Trainer(Trainer {
        print_id: "test-super-potion",
        name: "Super Potion",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::HealChosenThenDiscardEnergyIfHealed(60),
    }));
    let card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: super_potion, owner: player });
    state.players[player.index()].hand.push(card);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::HealTarget { target: active }).unwrap();

    assert_eq!(state.pokemon(active).damage, 20, "60 damage healed off 80");
    assert!(
        matches!(state.phase, Phase::ChoosingOwnEnergyToDiscardForAttack { attacker, .. } if attacker == active),
        "healing removed damage, so the owner must now discard an Energy"
    );

    apply(&mut state, Action::DiscardOwnEnergyForAttack { card: energy }).unwrap();
    assert!(!state.pokemon(active).attached.contains(&energy));
    assert!(state.players[player.index()].discard.contains(&energy));
    assert_eq!(state.phase, Phase::Main);
}

#[test]
fn super_potion_asks_nothing_when_there_was_no_damage_to_heal() {
    let (mut state, _db) = game_through_setup(1);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    assert_eq!(state.pokemon(active).damage, 0);

    let super_potion = state.db.add(CardDef::Trainer(Trainer {
        print_id: "test-super-potion-no-damage",
        name: "Super Potion",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::HealChosenThenDiscardEnergyIfHealed(60),
    }));
    let card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: super_potion, owner: player });
    state.players[player.index()].hand.push(card);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::HealTarget { target: active }).unwrap();

    assert_eq!(state.phase, Phase::Main, "nothing was healed, so no discard is asked");
}

#[test]
fn super_potion_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Super Potion" && c.playable.is_some()),
        "Super Potion should play"
    );
}

#[test]
fn mistys_vitality_ends_the_turn_even_with_nothing_to_find() {
    let mut db = CardDb::new();
    let colorless = mon(&mut db, "test-mon-no-water", "Testmon", Type::Colorless);
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-colorless-energy-no-water",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    let mut decklist = vec![colorless; 4];
    while decklist.len() < 60 {
        decklist.push(energy);
    }
    let mut state = GameState::new(db, [decklist.clone(), decklist], Box::new(SeededRng::new(1)));
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
    let player = state.current;
    let turn_before = state.turn_number;

    let mistys_vitality = state.db.add(CardDef::Trainer(Trainer {
        print_id: "test-mistys-vitality",
        name: "Misty's Vitality",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: sim::card::Zone::Deck,
            slots: vec![sim::card::Slot {
                filter: sim::card::CardFilter::BasicEnergyOfType(Type::Water),
                to: sim::card::Destination::Attach(sim::card::TargetFilter::AnyInPlay),
                limit: 4,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: Some(sim::card::Then::EndTurnAlways),
        },
    }));
    let card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: mistys_vitality, owner: player });
    state.players[player.index()].hand.push(card);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(
        state.turn_number > turn_before || state.current != player,
        "the turn ends even though nothing was found"
    );
}

#[test]
fn mistys_vitality_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Misty's Vitality" && c.playable.is_some()),
        "Misty's Vitality should play"
    );
}
