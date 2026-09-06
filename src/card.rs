//! What a printed card says.

use crate::ids::CardDefId;

/// The nine types plus Colorless. Weakness, Resistance, and an attack cost
/// all read this.
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

/// What kind of Trainer a card is. Each has its own limit in rule 13: a
/// Supporter and a Stadium once a turn, an Item and a Tool without limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrainerKind {
    Supporter,
    Item,
    Tool,
    Stadium,
}

/// A Special Condition. Only the Active can carry one (rule 49).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    Asleep,
    Paralyzed,
    Confused,
    Burned,
    Poisoned,
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
    /// The Special Condition the attack puts on the defending Pokémon.
    pub inflicts: Option<Condition>,
}

/// A Pokémon as printed. The card set carries only Basics, so there is no
/// evolution field yet.
#[derive(Debug, Clone)]
pub struct Pokemon {
    pub name: &'static str,
    pub hp: u32,
    pub kind: Type,
    pub weakness: Option<Type>,
    pub resistance: Option<Type>,
    pub retreat_cost: u8,
    /// What a knockout of this Pokémon is worth, before any card adjusts it.
    /// One for an ordinary Pokémon, two for a Pokémon ex, three for a Mega
    /// Evolution ex.
    pub prizes: u32,
    /// The name of the Pokémon this one evolves from. `None` on a Basic.
    pub evolve_from: Option<&'static str>,
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

    /// A Basic — one that a player places directly, rather than evolving
    /// onto an existing Pokémon (rule 9). Before evolution existed, every
    /// admitted Pokémon was a Basic, and this checked only the enum variant;
    /// that stopped being enough once a Stage 1 or 2 could be admitted too.
    pub fn is_basic_pokemon(&self) -> bool {
        matches!(self, CardDef::Pokemon(p) if p.evolve_from.is_none())
    }

    pub fn is_energy(&self) -> bool {
        matches!(self, CardDef::Energy(_))
    }
}

/// Every printed card the game knows about, built from the literals in
/// [`crate::cards`].
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
