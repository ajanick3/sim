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
    /// Attached straight to a Pokémon the chooser controls, filtered by
    /// `TargetFilter`, picked at the same time as the card. `Crispin` is why
    /// this exists at all: the second Energy it finds goes onto a Pokémon
    /// rather than into a zone. `TargetFilter::AnyInPlay` is what it needed.
    Attach(TargetFilter),
    /// On top of the Library, in the order the cards were taken — not
    /// shuffled in. `Ciphermaniac's Codebreaking` shuffles what is left of
    /// the deck *before* placing these, so the two land known, on top of an
    /// otherwise-scrambled deck.
    TopOfLibraryInOrder,
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
    /// A Pokémon ex, which is a Pokémon worth more than 1 Prize. The prize
    /// value is read from the name (ADR 0010), so in this pool a card worth
    /// more than 1 is exactly a card printed `ex`.
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
/// a Benched Pokémon.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnBonusTarget {
    /// A Pokémon ex. `Black Belt's Training`.
    OpponentActiveEx,
    /// A Pokémon without a Rule Box. `Gladion's Final Battle` — the
    /// opposite restriction `OpponentActiveEx` reads.
    OpponentActiveWithoutRuleBox,
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
    /// card from hand on top of the Library. `Academy at Night`.
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
    /// Shuffle the player's hand into their Library, then draw. A second
    /// count applies when they hold exactly 6 Prizes.
    ShuffleHandThenDraw { normal: u32, at_six_prizes: u32 },
    /// Both players shuffle their hand into their Library, then draw — the
    /// player who played the card drawing `you`, the opponent drawing
    /// `opponent`. `Judge` prints the same count for both; `Unfair Stamp`
    /// is why the two are named separately rather than shared.
    BothShuffleHandThenDraw { you: u32, opponent: u32 },
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
    /// Look at the bottom `n` cards of the Library; the player may take a
    /// Pokémon found there, then the rest shuffle back in. `Dusk Ball`
    /// reads the opposite end from every peeked search built so far,
    /// which all read the top — the end `draw` pops from.
    LookAtBottomOfLibrary { count: u32 },
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
    /// The attacker cannot use any attack during their own very next
    /// turn. `N's Zekrom`'s `Rampaging Thunder`.
    AttackerCannotAttackNextTurn,
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
