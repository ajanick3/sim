//! The Milestone 1 card set: two synthetic Basics and one Energy.
//!
//! These are literals, not data. The engine is what Milestone 1 builds; real
//! cards arrive with the JSON the `pkmn` repo exports.

use crate::card::{Attack, CardDb, CardDef, Energy, Pokemon, Type};
use crate::ids::CardDefId;

pub struct Milestone1 {
    pub db: CardDb,
    pub sparkmouse: CardDefId,
    pub cinderpup: CardDefId,
    pub lightning_energy: CardDefId,
}

pub fn milestone1() -> Milestone1 {
    let mut db = CardDb::new();

    // Weak to Fighting, which nothing here is: the field is exercised by the
    // pair below, not by this one.
    let sparkmouse = db.add(CardDef::Pokemon(Pokemon {
        name: "Sparkmouse",
        hp: 60,
        kind: Type::Lightning,
        weakness: Some(Type::Fighting),
        resistance: None,
        retreat_cost: 1,
        attacks: vec![
            Attack {
                name: "Nibble",
                cost: 1,
                base_damage: 10,
            },
            Attack {
                name: "Spark Tackle",
                cost: 2,
                base_damage: 30,
            },
        ],
    }));

    // Weak to Lightning, so Sparkmouse doubles into it. Resistant to Lightning
    // it is not — the two modifiers must not be testable at once by accident.
    let cinderpup = db.add(CardDef::Pokemon(Pokemon {
        name: "Cinderpup",
        hp: 70,
        kind: Type::Fire,
        weakness: Some(Type::Lightning),
        resistance: None,
        retreat_cost: 2,
        attacks: vec![Attack {
            name: "Ember",
            cost: 2,
            base_damage: 40,
        }],
    }));

    let lightning_energy = db.add(CardDef::Energy(Energy {
        name: "Lightning Energy",
        kind: Type::Lightning,
    }));

    Milestone1 {
        db,
        sparkmouse,
        cinderpup,
        lightning_energy,
    }
}

/// A legal 60: 4 of each Pokémon and 52 Energy. Basic Energy has no copy limit
/// (rule 2), so this passes deck construction.
pub fn starter_decklist(set: &Milestone1) -> Vec<CardDefId> {
    let mut decklist = Vec::new();
    for _ in 0..4 {
        decklist.push(set.sparkmouse);
    }
    for _ in 0..4 {
        decklist.push(set.cinderpup);
    }
    while decklist.len() < 60 {
        decklist.push(set.lightning_energy);
    }
    decklist
}
