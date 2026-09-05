//! What a printed card says.

use crate::ids::CardDefId;

/// The nine types plus Colorless. Weakness and Resistance read this; an
/// attack cost in Milestone 1 does not (see [`Attack::cost`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Grass,
    Fire,
    Water,
    Lightning,
    Psychic,
    Fighting,
    Darkness,
    Metal,
    Dragon,
    Colorless,
}

/// An attack as printed.
#[derive(Debug, Clone)]
pub struct Attack {
    pub name: &'static str,
    /// What the attack costs, one entry per Energy. A `Colorless` entry takes
    /// any Energy; every other entry takes its own type.
    pub cost: Vec<Type>,
    /// Damage before Weakness, Resistance, and any other effect.
    pub base_damage: u32,
}

/// A Pokémon as printed. Milestone 1 carries only Basics, so there is no
/// evolution field yet.
#[derive(Debug, Clone)]
pub struct Pokemon {
    pub name: &'static str,
    pub hp: u32,
    pub kind: Type,
    pub weakness: Option<Type>,
    pub resistance: Option<Type>,
    pub retreat_cost: u8,
    pub attacks: Vec<Attack>,
}

/// A basic Energy card as printed.
#[derive(Debug, Clone)]
pub struct Energy {
    pub name: &'static str,
    pub kind: Type,
}

#[derive(Debug, Clone)]
pub enum CardDef {
    Pokemon(Pokemon),
    Energy(Energy),
}

impl CardDef {
    pub fn name(&self) -> &'static str {
        match self {
            CardDef::Pokemon(p) => p.name,
            CardDef::Energy(e) => e.name,
        }
    }

    pub fn as_pokemon(&self) -> Option<&Pokemon> {
        match self {
            CardDef::Pokemon(p) => Some(p),
            CardDef::Energy(_) => None,
        }
    }

    pub fn is_basic_pokemon(&self) -> bool {
        matches!(self, CardDef::Pokemon(_))
    }

    pub fn is_energy(&self) -> bool {
        matches!(self, CardDef::Energy(_))
    }
}

/// Every printed card the game knows about. Milestone 1 builds this from
/// literals in [`crate::cards`].
#[derive(Debug, Clone, Default)]
pub struct CardDb {
    defs: Vec<CardDef>,
}

impl CardDb {
    pub fn new() -> CardDb {
        CardDb { defs: Vec::new() }
    }

    pub fn add(&mut self, def: CardDef) -> CardDefId {
        let id = CardDefId(self.defs.len() as u32);
        self.defs.push(def);
        id
    }

    pub fn get(&self, id: CardDefId) -> &CardDef {
        &self.defs[id.index()]
    }
}
