//! The synthetic card set: three Basics and the Energy they need.
//!
//! These are literals, not data. The engine is what this project builds, so
//! the set stays small and grows a card only when that card teaches the engine
//! something it cannot yet express. Real cards wait in `data/cards.json`.

use crate::card::{Attack, CardDb, CardDef, Energy, Pokemon, Type};
use crate::ids::CardDefId;

pub struct Milestone1 {
    pub db: CardDb,
    pub sparkmouse: CardDefId,
    pub cinderpup: CardDefId,
    pub aquabear: CardDefId,
    pub lightning_energy: CardDefId,
    pub fire_energy: CardDefId,
    pub water_energy: CardDefId,
}

pub fn milestone1() -> Milestone1 {
    let mut db = CardDb::new();

    // Weak to Fighting, which nothing here is: the field is exercised by the
    // pair below, not by this one.
    let sparkmouse = db.add(CardDef::Pokemon(Pokemon {
        print_id: "synthetic-sparkmouse",
        name: "Sparkmouse",
        hp: 60,
        kind: Type::Lightning,
        weakness: Some(Type::Fighting),
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        evolve_from: None,
        attacks: vec![
            Attack {
                name: "Nibble",
                cost: vec![Type::Lightning],
                base_damage: 10,
                inflicts: None,
            },
            Attack {
                name: "Spark Tackle",
                cost: vec![Type::Lightning, Type::Colorless],
                base_damage: 30,
                inflicts: None,
            },
        ],
    }));

    // Weak to Water, so Aquabear doubles into it. Resistant to Water it is
    // not — the two modifiers must not be testable at once by accident.
    let cinderpup = db.add(CardDef::Pokemon(Pokemon {
        print_id: "synthetic-cinderpup",
        name: "Cinderpup",
        hp: 70,
        kind: Type::Fire,
        weakness: Some(Type::Water),
        resistance: None,
        retreat_cost: 2,
        prizes: 1,
        evolve_from: None,
        attacks: vec![Attack {
            name: "Ember",
            cost: vec![Type::Fire, Type::Fire],
            base_damage: 40,
            inflicts: None,
        }],
    }));

    // Weak to Fire, so Cinderpup doubles into it. Resistant to Fire it is
    // not — the two modifiers must not be testable at once by accident.
    let aquabear = db.add(CardDef::Pokemon(Pokemon {
        print_id: "synthetic-aquabear",
        name: "Aquabear",
        hp: 70,
        kind: Type::Water,
        weakness: Some(Type::Fire),
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        evolve_from: None,
        attacks: vec![Attack {
            name: "Bubblebeam",
            cost: vec![Type::Water, Type::Colorless],
            base_damage: 30,
            inflicts: None,
        }],
    }));

    let lightning_energy = db.add(CardDef::Energy(Energy {
        print_id: "synthetic-lightning-energy",
        name: "Lightning Energy",
        kind: Type::Lightning,
    }));

    let fire_energy = db.add(CardDef::Energy(Energy {
        print_id: "synthetic-fire-energy",
        name: "Fire Energy",
        kind: Type::Fire,
    }));

    let water_energy = db.add(CardDef::Energy(Energy {
        print_id: "synthetic-water-energy",
        name: "Water Energy",
        kind: Type::Water,
    }));

    Milestone1 {
        db,
        sparkmouse,
        cinderpup,
        aquabear,
        lightning_energy,
        fire_energy,
        water_energy,
    }
}

/// A legal 60: 4 of each Pokémon, then Energy of both types. Basic Energy has
/// no copy limit (rule 2), so this passes deck construction.
pub fn starter_decklist(set: &Milestone1) -> Vec<CardDefId> {
    let mut decklist = Vec::new();
    for _ in 0..4 {
        decklist.push(set.sparkmouse);
    }
    for _ in 0..4 {
        decklist.push(set.cinderpup);
    }
    for _ in 0..4 {
        decklist.push(set.aquabear);
    }
    while decklist.len() < 60 {
        decklist.push(set.lightning_energy);
        decklist.push(set.fire_energy);
        decklist.push(set.water_energy);
    }
    decklist
}
