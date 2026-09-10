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
