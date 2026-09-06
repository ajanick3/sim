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
    let pokemon = import.admitted[0];
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
