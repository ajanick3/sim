//! Ticket 05: a card definition carries its print id.

use sim::card::{CardDb, CardDef, Pokemon, Type};
use sim::import::load;

fn bulbasaur() -> Pokemon {
    Pokemon {
        print_id: "test-a",
        name: "Test Mon",
        hp: 60,
        kind: Type::Grass,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        evolve_from: None,
        attacks: vec![],
    }
}

#[test]
fn a_definition_carries_the_print_id_it_came_from() {
    let mut db = CardDb::new();
    let id = db.add(CardDef::Pokemon(bulbasaur()));
    assert_eq!(db.get(id).print_id(), "test-a");
}

#[test]
fn two_cards_of_the_same_name_and_different_text_stay_separate() {
    let mut db = CardDb::new();
    let mut first = bulbasaur();
    first.print_id = "printing-one";
    first.hp = 60;
    let mut second = bulbasaur();
    second.print_id = "printing-two";
    second.hp = 90;

    let one = db.add(CardDef::Pokemon(first));
    let two = db.add(CardDef::Pokemon(second));

    assert_eq!(db.get(one).name(), db.get(two).name(), "same printed name");
    assert_ne!(
        db.get(one).print_id(),
        db.get(two).print_id(),
        "different printings stay identifiable"
    );
    assert_ne!(
        db.get(one).as_pokemon().unwrap().hp,
        db.get(two).as_pokemon().unwrap().hp,
        "and their own stats, never merged"
    );
}

#[test]
fn the_imported_print_id_matches_the_artifacts_card_id() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = load(&json).unwrap();

    let chikorita = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .find(|def| def.name() == "Chikorita")
        .expect("Chikorita is admitted");
    assert_eq!(chikorita.print_id(), "me01-008");
}

#[test]
fn basic_energy_carries_a_stable_print_id() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let mut import = load(&json).unwrap();
    let id = import.basic_energy(Type::Fire);
    assert_eq!(import.db.get(id).print_id(), "basic-fire-energy");
}
