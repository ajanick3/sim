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

/// A zone a Trainer effect moves cards between. Only the zones a committed
/// card actually reads or writes; a Pokémon's attached cards are a separate
/// mechanism; the Bench and Active hold Pokémon in play, not loose cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    Hand,
    Discard,
    /// The draw pile. Moving a card into it shuffles when the choice ends.
    Library,
}

/// What a card must be for a Trainer effect to offer it. A value, per
/// [ADR 0009](../docs/adr/0009-an-effect-is-a-value-the-engine-executes.md) —
/// never a closure the engine could not inspect or compare.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardFilter {
    AnyPokemon,
    /// A Pokémon worth exactly 1 Prize. Every Pokémon worth more carries a
    /// Rule Box (ADR 0010), which is the fact a printed card actually reads.
    PokemonWithoutRuleBox,
    /// A Pokémon, or a basic Energy card. No special Energy is ever admitted
    /// (`Refusal::IsASpecialEnergy`), so an admitted Energy is always basic.
    PokemonOrBasicEnergy,
}

/// What happens once a `Deciding` phase ends, beyond the cards it moved. A
/// value, so a card that needs one names it rather than the engine guessing
/// from which zones were involved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Then {
    /// Draw this many cards for each one the phase moved.
    DrawPerCardMoved(u32),
}

/// A Trainer's effect: a value the engine executes, never text read at run
/// time (ADR 0009). `Phase::Deciding` and the generalized `Phase::Promoting`
/// cover most of these (ADR 0012); the rest resolve with no phase at all,
/// the moment the card is played.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrainerEffect {
    /// Move up to `limit` cards matching `filter` from one zone to another,
    /// the player's choice each time.
    Decide {
        from: Zone,
        to: Zone,
        filter: CardFilter,
        limit: u32,
        then: Option<Then>,
    },
    /// Switch the opponent's Active with one of their Benched Pokémon,
    /// chosen by the player who played the card.
    SwitchOpponentActive,
    /// Shuffle the player's hand into their Library, then draw. A second
    /// count applies when they hold exactly 6 Prizes.
    ShuffleHandThenDraw { normal: u32, at_six_prizes: u32 },
    /// Both players shuffle their hand into their Library, then draw.
    BothShuffleHandThenDraw { count: u32 },
    /// Flip a coin; on heads, discard one Energy attached to a Pokémon the
    /// opponent controls, the player's choice of which.
    CoinFlipDiscardOpponentEnergy,
}

/// A Trainer as printed.
#[derive(Debug, Clone)]
pub struct Trainer {
    pub print_id: &'static str,
    pub name: &'static str,
    pub kind: TrainerKind,
    pub effect: TrainerEffect,
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

/// A Pokémon as printed.
#[derive(Debug, Clone)]
pub struct Pokemon {
    /// The id of the printed card this definition came from, such as
    /// `me01-055`. A literal in [`crate::cards`] carries a synthetic one.
    ///
    /// 412 Pokémon names are printed with differing behaviour across
    /// Standard — more than one card can share a name — so the name alone
    /// cannot say which printed card a definition is. The id can.
    pub print_id: &'static str,
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
    /// See [`Pokemon::print_id`].
    pub print_id: &'static str,
    pub name: &'static str,
    pub kind: Type,
}

#[derive(Debug, Clone)]
pub enum CardDef {
    Pokemon(Pokemon),
    Energy(Energy),
    Trainer(Trainer),
}

impl CardDef {
    pub fn name(&self) -> &'static str {
        match self {
            CardDef::Pokemon(p) => p.name,
            CardDef::Energy(e) => e.name,
            CardDef::Trainer(t) => t.name,
        }
    }

    /// The id of the printed card this definition came from. See
    /// [`Pokemon::print_id`].
    pub fn print_id(&self) -> &'static str {
        match self {
            CardDef::Pokemon(p) => p.print_id,
            CardDef::Energy(e) => e.print_id,
            CardDef::Trainer(t) => t.print_id,
        }
    }

    pub fn as_pokemon(&self) -> Option<&Pokemon> {
        match self {
            CardDef::Pokemon(p) => Some(p),
            CardDef::Energy(_) | CardDef::Trainer(_) => None,
        }
    }

    pub fn as_trainer(&self) -> Option<&Trainer> {
        match self {
            CardDef::Trainer(t) => Some(t),
            CardDef::Pokemon(_) | CardDef::Energy(_) => None,
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
