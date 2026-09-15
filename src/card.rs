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
    Deck,
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
    /// Attached straight to a Pokémon the chooser controls, filtered by
    /// `TargetFilter`, picked at the same time as the card. `Crispin` is why
    /// this exists at all: the second Energy it finds goes onto a Pokémon
    /// rather than into a zone. `TargetFilter::AnyInPlay` is what it needed.
    Attach(TargetFilter),
    /// On top of the Deck, in the order the cards were taken — not
    /// shuffled in. `Ciphermaniac's Codebreaking` shuffles what is left of
    /// the deck *before* placing these, so the two land known, on top of an
    /// otherwise-scrambled deck.
    TopOfDeckInOrder,
}

/// What Pokémon `Destination::Attach` may target, beyond "the chooser
/// controls it." A value, the same discipline every filter in this module
/// holds to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetFilter {
    /// Any Pokémon the chooser controls, in play. What `Crispin` needed.
    AnyInPlay,
    /// A Benched Pokémon whose printed name starts with this prefix.
    /// `N's PP Up` targets only a Benched Pokémon named "N's ...".
    BenchedNameStartsWith(&'static str),
    /// A Benched Pokémon printed as this type. `Wondrous Patch` targets
    /// only a Benched Psychic Pokémon.
    BenchedOfType(Type),
    /// A Pokémon in play, Active or Benched, printed as this type.
    /// `Philippe` attaches to a Metal Pokémon either way.
    OfType(Type),
    /// A Pokémon in play, Active or Benched, printed at this stage.
    /// `Rosa's Encouragement` targets a Stage 2 either way — unlike
    /// `BenchedNameStartsWith` and `BenchedOfType`, nothing here confines
    /// it to the Bench.
    OfStage(Stage),
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
    /// Any Energy card, basic or special. `Colress's Tenacity` prints it
    /// as "an Energy card" — the special-Energy prints this engine does
    /// admit (`Enriching Energy` among them) count too, unlike
    /// `BasicEnergy`.
    AnyEnergy,
    /// A Pokémon ex, which is a Pokémon worth more than 1 Prize —
    /// `prizes > 1`, computed from `Marker::Ex`/`Marker::Mega` (ADR
    /// 0010).
    PokemonEx,
    /// A Basic Pokémon whose printed HP is at most this much. Both halves are
    /// printed on the card that reads them: `Buddy-Buddy Poffin` wants a
    /// Basic with 70 HP or less.
    BasicPokemonWithHpAtMost(u32),
    /// Any Trainer card, of any kind. `Team Rocket's Petrel` searches for
    /// one without naming a kind at all.
    AnyTrainer,
    /// A basic Energy card of this type. `BasicEnergy` matches every one;
    /// `Wondrous Patch` wants only a Psychic one.
    BasicEnergyOfType(Type),
    /// A Trainer of exactly this kind. `Pokégear 3.0` wants a Supporter,
    /// where `AnyTrainer` matches every kind at once.
    TrainerOfKind(TrainerKind),
    /// A Pokémon of this type, or a basic Energy of the same type.
    /// `Bug Catching Set` wants "a Grass Pokémon or a Basic Grass Energy
    /// card" — one filter admitting two kinds of card, the same shape
    /// `PokemonOrBasicEnergy` already is, narrowed to one type.
    PokemonOfTypeOrBasicEnergyOfType(Type),
    /// A Pokémon without a Rule Box, or a basic Energy of any type. The
    /// same shape `PokemonOrBasicEnergy` already is, narrowed the other
    /// way `PokemonOfTypeOrBasicEnergyOfType` is — by a Rule Box instead
    /// of by a type. `Lana's Aid` is the first card to exclude one.
    PokemonWithoutRuleBoxOrBasicEnergy,
    /// Any card at all — a Pokémon, an Energy, or a Trainer alike.
    /// `Ciphermaniac's Codebreaking` names no kind: "search your deck for
    /// 2 cards" is the whole of its filter.
    AnyCard,
    /// A Supporter whose printed name holds this substring. `Team Rocket's
    /// Transceiver` wants "a Supporter card that has \"Team Rocket\" in its
    /// name" — a family of cards named by a shared word, not by a kind.
    SupporterNameContains(&'static str),
    /// A Pokémon whose printed name matches exactly. `Duskull`'s
    /// `Come and Get You` searches its own discard pile for copies of
    /// itself, unlike `SupporterNameContains`'s substring match.
    PokemonNamed(&'static str),
    /// A Pokémon that evolves from something, printed at this type.
    /// `Genesect ex`'s `Protect Charge` wants an Evolution Metal
    /// Pokémon — the same shape `EvolutionPokemon` already is,
    /// narrowed by type the way `PokemonOfTypeOrBasicEnergyOfType`
    /// narrows `PokemonOrBasicEnergy`.
    EvolutionPokemonOfType(Type),
    /// A Pokémon of this type printed at this HP or less, any stage —
    /// the same shape `BasicPokemonWithHpAtMost` already is, without
    /// the Basic restriction, narrowed by type instead.
    /// `Fan Rotom`'s `Fan Call`.
    PokemonOfTypeWithHpAtMost(Type, u32),
    /// A Basic Pokémon of this type. `Telepathic Psychic Energy`.
    BasicPokemonOfType(Type),
    /// A Basic Pokémon of this type, or a Basic Energy of it — the same
    /// pairing `PokemonOfTypeOrBasicEnergyOfType` reads, narrowed to
    /// Basic on the Pokémon side too. `Fighting Gong`.
    BasicPokemonOfTypeOrBasicEnergyOfType(Type),
    /// A Pokémon of this type, any stage. `Canari`.
    PokemonOfType(Type),
    /// A Basic Pokémon whose printed name contains this word.
    /// `Team Rocket's Proton`, `Hop's Bag`.
    BasicPokemonNameContains(&'static str),
    /// A Pokémon carrying `Marker::Tera`. `Tera Orb`.
    TeraPokemon,
    /// A Pokémon carrying `Marker::Mega` — every Mega Evolution
    /// Pokémon ex. `Mega Signal`.
    MegaPokemon,
    /// A Pokémon Tool card whose printed name contains this word.
    /// `TM Machine`.
    ToolNameContains(&'static str),
    /// A Pokémon of this type, or a Stadium card — the same "one
    /// filter, two kinds of card" shape `PokemonOrBasicEnergy` and
    /// `PokemonOfTypeOrBasicEnergyOfType` already are, but the second
    /// kind is a Trainer kind rather than Energy. `Celebi`'s
    /// `Traverse Time`.
    PokemonOfTypeOrStadium(Type),
}

/// What happens once a `Deciding` phase ends, beyond the cards it moved. A
/// value, so a card that needs one names it rather than the engine guessing
/// from which zones were involved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Then {
    /// Draw this many cards for each one the phase moved.
    DrawPerCardMoved(u32),
    /// If the search moved at least one card, the turn ends — not if it
    /// found nothing or the player declined. `Lumiose City`'s own
    /// once-a-turn search, not a played card's effect, but the same
    /// `Decide` machinery either way.
    EndTurnIfMoved,
    /// Whatever the search's own peeked window did not take is
    /// discarded, rather than left in the deck to be shuffled back in —
    /// the one slot's `peek` size names the window. `Explorer's
    /// Guidance`.
    DiscardRestOfPeek,
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
    /// This slot also refuses a card whose Energy type matches the one the
    /// previous slot took. `Crispin` prints "of different types" for its two
    /// Basic Energy, and no static `CardFilter` can read a fact that depends
    /// on what a different slot's choice was.
    pub excludes_type_of_previous: bool,
    /// Read only the first this-many cards of the zone, nearest to being
    /// drawn, rather than the whole of it. `None` is every search built
    /// before `Pokégear 3.0`: the whole zone. `Pokégear 3.0` and
    /// `Bug Catching Set` both peek exactly 7.
    pub peek: Option<u32>,
}

/// What runs once `Action::Promote` completes a live switch, reading what
/// the switch actually did — never a knockout or `Boss's Orders`, which
/// carry none.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromoteFollowUp {
    /// If the Pokémon just displaced to the Bench is worth more than 1
    /// Prize, heal this many points of damage from it. `AZ's Tranquility`.
    HealDisplacedIfEx(u32),
    /// Draw until the player holds this many cards in hand. `Surfer`.
    DrawUpTo(u32),
    /// The player who chose this switch also switches their own Active
    /// with one of their own Benched Pokémon — opening a second
    /// `Phase::Promoting`, on their own side. Silently skipped if their
    /// Bench is empty; the outer switch already happened either way.
    /// `Prime Catcher`.
    AlsoSwitchOwnActive,
}

/// What runs once a `DiscardingFromHand` ends. `Hand Trimmer`'s only use
/// so far: the opponent discards down to a size, then the player does
/// the same to their own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscardFollowUp {
    /// The acting player discards from their own hand down to this size.
    AlsoDiscardOwnHandDownTo(u32),
}

/// What a this-turn damage bonus restricts itself to — always the
/// opponent's Active, since nothing in the pool bonuses an attack against
/// a Benched Pokémon. The first two variants read the defender; the third
/// reads the attacker instead, the opposite half of the same attack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnBonusTarget {
    /// A Pokémon ex. `Black Belt's Training`.
    OpponentActiveEx,
    /// A Pokémon without a Rule Box. `Gladion's Final Battle` — the
    /// opposite restriction `OpponentActiveEx` reads.
    OpponentActiveWithoutRuleBox,
    /// Restricted by the attacking Pokémon's own Type, not by anything
    /// about the defender. `Premium Power Pro`.
    AttackerIsType(Type),
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
    /// Switch the player's own Active with one of their own Benched
    /// Pokémon, by their own choice — not after a knockout, and not the
    /// opponent's. `Switch` is the plain form of this.
    SwitchOwnActive,
    /// The same switch, with a follow-up that reads what the switch
    /// actually did once `Action::Promote` completes it. `AZ's
    /// Tranquility` and `Surfer` are why this exists.
    SwitchOwnActiveWithFollowUp(PromoteFollowUp),
    /// Switch in one of the opponent's Benched Pokémon, chosen by the
    /// player; if that switch happens, the player also switches their own
    /// Active with one of their own Benched Pokémon. `Prime Catcher` is a
    /// `SwitchOpponentActive` with `PromoteFollowUp::AlsoSwitchOwnActive`
    /// chained onto it, not a new shape of switch.
    SwitchOpponentActiveThenOwn,
    /// Switch the player's own Active with a Benched Pokémon carrying
    /// this name prefix, then switch in one of the opponent's Benched
    /// Pokémon too. `Team Rocket's Giovanni` — the reverse order
    /// `SwitchOpponentActiveThenOwn` already reads, and restricted on
    /// its own side where that switch is not.
    SwitchOwnNamePrefixThenOpponent(&'static str),
    /// Switch in a Basic from the opponent's Bench, chosen by the
    /// player, then Confuse it. `Lisia's Appeal`.
    SwitchOpponentActiveBasicThenConfuse,
    /// Choose one of the player's own Pokémon in play, and protect it
    /// from an ex the opponent attacks with next turn: no damage, no
    /// effect. `Acerola's Mischief`.
    ProtectChosenFromExNextTurn,
    /// Choose one of the player's own Pokémon in play, Active or
    /// Benched, and return it with everything attached to hand.
    /// `Scoop Up Cyclone`.
    ReturnChosenToHand,
    /// Devolve one of the player's own evolved Pokémon, any number of
    /// evolution cards, into their hand; the Pokémon cannot evolve again
    /// this turn. `Strange Timepiece`.
    DevolveChosen,
    /// Swap a Basic Pokémon in play for one in the discard, keeping the
    /// same damage, attachments, Special Conditions, and time in play.
    /// `Transformation Tome`.
    SwapBasicWithDiscard,
    /// A static effect, read wherever Retreat Cost is read rather than
    /// dispatched at play time — a Tool attaches and then does nothing
    /// until read. `Air Balloon` is the first static effect built.
    ReducesRetreatCost(u32),
    /// A static effect, read wherever HP is read. `Hero's Cape`.
    IncreasesHp(u32),
    /// A static effect, read in `damage_dealt`: this many more damage
    /// against the opponent's Active, but only while the Pokémon this is
    /// attached to carries no Rule Box and that Active is worth more than
    /// 1 Prize. `Brave Bangle`.
    BonusDamageWithoutRuleBoxVsEx(u32),
    /// A static effect, read in `damage_dealt`: this many more damage
    /// against the opponent's Active, but only while the Pokémon this is
    /// attached to is Poisoned. `Binding Mochi`.
    BonusDamageIfPoisonedVsActive(u32),
    /// A static effect, read in `knock_out_the_dead`: if the Lillie's
    /// Pokémon this is attached to is Knocked Out by an attack — not a
    /// checkup — the opponent takes 1 fewer Prize. `Lillie's Pearl`.
    FewerPrizeIfLilliesKnockedOutByAttack,
    /// A triggered effect, read in `attack`: when the Pokémon this is
    /// attached to is damaged by an opponent's attack — even if Knocked
    /// Out by it — this many damage counters go on the attacker.
    /// `Punk Helmet`.
    DamagesAttackerWhenDefenderIsHit(u32),
    /// The same trigger as `DamagesAttackerWhenDefenderIsHit`: the
    /// Pokémon's owner draws this many cards. `Lucky Helmet`.
    DrawsWhenDefenderIsHit(u32),
    /// The same trigger, but with a choice: move one Energy off the
    /// attacker onto one of the attacker's own Benched Pokémon, the
    /// defender's controller's choice. Opens `Phase::MovingEnergyForHandheldFan`
    /// rather than resolving outright. `Handheld Fan`.
    MovesEnergyFromAttackerToTheirBench,
    /// A triggered effect, read at the end of the turn: the player may
    /// attach a Basic Energy from their discard to the Pokémon this is
    /// attached to, if it is still the Active. `Powerglass`.
    MayAttachBasicEnergyFromDiscardAtTurnEnd,
    /// A Stadium's static effect: every Pokémon in play at this printed
    /// Stage loses this much HP, both sides alike. `Gravity Mountain`.
    ReducesHpForStage(Stage, u32),
    /// A Stadium's static effect: every Pokémon in play whose name starts
    /// with this has no Retreat Cost, both sides alike. `N's Castle`.
    RemovesRetreatCostForNamePrefix(&'static str),
    /// A Stadium's own once-a-turn action, offered directly in the acting
    /// player's Main phase rather than dispatched at play time: put a
    /// card from hand on top of the Deck. `Academy at Night`.
    MayPutHandCardOnTopOfDeck,
    /// A Stadium's own once-a-turn action, gated on a fact from earlier
    /// this same turn rather than always offered: may draw 2, but only
    /// for a player who played a Supporter named "Team Rocket" something
    /// from hand this turn. `Team Rocket's Factory`.
    MayDrawTwoIfPlayedTeamRocketSupporter,
    /// A Stadium's own once-a-turn action: search for a Basic Pokémon
    /// onto the Bench; if one is taken, the turn ends. No-op at play
    /// time, the same as every other Stadium static — the search itself
    /// runs only from `Action::UseLumioseCity`, through the same
    /// `enter_slot` a played card's own `Decide` uses.
    /// `Lumiose City`.
    MaySearchBasicToBenchThenMaybeEndTurn,
    /// A Stadium's static effect: every attached Tool, both sides alike,
    /// has no effect. Read wherever a Tool's own effect would be —
    /// `effective_hp`, `effective_retreat_cost`, `damage_dealt`'s and
    /// `attack`'s own Tool loops, `powerglass_owner` — rather than
    /// removing the Tool or its attachment. `Jamming Tower`.
    ToolsHaveNoEffect,
    /// A Stadium's static effect: a Basic Pokémon of any other type,
    /// benched this turn, takes this much damage — read wherever a
    /// Pokémon newly arrives on a Bench (`PlayBasic`, a search's own
    /// `Destination::Bench`), both sides alike. `Risky Ruins`.
    DamagesNonDarknessBasicBenched(u32),
    /// A Stadium's static effect: a Grass Pokémon evolving into a Grass
    /// Pokémon may do so the same turn it was played — bypassing rules
    /// 18-20's "in play since the start of the turn" — except the first
    /// turn of the game, which nothing built ever offers an evolve on
    /// anyway. Read at both evolution eligibility sites (`Evolve`,
    /// `EvolveSkippingOneStage`) alongside the check it relaxes.
    /// `Forest of Vitality`.
    GrassCanEvolveTheTurnItIsPlayed,
    /// A Stadium's static effect: a Pokémon carrying any Energy, either
    /// side, recovers from every Special Condition and cannot be given a
    /// new one. Read at `inflict` (refuses to add one) and swept
    /// immediately at the two moments a Pokémon can newly qualify:
    /// playing this Stadium, and attaching Energy while it is already in
    /// play. `Festival Grounds`.
    EnergizedPokemonImmuneToSpecialConditions,
    /// Draw this many cards. No cost, no target. `Cheren`,
    /// `Friends in Paldea`, `Urbain`.
    Draw(u32),
    /// Draw until the player holds this many cards, stopping early if the
    /// deck runs out. `Iris's Fighting Spirit`.
    DrawUpToHandSize(u32),
    /// Draw up to `base` cards in hand, or up to `bonus` instead if every
    /// one of the player's own Pokémon in play carries this name prefix.
    /// `Team Rocket's Ariana`.
    DrawUpToHandSizeOrMoreIfAllOwnNamePrefix {
        base: u32,
        bonus: u32,
        prefix: &'static str,
    },
    /// Flip a coin, then draw `heads` on heads or `tails` on tails.
    /// `Picnicker`.
    CoinFlipDraw { heads: u32, tails: u32 },
    /// Flip a coin; on heads, resolve the wrapped effect. On tails,
    /// nothing. `Poké Ball`.
    CoinFlipThen(Box<TrainerEffect>),
    /// A Tool: while its carrier is the Active, the Retreat Cost of both
    /// Active Pokémon is `amount` higher. `Gravity Gemstone`.
    RaisesBothActiveRetreatWhileCarrierActive(u32),
    /// Draw one card for each Mega Evolution Pokémon ex the opponent has
    /// in play. `Jett`.
    DrawPerOpponentMegaEx,
    /// Draw one card for each of the player's own Pokémon in play that
    /// carries this marker. `Awakening Drum` reads `Ancient`.
    DrawPerOwnPokemonWithMarker(Marker),
    /// Attach a Basic Energy from the player's discard to each of their
    /// own Future Pokémon that has none of the deck's Basic Energy on it
    /// yet; the engine takes the first that fits. `Reboot Pod`.
    AttachBasicEnergyFromDiscardToEachFuture,
    /// A Stadium: at Pokémon Checkup, a Poisoned non-Darkness Pokémon
    /// either side takes this much more. `Perilous Jungle`.
    StadiumExtraPoisonDamage(u32),
    /// Grant a side-wide effect for the opponent's next turn — see
    /// `SideShield` and ADR 0098. `Jasmine's Gaze`, `Iron Defender`,
    /// `Roxie's Performance`.
    GrantSideShieldNextTurn(SideShield),
    /// Draw `base`, then `bonus` more when the opponent has at most this
    /// many Prizes remaining. `Emcee's Hype`.
    DrawThenBonusIfOpponentPrizesAtMost { base: u32, bonus: u32, at_most: usize },
    /// Draw `base`, then `bonus` more when the player's hand then holds at
    /// least this many cards. `Billy & O'Nare`.
    DrawThenBonusIfHandAtLeast { base: u32, bonus: u32, at_least: usize },
    /// The opponent reveals their hand; draw one card for each Pokémon in
    /// it. `Emma`.
    DrawPerPokemonInOpponentHand,
    /// Discard the player's whole hand, then draw this many. Unlike
    /// `ShuffleHandThenDraw`, what is given up does not go back in the
    /// deck. `Carmine`.
    DiscardHandThenDraw(u32),
    /// Discard this many cards from the top of the player's own deck.
    /// `Hole-Digging Shovel`.
    DiscardTopOfDeck(u32),
    /// Inflict up to two Special Conditions on the opponent's Active.
    /// `Dangerous Laser` reads Burned and Confused.
    InflictOnOpponentActive(Condition, Option<Condition>),
    /// A Tool: the Pokémon it is attached to takes `amount` less damage
    /// from attacks by an opponent's Pokémon of `kind`, after Weakness
    /// and Resistance. The "-Berry" Tools (`Babiri`, `Colbur`, `Occa`,
    /// `Passho`, `Payapa`, `Haban`).
    ReducesDamageFromType { kind: Type, amount: u32 },
    /// A Tool on the attacker: its attacks do `amount` more damage to an
    /// opponent's Active that is a Pokémon ex, before Weakness and
    /// Resistance. `Maximum Belt`.
    BonusDamageVsActiveEx(u32),
    /// A Tool on the defender: it takes `amount` less damage from attacks
    /// by an opponent's Pokémon that has an Ability, after Weakness and
    /// Resistance. `Sacred Charm`.
    ReducesDamageFromAbilityHolders(u32),
    /// A Stadium: every Basic Pokémon in play, both sides, gets this much
    /// more HP. `Lively Stadium`.
    StadiumBoostsBasicHp(u32),
    /// A Tool: the carrier gets this much more HP, but only when its
    /// printed name contains the word. `Cynthia's Power Weight`.
    IncreasesHpForNamePrefix { word: &'static str, amount: u32 },
    /// A Stadium: a Pokémon whose name contains the word, either side,
    /// takes `amount` less damage from the opponent's attacks, after
    /// Weakness and Resistance. `Granite Cave`.
    StadiumReducesDamageForNamePrefix { word: &'static str, amount: u32 },
    /// A Stadium: a Pokémon of `kind`, either side, takes `amount` less
    /// damage from the opponent's attacks, after Weakness and Resistance.
    /// `Full Metal Lab`.
    StadiumReducesDamageToType { kind: Type, amount: u32 },
    /// Confuse both Active Pokémon that are not of this type. `Dark Bell`
    /// spares a Darkness Active on either side.
    ConfuseBothActivesExceptType(Type),
    /// The opponent switches their own Active to the Bench, choosing the
    /// replacement — a gust that hands the choice to them, unlike
    /// `SwitchOpponentActive` where the player chooses. `Repel`.
    SwitchOutOpponentActive,
    /// Shuffle the player's hand into their Deck, flip a coin, then
    /// draw `heads` or `tails`. `Drasna`.
    ShuffleHandThenCoinFlipDraw { heads: u32, tails: u32 },
    /// Both players shuffle their hand into their Deck, flip one coin,
    /// then draw — the player and the opponent each drawing the amount
    /// their side's `*_heads` / `*_tails` names for that flip. `Harlequin`.
    BothShuffleHandThenCoinFlipDraw {
        you_heads: u32,
        opponent_heads: u32,
        you_tails: u32,
        opponent_tails: u32,
    },
    /// Shuffle the player's hand into their Deck, then draw. A second
    /// count applies when they hold exactly 6 Prizes.
    ShuffleHandThenDraw { normal: u32, at_six_prizes: u32 },
    /// Both players shuffle their hand into their Deck, then draw — the
    /// player who played the card drawing `you`, the opponent drawing
    /// `opponent`. `Judge` prints the same count for both; `Unfair Stamp`
    /// is why the two are named separately rather than shared.
    BothShuffleHandThenDraw { you: u32, opponent: u32 },
    /// The opponent shuffles their hand and puts it under their Deck, then
    /// draws — but only if they held anything. The cards go to the bottom
    /// rather than being shuffled in, so what they gave up is the last thing
    /// they draw again.
    OpponentHandToBottomThenDraw { count: u32 },
    /// Move one Energy attached to a Pokémon the player controls onto
    /// another Pokémon they control. Both ends are attachments rather than
    /// zones, so this is neither a move between zones nor an attachment from
    /// one.
    MoveAttachedEnergy,
    /// Move up to `limit` Energy from the player's Benched Pokémon onto
    /// their Active, one at a time, never the other way — `N's Plan`'s
    /// direction, where `MoveAttachedEnergy` moves exactly one, between
    /// any two Pokémon the player controls.
    MoveEnergyFromBenchToActive { limit: u32 },
    /// Evolve a Basic Pokémon in play straight into a Stage 2 from hand,
    /// skipping the Stage 1 between them. `Rare Candy` is the only card that
    /// needs this: rule 19's ordinary evolution matches a card's
    /// `evolve_from` to the Pokémon it is played on, one stage at a time,
    /// and this reads `evolves_from_basic` instead.
    EvolveSkippingOneStage,
    /// Search the deck for a no-Ability card that evolves one of the
    /// player's own Pokémon, and evolve it, even one put into play this
    /// turn. `Salvatore`.
    SearchDeckToEvolveNoAbility,
    /// Flip a coin; on heads, discard one Energy attached to a Pokémon the
    /// opponent controls, the player's choice of which.
    CoinFlipDiscardOpponentEnergy,
    /// Discard one Energy attached to a Pokémon the opponent controls, the
    /// player's choice of which — the same phase
    /// `CoinFlipDiscardOpponentEnergy` opens on heads, entered outright,
    /// with no flip in front of it.
    DiscardOpponentEnergy,
    /// Nothing beyond where the card itself goes. A Stadium that stays in
    /// play and does no more is this. `known_trainer_effect` never produces
    /// it: a real Stadium carries a continuous rule the engine cannot run,
    /// and ADR 0008 refuses a card it cannot run in full.
    Nothing,
    /// Heal this many points of damage from the player's own Active,
    /// floored at zero. `Jumbo Ice Cream` is the first card that ever
    /// takes damage away rather than adding it or moving it between
    /// Pokémon.
    HealActive(u32),
    /// Heal this many points of damage from a Pokémon the player chooses,
    /// Active or Benched, and clear every Special Condition from it.
    /// `Pokémon Center Lady` is the first card to heal a target rather
    /// than the fixed Active `Jumbo Ice Cream` reads.
    HealChosen(u32),
    /// Heal all damage from one of the player's own Pokémon, restricted to
    /// one with this much remaining HP or less. `Bianca's Devotion`.
    HealFullyIfRemainingHpAtMost(u32),
    /// Heal this many points from every Pokémon the player has in play,
    /// or every one of a type when `of_type` is set. No choice, no
    /// condition clear. `Fennel`, `Clemont's Quick Wit`.
    HealEachYours { amount: u32, of_type: Option<Type> },
    /// Heal this many points from a Pokémon the player chooses — like
    /// `HealChosen` but without the Special Condition clear, optionally
    /// restricted to a type. `Potion`, `Poké Vital A`, `Jacinthe`.
    HealChosenPlain { amount: u32, of_type: Option<Type> },
    /// Heal this many from the player's own Active and clear every
    /// Special Condition on it. `Lumiose Galette` prints "a Special
    /// Condition" singular; the Active only carries one at a time.
    HealActiveAndClearConditions(u32),
    /// This turn, the player's attacks do this much more damage to a
    /// restricted target, before Weakness and Resistance. Cleared the
    /// moment the turn ends — distinct from a Tool or Stadium's static
    /// effect, which lasts as long as the card stays in play regardless of
    /// whose turn it is.
    BonusDamageThisTurn(u32, TurnBonusTarget),
    /// The player picks one of these two, and only the one picked runs.
    /// `Kieran` is the only card whose own text branches this way.
    ChooseOneOf(Box<TrainerEffect>, Box<TrainerEffect>),
    /// Draw one card for each Pokémon on the opponent's Bench.
    /// `Morty's Conviction` is the first draw counted from the board
    /// rather than a fixed number.
    DrawPerOpponentBenched,
    /// The opponent discards, their own choice, until their hand holds
    /// this many cards. `Xerosic's Machinations`.
    OpponentDiscardsDownTo(u32),
    /// Both players discard, each their own choice, down to this many
    /// cards — the opponent first. `Hand Trimmer` is `OpponentDiscardsDownTo`
    /// with a `DiscardFollowUp` chained onto it, not a new shape.
    BothDiscardDownTo(u32),
    /// Look at the bottom `n` cards of the Deck; the player may take a
    /// Pokémon found there, then the rest shuffle back in. `Dusk Ball`
    /// reads the opposite end from every peeked search built so far,
    /// which all read the top — the end `draw` pops from.
    LookAtBottomOfDeck { count: u32 },
    /// The player discards up to `limit` cards matching `filter` from the
    /// opponent's hand, their own choice of which. `Eri`'s filter is an
    /// Item card; nothing before it read a zone the opponent controls.
    DiscardFromOpponentsHand { filter: CardFilter, limit: u32 },
    /// Heal every point of damage from a chosen Mega Evolution ex the
    /// player controls, and if any damage was actually healed, move
    /// every card attached to it into the player's hand. `Wally's
    /// Compassion` is the only card that heals to full rather than a
    /// fixed amount, and the only one whose follow-up reads whether the
    /// heal did anything at all.
    HealMegaExAndTakeEnergyIfHealed,
    /// Choose up to 2 Darkness Pokémon; search for a Basic Darkness
    /// Energy and attach it to each one chosen, one search per target;
    /// then, if the Active got one this way, it is Poisoned. `Janine's
    /// Secret Art` is bespoke: nothing else chooses several targets
    /// before searching once per target, or applies a Special Condition
    /// outside attack resolution.
    JaninesSecretArt,
    /// Choose up to 2 Pokémon Tools attached to any Pokémon in play —
    /// either player's — and discard them, one at a time. Opens
    /// `Phase::DiscardingToolsAnywhere`. `Tool Scrapper`.
    MayDiscardUpToTwoToolsAnywhere,
    /// Grants `GameState::bonus_prize_if_own_tera_attacker_knocks_out`
    /// to the player for the rest of this turn: if the opponent's
    /// Active is Knocked Out by damage from an attack used by this
    /// player's own Tera Pokémon, they take one more Prize. Read in
    /// `knock_out_the_dead`. `Briar`.
    GrantsBonusPrizeIfOwnTeraAttackerKnocksOutThisTurn,
    /// Discard a Special Energy from one of the opponent's Pokémon,
    /// choosing which — opens
    /// `Phase::DiscardingOpponentSpecialEnergy`. No Special Energy
    /// anywhere on the opponent's board opens no phase. `Enhanced
    /// Hammer`.
    DiscardOpponentSpecialEnergy,
    /// A standing effect, not a resolved-once one: while this Stadium
    /// is in play, an attack used by a Pokémon carrying `Marker::Tera`
    /// — either player's — costs one more Colorless Energy. Read
    /// directly by `legal_actions`, the one site that ever checks
    /// whether an attack is paid for; nothing else needs the cost
    /// once an attack is legal to use, so no other site reads this.
    /// `Nighttime Mine`.
    TeraAttacksCostMore,
    /// A standing effect, not a resolved-once one: while this Stadium
    /// is in play, a player with a Pokémon carrying `Marker::Tera` in
    /// play may have up to 8 Pokémon on their own Bench, read
    /// directly by `GameState::bench_limit`. The rest of the card's
    /// own text — discarding down to 5 the moment a player's last
    /// Tera Pokémon leaves play, and discarding both players down to
    /// 5 (this card's own owner first) the moment this card itself
    /// leaves play — is reactive, not read from this variant at all:
    /// `settle` and `stadium_left_play` carry it. `Area Zero
    /// Underdepths`.
    TeraPokemonRaisesBenchLimit,
    /// A standing effect, not a resolved-once one: while this Stadium
    /// is in play, no attack or Ability effect may place a damage
    /// counter onto any Benched Pokémon, either player's. Damage from
    /// an attack landing normally is untouched — only a direct
    /// placement bypassing the ordinary damage order is blocked.
    /// Read directly by `GameState::bench_damage_counters_blocked` at
    /// each site that would place a damage counter onto a
    /// specifically-Benched target. A *move* (Munkidori's
    /// Adrena-Brain) still takes its counters off the source — that
    /// half of the effect isn't a placement — but the counters vanish
    /// rather than landing on a blocked Bench target, so `apply`
    /// reads the block itself rather than `legal_actions` filtering
    /// the target out. `Battle Cage`.
    PreventsDamageCountersOnBench,
    /// A standing effect, not a resolved-once one: while this Stadium
    /// is in play, no `{C}` (Colorless) Pokémon in play, either
    /// player's, has an Ability — a Pokémon of any other type keeps
    /// its own. Read directly by `GameState::abilities_disabled_for`
    /// at every site that would otherwise offer, trigger, or read a
    /// standing Ability effect. `Team Rocket's Watchtower`.
    AbilitiesDisabled,
}

/// A side-wide effect a card grants for the opponent's next turn, held in
/// `GameState::side_shield_next_turn` with the player who granted it. See
/// ADR 0098.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SideShield {
    /// Every one of the granting player's Pokémon takes this much less
    /// damage from the opponent's attacks. `Jasmine's Gaze`.
    DamageReduction(u32),
    /// The granting player's Pokémon of this type take this much less.
    /// `Iron Defender`.
    DamageReductionForType(Type, u32),
    /// The opponent's Poisoned Pokémon cannot retreat.
    /// `Roxie's Performance`.
    OpponentPoisonedCannotRetreat,
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
    /// A Pokémon of the player's own was Knocked Out during the opponent's
    /// last turn. `Unfair Stamp` reads this; nothing else in the pool
    /// needed history rather than the board as it stands.
    KnockedOutDuringOpponentsLastTurn,
    /// The player's own Active carries at least this many Energy, of any
    /// kind. `Jumbo Ice Cream` is the only card that reads a specific
    /// Pokémon's own attachments rather than the hand or the board at
    /// large.
    ActiveHasAtLeastEnergy(u32),
    /// The player holds more Prize cards than the opponent.
    /// `OpponentPrizesAtMost` reads the opponent's count alone;
    /// `Rosa's Encouragement` is the first to compare the two.
    MorePrizesThanOpponent,
    /// The player's hand holds exactly this many cards, this one among
    /// them. `Gladion's Final Battle` reads "only when it is the last card
    /// in your hand" — checked before the card leaves it.
    HandSizeIs(u32),
    /// A second copy of this exact card sits in hand too, and is
    /// consumed alongside it. `Transformation Tome` prints "You must
    /// play 2 Transformation Tome cards at once" — a cost paid in a
    /// second physical copy of itself, not in cards the player chooses.
    SecondCopyOfThisInHand,
    /// The opponent holds exactly this many Prize cards — narrower
    /// than `OpponentPrizesAtMost`, which also allows fewer. `Briar`.
    OpponentPrizesExactly(usize),
    /// The player has a Pokémon carrying `Marker::Tera` in play — read
    /// from the board, and costs nothing. `Glass Trumpet`.
    OwnTeraPokemonInPlay,
    /// The player's own Active carries this name prefix. `Team Rocket's
    /// Giovanni`, which also needs a Benched Pokémon of the same prefix
    /// to switch into — read at `legal_actions` time, not here, the same
    /// way every other Requirement costs nothing to check but does not
    /// itself guarantee the effect resolves to completion.
    ActiveNamePrefix(&'static str),
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
        // `Lumiose City`'s search shape is fixed, not read from its own
        // stored effect (which stays a no-op at play time — a Stadium's
        // once-a-turn action isn't dispatched by playing it).
        const LUMIOSE_CITY_SLOT: [Slot; 1] = [Slot {
            filter: CardFilter::PokemonOfStage(Stage::Basic),
            to: Destination::Bench,
            limit: 1,
            excludes_type_of_previous: false,
            peek: None,
        }];
        match &self.effect {
            TrainerEffect::Decide { slots, .. } => slots,
            // `Poké Ball` wraps its search in a coin flip; once heads
            // opens it, the slot read-back still needs to find it.
            TrainerEffect::CoinFlipThen(inner) => match &**inner {
                TrainerEffect::Decide { slots, .. } => slots,
                _ => &[],
            },
            TrainerEffect::MaySearchBasicToBenchThenMaybeEndTurn => &LUMIOSE_CITY_SLOT,
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
    /// What the attack's own printed text does, beyond damage and
    /// `inflicts` — read in `attack()`, the same place `resolve_trainer`
    /// reads a `TrainerEffect`, but a separate enum: a Trainer and an
    /// attack dispatch through different machinery.
    pub effect: Option<AttackEffect>,
}

/// An attack's own printed effect, beyond damage and a plain Special
/// Condition. Milestone 11's own vocabulary, the counterpart to
/// `TrainerEffect` — grows one variant per shape the sample decks
/// actually need, the same discipline `TrainerEffect` was built under.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackEffect {
    /// This many damage counters land on the attacker itself, alongside
    /// whatever the attack did to the defender. `Carvanha`'s "This
    /// Pokémon also does 10 damage to itself."
    Recoil(u32),
    /// This attack's damage is entirely `per_unit * count`, where `count`
    /// is read fresh from the board at attack time — not added to a
    /// printed base, since the printed base is the "×" marker itself.
    /// Computed before `damage_dealt` runs, so Weakness, Resistance, and
    /// any Tool or Stadium bonus still apply to the total, the same as
    /// a plain attack's damage.
    DamagePerCount(Count, u32),
    /// This attack's damage skips Weakness, Resistance, and any other
    /// effect on the defender's own side — read in `attack`, which
    /// calls a variant of the damage order that omits step 33 entirely
    /// rather than `damage_dealt` reading this itself; `damage_dealt`
    /// stays the public, unconditioned order every existing caller
    /// (including several tests) already depends on.
    IgnoresDefendersEffects,
    /// The defender is given this Special Condition outright, no flip.
    /// `Brute Bonnet`'s Poison.
    InflictsCondition(Condition),
    /// The same infliction `InflictsCondition` grants, but on the
    /// attacker itself rather than the defender — this attack's own
    /// printed text names its own Pokémon. `Annihilape`'s `Tantrum`.
    InflictsConditionOnSelf(Condition),
    /// Inflict this Condition on the defender, then it cannot retreat
    /// during the opponent's very next turn — the same lifetime
    /// `DefenderCannotRetreatNextTurn` already carries, checked at
    /// the same retreat-offering site, but combined with an
    /// infliction the way `Attack` holding only one `AttackEffect`
    /// requires. `Pecharunt`'s `Poison Chain`.
    InflictsConditionAndDefenderCannotRetreatNextTurn(Condition),
    /// Both Active Pokémon are Knocked Out outright, no damage
    /// involved — modeled by raising both to their own effective HP
    /// and letting the ordinary `settle` sweep (`knock_out_the_dead`)
    /// take it from there, the same way
    /// `OncePerTurnMayDamageOpponentThenKnockOutSelf` raises its own
    /// carrier's damage rather than a separate forced-knockout
    /// primitive — so Prizes, and any Ability that reads a Knockout,
    /// still run through the one ordinary path. `Annihilape`'s
    /// `Destined Fight`.
    KnocksOutBothActivePokemon,
    /// Flip a coin; on heads, the defender is given this Special
    /// Condition. `Zeraora`, `Dedenne`.
    CoinFlipInflicts(Condition),
    /// Flip a coin; on heads, this many more damage. Read before
    /// `damage_dealt`, the same as `DamagePerCount`, so Weakness and
    /// Resistance still apply to the total. `Applin`.
    CoinFlipBonusDamage(u32),
    /// The defending Pokémon cannot retreat during the opponent's very
    /// next turn. `Yveltal`, `Wellspring Mask Ogerpon ex`.
    DefenderCannotRetreatNextTurn,
    /// The same retreat lock `DefenderCannotRetreatNextTurn` grants
    /// ("during your opponent's next turn"), combined with this much
    /// more damage from any attack the granting player deals to that
    /// same Pokémon during their own very next turn — one turn later
    /// than the retreat lock, since "during your next turn" and
    /// "during your opponent's next turn" name different turns.
    /// `Attack` holds only one `AttackEffect`, so both consequences
    /// read from this one variant: the retreat lock reuses
    /// `opponent_next_turn_restriction`, and the bonus damage sets
    /// its own `bonus_damage_to_pokemon_on_granting_players_next_turn`.
    /// `Stunfisk`'s `Pouncing Trap`.
    DefenderCannotRetreatAndTakesMoreDamageNextTurn(u32),
    /// During the opponent's very next turn, if this Pokémon is
    /// damaged by an attack — even if that attack Knocks it Out —
    /// this many damage counters (as raw damage) go on the Attacking
    /// Pokémon. The same "even if Knocked Out" ordering
    /// `trigger_defenders_tool` already reads before `settle` decides
    /// a knockout, granted by `opponent_next_turn_restriction` for
    /// one turn rather than a permanently attached Tool. `Mega
    /// Slowbro ex`'s `Shellnado Spin`.
    GrantsSelfCountersAttackerIfDamagedNextTurn(u32),
    /// The attacker cannot use any attack during their own very next
    /// turn. `N's Zekrom`'s `Rampaging Thunder`.
    AttackerCannotAttackNextTurn,
    /// The narrower mirror of `AttackerCannotAttackNextTurn`: only
    /// this one named attack is locked out during the attacker's own
    /// very next turn, read by name off `GameState::locked_attack_next_turn`
    /// rather than this variant itself (which carries no name of its
    /// own — the attack that used it is read at the moment it fires).
    /// `Koraidon ex`'s and `Annihilape`'s own `Impact Blow`.
    CannotUseThisAttackNextTurn,
    /// Put this many damage counters (10 damage each) on the opponent's
    /// Benched Pokémon, in any combination the player chooses — opens
    /// `Phase::DistributingDamageCounters`, since the first read of a
    /// Benched Pokémon taking damage at all. `Dragapult ex`.
    DamageCountersToOpponentBenchAnyWay(u32),
    /// Discard every Energy attached to the attacker, then deal this
    /// much flat damage (no Weakness or Resistance) to one Benched
    /// Pokémon the player chooses. The discard is unconditional; the
    /// damage opens `Phase::ChoosingBenchDamageTarget` only when the
    /// opponent has a Bench to choose from. `N's Darmanitan`'s
    /// `Flamebody Cannon`.
    DiscardsOwnEnergyThenDamagesChosenBenched(u32),
    /// Switch the attacker with one of its own Benched Pokémon, the
    /// player's choice — the same `Phase::Promoting` shape
    /// `TrainerEffect::SwitchOwnActive` already opens, read from an
    /// attack instead. `Abra`'s `Teleportation Attack`.
    SwitchOwnActive,
    /// Search the deck for up to this many Basic Pokémon and put them
    /// onto the Bench, then shuffle — opens
    /// `Phase::SearchingDeckForBasics`, a search read from an attack
    /// rather than a Trainer's `Decide`. `Drilbur` and `Toxel`'s
    /// `Call for Family`.
    SearchDeckForBasicPokemonToBench(u32),
    /// Search the entire deck for an Item card and put it into
    /// hand, then shuffle — opens `Phase::SearchingDeckForItem`.
    /// No Item card in the deck opens no phase. `Patrat`'s
    /// `Procurement`.
    SearchDeckForItemCardToHand,
    /// Search the entire deck for up to this many cards, of any
    /// kind, and put them into hand, then shuffle — opens
    /// `Phase::SearchingDeckForAnyCards`. An empty deck opens
    /// no phase. `Noctowl`'s `Talon Hunt`.
    SearchDeckForUpToCardsOfAnyKindToHand(u32),
    /// The opponent reveals their hand. The engine already tracks every
    /// zone in full (`view::PlayerView` hides a zone only at render
    /// time, and nothing in `legal_actions` ever gates a choice on
    /// hidden opponent-hand knowledge), so there is no state for this
    /// to change — it lands in the log alone. `Hoothoot`'s `Silent Wing`.
    RevealOpponentsHand,
    /// Draw this many cards outright, no choice involved.
    /// `Mega Sharpedo ex`'s `Greedy Fang`.
    DrawCards(u32),
    /// This much more damage, but only if the attacker already carries
    /// at least one damage counter — a threshold read once, before
    /// `damage_dealt_with` runs, the same as `CoinFlipBonusDamage`.
    /// `Mega Sharpedo ex`'s `Hungry Jaws`.
    BonusDamageIfOwnDamaged(u32),
    /// This much more damage, but only if the attacker already
    /// carries at least this many damage counters, and — unlike
    /// every other bonus-damage variant — this attack's damage is
    /// never affected by Weakness (Resistance still applies). Read
    /// in two places: the bonus itself in the `base` match, same as
    /// `BonusDamageIfOwnDamaged`, and the Weakness skip inside
    /// `damage_dealt_with`, which every other variant leaves alone.
    /// `Chi-Yu`'s `Whirling Envy`.
    BonusDamageIfOwnDamageCountersAtLeastIgnoringWeakness(u32, u32),
    /// The player may put up to this many Energy attached to the
    /// opponent's Active Pokémon into the opponent's hand, choosing
    /// which — opens `Phase::MovingOpponentsActiveEnergyToHand`. No
    /// Energy attached opens no phase. `Slowking`'s
    /// `Wash the Slate Clean`.
    MayReturnOpponentsActiveEnergyToHand(u32),
    /// Put a Trainer card from the player's own discard pile into their
    /// hand, choosing which if more than one qualifies — opens
    /// `Phase::TakingTrainerFromDiscard`. No Trainer in the discard
    /// pile opens no phase. `Dedenne`'s `Electromagnetic Sonar`.
    TakeTrainerFromDiscard,
    /// Search the deck for a card that evolves from the attacker
    /// itself and evolve it directly, then shuffle — the same
    /// hand-skipping evolution `TrainerEffect::EvolveSkippingOneStage`
    /// (Rare Candy) already runs, but pulled from the deck instead
    /// of the hand. `Dwebble`'s `Ascension`.
    SearchDeckToEvolveSelf,
    /// This much more damage, but only if the opponent's Active
    /// Pokémon is a Pokémon ex — read once, the same pre-`damage_dealt_with`
    /// slot every other conditional bonus already occupies.
    /// `Moltres`'s `Fighting Wings`.
    BonusDamageIfDefenderIsEx(u32),
    /// Put a Pokémon card from the player's own discard pile into
    /// their hand, choosing which if more than one qualifies — opens
    /// `Phase::TakingPokemonFromDiscard`. No Pokémon in the discard
    /// pile opens no phase. `Slowpoke`'s `Dangle Tail`.
    TakePokemonFromDiscard,
    /// Put up to this many copies of the named Pokémon from the
    /// player's own discard pile onto the Bench — opens
    /// `Phase::SearchingDiscardForNamedToBench`. `Duskull`'s
    /// `Come and Get You`.
    SearchDiscardForNamedToBench(&'static str, u32),
    /// Switch the opponent's Active Pokémon, their choice of which —
    /// the same `Phase::Promoting` shape
    /// `TrainerEffect::SwitchOpponentActive` already opens, read from
    /// an attack instead. `Bayleef`'s `Push Down`.
    SwitchOpponentActive,
    /// The defending Pokémon deals this much less damage during the
    /// opponent's very next turn, before Weakness and Resistance —
    /// stored in `opponent_next_turn_restriction`, the same lifetime
    /// `DefenderCannotRetreatNextTurn` already carries. `Buneary`'s
    /// `Charm`, `Chikorita`'s `Growl`.
    DefenderDealsLessDamageNextTurn(u32),
    /// On heads, the attacker itself is immune to all damage and
    /// every other effect of any attack against it during the
    /// opponent's very next turn — stored as the attacker in
    /// `opponent_next_turn_restriction`, the same lifetime
    /// `DefenderCannotRetreatNextTurn` grants, but read as a full
    /// short-circuit at the top of `attack` rather than inside
    /// `damage_dealt_with`, since it blocks more than damage.
    /// `Dunsparce`'s `Dig`, `Elgyem`'s `Hide`.
    CoinFlipSelfInvulnerableNextTurn,
    /// The opponent cannot play any Item card during their very next
    /// turn — the same `opponent_next_turn_restriction` lifetime
    /// `DefenderCannotRetreatNextTurn` already carries, but read at
    /// the Item-offering site in `legal_actions` rather than at
    /// retreat's. `Budew`'s `Itchy Pollen`.
    OpponentCannotPlayItemsNextTurn,
    /// If the opponent has a Stadium in play, discard it; if that
    /// happens, the opponent cannot play any Stadium card from hand
    /// during their very next turn — the same
    /// `opponent_next_turn_restriction` lifetime
    /// `OpponentCannotPlayItemsNextTurn` already carries, read at
    /// the Stadium-offering site instead of the Item one. Read in
    /// `attack_with` rather than `resolve_attack_effect`, since it
    /// runs no matter what — flat 40 damage, unaffected by whether a
    /// Stadium was there to discard. `Chi-Yu`'s `Scorching Earth`.
    DiscardsOpponentsStadiumThenOpponentCannotPlayStadiumsNextTurn,
    /// The player may shuffle exactly `count` Energy attached to the
    /// attacker into their own deck; if they do, this attack also
    /// deals `damage` (flat) to one Benched Pokémon they choose. Opens
    /// `Phase::DecidingToShuffleEnergyForBenchDamage` only when the
    /// attacker carries at least `count` Energy — otherwise the
    /// option was never really available. Which Energy cards shuffle
    /// is not the player's choice: they are interchangeable for this
    /// effect, so the engine takes the first `count` it finds.
    /// `Wellspring Mask Ogerpon ex`'s `Torrential Pump`.
    MayShuffleFixedEnergyThenDamageChosenBenched { count: u32, damage: u32 },
    /// This much more damage, but only if the opponent's Active
    /// Pokémon is printed at exactly this Stage — the mirror of
    /// `BonusDamageIfDefenderIsEx`, read against `Stage` instead of
    /// prize value. `Paldean Tauros`'s `Spirited Tackle`.
    BonusDamageIfDefenderIsStage(Stage, u32),
    /// This much more damage, but only if the attacker's own owner
    /// has this many cards or fewer left in their deck. `Rabsca`'s
    /// `Counterturn`.
    BonusDamageIfOwnDeckAtMost(usize, u32),
    /// This attack's own printed base damage, plus this much more for
    /// each unit of `Count` — the additive mirror of `DamagePerCount`,
    /// which replaces the base entirely rather than adding to it.
    /// `Koraidon ex`'s `Retribution Strike` ("20+ ... 10 more for
    /// each damage counter" — a floor, not a pure multiply).
    BonusDamagePerCount(Count, u32),
    /// This much more damage, but only if any of the attacker's own
    /// Pokémon were Knocked Out by attack damage during the
    /// opponent's last turn — the same `knocked_out_last_turn` flag
    /// `AbilityEffect::OncePerTurnIfKnockedOutLastTurnMayDrawCards`
    /// already reads. `Koraidon ex`'s `Orichalcum Fang`.
    BonusDamageIfOwnKnockedOutLastTurn(u32),
    /// This much more damage, but only if any of the attacker's own
    /// Benched Pokémon carry damage. `Koraidon ex`'s `Revenge
    /// Buster`.
    BonusDamageIfOwnBenchDamaged(u32),
    /// This much more damage, but only if any of the attacker's own
    /// Pokémon in play share a type with any of the opponent's
    /// Pokémon in play. `Enamorus`'s `Love Resonance`.
    BonusDamageIfSharedTypeInPlay(u32),
    /// This much more damage, but only if a Stadium is in play,
    /// either player's; if it landed, the Stadium is then discarded
    /// — read in `resolve_attack_effect`, after damage, matching the
    /// card's own "Then, discard that Stadium." `Chi-Yu`'s `Ground
    /// Melter`.
    BonusDamageIfStadiumInPlayThenDiscardsIt(u32),
    /// This much more damage, but only if the attacker has any
    /// Energy of this type attached — a threshold check, unlike
    /// `Count::OwnEnergyOfTypeAttachedCount`'s own per-unit
    /// multiplier. `Stunfisk`'s `Muddy Bolt`.
    BonusDamageIfOwnEnergyOfTypeAttached(Type, u32),
    /// Flip a coin. If heads, the defender is inflicted with this
    /// condition, and the player picks one Energy attached to the
    /// defender to discard — opens `Phase::DiscardingDefenderEnergyForAttack`
    /// if the defender carries any, no discard otherwise.
    /// `Stunfisk`'s `Paralyzing Crackle`.
    CoinFlipInflictsAndDiscardsDefenderEnergy(Condition),
    /// The same coin flip and discard
    /// `CoinFlipInflictsAndDiscardsDefenderEnergy` grants, without
    /// the inflicted Condition — just a discard, on heads.
    /// `Goldeen`'s `Whirlpool`.
    CoinFlipDiscardsDefenderEnergy,
    /// Discard exactly this many Energy cards attached to the
    /// attacker, the player's choice of which — the attack's own
    /// listed cost already guarantees at least this many are
    /// attached. Opens `Phase::ChoosingOwnEnergyToDiscardForAttack`.
    /// `Zeraora`'s `Strong Volt`, `Metagross`'s `Luster Blast`.
    DiscardsFixedOwnEnergyChosen(u32),
    /// The player may discard, one at a time up to this many, Energy
    /// of this type attached to the attacker — discarding none at
    /// all is a legal choice. If they discard at least one, this
    /// attack does this much more damage regardless of how many were
    /// actually discarded (the ruling on this exact card: choosing
    /// the bonus with zero qualifying Energy attached still grants
    /// it). Opens `Phase::DecidingToDiscardOwnEnergyForBonusDamage`.
    /// `Metagross`'s `Metallic Hammer`.
    MayDiscardUpToOwnEnergyOfTypeForFlatBonusDamage(Type, u32, u32),
    /// The player may discard any number of Basic Energy cards from
    /// anywhere among their own Pokémon, not only the attacker — this
    /// attack does this much damage for each card discarded that way.
    /// Opens `Phase::DiscardingAnyBasicEnergyForDamagePerCard`.
    /// `Raging Bolt ex`'s `Bellowing Thunder`.
    MayDiscardAnyOwnBasicEnergyForDamagePerCard(u32),
    /// Discard this many cards from the top of the opponent's
    /// deck — fewer if their deck holds fewer. `Mega Excadrill
    /// ex`'s `Undermine`.
    DiscardsTopOfOpponentsDeck(u32),
    /// Shuffle every Energy attached to the attacker into its
    /// owner's deck, then deal this much flat damage to one of the
    /// opponent's Pokémon, the player's choice of which — Weakness
    /// and Resistance apply only if the chosen target is the
    /// opponent's Active, never if it's Benched (the card's own
    /// parenthetical). Opens
    /// `Phase::ChoosingAnyOpponentPokemonDamageTargetWeaknessIfActive`.
    /// A Tool bonus, a turn bonus, and `Cobalt Command` never reach
    /// this damage even against an Active target, the same
    /// documented gap `MayDiscardAnyOwnBasicEnergyForDamagePerCard`
    /// already carries (ADR 0085) — this damage is computed outside
    /// `damage_dealt_with`'s own one pass too. `Mega Skarmory ex`'s
    /// `Sonic Ripper`.
    ShufflesOwnEnergyThenDamagesChosenOpponentPokemonWeaknessIfActive(u32),
    /// Put this many damage counters on every Pokémon in play, both
    /// sides, that is printed with an Ability — carried regardless
    /// of whether that Ability is currently disabled (`Team Rocket's
    /// Watchtower`, `Midnight Fluttering`); the card's own text reads
    /// "has an Ability," not "can use one." No choice involved.
    /// `Cofagrigus`'s `Law of the Underworld`.
    DamagesEveryPokemonWithAnAbility(u32),
    /// Move all damage counters from 1 of the player's own Benched
    /// Pokémon (their choice) to 1 of the opponent's Pokémon, Active
    /// or Benched (their choice too) — two sequential picks, one
    /// phase each. `Cofagrigus`'s `Extended Damagriiigus`.
    MovesAllDamageFromChosenOwnBenchedToChosenOpponentPokemon,
    /// The player picks 1 of their own Benched Pokémon first, then
    /// searches the deck for up to this many Basic Energy cards of
    /// this type and attaches all of them to that one chosen
    /// Pokémon, then shuffles — target chosen before the search,
    /// unlike `SearchEnergyAttachToBenchedOfType`, which restricts
    /// the target to a Pokémon already matching the Energy's own
    /// type and so never really chooses. `Smoochum`'s `Delightful
    /// Kiss`.
    SearchesBasicEnergyOfTypeAttachToChosenBenched(Type, u32),
    /// Search the deck for up to this many cards, each independently
    /// a Pokémon of this type or a Stadium (`CardFilter::PokemonOf-
    /// TypeOrStadium`), reveal them, and put them into hand, then
    /// shuffle. An empty deck opens no phase. `Celebi`'s
    /// `Traverse Time`.
    SearchDeckForUpToPokemonOfTypeOrStadiumToHand(Type, u32),
    /// If the defender carries exactly this many damage counters, it
    /// is Knocked Out outright — modeled the same way `Cursed Blast`'s
    /// own forced Knockout already is, by raising its own damage to
    /// its effective HP rather than a separate forced-knockout
    /// primitive, so the ordinary `knock_out_the_dead` sweep still
    /// awards the Prize. An "at least" reading would be wrong here —
    /// the card says exactly. `Mega Absol ex`'s `Terminal Period`.
    KnocksOutDefenderIfExactDamageCounters(u32),
    /// This much more damage, but only if the attacker itself moved
    /// from its own owner's Bench to the Active Spot this turn —
    /// read from `GameState::promoted_from_bench_this_turn`, set at
    /// every site that promotes a Pokémon from the Bench (a
    /// Knockout, a forced switch, retreating, a switch-in Ability).
    /// `Mega Lopunny ex`'s `Gale Thrust`.
    BonusDamageIfSelfPromotedThisTurn(u32),
    /// The opponent reveals their hand; the player picks one card
    /// from it to discard. Opens
    /// `Phase::ChoosingCardFromOpponentsHandToDiscard` — the first
    /// effect in the engine that lets a player choose from the
    /// *opponent's* hand rather than their own; the engine already
    /// tracks every zone in full, so "reveals" itself changes no
    /// state, the same way `RevealOpponentsHand` doesn't. No cards
    /// in hand opens no phase. `Mega Absol ex`'s `Claw of Darkness`.
    DiscardsChosenFromOpponentsHand,
    /// This much more damage, but only if the attacker has at least
    /// this many Energy cards attached beyond its own printed
    /// attack cost — a threshold read against the cost, unlike every
    /// other Energy-count bonus, which reads a fixed number or a
    /// fixed type regardless of what the attack itself costs.
    /// `Mega Excadrill ex`'s `Maximum Drilling`.
    BonusDamageIfExtraEnergyAttached(u32, u32),
    /// Discard the whole hand, then draw this many cards outright —
    /// unlike `TrainerEffect::ShuffleHandThenDraw`, the discarded
    /// hand goes to the discard pile, not back into the deck.
    /// `Raging Bolt ex`'s `Burst Roar`.
    DiscardsHandThenDrawsCards(u32),
    /// Flip this many coins; this much damage for each heads. Read
    /// once, the same pre-`damage_dealt_with` slot `DamagePerCount`
    /// already occupies, but counted from flips rather than a board
    /// fact. `Combusken`'s `Double Kick`.
    DamagePerCoinFlipHeads { flips: u32, per_head: u32 },
    /// Flip a coin until it lands tails; this much more damage, on
    /// top of the attack's own printed base, for each heads flipped
    /// along the way. `Mega Kangaskhan ex`'s `Rapid-Fire Combo`.
    DamagePerCoinFlipUntilTails(u32),
    /// Put the attacker itself, and every card attached to it, into
    /// the owner's hand — the whole stack together, the same "moves
    /// together" rule 22 already keeps for a knockout, but to hand
    /// rather than discard. Opens `Phase::Promoting` when the owner
    /// has a Bench to promote from; does nothing otherwise, since
    /// there would be no Active left to leave. `Meowth ex`'s
    /// `Tuck Tail`.
    ReturnSelfAndAttachedToHand,
    /// This much flat damage (no Weakness or Resistance) to one of
    /// the opponent's Pokémon, the player's choice of which — unlike
    /// `DamageCountersToOpponentBenchAnyWay` and
    /// `DiscardsOwnEnergyThenDamagesChosenBenched`, the choice is not
    /// limited to the Bench; the opponent's Active is offered too.
    /// `Fezandipiti ex`'s `Cruel Arrow`.
    DamageChosenOpponentPokemon(u32),
    /// The same flat, unaffected-by-anything damage
    /// `DamageChosenOpponentPokemon` deals, but to two distinct
    /// opponent Pokémon of the player's choosing rather than one —
    /// Active or Benched either pick, and immune to any Bench-shield
    /// effect (`Battle Cage`, `Flower Curtain`, `Spherical Shield`)
    /// the same way it is immune to Weakness and Resistance, since
    /// its own text says "isn't affected... by any effects on those
    /// Pokémon." `Iron Crown ex`'s `Twin Shotels`.
    DamageTwoChosenOpponentPokemon(u32),
    /// The mirror of `DamageChosenOpponentPokemon`, but the flat
    /// amount is computed from a counted board fact rather than
    /// printed outright — opens the same
    /// `Phase::ChoosingAnyOpponentPokemonDamageTarget`, since nothing
    /// about the choice itself differs once the count is read.
    /// `Genesect`'s `Bug's Cannon`.
    DamagePerCountToChosenOpponentPokemon(Count, u32),
    /// Place this many damage counters (10 damage each) on the
    /// defender, for each unit of a counted board fact — "place
    /// damage counters" bypasses the ordinary damage order entirely,
    /// the same way `DamageCountersToOpponentBenchAnyWay` already
    /// does, rather than running through `damage_dealt_with` the way
    /// `DamagePerCount` does. `Alakazam`'s `Powerful Hand`.
    PlaceDamageCountersOnDefenderPerCount(Count, u32),
    /// This much flat damage (no Weakness or Resistance) to one of
    /// the opponent's Benched Pokémon ex, the player's choice of
    /// which — narrower than `DamageChosenOpponentPokemon`, which
    /// offers every Pokémon the opponent has. Opens
    /// `Phase::ChoosingBenchedExDamageTarget`. No Benched Pokémon ex
    /// opens no phase. `Shaymin`'s `Pinpoint Dive`.
    DamageChosenOpponentBenchedEx(u32),
    /// Place this many damage counters (10 damage each) on one of the
    /// opponent's Benched Pokémon, the player's choice of which — the
    /// same shape `DamageChosenOpponentBenchedEx` takes, widened from
    /// only a Benched ex to any Benched Pokémon. Opens
    /// `Phase::ChoosingAnyBenchedDamageTarget`. No Benched Pokémon
    /// opens no phase. `Annihilape`'s `Ghostly Blow`.
    PlacesDamageCountersOnChosenOpponentBenched(u32),
    /// Discard all Energy from the attacker, then the same flat
    /// damage `DamageChosenOpponentBenchedEx` deals — the same
    /// discard-then-damage shape `DiscardsOwnEnergyThenDamages-
    /// ChosenBenched` already takes, narrowed to a Benched Pokémon
    /// ex the same way `DamageChosenOpponentBenchedEx` narrows
    /// `DamageChosenOpponentPokemon`. `Zeraora`'s `Thunder Raid`.
    DiscardsOwnEnergyThenDamagesChosenBenchedEx(u32),
    /// Discard all Energy from the attacker, then this much flat
    /// damage (no Weakness or Resistance) to 3 of the opponent's
    /// Pokémon, chosen one at a time — the same discard-then-damage
    /// shape `DiscardsOwnEnergyThenDamagesChosenBenchedEx` takes,
    /// widened from a single Benched Pokémon ex to any 3 distinct
    /// Pokémon the opponent has, Active or Benched. `Kyurem`'s
    /// `Trifrost`.
    DiscardsOwnEnergyThenDamagesThreeChosenOpponentPokemon(u32),
    /// Choose one of a Benched Pokémon's own attacks, whose name
    /// starts with this prefix, and use it as this attack — no
    /// damage or effect of its own, since it borrows a second card's
    /// entirely. Opens `Phase::ChoosingBenchedPokemonAttackToCopy`;
    /// no Benched Pokémon matching the prefix opens no phase, so
    /// nothing happens. Once chosen, `engine::attack_with` runs the
    /// copied `Attack` value exactly as it would the attacker's own
    /// printed one — the same dispatch, reading a different `Attack`.
    /// `N's Zoroark ex`'s `Night Joker`, prefixed on `"N's "`.
    CopiesChosenBenchedPokemonAttackByNamePrefix(&'static str),
    /// Discard the top card of the deck outright; if it turns out
    /// to be a Pokémon without a Rule Box (1 Prize), choose 1 of its
    /// own attacks and use it as this attack — the same
    /// choose-a-second-card's-`Attack`-and-run-it shape
    /// `CopiesChosenBenchedPokemonAttackByNamePrefix` already takes,
    /// with the card found by discarding rather than a player's own
    /// choice among the Bench. Opens
    /// `Phase::ChoosingDiscardedPokemonAttackToCopy`; any other kind
    /// of card, or a Pokémon with a Rule Box, discards and does
    /// nothing more. `Slowking`'s `Seek Inspiration`.
    DiscardsTopOfDeckThenCopiesItsAttackIfNoRuleBox,
    /// Search the deck for an Energy card and attach it to one of the
    /// player's own Benched Pokémon of this type, then shuffle the
    /// deck. No qualifying Energy or no qualifying Bench target opens
    /// no phase. `Shaymin`'s `Send Flowers`.
    SearchEnergyAttachToBenchedOfType(Type),
    /// The attacker takes this much less damage from attacks during
    /// the opponent's very next turn, after Weakness and Resistance —
    /// the mirror of `DefenderDealsLessDamageNextTurn`'s own lifetime
    /// and reduction, but read on the defending side of a later
    /// attack and after Weakness/Resistance rather than before.
    /// `Genesect ex`'s `Protect Charge`.
    SelfDamageReductionNextTurn(u32),
    /// Put an Energy attached to the attacker into the owner's hand,
    /// choosing which if more than one is attached — opens
    /// `Phase::ChoosingOwnEnergyToHand`. No Energy attached opens no
    /// phase. `Chien-Pao`'s `Icicle Loop`.
    MoveOwnAttachedEnergyToHand,
    /// Move an Energy attached to the attacker to a chosen own
    /// Benched Pokémon, choosing which Energy if more than one is
    /// attached — opens `Phase::ChoosingEnergyAndBenchedTargetToMove`.
    /// No Energy attached, or no own Bench, opens no phase.
    /// `Shaymin`'s `Reflect Energy`.
    MoveOwnAttachedEnergyToChosenBenched,
    /// "If there is no Stadium in play, this attack does nothing" —
    /// the whole attack fizzles outright, checked at the top of
    /// `attack`, the same short-circuit shape
    /// `CoinFlipSelfInvulnerableNextTurn` already takes. `Fan Rotom`'s
    /// `Assault Landing`.
    FizzlesWithNoStadiumInPlay,
    /// Move an Energy from one of the opponent's Pokémon to another —
    /// the attacker's own choice of both ends, but on the opponent's
    /// board, unlike `TrainerEffect::MoveAttachedEnergy`'s own board.
    /// Opens `Phase::MovingOpponentsEnergy`. No move available (fewer
    /// than 2 Pokémon in play, or none carrying Energy) opens no
    /// phase. `Elgyem`'s `Slight Shift`.
    MoveOpponentsEnergyBetweenTheirPokemon,
    /// Before dealing damage, discard every Pokémon Tool attached to
    /// the defender. Unconditional, no phase — nothing for the
    /// player to choose. `Seaking`'s `Peck Off`.
    DiscardsDefendersTools,
}

/// What `AttackEffect::DamagePerCount` counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Count {
    /// Damage counters already on the attacker itself. `N's Reshiram`.
    OwnDamageCounters,
    /// Basic Energy cards in the opponent's discard pile. `N's Darmanitan`.
    OpponentBasicEnergyInDiscard,
    /// The opponent's Pokémon ex in play. `Dudunsparce ex`.
    OpponentPokemonExInPlay,
    /// The player's own Basic Pokémon in play. `Passimian`.
    OwnBasicPokemonInPlay,
    /// The player's own Pokémon whose name holds this substring, only
    /// counting one already carrying damage. `Paldean Tauros`.
    OwnDamagedWithNamePrefix(&'static str),
    /// The opponent's Benched Pokémon. `Zeraora`'s `Combat Thunder`.
    OpponentBenchedPokemonCount,
    /// Energy attached to the attacker and the defender combined.
    /// `Teal Mask Ogerpon ex`'s `Myriad Leaf Shower`.
    EnergyOnBothActivesCount,
    /// Prize cards the opponent has already taken (6 minus how many
    /// remain). `Pecharunt ex`'s `Irritated Outburst`.
    OpponentPrizesTakenCount,
    /// Energy of this type attached to the attacker itself.
    /// `Genesect`'s `Bug's Cannon`, `Seaking`'s `Hydro Jet`.
    OwnEnergyOfTypeAttachedCount(Type),
    /// Cards in the attacker's owner's own hand. `Alakazam`'s
    /// `Powerful Hand`.
    OwnHandSizeCount,
    /// Every Benched Pokémon in play, both sides combined. `Lillie's
    /// Clefairy ex`'s `Full Moon Rondo`.
    BothBenchedPokemonCount,
    /// Energy attached to the defender alone. `Rabsca`'s `Psychic`.
    DefenderEnergyAttachedCount,
    /// Damage counters already on the defender. `Brute Bonnet`'s
    /// `Relentless Punches`.
    DefenderDamageCounters,
    /// Energy of this type attached to every one of the attacker's
    /// own Pokémon in play, not only the attacker itself — unlike
    /// `OwnEnergyOfTypeAttachedCount`, which reads the attacker
    /// alone. `Hydrapple ex`'s `Syrup Storm`.
    OwnEnergyOfTypeAttachedAcrossSideCount(Type),
    /// The player's own Benched Pokémon, unlike
    /// `OpponentBenchedPokemonCount` and `BothBenchedPokemonCount`.
    /// `Dipplin`'s `Do the Wave`.
    OwnBenchedPokemonCount,
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
    /// The name of the Basic two links below this one in the evolution
    /// line, for a Stage 2 — resolved at import by walking the pool, even
    /// where the Stage 1 between them is refused. `None` on a Basic or a
    /// Stage 1, and on a Stage 2 whose chain does not resolve. `Rare Candy`
    /// is the only card that reads this: it evolves a Basic straight into a
    /// Stage 2, skipping the Stage 1 `evolve_from` alone would name.
    pub evolves_from_basic: Option<&'static str>,
    pub attacks: Vec<Attack>,
    /// Every real card prints at most one. `None` for the pool's plain
    /// majority; `Some` only once the engine can run what it says.
    pub ability: Option<Ability>,
    /// What this print carries beyond its species name — `ex`, `Mega`,
    /// `Tera` — any number at once. Read from real-world knowledge of
    /// the print, the same discipline `known_attack`/`known_ability`
    /// know a print by: `ex` and `Mega` are read straight from the
    /// printed name (`markers_for` in `src/import.rs`); `Tera` is not
    /// — the artifact carries no field for it, and two prints can
    /// share an identical name where only one is Tera, so it is read
    /// from a fixed print-id table instead. `prizes` is computed
    /// from this, not the other way around.
    pub markers: Vec<Marker>,
}

/// What a Pokémon print carries beyond its species name. See
/// [`Pokemon::markers`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Marker {
    Ex,
    Mega,
    /// Pokémon TCG stopped printing new Tera cards after the sets this
    /// project's own artifact already carries — this is a closed,
    /// permanent list, not one to revisit as later sets import.
    Tera,
    /// The Paradox "Ancient" trait — unlike `Tera`, nobody has
    /// confirmed this list is closed, so treat it as open. A print
    /// carries this by exact print id in `import::ANCIENT_PRINT_IDS`,
    /// scoped only to the names the sample decks actually use — see
    /// that const's own doc comment before assuming a name not there
    /// is untagged. No effect reads this yet.
    Ancient,
    /// The Paradox "Future" trait — the same open-list caveat
    /// `Ancient` carries applies here too. A print carries this by
    /// exact print id in `import::FUTURE_PRINT_IDS`, same deck-scoped
    /// caveat. Read by
    /// `AbilityEffect::PassiveFutureAttacksDoBonusDamageToActiveExceptNamed`,
    /// `Iron Crown ex`'s `Cobalt Command`.
    Future,
}

/// An Ability as printed. Milestone 8's own vocabulary, parallel to
/// `Attack` and `Trainer`'s own effect fields but read from a
/// Pokémon's standing power rather than something it does when
/// attacking or something played from hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ability {
    pub name: &'static str,
    pub effect: AbilityEffect,
}

/// Whether an Ability's "once during your turn" limit covers every copy
/// the player controls at once, rather than one use per Pokémon. True
/// only for the prints that spell out "You can't use more than 1 [Name]
/// Ability each turn" (or a wording that means the same). Matched by
/// name, the same real-world-knowledge discipline `known_ability` and
/// `known_energy` hold; each name is unique to its print. `Last-Ditch
/// Catch` prints the clause as "more than 1 Ability that has
/// 'Last-Ditch' in its name" — no other "Last-Ditch" Ability is in the
/// pool, so its own name is a faithful key. See ADR 0097.
pub fn ability_is_scoped_to_its_name(name: &str) -> bool {
    matches!(
        name,
        "Fan Call"
            | "Flip the Script"
            | "Last-Ditch Catch"
            | "Lunar Cycle"
            | "Run Errand"
            | "Subjugating Chains"
    )
}

/// What an Ability actually does, once the engine can run it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityEffect {
    /// Once during the player's own turn, only while this Pokémon is
    /// the Active, the player may draw this many cards. No cost, no
    /// target — the simplest shape an Ability comes in.
    /// `Mega Kangaskhan ex`'s `Run Errand`.
    OncePerTurnWhileActiveMayDrawCards(u32),
    /// Once during the player's own turn, the moment this Pokémon is
    /// played from hand onto the Bench, the player may search their
    /// deck for a Supporter card and take it to hand — opens
    /// `Phase::DecidingToUseLastDitchCatch`. Triggered at the moment
    /// of the play itself, not offered later as a standing choice the
    /// way `OncePerTurnWhileActiveMayDrawCards` is.
    /// `Meowth ex`'s `Last-Ditch Catch`.
    WhenBenchedFromHandMaySearchSupporter,
    /// Once during the player's own turn, the moment this Pokémon is
    /// played from hand to evolve one of the player's own Pokémon,
    /// the player may draw this many cards. The same "tied to a
    /// moment" shape `WhenBenchedFromHandMaySearchSupporter` already
    /// takes, hooked to a different play. `Kadabra`'s and
    /// `Alakazam`'s `Psychic Draw` — the same name, two prints, two
    /// counts, resolved by species name alone (`known_ability` is
    /// already keyed by species, so no print-override table is
    /// needed the way `known_trainer_by_print` is).
    WhenEvolvedFromHandMayDrawCards(u32),
    /// The same "evolved from hand" trigger `WhenEvolvedFromHandMay-
    /// DrawCards` reads, but gated on the player having any
    /// `Marker::Tera` Pokémon in play, and searching up to this many
    /// Trainer cards of any kind rather than drawing. `Noctowl`'s
    /// `Jewel Seeker`.
    WhenEvolvedFromHandMaySearchTrainersIfOwnTeraInPlay(u32),
    /// Once during the player's own turn, only if any of the
    /// player's own Pokémon were Knocked Out during the opponent's
    /// last turn, the player may draw this many cards — the same
    /// `knocked_out_last_turn` fact `Requirement::KnockedOutDuringOpponentsLastTurn`
    /// already reads for `Unfair Stamp`, read here for an Ability
    /// instead of a Trainer's requirement.
    /// `Fezandipiti ex`'s `Flip the Script`.
    OncePerTurnIfKnockedOutLastTurnMayDrawCards(u32),
    /// Once during the player's own turn, the player may attach a
    /// Basic Energy of this type from hand to the Pokémon carrying
    /// this Ability. If they do, they draw a card. Opens
    /// `Phase::DecidingToUseTealDance` only when a qualifying Energy
    /// is in hand. `Teal Mask Ogerpon ex`'s `Teal Dance`.
    OncePerTurnMayAttachBasicEnergyOfTypeThenDraw(Type),
    /// Once during the player's own turn, the player may draw this
    /// many cards. If any were actually drawn, this Pokémon and every
    /// card attached to it shuffle into the player's own deck —
    /// the same "moves together" rule 22 already keeps for a
    /// knockout, but into the deck rather than discard or hand.
    /// Opens `Phase::Promoting` if this Pokémon was the Active and
    /// the player has a Bench to promote from; otherwise it simply
    /// stays in play, since there would be nothing to replace it
    /// with. `Dudunsparce`'s `Run Away Draw`.
    OncePerTurnMayDrawThenShuffleSelfIntoDeck(u32),
    /// Once during the player's own turn, only while this Pokémon is
    /// the Active, the player may shuffle it and every card attached
    /// to it into the deck — the same "moves together" shape
    /// `OncePerTurnMayDrawThenShuffleSelfIntoDeck` takes, but with no
    /// draw and no condition beyond being Active. `Abra`'s `Beam`.
    OncePerTurnWhileActiveMayShuffleSelfIntoDeck,
    /// Once during the player's own turn, the player may put this
    /// many damage counters on one of the opponent's Pokémon,
    /// choosing which — a direct placement, so a target sitting on
    /// the Bench under `Battle Cage` takes nothing (the ruling on
    /// this very Ability confirms it: the counters never land, but
    /// the Ability was still used). If they do, the Pokémon carrying
    /// this Ability is Knocked Out outright regardless of whether the
    /// placement landed — modeled by raising its own damage to its
    /// effective HP rather than a separate forced-knockout primitive,
    /// so the ordinary `knock_out_the_dead` sweep still awards the
    /// Prize. `Dusclops`'s and `Dusknoir`'s `Cursed Blast`.
    OncePerTurnMayDamageOpponentThenKnockOutSelf(u32),
    /// Once during the player's own turn, the player may search the
    /// deck for up to `limit` Evolution Pokémon of this type and
    /// put them into hand — opens
    /// `Phase::SearchingDeckForEvolutionPokemonOfType`. No
    /// qualifying card in the deck opens no phase.
    /// `Genesect ex`'s `Protect Charge`.
    OncePerTurnMaySearchEvolutionPokemonOfType(Type, u32),
    /// Once during the player's own turn, the player may attach a
    /// Basic Energy card from their own discard pile to one of their
    /// own Pokémon, choosing both. Opens
    /// `Phase::DecidingToUseSeethingSpirit`. No qualifying Energy in
    /// the discard pile opens no phase. `Blaziken ex`'s
    /// `Seething Spirit`.
    OncePerTurnMayAttachBasicEnergyFromDiscardToChosen,
    /// Once during the player's own turn, discard a card from hand —
    /// the cost this Ability itself asks to be paid, not a "may" —
    /// then draw this many cards. Opens
    /// `Phase::DiscardingHandCardThenDrawing`. An empty hand offers
    /// nothing to discard, so the Ability itself is not offered.
    /// `N's Zoroark ex`'s `Trade`.
    OncePerTurnMayDiscardFromHandThenDrawCards(u32),
    /// Once during the player's own turn, the player may attach a
    /// Basic Energy card of this type from their hand to any of
    /// their own Pokémon, their choice of both — the same combined
    /// card-and-target choice
    /// `OncePerTurnMayAttachBasicEnergyFromDiscardToChosen` already
    /// takes, but from the hand rather than the discard pile. If
    /// they do, heal this many damage from whichever Pokémon
    /// received it. `Hydrapple ex`'s `Ripening Charge`.
    OncePerTurnMayAttachBasicEnergyOfTypeFromHandToChosenThenHeal(Type, u32),
    /// Once during the player's own turn, the moment this Pokémon is
    /// played from hand onto the Bench, the player may discard
    /// whichever Stadium is in play — opens
    /// `Phase::DecidingToUseSnowSink`. No Stadium in play opens no
    /// phase. `Chien-Pao`'s `Snow Sink`.
    WhenBenchedFromHandMayDiscardStadium,
    /// The moment this Pokémon is played from hand onto the Bench,
    /// the player may switch it in for the Active. If they do, they
    /// may then move any amount of Energy from their other Pokémon to
    /// this one, one card at a time until they stop. Opens
    /// `Phase::DecidingToSwitchInForRapidVernier`. `Iron Leaves ex`'s
    /// `Rapid Vernier`.
    WhenBenchedFromHandMaySwitchThenMoveAnyEnergy,
    /// Once during the player's own turn, the player may search the
    /// deck for a Basic Energy of this type and attach it to a
    /// Benched Pokémon of the same type, choosing which — opens
    /// `Phase::SearchingForSinisterSurgeTarget`. If they do, that
    /// Pokémon takes this much damage. No qualifying Energy in the
    /// deck, or no Benched Pokémon of the type to attach it to,
    /// opens no phase. Which Energy card is found is not the player's
    /// choice — every Basic Energy of a type is interchangeable, the
    /// same reasoning ADR 0068 already gave for a fixed-count Energy
    /// cost. `Toxtricity`'s `Sinister Surge`.
    OncePerTurnMaySearchBasicEnergyOfTypeAttachToBenchedThenDamage(Type, u32),
    /// Once during the player's own first turn only, the player may
    /// search the deck for up to `limit` Pokémon of this type
    /// printed at this HP or less, and put them into hand. Opens
    /// `Phase::SearchingForFanCall`. `Fan Rotom`'s `Fan Call`.
    OnceDuringFirstTurnMaySearchPokemonOfTypeWithHpAtMost(Type, u32, u32),
    /// Once during the player's own turn, the player may switch a
    /// Benched Pokémon of this type — except one printed with this
    /// name — with the Active, choosing which. If they do, the newly
    /// Active Pokémon is Poisoned. Opens
    /// `Phase::DecidingToUseSubjugatingChains`. No qualifying Bench
    /// Pokémon opens no phase. `Pecharunt ex`'s `Subjugating Chains`.
    OncePerTurnMaySwitchBenchedOfTypeExcludingNamedThenPoison(Type, &'static str),
    /// A standing effect, not a choice: while this Pokémon is in
    /// play, every Basic Pokémon the owner has in play — Active or
    /// Benched, this one included — has no Retreat Cost. Never
    /// offered as `Action::UseAbility`; read directly by
    /// `effective_retreat_cost`. `Latias ex`'s `Skyliner`.
    PassiveOwnBasicPokemonHaveNoRetreatCost,
    /// Once during the player's own turn, only while this Pokémon has
    /// any Energy of this type attached, the player may move up to
    /// this many damage counters (10 damage each) from one of their
    /// own Pokémon to one of the opponent's, choosing the source, the
    /// target, and how many. Opens
    /// `Phase::MovingDamageCountersFromOwnToOpponent`. No own Pokémon
    /// carrying any damage opens no phase. `Munkidori`'s
    /// `Adrena-Brain`.
    OncePerTurnIfEnergyOfTypeAttachedMayMoveDamageCountersToOpponent(Type, u32),
    /// Once during the player's own turn, the player may look at the
    /// top this-many cards of their own deck and put one of them
    /// into hand, choosing which — the rest go to the bottom of the
    /// deck, in the order they were seen. Opens
    /// `Phase::LookingAtTopCardsToTakeOne`. An empty deck opens no
    /// phase. `Drakloak`'s `Recon Directive`.
    OncePerTurnMayLookAtTopCardsTakeOneRestToBottom(u32),
    /// Once during the player's own turn, the player may look at the
    /// top this-many cards of their own deck and attach any number
    /// of Basic Energy of this type found there to their own
    /// Pokémon, in any way they like — resolved one seen card at a
    /// time: attach it to a chosen own Pokémon, or leave it, either
    /// way it leaves the top of the deck. Opens
    /// `Phase::ResolvingEnergyFoundInTopPeek`. An empty deck opens
    /// no phase. `Metang`'s `Metal Maker`.
    OncePerTurnMayLookAtTopCardsAttachFoundBasicEnergyOfType(u32, Type),
    /// A standing effect, not a choice: while this Pokémon is in
    /// play, it takes no damage from an attack whose attacker is an
    /// ex Pokémon. Never offered as `Action::UseAbility`; read
    /// directly by `damage_dealt_with`. `Crustle`'s
    /// `Mysterious Rock Inn`.
    PassiveImmuneToDamageFromOpponentEx,
    /// Once during the player's own turn, only while this Pokémon is
    /// the Active, the player may look at the top this-many cards of
    /// their own deck and take a Supporter card found there into
    /// hand, choosing which if more than one — the rest, and the
    /// Supporter if none is taken, shuffle back into the deck.
    /// Opens `Phase::LookingAtTopCardsForSupporter`. No Supporter in
    /// the peek opens no phase. `Tatsugiri`'s `Attract Customers`.
    OncePerTurnWhileActiveMayLookAtTopCardsTakeASupporter(u32),
    /// A standing effect, not a choice: while this Pokémon is in
    /// play, no damage counters on any Pokémon — either player's —
    /// can be moved to another Pokémon. Never offered as
    /// `Action::UseAbility`; read directly by the eligibility check
    /// for any effect that moves damage counters, such as
    /// `OncePerTurnIfEnergyOfTypeAttachedMayMoveDamageCountersToOpponent`.
    /// `Patrat`'s `Watchful Eye`.
    PassiveBlocksDamageCounterMovement,
    /// A standing effect, not a choice: while this Pokémon is in
    /// play, on either side, every Pokémon loses any Ability that
    /// requires knocking itself out to use. Never offered as
    /// `Action::UseAbility`; read directly by the eligibility check
    /// for any such effect, such as
    /// `OncePerTurnMayDamageOpponentThenKnockOutSelf`. `Psyduck`'s
    /// `Damp`.
    PassiveDisablesSelfKnockOutAbilities,
    /// A standing effect, not a choice: while this Pokémon is in
    /// play, every opponent's Pokémon printed with `from` as its
    /// Weakness type has `to` instead — its printed Weakness (if any
    /// other type) is overridden outright, not added to. Read
    /// directly by `GameState::effective_weakness`, the one site that
    /// ever reads a Pokémon's Weakness for damage. `Lillie's
    /// Clefairy ex`'s `Fairy Zone` (`{N}` Dragon to `{P}` Psychic).
    PassiveSetsOpponentTypeWeaknessTo(Type, Type),
    /// A standing effect, not a choice: while this Pokémon is in
    /// play, no attack from the opponent's Pokémon may do any damage
    /// — flat damage or a placed counter alike, unlike `Battle
    /// Cage`'s narrower carve-out — to a Benched Pokémon of this
    /// Pokémon's own owner that has no Rule Box (a Pokémon ex,
    /// Pokémon V, etc. does). Read directly by
    /// `GameState::bench_attack_damage_blocked` at every site that
    /// would deal attack damage to a specifically-Benched target.
    /// `Shaymin`'s `Flower Curtain`.
    PassivePreventsAttackDamageToNonRuleBoxBench,
    /// A standing effect, not a choice: while this Pokémon is in
    /// play, no attack from the opponent's Pokémon may do any damage
    /// or any other effect — to *any* Benched Pokémon of this
    /// Pokémon's own owner, Rule Box or not (unlike `Flower
    /// Curtain`'s carve-out). Read directly by
    /// `GameState::bench_attack_effect_blocked` at every site that
    /// would apply an attack's damage or non-damage effect to a
    /// specifically-Benched target — a Benched target still touched
    /// by an effect it wasn't the chosen target of is protected too
    /// (the ruling on Shiftry's Expelling Tornado confirms this).
    /// `Rabsca`'s `Spherical Shield`.
    PassivePreventsAttackEffectsOnBench,
    /// A standing effect, not a choice: while this Pokémon is in
    /// play, an attack used by any of this Pokémon's own owner's
    /// `Marker::Future` Pokémon — except one printed `Iron Crown ex`
    /// by name, not only this Ability's own carrier — does this much
    /// more damage to the opponent's Active Pokémon, before Weakness
    /// and Resistance. Read directly in `damage_dealt_with`, the
    /// same slot an attacker's own Tool bonus already occupies, since
    /// every call there already computes damage against the
    /// opponent's Active by construction (the single-Active format).
    /// `Iron Crown ex`'s `Cobalt Command`.
    PassiveFutureAttacksDoBonusDamageToActiveExceptNamed(u32),
    /// A standing effect, not a choice: while this Pokémon carries at
    /// least 2 damage counters, its own attacks do this much more
    /// damage to the opponent's Active Pokémon, before Weakness and
    /// Resistance — narrower than `Cobalt Command`'s own shape,
    /// since this reads only its own carrier's damage and only its
    /// own carrier's attacks, not a whole side. `Annihilape`'s `Lose
    /// Cool`.
    PassiveBonusDamageToActiveIfSelfDamaged(u32),
    /// A standing effect, not a choice: this Pokémon can never become
    /// Asleep. Read directly by `GameState::inflict`, the one site
    /// that ever applies a Special Condition. `Hoothoot`'s
    /// `Insomnia`.
    PassiveImmuneToAsleep,
    /// A standing effect, not a choice: while this Pokémon is in the
    /// Active Spot, the opponent's Active Pokémon has no Ability —
    /// except one that is itself this same effect (a mirror match).
    /// Folded into `GameState::abilities_disabled_for` rather than
    /// read at its own sites. `Flutter Mane`'s `Midnight Fluttering`.
    PassiveDisablesOpponentActiveAbilityExceptSelf,
    /// A standing effect, not a choice: the named attack costs one
    /// less `{C}` for each Prize card the opponent has already
    /// taken — a reduction, unlike every other cost modifier built
    /// so far (`Nighttime Mine`'s surcharge among them), which only
    /// ever adds. Read directly in `legal_actions`' own attack-cost
    /// loop, capped at removing every `{C}` the cost has and never
    /// touching a non-Colorless requirement. `Bloodmoon Ursaluna
    /// ex`'s `Seasoned Skill`, on `Blood Moon`.
    PassiveNamedAttackCostsLessPerOpponentPrizeTaken(&'static str),
    /// A standing effect, not a choice: while this Pokémon has a
    /// Pokémon Tool attached, the opponent cannot play any ACE SPEC
    /// card from their hand — an Item, Tool, Stadium, or Special
    /// Energy carrying that classification (never a Supporter).
    /// Read at every site that offers playing or attaching one:
    /// `PlayTrainer`'s own timing check and the Energy-attach
    /// offering, both in `legal_actions`. This Pokémon need not be
    /// Active — the card's own text names no Spot. `Genesect`'s `ACE
    /// Nullifier`.
    PassiveBlocksOpponentAceSpecPlaysIfSelfHasTool,
    /// A standing effect, not a choice: while any card in the
    /// opponent's discard pile carries this substring in its name,
    /// the named attack costs just one `{C}` — an override of the
    /// whole printed cost, not a per-count reduction the way
    /// `PassiveNamedAttackCostsLessPerOpponentPrizeTaken` is. Read
    /// directly in `legal_actions`' own attack-cost loop, alongside
    /// that discount. `Kyurem`'s `Plasma Bane`, on `Trifrost`.
    PassiveNamedAttackCostsJustColorlessIfOpponentDiscardNameContains(&'static str, &'static str),
    /// A standing effect, not a choice: if this Pokémon would be
    /// Knocked Out by damage from an attack (not a checkup — read
    /// from the same `attacking_defender` flag `Lillie's Pearl`
    /// already reads to tell the two apart), flip a coin; on heads,
    /// it is not Knocked Out, and its remaining HP becomes 10 — read
    /// directly in `knock_out_the_dead`, before a Prize is ever
    /// taken, the first Ability in this engine that can prevent a
    /// Knockout outright rather than only redirect or reduce its
    /// damage. `Annihilape`'s `Durable Body`.
    PassiveCoinFlipPreventsAttackKnockOutAtTenHp,
    /// A standing effect, not a choice: while `Festival Grounds` is
    /// in play, this Pokémon may use an attack it has a second time
    /// this turn — read at `Action::Attack`'s own site, right after
    /// the first attack resolves, rather than through `UseAbility`:
    /// nothing here is a separate action, only whether the ordinary
    /// end-of-turn that follows any attack is skipped once. Guarded
    /// by `festival_lead_extra_swing_used` so the door opens only
    /// once per turn, no matter how many further attacks that then
    /// allows. `Goldeen`'s and `Seaking`'s `Festival Lead`.
    PassiveFestivalLead,
    /// Once during the player's own turn, if their Active Pokémon
    /// carries an Ability by this name, search the deck for any
    /// one card into hand. Opens
    /// `Phase::SearchingDeckForAnyCard`. `Thwackey`'s `Boom Boom
    /// Groove`, on `"Festival Lead"`.
    OncePerTurnMaySearchAnyCardIfActiveHasNamedAbility(&'static str),
    /// A standing effect, not a choice: while any of the player's own
    /// Pokémon carries this Ability, undisabled, every Basic Grass
    /// Energy attached to any of their own Pokémon pays as if it
    /// were two — read directly in `pays_cost`, per Pokémon, rather
    /// than moving or duplicating any card. Doesn't stack: a second
    /// carrier in play changes nothing, since this is read as a
    /// single board-wide fact, not once per carrier. `Meganium`'s
    /// `Wild Growth`.
    PassiveDoublesBasicGrassEnergyForCost,
    /// A standing effect, not a choice: while this Pokémon is in the
    /// Active Spot, the opponent's own Poisoned Pokémon take this
    /// many more damage counters (as raw damage) at Pokémon Checkup
    /// — read directly in `resolve_checkup`, added to the ordinary
    /// Poison damage. `Pecharunt`'s `Toxic Subjugation`.
    PassiveBonusCheckupDamageToOpponentsPoisonedWhileActive(u32),
}

/// An Energy card as printed — a Basic Energy every deck supplies for
/// itself (`effect: None`), or a Special Energy the artifact names
/// (`effect: Some(_)`), matched by print name the same way an attack
/// or an Ability is (`known_energy` in `src/import.rs`).
#[derive(Debug, Clone)]
pub struct Energy {
    /// See [`Pokemon::print_id`].
    pub print_id: &'static str,
    pub name: &'static str,
    pub kind: Type,
    pub effect: Option<EnergyEffect>,
}

/// What a Special Energy actually does, once the engine can run it —
/// the same discipline `AttackEffect` and `AbilityEffect` already
/// hold: a value the engine executes, never text read at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnergyEffect {
    /// A standing effect, not a choice: while this card is attached,
    /// its carrier's HP is this much higher. Never offered as an
    /// action; read directly by `effective_hp`, the same way a
    /// Tool's own `IncreasesHp` already is. `Growing Grass Energy`.
    IncreasesCarrierHp(u32),
    /// The moment this card is attached from hand — never when it
    /// arrives any other way (dealt directly onto the board, moved
    /// by another effect) — the player draws this many cards. Read
    /// directly by `Action::AttachEnergy`'s own apply handler, the
    /// only site a card ever leaves hand to attach. `Enriching
    /// Energy`.
    DrawCardsOnAttachFromHand(u32),
    /// The moment this card is attached from hand to a Pokémon of the
    /// first type — never when it arrives any other way, the same
    /// gate `DrawCardsOnAttachFromHand` already reads by — search the
    /// deck for up to this many Basic Pokémon of the second type
    /// and put them onto the Bench, then shuffle. Opens
    /// `Phase::SearchingDeckForBasicsOfType`. No qualifying Basic
    /// Pokémon in the deck still attaches the Energy — only the
    /// search is skipped. `Telepathic Psychic Energy`.
    WhenAttachedToTypeSearchesBasicPokemonOfTypeToBench(Type, Type, u32),
    /// A standing effect, not a choice: while the carrier is Active
    /// and takes damage from an opponent's attack — even a hit that
    /// knocks it out — the attacker takes this much damage right
    /// back, placed directly (no Weakness or Resistance), the same
    /// convention `PlaceDamageCountersOnDefenderPerCount` already
    /// reads by. Read at the one site the base attack's own damage
    /// ever lands on the primary defender — a direct-Bench-targeting
    /// attack never reaches it, so "while Active" is automatic here.
    /// `Spiky Energy`.
    CountersAttackerOnDamageTakenWhileActive(u32),
    /// A standing effect, not a choice: damage still lands normally,
    /// but every other effect an opponent's attack would apply
    /// directly to the carrier — a Special Condition, a next-turn
    /// restriction naming the carrier — does not. An effect already
    /// in place is not retroactively removed. Read at each such
    /// effect's own grant site, not centrally — the printed text
    /// names what is "done to" the carrier, not every consequence an
    /// attack has anywhere on the board. `Mist Energy`.
    PreventsAttackEffectsOnCarrier,
    /// If this card is discarded as part of an attack's own effect,
    /// by the Pokémon it was attached to (never a Trainer, an
    /// opponent's effect, or a retreat cost), it returns to that
    /// Pokémon once the discard is done. Read at the one site an
    /// attack's own effect discards its own attacker's Energy,
    /// `DiscardsOwnEnergyThenDamagesChosenBenched`. `Boomerang
    /// Energy`.
    ReattachesAfterOwnDiscardByAttackEffect,
    /// A standing effect, not a choice: while attached to a Basic
    /// Pokémon, this card pays any single named type an attack's
    /// cost still needs, contributing exactly one unit of Energy —
    /// attached to a Stage 1 or 2 Pokémon it pays only Colorless,
    /// like a plain Basic Energy of no named type. Read directly by
    /// `pays_cost`, in place of its own printed `kind`, which stays
    /// `Colorless` (the value it falls back to off a Basic).
    /// `Prism Energy`.
    ProvidesAnyTypeIfAttachedToBasic,
    /// While attached, the carrier has no Retreat Cost. Read by
    /// `effective_retreat_cost`. `Magnetic Metal Energy`.
    CarrierHasNoRetreatCost,
    /// While attached, the carrier cannot be affected by any Special
    /// Condition, and sheds every one it has the moment this card is
    /// attached. Read by `inflict` and by the attach step.
    /// `Bubbly Water Energy`.
    CarrierImmuneToSpecialConditions,
    /// A standing effect, not a choice: an attack by the carrier does
    /// this much more damage to the opponent's Active, before Weakness
    /// and Resistance, but only while the carrier's own type matches
    /// the type this card provides. Read in `damage_dealt_with`
    /// alongside the attacker's Tool bonuses. `Voltaic Lightning
    /// Energy`.
    CarrierAttacksHitOpponentActiveHarder(u32),
    /// A standing effect, not a choice: while the carrier sits on its
    /// owner's Bench, no attack from the opponent's Pokémon does damage
    /// to it — but only while the carrier's own type matches the type
    /// this card provides. Read by `bench_attack_damage_blocked`.
    /// `Shadowy Darkness Energy`.
    PreventsBenchDamageWhileCarrierTypeMatches,
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

    pub fn as_energy(&self) -> Option<&Energy> {
        match self {
            CardDef::Energy(e) => Some(e),
            CardDef::Pokemon(_) | CardDef::Trainer(_) => None,
        }
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
