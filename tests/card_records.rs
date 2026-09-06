//! Card-records ticket 01: every card in the artifact stays readable.

use sim::import::load;

fn artifact() -> String {
    std::fs::read_to_string("data/cards.json").expect("the artifact is committed")
}

#[test]
fn a_refused_card_can_be_read_back_in_full() {
    let import = load(&artifact()).unwrap();

    // me01-113 is a Supporter, refused outright: its rules text has nowhere
    // to go in a CardDef. The raw record still holds it.
    let card = import
        .cards
        .iter()
        .find(|c| c.id == "me01-113")
        .expect("the artifact holds this card");
    assert!(card.playable.is_none(), "a Supporter is never admitted");
    let effect = card.raw["effect"]
        .as_str()
        .expect("a Trainer's raw record keeps its printed text");
    assert!(!effect.is_empty());
    assert_eq!(card.raw["category"].as_str(), Some("Trainer"));
}

#[test]
fn an_admitted_card_keeps_its_raw_record_too() {
    let import = load(&artifact()).unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Chikorita")
        .expect("Chikorita is in the artifact");
    assert!(card.playable.is_some());
    assert_eq!(card.raw["hp"].as_u64(), Some(70));
}

#[test]
fn the_admitted_cards_still_play_and_coverage_does_not_move() {
    let import = load(&artifact()).unwrap();
    let (admitted, total) = import.coverage();
    assert_eq!(total, 3051);
    // Ticket 03 raised this after this test was written, by admitting real
    // Trainers; the number moved for a reason unrelated to this ticket.
    assert_eq!(admitted, 368, "keeping the raw record changes no legality");
}
