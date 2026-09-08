//! The whole game, as one value.

use crate::card::{CardDb, CardDef, Condition, Pokemon, Type};
use crate::ids::{CardDefId, CardId, PlayerId, PokemonId};
use crate::rng::{Rng, shuffle};

/// Whether a condition rotates the card, which is what makes Asleep,
/// Confused, and Paralyzed exclude each other.
fn rotates(condition: Condition) -> bool {
    matches!(
        condition,
        Condition::Asleep | Condition::Confused | Condition::Paralyzed
    )
}

/// The bench holds 5.
pub const BENCH_LIMIT: usize = 5;
/// Each player sets 6 Prize cards aside.
pub const PRIZE_COUNT: usize = 6;
/// The opening hand.
pub const OPENING_HAND: usize = 7;

/// One physical card. `owner` never changes: a knocked-out Pokémon and its
/// attachments go to *its owner's* discard, not the opponent's.
#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub def: CardDefId,
    pub owner: PlayerId,
}

/// One Pokémon in play, with what has happened to it.
#[derive(Debug, Clone)]
pub struct PokemonInPlay {
    /// Every card this Pokémon has been, oldest first: the Basic it was
    /// placed as, then each evolution played on top of it. A knockout
    /// discards the whole stack together (rule 38); the current name, HP, and
    /// attacks read from the last one.
    pub cards: Vec<CardId>,
    pub owner: PlayerId,
    /// Damage in points, not counters. A counter is 10 points.
    pub damage: u32,
    pub attached: Vec<CardId>,
    /// The turn this Pokémon came into play. Evolution reads it; Milestone 1
    /// records it so the rule has somewhere to land.
    pub played_on_turn: u32,
    /// The Special Conditions on this Pokémon. Only the Active carries any.
    pub conditions: Vec<Condition>,
    pub knocked_out: bool,
    /// Set when this Pokémon was devolved this turn (`Strange Timepiece`).
    /// Cleared in `begin_turn`, the same "this turn" lifetime `turn_bonus`
    /// already carries.
    pub cannot_evolve_this_turn: bool,
}

impl PokemonInPlay {
    /// The card that says what this Pokémon currently is.
    pub fn top_card(&self) -> CardId {
        *self
            .cards
            .last()
            .expect("a Pokémon in play is at least one card")
    }
}

/// Something a player or a Pokémon may do only once in a turn. Rule 13 sets
/// most of them; rule 20 sets the last, which belongs to a Pokémon rather
/// than to a player, and that is why a limit names its own owner.
///
/// A once-per-game limit would be the same shape with a list that is never
/// cleared. Nothing needs one: `Legacy Energy` is the only card in the pool
/// that reads "once per game", and it is a special Energy the engine refuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Limit {
    EnergyAttached(PlayerId),
    Retreated(PlayerId),
    SupporterPlayed(PlayerId),
    StadiumPlayed(PlayerId),
    Evolved(PokemonId),
    /// A Stadium's own once-a-turn action — `Academy at Night`,
    /// `Team Rocket's Factory`, `Lumiose City`. Only one Stadium is ever in
    /// play, so one variant covers whichever it is.
    StadiumEffectUsed(PlayerId),
    /// An Ability named "You can't use more than 1 [Name] Ability each
    /// turn" — keyed by the player using it and the Ability's own
    /// name, not by which Pokémon carries it, since the restriction is
    /// printed to cover every copy the player controls at once.
    AbilityUsed(PlayerId, &'static str),
}

/// One player's zones.
#[derive(Debug, Clone)]
pub struct PlayerState {
    /// The draw pile. `Deck` is the tournament decklist, so the pile in play
    /// is the Library.
    pub library: Vec<CardId>,
    pub hand: Vec<CardId>,
    pub discard: Vec<CardId>,
    pub prizes: Vec<CardId>,
    pub active: Option<PokemonId>,
    pub bench: Vec<PokemonId>,
}

impl PlayerState {
    fn empty() -> PlayerState {
        PlayerState {
            library: Vec::new(),
            hand: Vec::new(),
            discard: Vec::new(),
            prizes: Vec::new(),
            active: None,
            bench: Vec::new(),
        }
    }

    /// Every Pokémon in play, Active first.
    pub fn in_play(&self) -> Vec<PokemonId> {
        let mut all = Vec::new();
        all.extend(self.active);
        all.extend(self.bench.iter().copied());
        all
    }
}

/// What the engine is waiting for.
///
/// Setup is four phases, not a function that runs to completion. Every choice
/// the rulebook gives a player at setup is an ordinary legal action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// The coin flip landed and this player chooses who takes the first turn.
    ChoosingWhoGoesFirst { winner: PlayerId },
    /// This player may draw one card for each mulligan the opponent took.
    TakingBonusDraws { player: PlayerId, remaining: usize },
    /// This player places a Basic from hand face down as their Active.
    PlacingActive { player: PlayerId },
    /// This player fills the Bench, and stops when they choose to.
    PlacingBench { player: PlayerId },
    /// This player resolves their own between-turn effects, in the order they
    /// choose (rule 47).
    Checkup { player: PlayerId },
    /// This player is retreating and chooses which Energy pays the cost.
    DiscardingForRetreat {
        player: PlayerId,
        /// Where the Active goes once the cost is paid.
        to: PokemonId,
        /// How many Energy the player still owes.
        remaining: u8,
    },
    /// The current player is taking their turn.
    Main,
    /// A Pokémon owned by `of` must be chosen from `of`'s Bench to become
    /// their new Active, and `chooser` picks it. It interrupts whoever's turn
    /// it is. After a knockout, `of == chooser`: you promote your own. A card
    /// that switches an opponent's Active makes them differ. `then` is
    /// `Some` only for a live switch with a follow-up — a knockout and
    /// `Boss's Orders` both carry `None`.
    Promoting {
        of: PlayerId,
        chooser: PlayerId,
        then: Option<crate::card::PromoteFollowUp>,
    },
    /// A Trainer effect is moving up to `remaining` cards matching `filter`
    /// from one zone to another, `chooser`'s choice each time. Most of the
    /// primitives in `docs/architecture/effects.md` share this one phase — a
    /// deck search, a discard-pile recovery, a hand thinned for a bigger
    /// draw are all an instance of it with different zones.
    Deciding {
        chooser: PlayerId,
        /// The Trainer being resolved, and the slot of it the search is on.
        /// The phase names the card rather than carrying a copy of its
        /// slots, the same way `Paying` does, so the next slot is read back
        /// from the card when this one ends.
        card: CardId,
        step: u32,
        from: crate::card::Zone,
        /// The slot's own destination, filter, and what is left of its
        /// limit, held inline because every choice reads them.
        to: crate::card::Destination,
        filter: crate::card::CardFilter,
        excludes_type_of_previous: bool,
        /// Read only the top this-many cards of `from`, not the whole zone.
        /// `None` for every search before `Pokégear 3.0`.
        peek: Option<u32>,
        remaining: u32,
        /// How many have been taken so far, across every slot. `then` reads
        /// it when the search ends; nothing else needs it.
        moved: u32,
        /// The last card a slot took, across the whole search — not reset
        /// between slots. `excludes_type_of_previous` is the only reader.
        previous: Option<CardId>,
        /// What to do, beyond the moves themselves, once the search ends.
        then: Option<crate::card::Then>,
    },
    /// `player` is paying what a card demanded before it may be played, and
    /// chooses which cards from hand pay it. `card` is the Trainer already
    /// played and discarded: its effect runs once the cost is paid, and the
    /// phase names the card rather than a copy of the effect.
    Paying {
        player: PlayerId,
        card: CardId,
        /// How many cards the player still owes.
        remaining: u32,
    },
    /// `player` moves one Energy between two Pokémon they control, and
    /// chooses both ends at once. A separate phase for each end would record
    /// a half-made move that no rule can read; the pair is small enough to
    /// enumerate, so the action carries both.
    MovingEnergy { player: PlayerId },
    /// `player` played `N's Plan` and moves up to `remaining` Energy from
    /// their Bench onto their Active, one at a time. The target is fixed —
    /// always the Active — so unlike `MovingEnergy` the action need only
    /// name the card, and ADR 0012 gives the player their own choice to
    /// stop before `remaining` runs out.
    MovingEnergyFromBenchToActive { player: PlayerId, remaining: u32 },
    /// `player` played a card whose own text branches into two named
    /// effects, and picks one. `card` names the Trainer, the same
    /// continuation `Deciding` and `Paying` already use, so the two
    /// effects live only in the card's own definition, never copied here.
    ChoosingOneOf { player: PlayerId, card: CardId },
    /// `player` played a card that heals a chosen Pokémon in play, and
    /// picks which one. `amount` is how much it heals — `Pokémon Center
    /// Lady` always names the same number, but the phase carries it so a
    /// second card healing a different amount needs no new phase.
    HealingChosen { player: PlayerId, amount: u32 },
    /// `player` played `Rare Candy` and chooses both the Stage 2 from hand
    /// and the Basic in play it evolves, skipping the Stage 1 between them.
    /// A card and a target are chosen together, the same reason
    /// `MovingEnergy` carries both at once: a card with nowhere to go, or a
    /// Pokémon nothing can evolve onto, is not a half-made choice worth its
    /// own step. `legal_actions` guaranteed a matching pair exists before
    /// the card could be played, so no action here declines.
    EvolvingWithRareCandy { player: PlayerId },
    /// `chooser` picks one Energy attached to a Pokémon `of` controls, in
    /// play, to discard. Crushing Hammer's heads case is the only card that
    /// needs this; a Pokémon's attachments are not a `Zone`, so `Deciding`
    /// cannot express it.
    DiscardingOpponentEnergy { chooser: PlayerId, of: PlayerId },
    /// The narrower mirror of `DiscardingOpponentEnergy`, offering
    /// only a Special Energy — one carrying an effect. `Enhanced
    /// Hammer`.
    DiscardingOpponentSpecialEnergy { chooser: PlayerId, of: PlayerId },
    /// `player`'s own Bench holds more than `BENCH_LIMIT` and must
    /// shrink to it — `Area Zero Underdepths`, either because
    /// `player`'s last Tera Pokémon just left play, or because the
    /// card itself did. `then`, when `Some`, names the other player,
    /// who discards down next once `player` finishes — only when
    /// this card itself left play, never the reactive case, which
    /// touches one player alone.
    DiscardingBenchDownTo { player: PlayerId, then: Option<PlayerId> },
    /// `chooser` discards up to `remaining` cards matching `filter` from
    /// `of`'s hand — the same shape as `DiscardingOpponentEnergy`, over a
    /// hand instead of a Pokémon's attachments. `chooser == of` is
    /// `Xerosic's Machinations`: the opponent discards their own hand,
    /// their own choice. `chooser != of` is `Eri`: the player chooses what
    /// to take from the opponent's.
    DiscardingFromHand {
        chooser: PlayerId,
        of: PlayerId,
        filter: crate::card::CardFilter,
        remaining: u32,
        /// What runs once this discard ends. `Hand Trimmer` chains a
        /// second `DiscardingFromHand` this way — the opponent discards
        /// down to a size, then the player does the same to their own
        /// hand — the same "name the next step" shape `Phase::Promoting`
        /// already carries in `then`.
        then: Option<crate::card::DiscardFollowUp>,
    },
    /// `player` played `Wally's Compassion` and picks a Mega Evolution ex
    /// they control to heal fully. A prize value of 3 is what a Mega
    /// Evolution ex is, per ADR 0010 — the same fact `TurnBonusTarget`
    /// already reads for a plain ex, one prize lower.
    HealingMegaEx { player: PlayerId },
    /// `player` played `Dusk Ball` and is looking at the bottom of their
    /// Library. `legal_actions` reads the bottom cards straight off the
    /// zone, the same way a `peek`ed `Decide` reads the top without
    /// storing anything here — the Library itself is the source of
    /// truth, not a copy of it a `Copy` `Phase` could not hold anyway.
    LookingAtBottomOfLibrary { player: PlayerId, count: u32 },
    /// `player` played `Strange Timepiece` and is devolving one of their
    /// own evolved Pokémon. `target` is `None` until chosen; once fixed,
    /// the player removes evolution cards one at a time — "any number" —
    /// until they stop or the Pokémon is back to its Basic.
    Devolving {
        player: PlayerId,
        target: Option<PokemonId>,
    },
    /// `player` played `Transformation Tome` and is swapping a Basic in
    /// play for one in their discard, keeping the same `PokemonInPlay` —
    /// damage, attachments, conditions, and `played_on_turn` all stay,
    /// only the card identifying what the Pokémon is changes.
    SwappingIdentity {
        player: PlayerId,
        target: Option<PokemonId>,
    },
    /// `Handheld Fan` triggered mid-attack: `chooser` (the Tool's owner)
    /// moves one Energy off `attacker` onto one of the attacker's own
    /// Benched Pokémon. Opened from inside `attack`, before `settle` runs
    /// at all — "even if Knocked Out" means the trigger cannot wait for
    /// a knockout to be decided first.
    MovingEnergyForHandheldFan {
        chooser: PlayerId,
        attacker: PokemonId,
    },
    /// `player`'s turn just ended, and their Active carries `Powerglass`.
    /// They may attach one Basic Energy from their own discard to it, or
    /// decline — opened from `settle` itself, before the checkup queues.
    AttachingFromDiscardForPowerglass { player: PlayerId },
    /// `player` used an attack that puts damage counters on the
    /// opponent's Bench "in any way you like" — placed one at a time,
    /// any combination, until `remaining` reaches zero or the Bench
    /// runs out of room to matter (never happens; a counter can always
    /// stack). `Dragapult ex`'s `Phantom Dive`.
    DistributingDamageCounters { player: PlayerId, remaining: u32 },
    /// `player` used an attack that discards its own Energy and then
    /// deals flat damage to one Benched Pokémon of the opponent's
    /// choosing which. `N's Darmanitan`'s `Flamebody Cannon`.
    ChoosingBenchDamageTarget { player: PlayerId, damage: u32 },
    /// `player` used an attack that searches the library for up to
    /// `remaining` more Basic Pokémon to the Bench — a search read
    /// from an attack, not a Trainer's `Decide`, so it names no card
    /// to read slots back from. `Drilbur`/`Toxel`'s `Call for Family`.
    SearchingLibraryForBasics { player: PlayerId, remaining: u32 },
    /// `player` attached a Special Energy that searches the library
    /// for up to `remaining` Basic Pokémon of `kind` to the Bench.
    /// `Telepathic Psychic Energy`.
    SearchingLibraryForBasicsOfType { player: PlayerId, kind: crate::card::Type, remaining: u32 },
    /// `player` used an attack that searches the entire library for
    /// an Item card to take into hand. `Patrat`'s `Procurement`.
    SearchingLibraryForItem { player: PlayerId },
    /// `player` used an attack that searches the entire library for
    /// up to `remaining` cards of any kind to take into hand.
    /// `Noctowl`'s `Talon Hunt`.
    SearchingLibraryForAnyCards { player: PlayerId, remaining: u32 },
    /// `player` used an attack that may move Energy off the opponent's
    /// Active into the opponent's hand, up to `remaining` more.
    /// `Slowking`'s `Wash the Slate Clean`.
    MovingOpponentsActiveEnergyToHand { player: PlayerId, remaining: u32 },
    /// `player` used an attack that puts a Trainer card from their own
    /// discard pile into their hand. `Dedenne`'s
    /// `Electromagnetic Sonar`.
    TakingTrainerFromDiscard { player: PlayerId },
    /// `player` used an attack that searches the library for a card to
    /// evolve the attacker into directly. `Dwebble`'s `Ascension`.
    SearchingLibraryToEvolveSelf { player: PlayerId, target: PokemonId },
    /// `player` used an attack that puts a Pokémon card from their own
    /// discard pile into their hand. `Slowpoke`'s `Dangle Tail`.
    TakingPokemonFromDiscard { player: PlayerId },
    /// `player` used an attack that may shuffle a fixed count of the
    /// attacker's own Energy into the library for bench damage.
    /// `Wellspring Mask Ogerpon ex`'s `Torrential Pump`.
    DecidingToShuffleEnergyForBenchDamage {
        player: PlayerId,
        attacker: PokemonId,
        count: u32,
        damage: u32,
    },
    /// `chooser` moves one Energy between two of `of`'s own Pokémon,
    /// choosing both ends at once — the same shape `MovingEnergy`
    /// already takes on a player's own board, split into two fields
    /// since here the chooser and the board's owner differ.
    /// `Elgyem`'s `Slight Shift`.
    MovingOpponentsEnergy { chooser: PlayerId, of: PlayerId },
    /// `player` just played `pokemon` from hand onto the Bench, and
    /// its own Ability may search the library for a Supporter card.
    /// `Meowth ex`'s `Last-Ditch Catch`.
    DecidingToUseLastDitchCatch { player: PlayerId, pokemon: PokemonId },
    /// `player` just evolved from hand, and the evolution's own
    /// Ability may draw `count` cards. `Kadabra`'s and `Alakazam`'s
    /// `Psychic Draw`.
    DecidingToUsePsychicDraw { player: PlayerId, name: &'static str, count: u32 },
    /// `player` used an attack that deals flat damage to one of the
    /// opponent's Pokémon of their choosing, Active or Benched alike.
    /// `Fezandipiti ex`'s `Cruel Arrow`.
    ChoosingAnyOpponentPokemonDamageTarget { player: PlayerId, damage: u32 },
    /// The narrower mirror of `ChoosingAnyOpponentPokemonDamageTarget`,
    /// offering only the opponent's Benched Pokémon ex. `Shaymin`'s
    /// `Pinpoint Dive`.
    ChoosingBenchedExDamageTarget { player: PlayerId, damage: u32 },
    /// `player` used an attack that searches the deck for an Energy
    /// card to attach to a chosen own Benched Pokémon of this type.
    /// `Shaymin`'s `Send Flowers`.
    SearchingForEnergyToAttachToBenchedOfType { player: PlayerId, kind: crate::card::Type },
    /// `player` opened `pokemon`'s own Ability that moves up to
    /// `limit` damage counters from an own Pokémon to an opponent's.
    /// `Munkidori`'s `Adrena-Brain`.
    MovingDamageCountersFromOwnToOpponent { player: PlayerId, pokemon: PokemonId, limit: u32 },
    /// `player` opened `pokemon`'s own Ability that peeks at the top
    /// `count` cards of the library — still in place, read fresh by
    /// both `legal_actions` and the taking action, since nothing else
    /// can change the library while this phase is open. `Drakloak`'s
    /// `Recon Directive`.
    LookingAtTopCardsToTakeOne { player: PlayerId, pokemon: PokemonId, count: u32 },
    /// `player` opened `pokemon`'s own Ability that peeks at the top
    /// of the library and may attach found Energy of `kind` one card
    /// at a time; `remaining` is how many of the original peek are
    /// still unresolved — read fresh from the library's own top
    /// slice, the same way `LookingAtTopCardsToTakeOne` is.
    /// `Metang`'s `Metal Maker`.
    ResolvingEnergyFoundInTopPeek {
        player: PlayerId,
        pokemon: PokemonId,
        kind: crate::card::Type,
        remaining: u32,
    },
    /// `player` opened `pokemon`'s own Ability that peeks at the top
    /// `count` cards of the library for a Supporter to take, the rest
    /// shuffling back — read fresh from the library's own top slice,
    /// the same way `LookingAtTopCardsToTakeOne` is. `Tatsugiri`'s
    /// `Attract Customers`.
    LookingAtTopCardsForSupporter { player: PlayerId, pokemon: PokemonId, count: u32 },
    /// `player` opened `pokemon`'s own Ability that attaches Energy
    /// from hand as the effect itself. `Teal Mask Ogerpon ex`'s
    /// `Teal Dance`.
    DecidingToUseTealDance { player: PlayerId, pokemon: PokemonId },
    /// `player` opened `pokemon`'s own Ability that damages a chosen
    /// opponent Pokémon and then Knocks itself out. `Dusclops`'s and
    /// `Dusknoir`'s `Cursed Blast`.
    DecidingCursedBlastTarget { player: PlayerId, pokemon: PokemonId, damage: u32 },
    /// `player` used an Ability that searches the library for up to
    /// `remaining` more Evolution Pokémon of `kind` to hand.
    /// `Genesect ex`'s `Protect Charge`.
    SearchingLibraryForEvolutionPokemonOfType {
        player: PlayerId,
        pokemon: PokemonId,
        kind: crate::card::Type,
        remaining: u32,
    },
    /// `player` opened `pokemon`'s own Ability that attaches a Basic
    /// Energy from the discard pile to a Pokémon of the player's
    /// choosing. `Blaziken ex`'s `Seething Spirit`.
    DecidingToUseSeethingSpirit { player: PlayerId, pokemon: PokemonId },
    /// `player` used an attack that puts one of the attacker's own
    /// attached Energy into their hand. `Chien-Pao`'s `Icicle Loop`.
    ChoosingOwnEnergyToHand { player: PlayerId, attacker: PokemonId },
    /// `player` used an attack that moves one of the attacker's own
    /// attached Energy to a chosen own Benched Pokémon. `Shaymin`'s
    /// `Reflect Energy`.
    ChoosingEnergyAndBenchedTargetToMove { player: PlayerId, attacker: PokemonId },
    /// `player` just played `pokemon` from hand onto the Bench, and
    /// its own Ability may discard whichever Stadium is in play.
    /// `Chien-Pao`'s `Snow Sink`.
    DecidingToUseSnowSink { player: PlayerId, pokemon: PokemonId },
    /// `player` just played `pokemon` from hand onto the Bench, and
    /// its own Ability may switch it in for the Active.
    /// `Iron Leaves ex`'s `Rapid Vernier`.
    DecidingToSwitchInForRapidVernier { player: PlayerId, pokemon: PokemonId },
    /// `player` switched `pokemon` in and may now move any amount of
    /// Energy from their other Pokémon to it, one card at a time.
    MovingAnyEnergyForRapidVernier { player: PlayerId, pokemon: PokemonId },
    /// `player` used an Ability that searches for a Basic Energy of
    /// `kind` and attaches it to a Benched Pokémon of the same type,
    /// choosing which, then deals `damage` to it. `Toxtricity`'s
    /// `Sinister Surge`.
    SearchingForSinisterSurgeTarget { player: PlayerId, pokemon: PokemonId, kind: crate::card::Type, damage: u32 },
    /// `player` used an Ability that searches the library for up to
    /// `remaining` more Pokémon of a type at some HP or less, to
    /// hand. `Fan Rotom`'s `Fan Call`.
    /// `player` may switch a Benched Pokémon of `kind` (except one
    /// named `excluding`) with the Active, choosing which. `name`
    /// carries the Ability's own printed name, to spend
    /// `Limit::AbilityUsed` once a switch actually happens.
    /// `Pecharunt ex`'s `Subjugating Chains`.
    DecidingToUseSubjugatingChains {
        player: PlayerId,
        name: &'static str,
        kind: crate::card::Type,
        excluding: &'static str,
    },
    /// `player` played `Tool Scrapper` and may discard up to
    /// `remaining` more Pokémon Tools attached anywhere in play,
    /// either player's.
    DiscardingToolsAnywhere { player: PlayerId, remaining: u32 },
    SearchingForFanCall {
        player: PlayerId,
        pokemon: PokemonId,
        kind: crate::card::Type,
        hp: u32,
        remaining: u32,
    },
    /// `player` used an attack that searches their own discard pile
    /// for up to `remaining` more copies of a named Pokémon to the
    /// Bench. `Duskull`'s `Come and Get You`.
    SearchingDiscardForNamedToBench { player: PlayerId, name: &'static str, remaining: u32 },
    /// `player` played `Janine's Secret Art` and is choosing up to 2 of
    /// their own Darkness Pokémon, in `chosen`, before any search runs.
    ChoosingJaninesTargets {
        player: PlayerId,
        remaining: u32,
        chosen: [Option<PokemonId>; 2],
    },
    /// One search per target `Janine's Secret Art` chose, run in order.
    /// `index` says which of `targets` this search attaches to;
    /// `attached_to_active` remembers whether the Active has received one
    /// yet, read once both searches are done to decide the Poison.
    JaninesSearch {
        player: PlayerId,
        targets: [Option<PokemonId>; 2],
        index: u8,
        attached_to_active: bool,
    },
    /// The game is decided.
    Over,
}

/// Why the game ended. Rule 44: taking the last Prize wins even when another
/// condition lands at the same moment, so the reasons are checked in order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WinReason {
    /// The winner took their last Prize card.
    AllPrizesTaken,
    /// The loser had no Pokémon to promote.
    NoPokemonInPlay,
    /// The loser could not draw at the start of their turn.
    CouldNotDraw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Outcome {
    pub winner: PlayerId,
    pub reason: WinReason,
}

pub struct GameState {
    pub db: CardDb,
    pub cards: Vec<Card>,
    pub pokemon: Vec<PokemonInPlay>,
    pub players: [PlayerState; 2],
    pub turn_number: u32,
    pub current: PlayerId,
    pub phase: Phase,
    /// An attack ends the turn, but a knockout it caused is settled first.
    /// The flag remembers that the turn still owes its ending.
    pub pending_end_turn: bool,
    /// The between-turn effects still to resolve, oldest first.
    pub checkup_pending: Vec<(PlayerId, PokemonId, Condition)>,
    /// The checkup has finished and the next turn is owed.
    pub pending_turn_start: bool,
    /// Setup bookkeeping: the bonus draws each player has not yet taken or
    /// declined, and whether they have finished their Bench.
    pub bonus_draws: [usize; 2],
    pub bench_placed: [bool; 2],
    pub outcome: Option<Outcome>,
    /// The Stadium in play and who played it. Rule 58: one at a time, and a
    /// new one discards the old to its own owner's pile — which is why the
    /// player is kept beside the card.
    pub stadium: Option<(PlayerId, CardId)>,
    /// The once-per-turn limits spent so far. Cleared when a turn begins.
    pub spent: Vec<Limit>,
    /// Whether a Pokémon this player owns was Knocked Out during the turn
    /// that just ended. Set when a knockout happens; cleared for a player
    /// when their own turn ends, so it reads true for exactly the one turn
    /// after the knockout and never again. `Unfair Stamp` is the only
    /// reader.
    pub knocked_out_last_turn: [bool; 2],
    /// Whether this player played a Supporter whose name holds "Team
    /// Rocket" from their hand, this turn. Cleared at `begin_turn`, the
    /// same "this turn" lifetime `turn_bonus` already carries.
    /// `Team Rocket's Factory` is the only reader.
    pub played_a_team_rocket_supporter_this_turn: [bool; 2],
    /// A restriction an attack granted "during your opponent's next
    /// turn" — a lifetime that outlives one `begin_turn`, unlike
    /// `turn_bonus`: it must survive the boundary into the target's own
    /// next turn, then clear at the one after that. Cleared in
    /// `begin_turn`, but only once it is the *granting* player's turn
    /// again — see the check there for why.
    /// The third field names who granted it — the player whose turn
    /// it was at the moment this was set, always `attacker`'s owner
    /// since every grant happens mid-attack. Explicit rather than
    /// inferred from the target's own owner, since a self-targeted
    /// restriction (`CoinFlipSelfInvulnerableNextTurn`,
    /// `SelfDamageReductionNextTurn`) names the granting player's own
    /// Pokémon as its target — inferring from target ownership alone
    /// would clear those the instant the granting turn ends, one turn
    /// too early.
    pub opponent_next_turn_restriction: Option<(PokemonId, crate::card::AttackEffect, PlayerId)>,
    /// The mirror of `opponent_next_turn_restriction`: a restriction on
    /// the *attacker's own* very next turn, granted mid-turn (so it must
    /// not apply to the turn granting it). `armed` becomes `true` the
    /// first `begin_turn` after granting — the opponent's turn starting
    /// — marking that the *next* time it is the target owner's own turn
    /// is the one restricted turn; the field clears at the `begin_turn`
    /// after that one ends.
    pub own_next_turn_restriction: Option<(PokemonId, crate::card::AttackEffect, bool)>,
    /// A bonus this turn's attacks carry, set by a card such as `Black
    /// Belt's Training`. Cleared at `begin_turn`, the same as `spent` —
    /// "this turn" ends there regardless of whose turn is starting.
    pub turn_bonus: Option<(u32, crate::card::TurnBonusTarget)>,
    /// Set by `Briar`: this turn only, if the opponent's Active is
    /// Knocked Out by damage from an attack used by this player's own
    /// Tera Pokémon, that player takes one more Prize. Cleared at
    /// `begin_turn`, the same "this turn" lifetime `turn_bonus`
    /// already carries.
    pub bonus_prize_if_own_tera_attacker_knocks_out: Option<PlayerId>,
    /// The defender an attack that just resolved was aimed at, if any.
    /// `settle`'s knockout check takes this — reads and clears it, every
    /// call, whether or not it matches a Pokémon that actually died —
    /// so `Lillie's Pearl` can tell a knockout an attack just caused from
    /// one a later checkup causes, without this leaking into a settle
    /// call an attack did not just precede.
    pub attacking_defender: Option<PokemonId>,
    pub rng: Box<dyn Rng>,
    /// What happened, in order, as prose for a reader. It is not the record
    /// a replay reads — that is [`GameState::history`].
    pub log: Vec<String>,
    /// Every action applied to this game, in order. A refused action is not
    /// here: it changed nothing, so there is nothing to replay. With the
    /// seed and the decklists, this is the whole game.
    pub history: Vec<crate::action::Action>,
}

impl GameState {
    /// Deal a new game and stop at the first choice.
    ///
    /// Shuffling, drawing, and mulliganing are not choices, so they happen
    /// here. Everything the rulebook lets a player decide — who goes first,
    /// the bonus draws, where each Pokémon goes — is a setup phase, and the
    /// caller drives it through [`crate::action::legal_actions`].
    pub fn new(db: CardDb, decklists: [Vec<CardDefId>; 2], rng: Box<dyn Rng>) -> GameState {
        let mut cards = Vec::new();
        let mut players = [PlayerState::empty(), PlayerState::empty()];

        for (slot, decklist) in decklists.iter().enumerate() {
            let owner = if slot == 0 {
                PlayerId::One
            } else {
                PlayerId::Two
            };
            for def in decklist {
                let id = CardId(cards.len() as u32);
                cards.push(Card { def: *def, owner });
                players[slot].library.push(id);
            }
        }

        let mut state = GameState {
            db,
            cards,
            pokemon: Vec::new(),
            players,
            turn_number: 0,
            current: PlayerId::One,
            phase: Phase::Main,
            pending_end_turn: false,
            checkup_pending: Vec::new(),
            pending_turn_start: false,
            bonus_draws: [0, 0],
            bench_placed: [false, false],
            outcome: None,
            stadium: None,
            spent: Vec::new(),
            knocked_out_last_turn: [false, false],
            played_a_team_rocket_supporter_this_turn: [false, false],
            opponent_next_turn_restriction: None,
            own_next_turn_restriction: None,
            turn_bonus: None,
            bonus_prize_if_own_tera_attacker_knocks_out: None,
            attacking_defender: None,
            rng,
            log: Vec::new(),
            history: Vec::new(),
        };

        let mulligans = [
            state.deal_opening_hand(PlayerId::One),
            state.deal_opening_hand(PlayerId::Two),
        ];

        // Rule 8: each mulligan the opponent took is worth one extra card, and
        // the player may take it or leave it.
        for player in [PlayerId::One, PlayerId::Two] {
            state.bonus_draws[player.index()] = mulligans[player.opponent().index()];
        }

        // Rule 5: the coin flip decides who chooses, not who starts.
        let winner = if state.rng.flip() {
            PlayerId::One
        } else {
            PlayerId::Two
        };
        state.log.push(format!("{winner:?} wins the coin flip."));
        state.phase = Phase::ChoosingWhoGoesFirst { winner };
        state
    }

    fn deal_opening_hand(&mut self, player: PlayerId) -> usize {
        let mut mulligans = 0;
        loop {
            let slot = player.index();
            shuffle(self.rng.as_mut(), &mut self.players[slot].library);
            for _ in 0..OPENING_HAND {
                self.draw(player);
            }
            if self.hand_has_basic(player) {
                self.log
                    .push(format!("{player:?} draws an opening hand of 7."));
                return mulligans;
            }
            // Rule 7: no Basic, so reveal, shuffle back, and draw again.
            let hand = std::mem::take(&mut self.players[slot].hand);
            self.players[slot].library.extend(hand);
            mulligans += 1;
            self.log.push(format!("{player:?} mulligans."));
        }
    }

    fn hand_has_basic(&self, player: PlayerId) -> bool {
        self.players[player.index()]
            .hand
            .iter()
            .any(|c| self.def_of(*c).is_basic_pokemon())
    }

    pub fn set_prizes(&mut self, player: PlayerId) {
        let slot = player.index();
        for _ in 0..PRIZE_COUNT {
            match self.players[slot].library.pop() {
                Some(card) => self.players[slot].prizes.push(card),
                None => break,
            }
        }
    }

    pub fn put_into_play(&mut self, player: PlayerId, card: CardId) -> PokemonId {
        let id = PokemonId(self.pokemon.len() as u32);
        self.pokemon.push(PokemonInPlay {
            cards: vec![card],
            owner: player,
            damage: 0,
            attached: Vec::new(),
            played_on_turn: self.turn_number,
            conditions: Vec::new(),
            knocked_out: false,
            cannot_evolve_this_turn: false,
        });
        id
    }

    pub fn remove_from_hand(&mut self, player: PlayerId, card: CardId) {
        let hand = &mut self.players[player.index()].hand;
        if let Some(at) = hand.iter().position(|c| *c == card) {
            hand.remove(at);
        }
    }

    /// Take the top card. `false` means the library was empty, which loses the
    /// game at the start of a turn (rule 12) and nowhere else.
    pub fn draw(&mut self, player: PlayerId) -> bool {
        let slot = player.index();
        match self.players[slot].library.pop() {
            Some(card) => {
                self.players[slot].hand.push(card);
                true
            }
            None => false,
        }
    }

    pub fn def_of(&self, card: CardId) -> &CardDef {
        self.db.get(self.cards[card.index()].def)
    }

    pub fn pokemon_def(&self, pokemon: PokemonId) -> &Pokemon {
        self.def_of(self.pokemon[pokemon.index()].top_card())
            .as_pokemon()
            .expect("a Pokémon in play is a Pokémon card")
    }

    pub fn pokemon(&self, id: PokemonId) -> &PokemonInPlay {
        &self.pokemon[id.index()]
    }

    pub fn player(&self, id: PlayerId) -> &PlayerState {
        &self.players[id.index()]
    }

    /// The Trainer effect of whichever Stadium is in play, if any. Read
    /// by both players alike — a Stadium is nobody's Pokémon, nobody's
    /// hand.
    pub fn stadium_effect(&self) -> Option<crate::card::TrainerEffect> {
        let (_, card) = self.stadium?;
        self.def_of(card).as_trainer().map(|t| t.effect.clone())
    }

    /// Whether `Forest of Vitality` lets this evolution skip rule 18-20's
    /// "in play since the start of the turn" check — both the target and
    /// the evolution card must print Grass.
    pub fn forest_of_vitality_applies(&self, target: PokemonId, evolution: &crate::card::Pokemon) -> bool {
        self.stadium_effect() == Some(crate::card::TrainerEffect::GrassCanEvolveTheTurnItIsPlayed)
            && self.pokemon_def(target).kind == crate::card::Type::Grass
            && evolution.kind == crate::card::Type::Grass
    }

    /// Whether `Jamming Tower` (or any Stadium with the same effect) is
    /// in play — every Tool read site checks this before reading what a
    /// Tool would otherwise say, rather than the Tool or its attachment
    /// being removed.
    pub fn tools_disabled(&self) -> bool {
        self.stadium_effect() == Some(crate::card::TrainerEffect::ToolsHaveNoEffect)
    }

    /// The HP this Pokémon actually has: the printed value, plus whatever
    /// an attached Tool like `Hero's Cape` adds, less whatever a Stadium
    /// like `Gravity Mountain` takes off, floored at zero.
    /// `pokemon_def(id).hp` stays the printed value — a card in a zone
    /// has nothing attached and no Stadium reads it, so only an in-play
    /// read needs this split, the same way `effective_retreat_cost`
    /// splits from the printed Retreat Cost.
    pub fn effective_hp(&self, id: PokemonId) -> u32 {
        let printed = self.pokemon_def(id).hp;
        let bonus: u32 = if self.tools_disabled() {
            0
        } else {
            self.pokemon(id)
                .attached
                .iter()
                .filter_map(|c| self.def_of(*c).as_trainer())
                .map(|t| match t.effect {
                    crate::card::TrainerEffect::IncreasesHp(amount) => amount,
                    _ => 0,
                })
                .sum()
        };
        // A Special Energy's own HP bonus, unlike a Tool's, is never
        // switched off by a Stadium like `Jamming Tower` — its printed
        // text names only Tools.
        let energy_bonus: u32 = self
            .pokemon(id)
            .attached
            .iter()
            .filter_map(|c| self.def_of(*c).as_energy())
            .map(|e| match e.effect {
                Some(crate::card::EnergyEffect::IncreasesCarrierHp(amount)) => amount,
                Some(
                    crate::card::EnergyEffect::DrawCardsOnAttachFromHand(_)
                    | crate::card::EnergyEffect::WhenAttachedToTypeSearchesBasicPokemonOfTypeToBench(
                        ..,
                    )
                    | crate::card::EnergyEffect::CountersAttackerOnDamageTakenWhileActive(_)
                    | crate::card::EnergyEffect::PreventsAttackEffectsOnCarrier
                    | crate::card::EnergyEffect::ReattachesAfterOwnDiscardByAttackEffect
                    | crate::card::EnergyEffect::ProvidesAnyTypeIfAttachedToBasic,
                )
                | None => 0,
            })
            .sum();
        let stadium_reduction = match self.stadium_effect() {
            Some(crate::card::TrainerEffect::ReducesHpForStage(stage, amount))
                if self.pokemon_def(id).stage == stage =>
            {
                amount
            }
            _ => 0,
        };
        (printed + bonus + energy_bonus).saturating_sub(stadium_reduction)
    }

    pub fn remaining_hp(&self, id: PokemonId) -> u32 {
        self.effective_hp(id).saturating_sub(self.pokemon(id).damage)
    }

    /// How many Energy cards are attached. Retreat pays by count, because a
    /// Retreat Cost is always Colorless.
    pub fn energy_attached(&self, id: PokemonId) -> u8 {
        self.attached_energy_types(id).len() as u8
    }

    /// Whether `id` has no Ability while `Team Rocket's Watchtower` (or
    /// any Stadium with the same effect) is in play. The card's own
    /// text names only `{C}` (Colorless) Pokémon — either player's,
    /// but never a Pokémon of any other type — so this reads `id`'s
    /// own printed type, not a single stadium-wide flag. Every site
    /// that would offer, trigger, or read a standing Ability effect
    /// checks this first, on the specific Pokémon that effect would
    /// come from, rather than the Ability being removed.
    pub fn abilities_disabled_for(&self, id: PokemonId) -> bool {
        self.stadium_effect() == Some(crate::card::TrainerEffect::AbilitiesDisabled)
            && self.pokemon_def(id).kind == crate::card::Type::Colorless
    }

    /// The Retreat Cost this Pokémon actually pays: the printed cost, less
    /// whatever an attached Tool like `Air Balloon` takes off, or zero
    /// outright under a Stadium like `N's Castle`. `pokemon_def(id).retreat_cost`
    /// stays the printed value — a card sitting in a zone has no Tool to
    /// read a reduction from, and no Stadium reads it either, so only an
    /// in-play read needs this split at all.
    pub fn effective_retreat_cost(&self, id: PokemonId) -> u32 {
        if let Some(crate::card::TrainerEffect::RemovesRetreatCostForNamePrefix(prefix)) =
            self.stadium_effect()
            && self.pokemon_def(id).name.starts_with(prefix)
        {
            return 0;
        }
        if self.pokemon_def(id).stage == crate::card::Stage::Basic {
            let owner = self.pokemon(id).owner;
            let has_skyliner = self.player(owner).in_play().iter().any(|p| {
                !self.abilities_disabled_for(*p)
                    && self.pokemon_def(*p).ability.is_some_and(|a| {
                        a.effect == crate::card::AbilityEffect::PassiveOwnBasicPokemonHaveNoRetreatCost
                    })
            });
            if has_skyliner {
                return 0;
            }
        }
        let printed = self.pokemon_def(id).retreat_cost as u32;
        let reduction: u32 = if self.tools_disabled() {
            0
        } else {
            self.pokemon(id)
                .attached
                .iter()
                .filter_map(|c| self.def_of(*c).as_trainer())
                .map(|t| match t.effect {
                    crate::card::TrainerEffect::ReducesRetreatCost(amount) => amount,
                    _ => 0,
                })
                .sum()
        };
        printed.saturating_sub(reduction)
    }

    /// Whether any Pokémon in play, on either side, carries
    /// `AbilityEffect::PassiveBlocksDamageCounterMovement`. `Patrat`'s
    /// `Watchful Eye`.
    pub fn damage_counter_movement_blocked(&self) -> bool {
        [PlayerId::One, PlayerId::Two].iter().any(|p| {
            self.player(*p).in_play().iter().any(|pokemon| {
                !self.abilities_disabled_for(*pokemon)
                    && self.pokemon_def(*pokemon).ability.is_some_and(|a| {
                        a.effect == crate::card::AbilityEffect::PassiveBlocksDamageCounterMovement
                    })
            })
        })
    }

    /// Whether any Pokémon in play, on either side, carries
    /// `AbilityEffect::PassiveDisablesSelfKnockOutAbilities`.
    /// `Psyduck`'s `Damp`.
    pub fn self_knockout_abilities_disabled(&self) -> bool {
        [PlayerId::One, PlayerId::Two].iter().any(|p| {
            self.player(*p).in_play().iter().any(|pokemon| {
                !self.abilities_disabled_for(*pokemon)
                    && self.pokemon_def(*pokemon).ability.is_some_and(|a| {
                        a.effect == crate::card::AbilityEffect::PassiveDisablesSelfKnockOutAbilities
                    })
            })
        })
    }

    /// Whether `id` carries `EnergyEffect::PreventsAttackEffectsOnCarrier`
    /// — every site that would apply an opponent's attack effect
    /// directly to a Pokémon checks this first. `Mist Energy`.
    pub fn attack_effects_on_it_prevented(&self, id: PokemonId) -> bool {
        self.pokemon(id).attached.iter().any(|c| {
            self.def_of(*c)
                .as_energy()
                .is_some_and(|e| e.effect == Some(crate::card::EnergyEffect::PreventsAttackEffectsOnCarrier))
        })
    }

    /// Whether `player` has a Pokémon carrying `Marker::Tera` in play,
    /// Active or Benched alike. `Area Zero Underdepths`, `Briar`,
    /// `Glass Trumpet`, `Nighttime Mine`.
    pub fn has_tera_in_play(&self, player: PlayerId) -> bool {
        self.player(player)
            .in_play()
            .iter()
            .any(|p| self.pokemon_def(*p).markers.contains(&crate::card::Marker::Tera))
    }

    /// How many Pokémon `player`'s Bench holds room for: `BENCH_LIMIT`
    /// (rule 14), or 8 while `Area Zero Underdepths` is in play and
    /// this player has a Tera Pokémon of their own in play.
    pub fn bench_limit(&self, player: PlayerId) -> usize {
        if self.stadium_effect() == Some(crate::card::TrainerEffect::TeraPokemonRaisesBenchLimit)
            && self.has_tera_in_play(player)
        {
            8
        } else {
            BENCH_LIMIT
        }
    }

    /// Whether `target`, sitting on its own owner's Bench, is
    /// protected from a damage counter that `by`'s Pokémon (an attack
    /// or an Ability effect) would place there directly. `Battle
    /// Cage` only ever blocks a *cross-side* placement — the ruling
    /// on Gardevoir ex's own `Psychic Embrace` confirms an effect
    /// still places its counters normally on its own side's Bench —
    /// so this reads `false` outright when `by` owns `target`. A
    /// Pokémon that is its owner's Active is never protected by this
    /// either way — Battle Cage names only the Bench, and ordinary
    /// attack damage is untouched regardless.
    pub fn bench_damage_counters_blocked(&self, by: PlayerId, target: PokemonId) -> bool {
        let owner = self.pokemon(target).owner;
        owner != by
            && self.player(owner).active != Some(target)
            && self.stadium_effect() == Some(crate::card::TrainerEffect::PreventsDamageCountersOnBench)
    }

    /// The cards in one of a player's zones. A Trainer effect moves between
    /// these; a Pokémon's attachments are not one of them.
    pub fn zone(&self, player: PlayerId, zone: crate::card::Zone) -> &Vec<CardId> {
        let side = self.player(player);
        match zone {
            crate::card::Zone::Hand => &side.hand,
            crate::card::Zone::Discard => &side.discard,
            crate::card::Zone::Library => &side.library,
        }
    }

    pub fn zone_mut(&mut self, player: PlayerId, zone: crate::card::Zone) -> &mut Vec<CardId> {
        let side = &mut self.players[player.index()];
        match zone {
            crate::card::Zone::Hand => &mut side.hand,
            crate::card::Zone::Discard => &mut side.discard,
            crate::card::Zone::Library => &mut side.library,
        }
    }

    /// Move one card from one zone to another, for the same player. Neither
    /// zone is a Pokémon's attachments; that is a different move entirely.
    pub fn move_card(
        &mut self,
        player: PlayerId,
        card: CardId,
        from: crate::card::Zone,
        to: crate::card::Zone,
    ) {
        let source = self.zone_mut(player, from);
        if let Some(at) = source.iter().position(|c| *c == card) {
            source.remove(at);
        }
        self.zone_mut(player, to).push(card);
    }

    /// Whether a card meets a Trainer effect's filter.
    pub fn matches_filter(&self, card: CardId, filter: crate::card::CardFilter) -> bool {
        use crate::card::CardFilter;
        match filter {
            CardFilter::AnyPokemon => self.def_of(card).as_pokemon().is_some(),
            CardFilter::PokemonWithoutRuleBox => self
                .def_of(card)
                .as_pokemon()
                .is_some_and(|p| p.prizes == 1),
            CardFilter::PokemonOrBasicEnergy => {
                let def = self.def_of(card);
                def.as_pokemon().is_some() || def.as_energy().is_some_and(|e| e.effect.is_none())
            }
            CardFilter::EvolutionPokemon => self
                .def_of(card)
                .as_pokemon()
                .is_some_and(|p| p.stage != crate::card::Stage::Basic),
            CardFilter::PokemonOfStage(stage) => self
                .def_of(card)
                .as_pokemon()
                .is_some_and(|p| p.stage == stage),
            // "Basic" excludes a Special Energy — its own printed
            // effect is what tells the two apart, the same field a
            // Special Energy needed once one could be admitted at all.
            CardFilter::BasicEnergy => self.def_of(card).as_energy().is_some_and(|e| e.effect.is_none()),
            CardFilter::PokemonEx => self.def_of(card).as_pokemon().is_some_and(|p| p.prizes > 1),
            CardFilter::BasicPokemonWithHpAtMost(hp) => self
                .def_of(card)
                .as_pokemon()
                .is_some_and(|p| p.stage == crate::card::Stage::Basic && p.hp <= hp),
            CardFilter::AnyTrainer => self.def_of(card).as_trainer().is_some(),
            CardFilter::BasicEnergyOfType(kind) => match self.def_of(card) {
                CardDef::Energy(energy) => energy.kind == kind && energy.effect.is_none(),
                CardDef::Pokemon(_) | CardDef::Trainer(_) => false,
            },
            CardFilter::TrainerOfKind(kind) => self
                .def_of(card)
                .as_trainer()
                .is_some_and(|t| t.kind == kind),
            CardFilter::PokemonOfTypeOrBasicEnergyOfType(kind) => match self.def_of(card) {
                CardDef::Pokemon(p) => p.kind == kind,
                CardDef::Energy(e) => e.kind == kind && e.effect.is_none(),
                CardDef::Trainer(_) => false,
            },
            CardFilter::AnyCard => true,
            CardFilter::PokemonWithoutRuleBoxOrBasicEnergy => {
                let def = self.def_of(card);
                def.as_pokemon().is_some_and(|p| p.prizes == 1)
                    || def.as_energy().is_some_and(|e| e.effect.is_none())
            }
            CardFilter::SupporterNameContains(word) => self
                .def_of(card)
                .as_trainer()
                .is_some_and(|t| t.kind == crate::card::TrainerKind::Supporter && t.name.contains(word)),
            CardFilter::PokemonNamed(name) => {
                self.def_of(card).as_pokemon().is_some_and(|p| p.name == name)
            }
            CardFilter::EvolutionPokemonOfType(kind) => self
                .def_of(card)
                .as_pokemon()
                .is_some_and(|p| p.stage != crate::card::Stage::Basic && p.kind == kind),
            CardFilter::PokemonOfTypeWithHpAtMost(kind, hp) => self
                .def_of(card)
                .as_pokemon()
                .is_some_and(|p| p.kind == kind && p.hp <= hp),
            CardFilter::BasicPokemonOfType(kind) => self
                .def_of(card)
                .as_pokemon()
                .is_some_and(|p| p.stage == crate::card::Stage::Basic && p.kind == kind),
            CardFilter::TeraPokemon => self
                .def_of(card)
                .as_pokemon()
                .is_some_and(|p| p.markers.contains(&crate::card::Marker::Tera)),
        }
    }

    /// Whether a Pokémon in play meets `Destination::Attach`'s target
    /// filter.
    pub fn matches_target(
        &self,
        player: PlayerId,
        pokemon: PokemonId,
        filter: crate::card::TargetFilter,
    ) -> bool {
        use crate::card::TargetFilter;
        match filter {
            TargetFilter::AnyInPlay => true,
            TargetFilter::BenchedNameStartsWith(prefix) => {
                self.player(player).bench.contains(&pokemon)
                    && self.pokemon_def(pokemon).name.starts_with(prefix)
            }
            TargetFilter::BenchedOfType(kind) => {
                self.player(player).bench.contains(&pokemon)
                    && self.pokemon_def(pokemon).kind == kind
            }
            TargetFilter::OfStage(stage) => {
                self.player(player).in_play().contains(&pokemon)
                    && self.pokemon_def(pokemon).stage == stage
            }
        }
    }

    /// Whether a card fills a search's slot: the slot's filter, and, when it
    /// excludes the previous slot's type, that this card's Energy type
    /// differs from the last one taken.
    pub fn matches_slot(
        &self,
        card: CardId,
        slot: &crate::card::Slot,
        previous: Option<CardId>,
    ) -> bool {
        if !self.matches_filter(card, slot.filter) {
            return false;
        }
        if !slot.excludes_type_of_previous {
            return true;
        }
        let Some(previous) = previous else {
            return true;
        };
        let kind_of = |card: CardId| match self.def_of(card) {
            crate::card::CardDef::Energy(e) => Some(e.kind),
            _ => None,
        };
        kind_of(card) != kind_of(previous)
    }

    /// The type each attached Energy provides, in the order attached.
    pub fn attached_energy_types(&self, id: PokemonId) -> Vec<Type> {
        self.pokemon(id)
            .attached
            .iter()
            .filter_map(|c| match self.def_of(*c) {
                CardDef::Energy(energy) => Some(energy.kind),
                CardDef::Pokemon(_) | CardDef::Trainer(_) => None,
            })
            .collect()
    }

    /// Whether the Energy attached to this Pokémon pays a cost.
    ///
    /// Every named type is matched first, because a Colorless entry takes any
    /// Energy and would otherwise eat the one Energy a named entry needed.
    /// A `Prism Energy` attached to a Basic Pokémon is a wildcard, tried
    /// only once every fixed-type card that could match a named entry is
    /// already spoken for — the same "named first" ordering, one layer
    /// deeper.
    pub fn pays_cost(&self, id: PokemonId, cost: &[Type]) -> bool {
        let carrier_is_basic = self.pokemon_def(id).stage == crate::card::Stage::Basic;
        let mut available = Vec::new();
        let mut wildcards = 0u32;
        for card in &self.pokemon(id).attached {
            let Some(energy) = self.def_of(*card).as_energy() else {
                continue;
            };
            if carrier_is_basic
                && energy.effect == Some(crate::card::EnergyEffect::ProvidesAnyTypeIfAttachedToBasic)
            {
                wildcards += 1;
            } else {
                available.push(energy.kind);
            }
        }
        for required in cost.iter().filter(|t| **t != Type::Colorless) {
            match available.iter().position(|kind| kind == required) {
                Some(at) => {
                    available.remove(at);
                }
                None if wildcards > 0 => wildcards -= 1,
                None => return false,
            }
        }
        let colorless = cost.iter().filter(|t| **t == Type::Colorless).count() as u32;
        available.len() as u32 + wildcards >= colorless
    }

    /// Clear the once-per-turn flags for whoever is about to play.
    pub fn begin_turn(&mut self) {
        // Every turn limit is cleared, not only the current player's. The
        // opponent's were unreadable during this turn anyway — every gate
        // asks about whoever is acting — so clearing all of them is the
        // same game and one line.
        self.spent.clear();
        // "This turn" ends here too, whoever set the bonus.
        self.turn_bonus = None;
        self.bonus_prize_if_own_tera_attacker_knocks_out = None;
        self.played_a_team_rocket_supporter_this_turn = [false, false];
        for pokemon in &mut self.pokemon {
            pokemon.cannot_evolve_this_turn = false;
        }
        // A restriction granted "during your opponent's next turn" is
        // cleared here only once it is the granting player's own turn
        // again — read from the explicit granting player, not the
        // target's owner (a self-targeted restriction names the
        // granting player's own Pokémon, so that inference would clear
        // it one turn too early).
        if let Some((_, _, granted_by)) = self.opponent_next_turn_restriction
            && self.current == granted_by
        {
            self.opponent_next_turn_restriction = None;
        }
        // The mirror, for a restriction on the attacker's own next
        // turn: arm it the first turn after granting (the opponent's,
        // which does not clear it — that only happens once armed), then
        // clear it once the target owner's one restricted turn ends.
        if let Some((target, effect, armed)) = self.own_next_turn_restriction {
            let is_targets_turn = self.pokemon[target.index()].owner == self.current;
            if armed && !is_targets_turn {
                self.own_next_turn_restriction = None;
            } else if !armed && !is_targets_turn {
                self.own_next_turn_restriction = Some((target, effect, true));
            }
        }
    }

    /// Whether a once-per-turn limit has been spent.
    pub fn is_spent(&self, limit: Limit) -> bool {
        self.spent.contains(&limit)
    }

    /// Spend a once-per-turn limit. Spending one twice is not an error; the
    /// gates that read it ask only whether it is spent.
    pub fn spend(&mut self, limit: Limit) {
        if !self.is_spent(limit) {
            self.spent.push(limit);
        }
    }

    pub fn has_condition(&self, id: PokemonId, condition: Condition) -> bool {
        self.pokemon(id).conditions.contains(&condition)
    }

    /// Whether `Festival Grounds` protects this Pokémon right now: the
    /// Stadium is in play, and it carries any Energy.
    pub fn immune_under_festival_grounds(&self, id: PokemonId) -> bool {
        self.stadium_effect()
            == Some(crate::card::TrainerEffect::EnergizedPokemonImmuneToSpecialConditions)
            && self
                .pokemon(id)
                .attached
                .iter()
                .any(|c| self.def_of(*c).is_energy())
    }

    /// Put a Special Condition on a Pokémon.
    ///
    /// Rule 55: Asleep, Confused, and Paralyzed all rotate the card, so the
    /// newest replaces whichever of them was there. Rule 56: Burned and
    /// Poisoned are independent of that and of each other. Rule 57: a second
    /// Burn or Poison replaces the first rather than stacking.
    pub fn inflict(&mut self, id: PokemonId, condition: Condition) {
        if self.immune_under_festival_grounds(id) {
            return;
        }
        if rotates(condition) {
            self.pokemon[id.index()].conditions.retain(|c| !rotates(*c));
        } else {
            self.remove_condition(id, condition);
        }
        self.pokemon[id.index()].conditions.push(condition);
    }

    /// Rule 27: a Pokémon that leaves the Active spot loses every condition.
    pub fn clear_conditions(&mut self, id: PokemonId) {
        self.pokemon[id.index()].conditions.clear();
    }

    pub fn remove_condition(&mut self, id: PokemonId, condition: Condition) {
        self.pokemon[id.index()]
            .conditions
            .retain(|c| *c != condition);
    }

    pub fn is_over(&self) -> bool {
        self.outcome.is_some()
    }

    /// Rule 15-18: the player going first draws, cannot play a Supporter, and
    /// skips their attack step.
    pub fn is_first_turn_of_game(&self) -> bool {
        self.turn_number == 0
    }
}
