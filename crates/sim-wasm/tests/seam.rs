//! The seam runs on the host. Every boundary method returns a JSON string,
//! so a test reads it back with `serde_json` and never needs a browser.

use sim_wasm::Game;

#[test]
fn a_new_synthetic_game_offers_legal_actions() {
    let game = Game::synthetic(1);
    let actions: Vec<String> = serde_json::from_str(&game.legal_actions()).unwrap();
    assert!(!actions.is_empty(), "a fresh game has at least one legal action");
}

#[test]
fn action_meta_is_index_aligned_with_the_labels() {
    let game = Game::synthetic(1);
    let labels: Vec<String> = serde_json::from_str(&game.legal_actions()).unwrap();
    let meta: Vec<serde_json::Value> = serde_json::from_str(&game.action_meta()).unwrap();
    assert_eq!(meta.len(), labels.len(), "one meta entry per legal action");
    for entry in &meta {
        assert!(entry["kind"].is_string(), "every entry names its Action variant");
        assert!(entry.get("card").is_some(), "every entry has a card slot");
        assert!(entry.get("target").is_some(), "every entry has a target slot");
    }
}

#[test]
fn action_meta_names_the_card_a_place_action_plays() {
    let game = Game::synthetic(1);
    let labels: Vec<String> = serde_json::from_str(&game.legal_actions()).unwrap();
    let meta: Vec<serde_json::Value> = serde_json::from_str(&game.action_meta()).unwrap();
    let placing = labels.iter().position(|l| l.starts_with("Place") || l.starts_with("Bench"));
    if let Some(i) = placing {
        assert!(
            meta[i]["card"].is_number(),
            "a place/bench action reports the hand card it plays"
        );
    }
}

#[test]
fn every_pokemon_in_view_carries_a_stable_id() {
    let mut game = Game::synthetic(1);
    // Get a Pokémon onto the board.
    for _ in 0..40 {
        if let Ok(()) = game.apply(0) {
            let view: serde_json::Value = serde_json::from_str(&game.view()).unwrap();
            let active = &view["sides"][0]["active"];
            if active.is_object() {
                assert!(active["id"].is_number(), "a Pokémon in view has a numeric id");
                assert!(
                    active["print_id"].as_str().is_some_and(|s| !s.is_empty()),
                    "and a non-empty TCGdex print id for its art"
                );
                return;
            }
        }
    }
}

#[test]
fn apply_by_index_grows_the_log() {
    let mut game = Game::synthetic(1);
    let before: Vec<String> = serde_json::from_str(&game.log()).unwrap();
    game.apply(0).expect("index 0 is a legal action");
    let after: Vec<String> = serde_json::from_str(&game.log()).unwrap();
    assert!(after.len() > before.len(), "an applied action writes a log line");
}

#[test]
fn apply_rejects_an_index_past_the_list() {
    let mut game = Game::synthetic(1);
    let count = serde_json::from_str::<Vec<String>>(&game.legal_actions())
        .unwrap()
        .len();
    assert!(game.apply(count).is_err(), "an out-of-range index is refused");
}

#[test]
fn the_view_reports_both_sides_and_the_viewer() {
    let game = Game::synthetic(1);
    let view: serde_json::Value = serde_json::from_str(&game.view()).unwrap();
    assert_eq!(view["sides"].as_array().unwrap().len(), 2);
    assert!(view["you"].is_number());
    assert!(view["phase"].is_string());
}
#[test]
fn a_card_in_view_carries_its_energy_type_slot() {
    let game = Game::synthetic(1);
    let view: serde_json::Value = serde_json::from_str(&game.view()).unwrap();
    let hand = view["your_hand"].as_array().unwrap();
    assert!(!hand.is_empty(), "the opening hand is not empty");
    for card in hand {
        assert!(
            card.get("energy_type").is_some(),
            "every card object has an energy_type field (string or null)"
        );
    }
    // The synthetic starter deck is Basic Energy plus three Basics, so at
    // least one hand card names a type.
    assert!(
        hand.iter().any(|c| c["energy_type"].is_string()),
        "some hand card is an Energy with a named type"
    );
}

#[test]
fn action_meta_carries_a_face_for_every_card_it_names() {
    // Drive the synthetic game so `Place` / `Bench` actions — which name
    // a hand card — appear, and check each carries its face.
    let mut game = Game::synthetic(1);
    for step in 0..20 {
        let meta: Vec<serde_json::Value> = serde_json::from_str(&game.action_meta()).unwrap();
        for entry in &meta {
            if entry["card"].is_number() {
                let face = &entry["card_face"];
                assert!(
                    face.is_object(),
                    "step {step}: an action that names a card carries a card_face: {entry}"
                );
                assert!(
                    face["print_id"].as_str().is_some_and(|s| !s.is_empty()),
                    "step {step}: the face has a print id"
                );
                assert!(
                    face["name"].as_str().is_some_and(|s| !s.is_empty()),
                    "step {step}: the face has a name"
                );
                assert!(face.get("category").is_some(), "the face has a category slot");
            }
        }
        if game.apply(0).is_err() || game.is_over() {
            break;
        }
    }
}

#[test]
fn a_targeted_action_names_a_pokemon_in_view() {
    let targeted = [
        "Retreat",
        "Promote",
        "UseAbility",
        "Evolve",
        "AttachEnergy",
        "HealTarget",
        // Phantom Dive's damage counters, and the plain bench-damage
        // shape it shares with cards like it: both name only a Bench
        // Pokémon, so the board can offer them as a tap with no card.
        "PlaceDamageCounter",
        "DamageBenchedPokemon",
    ];
    let mut game = Game::synthetic(3);
    for _ in 0..60 {
        let meta: Vec<serde_json::Value> = serde_json::from_str(&game.action_meta()).unwrap();
        let view: serde_json::Value = serde_json::from_str(&game.view()).unwrap();
        let mut ids: Vec<u64> = Vec::new();
        for s in view["sides"].as_array().unwrap() {
            for slot in [&s["active"]].into_iter().chain(s["bench"].as_array().unwrap()) {
                if let Some(id) = slot["id"].as_u64() {
                    ids.push(id);
                }
            }
        }
        for entry in &meta {
            if targeted.contains(&entry["kind"].as_str().unwrap_or("")) {
                let target = entry["target"].as_u64();
                assert!(target.is_some(), "a {} action names a target: {entry}", entry["kind"]);
                assert!(
                    ids.contains(&target.unwrap()),
                    "the target is a Pokémon in view: {entry}"
                );
            }
        }
        if game.apply(0).is_err() || game.is_over() {
            break;
        }
    }
}

#[test]
fn a_view_always_reports_the_damage_counter_split() {
    // `counters_to_place` tells the board how many Phantom Dive-style
    // counters are still to place. It is null in every other phase, but
    // the key is always there so the front end never guesses.
    let game = Game::synthetic(1);
    let view: serde_json::Value = serde_json::from_str(&game.view()).unwrap();
    assert!(view.get("counters_to_place").is_some(), "the key is always present");
    assert!(
        view["counters_to_place"].is_null(),
        "null outside the distribution phase"
    );
}
