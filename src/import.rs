//! Reading the card artifact.
//!
//! The engine plays a small part of the game, so it can run a small part of
//! the cards. A card whose text the engine cannot execute is refused by name
//! and by reason, never half-loaded: a card that silently loses its effect
//! would play a game that is not Pokémon.
//!
//! Reading the file is the caller's job. This module takes the JSON as a
//! string, so the engine keeps no I/O.

use serde_json::Value;

use crate::card::{Attack, CardDb, CardDef, Energy, Pokemon, Type};
use crate::ids::CardDefId;

/// Why the engine cannot run a card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// A Trainer, an Energy, or an evolving Pokémon. The engine plays Basics.
    NotABasicPokemon,
    /// An ability is text the engine cannot execute.
    HasAnAbility,
    /// The attack carries an effect line, which is English, not a rule.
    AttackHasText,
    /// Damage such as `30+` or `60×` depends on an effect.
    DamageIsNotANumber,
    /// The card has nothing to do on a turn.
    HasNoAttack,
    /// A type, a Weakness, or a Resistance the engine does not model.
    UnknownSymbol,
}

/// One refused card, named so a reader can go and look at it.
#[derive(Debug, Clone)]
pub struct Refused {
    pub id: String,
    pub name: String,
    pub because: Refusal,
}

/// The result of reading the artifact: the cards the engine can play, and an
/// account of every card it cannot.
#[derive(Debug)]
pub struct Import {
    pub db: CardDb,
    pub admitted: Vec<CardDefId>,
    pub refused: Vec<Refused>,
    basic_energy: Vec<(Type, CardDefId)>,
}

impl Import {
    /// The basic Energy of a type, added to the database on first ask.
    ///
    /// Basic Energy is not printed in the Standard sets — it comes from a
    /// separate product and carries no regulation mark — so the artifact holds
    /// none. It is rules furniture rather than a card to import, so the engine
    /// supplies it.
    pub fn basic_energy(&mut self, kind: Type) -> CardDefId {
        if let Some(id) = self.basic_energy.iter().find(|(k, _)| *k == kind) {
            return id.1;
        }
        let id = self.db.add(CardDef::Energy(Energy {
            name: basic_energy_name(kind),
            kind,
        }));
        self.basic_energy.push((kind, id));
        id
    }

    /// How many cards of those read the engine can play.
    pub fn coverage(&self) -> (usize, usize) {
        (
            self.admitted.len(),
            self.admitted.len() + self.refused.len(),
        )
    }

    /// How many cards each reason refused.
    pub fn refusals_by_reason(&self) -> Vec<(Refusal, usize)> {
        let reasons = [
            Refusal::NotABasicPokemon,
            Refusal::HasAnAbility,
            Refusal::AttackHasText,
            Refusal::DamageIsNotANumber,
            Refusal::HasNoAttack,
            Refusal::UnknownSymbol,
        ];
        reasons
            .into_iter()
            .map(|reason| {
                let count = self.refused.iter().filter(|r| r.because == reason).count();
                (reason, count)
            })
            .filter(|(_, count)| *count > 0)
            .collect()
    }
}

/// Read the artifact. The error is for a file that is not the artifact at all.
pub fn load(json: &str) -> Result<Import, String> {
    let root: Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
    let cards = root["cards"]
        .as_array()
        .ok_or("the artifact holds no card list")?;

    let mut import = Import {
        db: CardDb::new(),
        admitted: Vec::new(),
        refused: Vec::new(),
        basic_energy: Vec::new(),
    };

    for card in cards {
        match read_card(card) {
            Ok(def) => {
                let id = import.db.add(def);
                import.admitted.push(id);
            }
            Err(because) => import.refused.push(Refused {
                id: card["id"].as_str().unwrap_or("?").to_string(),
                name: card["name"].as_str().unwrap_or("?").to_string(),
                because,
            }),
        }
    }

    Ok(import)
}

fn read_card(card: &Value) -> Result<CardDef, Refusal> {
    if card["category"].as_str() != Some("Pokemon") || card["stage"].as_str() != Some("Basic") {
        return Err(Refusal::NotABasicPokemon);
    }
    if card["abilities"].as_array().is_some_and(|a| !a.is_empty()) {
        return Err(Refusal::HasAnAbility);
    }

    let attacks_json = card["attacks"].as_array().ok_or(Refusal::HasNoAttack)?;
    if attacks_json.is_empty() {
        return Err(Refusal::HasNoAttack);
    }
    let mut attacks = Vec::new();
    for attack in attacks_json {
        attacks.push(read_attack(attack)?);
    }

    let kind = card["types"]
        .as_array()
        .and_then(|types| types.first())
        .and_then(Value::as_str)
        .and_then(read_type)
        .ok_or(Refusal::UnknownSymbol)?;

    Ok(CardDef::Pokemon(Pokemon {
        name: leak(card["name"].as_str().unwrap_or("?")),
        hp: card["hp"].as_u64().ok_or(Refusal::UnknownSymbol)? as u32,
        kind,
        weakness: read_modifier(&card["weaknesses"], &["×2", "x2"])?,
        resistance: read_modifier(&card["resistances"], &["-30"])?,
        retreat_cost: card["retreat"].as_u64().unwrap_or(0) as u8,
        attacks,
    }))
}

fn read_attack(attack: &Value) -> Result<Attack, Refusal> {
    if attack["effect"].as_str().is_some_and(|e| !e.is_empty()) {
        return Err(Refusal::AttackHasText);
    }
    let base_damage = match &attack["damage"] {
        Value::Number(damage) => damage.as_u64().ok_or(Refusal::DamageIsNotANumber)? as u32,
        Value::Null => 0,
        _ => return Err(Refusal::DamageIsNotANumber),
    };

    let mut cost = Vec::new();
    for symbol in attack["cost"].as_array().unwrap_or(&Vec::new()) {
        let kind = symbol
            .as_str()
            .and_then(read_type)
            .ok_or(Refusal::UnknownSymbol)?;
        cost.push(kind);
    }

    Ok(Attack {
        name: leak(attack["name"].as_str().unwrap_or("?")),
        cost,
        base_damage,
        inflicts: None,
    })
}

/// A Weakness or a Resistance, but only in the one shape the engine applies.
/// TCGdex writes the multiplier with either a multiplication sign or an x.
fn read_modifier(field: &Value, allowed: &[&str]) -> Result<Option<Type>, Refusal> {
    let Some(entries) = field.as_array() else {
        return Ok(None);
    };
    let Some(first) = entries.first() else {
        return Ok(None);
    };
    let value = first["value"].as_str().unwrap_or("");
    if !allowed.contains(&value) {
        return Err(Refusal::UnknownSymbol);
    }
    first["type"]
        .as_str()
        .and_then(read_type)
        .ok_or(Refusal::UnknownSymbol)
        .map(Some)
}

fn basic_energy_name(kind: Type) -> &'static str {
    match kind {
        Type::Grass => "Grass Energy",
        Type::Fire => "Fire Energy",
        Type::Water => "Water Energy",
        Type::Lightning => "Lightning Energy",
        Type::Psychic => "Psychic Energy",
        Type::Fighting => "Fighting Energy",
        Type::Darkness => "Darkness Energy",
        Type::Metal => "Metal Energy",
        Type::Dragon => "Dragon Energy",
        Type::Colorless => "Colorless Energy",
    }
}

fn read_type(name: &str) -> Option<Type> {
    Some(match name {
        "Grass" => Type::Grass,
        "Fire" => Type::Fire,
        "Water" => Type::Water,
        "Lightning" => Type::Lightning,
        "Psychic" => Type::Psychic,
        "Fighting" => Type::Fighting,
        "Darkness" => Type::Darkness,
        "Metal" => Type::Metal,
        "Dragon" => Type::Dragon,
        "Colorless" => Type::Colorless,
        _ => return None,
    })
}

/// A card name outlives the game that reads it. The literals in
/// [`crate::cards`] are `&'static str`, and an imported name joins them by
/// being leaked once, at load, and never freed.
fn leak(name: &str) -> &'static str {
    Box::leak(name.to_string().into_boxed_str())
}
