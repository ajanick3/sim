//! The Standard path, against the real card artifact and two deck files.
//! One curated matchup, read from the repository at its manifest-relative
//! path so the test needs no working directory.

use sim_wasm::{CardData, Game};

const CARDS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/cards.json");
const DRAGAPULT: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../decks/2026-worlds/03-brent-tonisson.txt");
const ALAKAZAM: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../decks/2026-worlds/02-diego-cassiraga.txt");

#[test]
fn the_curated_matchup_starts_and_offers_actions() {
    let json = std::fs::read_to_string(CARDS).unwrap();
    let mut data = CardData::new(&json).unwrap();

    let a = std::fs::read_to_string(DRAGAPULT).unwrap();
    let b = std::fs::read_to_string(ALAKAZAM).unwrap();
    let game = Game::standard(&mut data, &a, &b, 7).unwrap();

    let actions: Vec<String> = serde_json::from_str(&game.legal_actions()).unwrap();
    assert!(!actions.is_empty());
}

#[test]
fn one_card_data_feeds_two_games() {
    let json = std::fs::read_to_string(CARDS).unwrap();
    let mut data = CardData::new(&json).unwrap();
    let a = std::fs::read_to_string(DRAGAPULT).unwrap();
    let b = std::fs::read_to_string(ALAKAZAM).unwrap();

    let one = Game::standard(&mut data, &a, &b, 1).unwrap();
    let two = Game::standard(&mut data, &a, &b, 2).unwrap();

    assert!(!serde_json::from_str::<Vec<String>>(&one.legal_actions()).unwrap().is_empty());
    assert!(!serde_json::from_str::<Vec<String>>(&two.legal_actions()).unwrap().is_empty());
}

#[test]
fn replay_reproduces_a_live_game() {
    let json = std::fs::read_to_string(CARDS).unwrap();
    let mut data = CardData::new(&json).unwrap();
    let a = std::fs::read_to_string(DRAGAPULT).unwrap();
    let b = std::fs::read_to_string(ALAKAZAM).unwrap();

    // Play a handful of moves live, always the first legal action.
    let mut live = Game::standard(&mut data, &a, &b, 9).unwrap();
    for _ in 0..12 {
        if serde_json::from_str::<Vec<String>>(&live.legal_actions()).unwrap().is_empty() {
            break;
        }
        live.apply(0).unwrap();
    }
    let indices: Vec<u32> = serde_json::from_str(&live.history()).unwrap();
    assert_eq!(indices.len(), 12, "twelve first-action moves were taken");

    // Rebuild from the recipe and confirm the same position.
    let replayed = Game::replay_standard(&mut data, &a, &b, 9, indices.clone()).unwrap();
    assert_eq!(live.log(), replayed.log());
    assert_eq!(live.view(), replayed.view());
    assert_eq!(live.legal_actions(), replayed.legal_actions());
    assert_eq!(replayed.history(), serde_json::to_string(&indices).unwrap());
}

#[test]
fn replay_rejects_an_illegal_index_and_says_which_move() {
    let json = std::fs::read_to_string(CARDS).unwrap();
    let mut data = CardData::new(&json).unwrap();
    let a = std::fs::read_to_string(DRAGAPULT).unwrap();
    let b = std::fs::read_to_string(ALAKAZAM).unwrap();

    let err = match Game::replay_standard(&mut data, &a, &b, 9, vec![0, 999]) {
        Ok(_) => panic!("index 999 is not a legal action"),
        Err(err) => err,
    };
    assert!(err.contains("move 1"), "the error names the bad move: {err}");
}
