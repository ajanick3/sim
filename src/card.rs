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

/// Where a Trainer effect sends a card it moved.
///
/// Every destination but one is a `Zone`, which holds loose cards. The Bench
/// is not: it holds Pokémon in play, so a card sent there stops being a card
/// and becomes a `PokemonInPlay`. `Buddy-Buddy Poffin` is why this is a value
/// rather than a `Zone` field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Destination {
    Zone(Zone),
    /// Onto the Bench of the player who is choosing, as a Pokémon in play.
    /// A full Bench takes no more, and the choice ends there.
    Bench,
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
    /// A Pokémon that evolves from something: a Stage 1 or a Stage 2.
    /// `Hilda` prints it as "an Evolution Pokémon".
    EvolutionPokemon,
    /// A Pokémon printed at exactly this stage. `Dawn` names all three.
    PokemonOfStage(Stage),
    /// A basic Energy card. Every admitted Energy is basic, since a special
    /// Energy carries rules text and is refused.
    BasicEnergy,
    /// A Pokémon ex, which is a Pokémon worth more than 1 Prize. The prize
    /// value is read from the name (ADR 0010), so in this pool a card worth
    /// more than 1 is exactly a card printed `ex`.
    PokemonEx,
    /// A Basic Pokémon whose printed HP is at most this much. Both halves are
    /// printed on the card that reads them: `Buddy-Buddy Poffin` wants a
    /// Basic with 70 HP or less.
    BasicPokemonWithHpAtMost(u32),
}

/// What happens once a `Deciding` phase ends, beyond the cards it moved. A
/// value, so a card that needs one names it rather than the engine guessing
/// from which zones were involved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Then {
    /// Draw this many cards for each one the phase moved.
    DrawPerCardMoved(u32),
}

/// One step of a search: what to look for, where it goes, and how many. A
/// card that searches for one thing carries one; `Hilda` carries two and
/// `Dawn` three, one of each kind, in the order they are printed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slot {
    pub filter: CardFilter,
    pub to: Destination,
    /// At most this many cards. A slot the player declines takes none.
    pub limit: u32,
}

/// A Trainer's effect: a value the engine executes, never text read at run
/// time (ADR 0009). `Phase::Deciding` and the generalized `Phase::Promoting`
/// cover most of these (ADR 0012); the rest resolve with no phase at all,
/// the moment the card is played.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrainerEffect {
    /// Take cards out of one zone, the player's choice each time, one slot
    /// after another. Most cards carry a single slot and repeat it up to its
    /// limit; a card that asks for one of each kind carries a slot for each.
    Decide {
        from: Zone,
        slots: Vec<Slot>,
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
    /// The opponent shuffles their hand and puts it under their Library, then
    /// draws — but only if they held anything. The cards go to the bottom
    /// rather than being shuffled in, so what they gave up is the last thing
    /// they draw again.
    OpponentHandToBottomThenDraw { count: u32 },
    /// Move one Energy attached to a Pokémon the player controls onto
    /// another Pokémon they control. Both ends are attachments rather than
    /// zones, so this is neither a move between zones nor an attachment from
    /// one.
    MoveAttachedEnergy,
    /// Flip a coin; on heads, discard one Energy attached to a Pokémon the
    /// opponent controls, the player's choice of which.
    CoinFlipDiscardOpponentEnergy,
    /// Nothing beyond where the card itself goes. A Stadium that stays in
    /// play and does no more is this. `known_trainer_effect` never produces
    /// it: a real Stadium carries a continuous rule the engine cannot run,
    /// and ADR 0008 refuses a card it cannot run in full.
    Nothing,
}

/// What a card demands before it may be played at all.
///
/// A requirement is not an effect. An effect runs once the card is played; a
/// requirement decides whether it may be played, which is `legal_actions`.
/// One of the two also costs the player something, and a cost is paid before
/// the effect runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Requirement {
    /// Discard this many cards from hand, other than this one, to play it.
    /// `Ultra Ball` prints the only one in the pool.
    DiscardOtherCardsFromHand(u32),
    /// The opponent holds at most this many Prize cards. Read from the
    /// board, and costs nothing.
    OpponentPrizesAtMost(usize),
}

/// A Trainer as printed.
#[derive(Debug, Clone)]
pub struct Trainer {
    pub print_id: &'static str,
    pub name: &'static str,
    pub kind: TrainerKind,
    /// What the card demands before it may be played. Most print none.
    pub requirement: Option<Requirement>,
    pub effect: TrainerEffect,
}

impl Trainer {
    /// The slots this card searches with, or none when its effect is not a
    /// search. A caller walking a search reads them here rather than
    /// matching the effect again.
    pub fn slots(&self) -> &[Slot] {
        match &self.effect {
            TrainerEffect::Decide { slots, .. } => slots,
            _ => &[],
        }
    }
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

/// How far along its evolution line a Pokémon card is printed. The artifact
/// carries this as its own field, so the engine reads it rather than
/// deducing it: `evolve_from` says *what* a card evolves from, and only a
/// walk of the whole line would say how deep that is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Basic,
    Stage1,
    Stage2,
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
    pub stage: Stage,
    /// The name of the Pokémon this one evolves from. `None` on a Basic, and
    /// always present on an Evolution — an evolution card that names nothing
    /// is refused at import, since nothing could ever evolve into it.
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
        matches!(self, CardDef::Pokemon(p) if p.stage == Stage::Basic)
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
