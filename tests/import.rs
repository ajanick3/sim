//! Ticket 07: the engine reads the card artifact, and refuses what it cannot run.

use sim::card::Type;
use sim::import::{Refusal, load};

/// One card, wrapped in the artifact's shape.
fn artifact(cards: &str) -> String {
    format!(r#"{{"schema":1,"count":1,"cards":[{cards}]}}"#)
}

const PLAYABLE: &str = r#"{
  "id":"me01-008","name":"Chikorita","category":"Pokemon","stage":"Basic",
  "regulationMark":"I","hp":70,"types":["Grass"],"retreat":1,
  "weaknesses":[{"type":"Fire","value":"×2"}],
  "attacks":[{"cost":["Grass","Colorless"],"name":"Tackle","damage":30}]
}"#;

#[test]
fn a_card_the_engine_can_run_is_admitted_with_its_numbers() {
    let import = load(&artifact(PLAYABLE)).unwrap();
    assert_eq!(import.refused.len(), 0, "nothing here is unplayable");
    assert_eq!(import.admitted.len(), 1);

    let card = import.db.get(import.admitted[0]);
    let pokemon = card.as_pokemon().expect("a Pokémon card");
    assert_eq!(pokemon.name, "Chikorita");
    assert_eq!(pokemon.hp, 70);
    assert_eq!(pokemon.kind, Type::Grass);
    assert_eq!(pokemon.weakness, Some(Type::Fire));
    assert_eq!(pokemon.retreat_cost, 1);
    assert_eq!(pokemon.attacks[0].cost, vec![Type::Grass, Type::Colorless]);
    assert_eq!(pokemon.attacks[0].base_damage, 30);
}

#[test]
fn an_attack_with_an_effect_line_is_refused() {
    let card = PLAYABLE.replace(
        r#""damage":30"#,
        r#""damage":30,"effect":"Discard an Energy from this Pokémon.""#,
    );
    let import = load(&artifact(&card)).unwrap();
    assert!(import.admitted.is_empty());
    assert!(
        matches!(import.refused[0].because, Refusal::AttackHasText),
        "the engine cannot run English, so it must not half-load the card"
    );
}

#[test]
fn a_damage_the_engine_cannot_read_is_refused() {
    let card = PLAYABLE.replace(r#""damage":30"#, r#""damage":"30+""#);
    let import = load(&artifact(&card)).unwrap();
    assert!(matches!(
        import.refused[0].because,
        Refusal::DamageIsNotANumber
    ));
}

#[test]
fn an_ability_is_refused() {
    let card = PLAYABLE.replace(
        r#""retreat":1"#,
        r#""retreat":1,"abilities":[{"type":"Ability","name":"Blaze","effect":"Do a thing."}]"#,
    );
    let import = load(&artifact(&card)).unwrap();
    assert!(matches!(import.refused[0].because, Refusal::HasAnAbility));
}

#[test]
fn an_evolution_is_refused() {
    let card = PLAYABLE.replace(r#""stage":"Basic""#, r#""stage":"Stage1""#);
    let import = load(&artifact(&card)).unwrap();
    assert!(matches!(import.refused[0].because, Refusal::IsAnEvolution));
}

#[test]
fn a_trainer_is_refused() {
    let card = PLAYABLE.replace(r#""category":"Pokemon""#, r#""category":"Trainer""#);
    let import = load(&artifact(&card)).unwrap();
    assert!(matches!(import.refused[0].because, Refusal::IsATrainer(_)));
}

#[test]
fn every_refusal_names_the_card_it_refused() {
    let card = PLAYABLE.replace(r#""damage":30"#, r#""damage":"30+""#);
    let import = load(&artifact(&card)).unwrap();
    assert_eq!(import.refused[0].id, "me01-008");
    assert_eq!(import.refused[0].name, "Chikorita");
}

#[test]
fn the_standard_artifact_loads_and_every_card_is_accounted_for() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = load(&json).unwrap();

    assert_eq!(
        import.admitted.len() + import.refused.len(),
        3051,
        "every card is either admitted or refused, never dropped"
    );
    assert!(
        import.admitted.len() > 200,
        "the engine can run some of Standard: {} cards",
        import.admitted.len()
    );
}

#[test]
fn the_engine_plays_a_game_with_imported_cards() {
    use sim::action::{Action, legal_actions};
    use sim::engine::apply;
    use sim::rng::SeededRng;
    use sim::state::GameState;

    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let mut import = load(&json).unwrap();

    // Basic Energy is not printed in these sets, so the engine supplies it.
    let energy = import.basic_energy(Type::Colorless);
    // A deck needs a Basic to reach a legal opening board (rule 9); the
    // artifact now admits Stage 1 and 2 cards too, so the first admitted
    // card is not always one.
    let pokemon = *import
        .admitted
        .iter()
        .find(|id| import.db.get(**id).is_basic_pokemon())
        .expect("the artifact admits at least one Basic");
    let mut decklist = vec![pokemon; 12];
    while decklist.len() < 60 {
        decklist.push(energy);
    }

    let mut state = GameState::new(
        import.db,
        [decklist.clone(), decklist],
        Box::new(SeededRng::new(4)),
    );
    let mut steps = 0;
    while !state.is_over() {
        assert!(steps < 20_000, "an imported game should finish");
        let actions = legal_actions(&state);
        let choice = actions
            .iter()
            .find(|a| matches!(a, Action::Attack { .. }))
            .copied()
            .unwrap_or(actions[0]);
        apply(&mut state, choice).unwrap();
        steps += 1;
    }
    assert!(state.outcome.is_some(), "a real card played a real game");
}

#[test]
fn the_eight_committed_trainers_are_admitted_from_the_real_data() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = load(&json).unwrap();

    let admitted_trainer = |name: &str| {
        import
            .admitted
            .iter()
            .map(|id| import.db.get(*id))
            .find(|def| def.name() == name)
            .and_then(|def| def.as_trainer())
    };

    for name in [
        "Boss's Orders",
        "Judge",
        "Lillie's Determination",
        "Night Stretcher",
        "Poké Pad",
        "Crushing Hammer",
        "Gwynn",
        "Sacred Ash",
    ] {
        assert!(
            admitted_trainer(name).is_some(),
            "{name} should be admitted with its hand-authored effect"
        );
    }
}

// --- Ticket 06: the evolution chain, walked past a refused print ---

#[test]
fn a_stage_2_names_its_basic_even_when_the_stage_1_is_refused() {
    use sim::card::Stage;

    // A Basic, admitted; a Stage 1 refused for an ability; a Stage 2 that
    // evolves from the Stage 1. The Stage 1 print never enters the CardDb,
    // but its name still answers what the Stage 2 evolves from.
    let basic = r#"{
      "id":"t-1","name":"Seedling","category":"Pokemon","stage":"Basic",
      "regulationMark":"I","hp":60,"types":["Grass"],"retreat":1,
      "attacks":[{"cost":["Grass"],"name":"Vine","damage":10}]
    }"#;
    let stage1_refused = r#"{
      "id":"t-2","name":"Bloomling","category":"Pokemon","stage":"Stage1",
      "evolveFrom":"Seedling","regulationMark":"I","hp":90,"types":["Grass"],
      "retreat":1,"abilities":[{"name":"Overgrow","effect":"Does something."}],
      "attacks":[{"cost":["Grass"],"name":"Slash","damage":40}]
    }"#;
    let stage2 = r#"{
      "id":"t-3","name":"Bigflower","category":"Pokemon","stage":"Stage2",
      "evolveFrom":"Bloomling","regulationMark":"I","hp":150,"types":["Grass"],
      "retreat":2,
      "attacks":[{"cost":["Grass","Colorless"],"name":"Bloom","damage":80}]
    }"#;
    let import = load(&artifact(&format!("{basic},{stage1_refused},{stage2}"))).unwrap();

    assert_eq!(
        import.refused.len(),
        1,
        "the Stage 1 is refused for its ability: {:?}",
        import.refused
    );
    let stage2 = import
        .admitted
        .iter()
        .filter_map(|id| import.db.get(*id).as_pokemon())
        .find(|p| p.name == "Bigflower")
        .expect("the Stage 2 is admitted; nothing about it is unplayable");
    assert_eq!(stage2.stage, Stage::Stage2);
    assert_eq!(stage2.evolve_from, Some("Bloomling"));
    assert_eq!(
        stage2.evolves_from_basic,
        Some("Seedling"),
        "the chain walks through Bloomling's name even though Bloomling \
         itself was refused and never entered the CardDb"
    );
}

#[test]
fn a_stage_2_whose_chain_does_not_resolve_carries_no_basic() {
    use sim::card::Stage;

    // A Stage 2 whose parent's name appears nowhere else in the pool: the
    // artifact is incomplete, or this print names a parent that was never
    // included. The card is still admitted — ordinary evolution reads
    // evolve_from, which is present — but Rare Candy has nothing to offer.
    let stage2 = r#"{
      "id":"t-4","name":"Orphan","category":"Pokemon","stage":"Stage2",
      "evolveFrom":"NoSuchPrint","regulationMark":"I","hp":150,
      "types":["Grass"],"retreat":2,
      "attacks":[{"cost":["Grass","Colorless"],"name":"Bloom","damage":80}]
    }"#;
    let import = load(&artifact(stage2)).unwrap();
    let orphan = import
        .admitted
        .iter()
        .filter_map(|id| import.db.get(*id).as_pokemon())
        .find(|p| p.name == "Orphan")
        .expect("evolve_from is present, so the card is admitted");
    assert_eq!(orphan.stage, Stage::Stage2);
    assert_eq!(orphan.evolve_from, Some("NoSuchPrint"));
    assert_eq!(orphan.evolves_from_basic, None);
}

// --- Ticket 07: match a card by print when its name is not enough ---

#[test]
fn a_card_can_be_matched_by_print_id_when_its_name_is_not_enough() {
    use sim::card::TrainerEffect;

    // Two prints share a name but do not share behaviour — the situation
    // `known_trainer` alone cannot tell apart, since it reads only the
    // name. `known_trainer_by_print` is checked first, keyed on the
    // print's own id, and wins here.
    let one = r#"{
      "id":"test-print-a","name":"Ambiguous Trainer","category":"Trainer",
      "trainerType":"Item","regulationMark":"I",
      "effect":"One printing of this name."
    }"#;
    let two = r#"{
      "id":"test-print-b","name":"Ambiguous Trainer","category":"Trainer",
      "trainerType":"Item","regulationMark":"I",
      "effect":"A different printing of the same name."
    }"#;
    let import = load(&artifact(&format!("{one},{two}"))).unwrap();

    let effect_of = |print_id: &str| {
        let playable = import.cards.iter().find(|c| c.id == print_id)?.playable?;
        import
            .db
            .get(playable)
            .as_trainer()
            .map(|t| t.effect.clone())
    };
    assert_eq!(effect_of("test-print-a"), Some(TrainerEffect::Nothing));
    assert_eq!(
        effect_of("test-print-b"),
        Some(TrainerEffect::SwitchOpponentActive)
    );
}
