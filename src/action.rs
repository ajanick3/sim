//! What a player may do, and what the engine will accept.
//!
//! [`legal_actions`] is the engine's primary interface. A human interface
//! numbers the list and reads a choice; a bot is
//! `(state, legal_actions) -> Action`. Neither can reach a state the other
//! cannot, and neither can cheat by acting outside the list.

use crate::card::{Condition, Destination, Requirement, TrainerEffect, TrainerKind};
use crate::ids::{CardId, PlayerId, PokemonId};
use crate::state::{GameState, Limit, Phase};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Name the player who takes the first turn. The coin flip's winner
    /// chooses, and may choose the opponent.
    ChooseWhoGoesFirst { first: PlayerId },
    /// Take one of the cards the opponent's mulligans owe you.
    TakeBonusDraw,
    /// Leave the rest of them.
    DeclineBonusDraws,
    /// Place a Basic from hand face down as the Active.
    PlaceActive { card: CardId },
    /// Place a Basic from hand face down on the Bench.
    PlaceOnBench { card: CardId },
    /// Stop filling the Bench.
    FinishPlacing,
    /// Put a Basic Pokémon from hand onto the Bench.
    PlayBasic { card: CardId },
    /// Evolve a Pokémon in play with the card from hand that names it.
    Evolve { card: CardId, target: PokemonId },
    /// Attach an Energy from hand. Once per turn.
    AttachEnergy { card: CardId, target: PokemonId },
    /// Attach a Tool to a Pokémon in play. Immediate, like `AttachEnergy`
    /// — a Tool always names its target at play time, unlike an Item.
    PlayTool { card: CardId, target: PokemonId },
    /// Retreat the Active, promoting a Benched Pokémon. Once per turn.
    Retreat { to: PokemonId },
    /// Discard one attached Energy toward a Retreat Cost.
    DiscardEnergy { card: CardId },
    /// Resolve one of your own between-turn effects.
    ResolveCheckup {
        pokemon: PokemonId,
        condition: Condition,
    },
    /// Attack with the Active. The turn ends after it.
    Attack { index: usize },
    /// End the turn without attacking.
    EndTurn,
    /// Choose a new Active after a knockout.
    Promote { pokemon: PokemonId },
    /// Play a Trainer from hand.
    PlayTrainer { card: CardId },
    /// Take one matching card during a Trainer effect's resolution.
    TakeCard { card: CardId },
    /// Take one matching card during a Trainer effect's resolution, and
    /// attach it straight to a Pokémon in play. The slot's destination is
    /// `Destination::Attach`; only that case needs a target.
    TakeCardOnto { card: CardId, target: PokemonId },
    /// Stop taking cards during a Trainer effect's resolution.
    FinishDeciding,
    /// Discard one Energy attached to a Pokémon the opponent controls.
    DiscardOpponentEnergy { card: CardId },
    /// Discard one Special Energy attached to a Pokémon the opponent
    /// controls, as part of `Phase::DiscardingOpponentSpecialEnergy`.
    DiscardOpponentSpecialEnergy { card: CardId },
    /// Discard one Energy attached to `Phase::DiscardingDefenderEnergyForAttack`'s
    /// own target Pokémon.
    DiscardDefenderEnergyForAttack { card: CardId },
    /// Discard one Energy attached to `Phase::ChoosingOwnEnergyToDiscardForAttack`'s
    /// own attacker.
    DiscardOwnEnergyForAttack { card: CardId },
    /// Accept `Phase::DecidingToDiscardOwnEnergyForBonusDamage`'s
    /// bonus — grants it outright, moves to actually discarding.
    AcceptDiscardOwnEnergyForBonusDamage,
    /// Decline it — no bonus, no discard.
    DeclineDiscardOwnEnergyForBonusDamage,
    /// Discard one Energy as part of
    /// `Phase::ChoosingOwnEnergyToDiscardForBonusDamage`.
    DiscardOwnEnergyForBonusDamage { card: CardId },
    /// Stop discarding early, as part of the same phase.
    FinishDiscardingOwnEnergyForBonusDamage,
    /// Discard one Basic Energy from any of the player's own Pokémon,
    /// as part of `Phase::DiscardingAnyBasicEnergyForDamagePerCard`.
    DiscardBasicEnergyForDamagePerCard { card: CardId },
    /// Stop discarding, applying whatever damage the cards discarded
    /// so far earned.
    FinishDiscardingBasicEnergyForDamagePerCard,
    /// Discard this own Benched Pokémon, as part of
    /// `Phase::DiscardingBenchDownTo`. Not a Knockout: no Prize, no
    /// `knocked_out` flag.
    DiscardBenchedPokemon { pokemon: PokemonId },
    /// Discard one card from hand toward what a card demanded to be played.
    PayWithCard { card: CardId },
    /// Put one card from hand on the bottom of the deck toward what a
    /// card demanded to be played. `Kofu`.
    PayWithCardToBottomOfDeck { card: CardId },
    /// Move one attached Energy onto another Pokémon you control.
    MoveEnergy { card: CardId, target: PokemonId },
    /// Move one Energy from a Benched Pokémon onto the Active.
    MoveEnergyToActive { card: CardId },
    /// Stop moving Energy onto the Active before the limit is spent.
    FinishMovingEnergyToActive,
    /// Heal the chosen Pokémon, and clear its Special Conditions.
    HealTarget { target: PokemonId },
    /// Pick the first (`true`) or second (`false`) of a card's two named
    /// effects. Only the one picked ever runs.
    ChooseOption { first: bool },
    /// Discard one card from a hand `Phase::DiscardingFromHand` names.
    DiscardFromHand { card: CardId },
    /// Stop discarding from that hand before the limit is spent.
    FinishDiscardingFromHand,
    /// Heal every point of damage from a chosen Mega Evolution ex, and
    /// move its attachments to hand if the heal did anything.
    HealMegaEx { target: PokemonId },
    /// Take this Pokémon found at the bottom of the Deck.
    /// `Phase::LookingAtBottomOfDeck` names the search it ends.
    TakeFromBottomOfDeck { card: CardId },
    /// Decline every Pokémon `Phase::LookingAtBottomOfDeck` found; the
    /// Deck still shuffles.
    DeclineBottomOfDeck,
    /// Choose which of the player's own evolved Pokémon `Phase::Devolving`
    /// devolves.
    ChooseDevolveTarget { target: PokemonId },
    /// Remove one evolution card from the Pokémon `Phase::Devolving`
    /// named, into hand.
    RemoveOneEvolutionCard,
    /// Stop devolving. Legal any time a target is already chosen, even
    /// having removed nothing yet, since "any number" includes zero.
    FinishDevolving,
    /// Choose which Basic Pokémon in play `Phase::SwappingIdentity`
    /// swaps out.
    ChooseIdentitySwapTarget { target: PokemonId },
    /// Finish the swap `Phase::SwappingIdentity` names, using this Basic
    /// from the discard.
    SwapIdentityWithDiscarded { card: CardId },
    /// `Handheld Fan`'s move: this Energy off the attacker, onto this one
    /// of the attacker's own Benched Pokémon.
    MoveEnergyForHandheldFan { card: CardId, target: PokemonId },
    /// `Powerglass`: attach this Basic Energy from discard to the
    /// Pokémon it is attached to.
    AttachFromDiscardForPowerglass { card: CardId },
    /// Decline `Powerglass`'s attach.
    DeclinePowerglass,
    /// `Academy at Night`'s once-a-turn action: put this card from hand
    /// on top of the Deck.
    PutOnTopOfDeckForAcademyAtNight { card: CardId },
    /// `Team Rocket's Factory`'s once-a-turn action.
    DrawTwoForTeamRocketsFactory,
    /// `Lumiose City`'s once-a-turn search — opens the same `Deciding`
    /// phase a played card's own `Decide` effect would.
    UseLumioseCity,
    /// Choose one of up to 2 targets for `Janine's Secret Art`.
    ChooseJaninesTarget { target: PokemonId },
    /// Stop choosing targets, whether 0, 1, or 2 have been picked.
    FinishChoosingJaninesTargets,
    /// Take the Basic Darkness Energy found for the current target of
    /// `Janine's Secret Art`, attaching it there.
    TakeEnergyForJanine { card: CardId },
    /// Nothing was found, or the player declines: move on to the next
    /// target, or end the card if there is none.
    FinishJaninesSearch,
    /// Evolve a Basic in play straight into the named Stage 2 from hand,
    /// skipping the Stage 1 between them.
    EvolveSkippingOneStage { card: CardId, target: PokemonId },
    /// Evolve a Pokémon in play from a card found in the deck, not the
    /// hand — and, unlike every other evolution, with no restriction on
    /// the turn it or its target came into play. `Salvatore`.
    EvolveFromDeck { card: CardId, target: PokemonId },
    /// Choose the Pokémon `Acerola's Mischief` protects from an ex next
    /// turn.
    ProtectFromEx { target: PokemonId },
    /// Return this Pokémon in play, with everything attached, to hand.
    /// `Scoop Up Cyclone`.
    ReturnToHand { target: PokemonId },
    /// Place `Phase::ChoosingDamageCounterTarget`'s damage counters on
    /// this chosen opponent's Pokémon. `Team Rocket's Venture Bomb`.
    PlaceDamageCountersOn { target: PokemonId },
    /// Discard 2 from hand to draw 1, `Prism Tower`'s own standing
    /// action — opens the discard, same shape as every other forced
    /// discard.
    UsePrismTower,
    /// Heal `amount` from every one of the player's own Pokémon,
    /// `Community Center`'s own standing action.
    UseCommunityCenter,
    /// `Levincia`'s once-a-turn action: open a `Deciding` phase moving
    /// up to 2 Basic Lightning Energy from discard to hand.
    UseLevincia,
    /// `Spikemuth Gym`'s once-a-turn action: open a `Deciding` phase
    /// searching the deck for a Marnie's Pokémon to hand.
    UseSpikemuthGym,
    /// `Mystery Garden`'s once-a-turn action: discard an Energy from
    /// hand, then draw up to the discarder's own in-play Pokémon count
    /// of the Stadium's type.
    UseMysteryGarden,
    /// `Surfing Beach`'s once-a-turn action: switch the Active with
    /// this Benched Pokémon of the Stadium's own type.
    SwitchForSurfingBeach { target: PokemonId },
    /// Put one damage counter on this Benched Pokémon, as part of
    /// `Phase::DistributingDamageCounters`.
    PlaceDamageCounter { target: PokemonId },
    /// Deal `Phase::ChoosingBenchDamageTarget`'s flat damage to this
    /// Benched Pokémon.
    DamageBenchedPokemon { target: PokemonId },
    /// Take this Basic Pokémon from the deck onto the Bench, as part
    /// of `Phase::SearchingDeckForBasics`.
    TakeBasicPokemonForCallForFamily { card: CardId },
    /// Take this Item, found by searching the whole deck, as part
    /// of `Phase::SearchingDeckForItem`.
    TakeItemFromDeck { card: CardId },
    /// Take this card, found by searching the whole deck, as part
    /// of `Phase::SearchingDeckForAnyCards`.
    TakeAnyCardFromDeck { card: CardId },
    /// Stop `Phase::SearchingDeckForAnyCards` before its limit is
    /// reached.
    FinishSearchingAnyCards,
    /// Stop `Phase::SearchingDeckForBasics` before its limit is
    /// spent.
    FinishCallForFamily,
    /// Take this card, found searching for a Basic Pokémon of a type,
    /// as part of `Phase::SearchingDeckForBasicsOfType`.
    TakeBasicPokemonOfTypeForEnergyAttach { card: CardId },
    /// Stop `Phase::SearchingDeckForBasicsOfType` before its limit
    /// is reached.
    FinishSearchingBasicsOfType,
    /// Move this Energy from the opponent's Active into their hand, as
    /// part of `Phase::MovingOpponentsActiveEnergyToHand`.
    MoveOpponentsActiveEnergyToHand { card: CardId },
    /// Stop `Phase::MovingOpponentsActiveEnergyToHand` before its
    /// limit is spent — the effect is optional ("may").
    FinishMovingOpponentsActiveEnergyToHand,
    /// Take this Trainer card from the discard pile into hand, as part
    /// of `Phase::TakingTrainerFromDiscard`.
    TakeTrainerFromDiscard { card: CardId },
    /// Evolve into this card from the deck, as part of
    /// `Phase::SearchingDeckToEvolveSelf`.
    EvolveWithAscension { card: CardId },
    /// Take this Pokémon card from the discard pile into hand, as part
    /// of `Phase::TakingPokemonFromDiscard`.
    TakePokemonFromDiscard { card: CardId },
    /// Accept `Phase::DecidingToShuffleEnergyForBenchDamage`'s cost.
    AcceptShuffleEnergyForBenchDamage,
    /// Decline it — nothing else about this attack changes.
    DeclineShuffleEnergyForBenchDamage,
    /// Move this Energy to this Pokémon, as part of
    /// `Phase::MovingOpponentsEnergy`.
    MoveOpponentsEnergy { card: CardId, target: PokemonId },
    /// Bench this named Pokémon from the discard pile, as part of
    /// `Phase::SearchingDiscardForNamedToBench`.
    TakeNamedFromDiscardToBench { card: CardId },
    /// Stop `Phase::SearchingDiscardForNamedToBench` before its limit
    /// is spent.
    FinishSearchingDiscardForNamedToBench,
    /// Use this Pokémon's own Ability. `Phase::Main` offers it only
    /// when its own gates (whose turn, in the Active Spot, not
    /// already spent) all hold.
    UseAbility { pokemon: PokemonId },
    /// Take this Supporter card from the deck into hand, as part
    /// of `Phase::DecidingToUseLastDitchCatch`.
    TakeSupporterForLastDitchCatch { card: CardId },
    /// Decline it — nothing else about this play changes.
    DeclineLastDitchCatch,
    /// Accept `Phase::DecidingToUsePsychicDraw`'s draw.
    AcceptPsychicDraw,
    /// Decline it.
    DeclinePsychicDraw,
    /// Accept `Phase::DecidingToUseJewelSeeker`'s search.
    AcceptJewelSeeker,
    /// Decline it.
    DeclineJewelSeeker,
    /// Take this Trainer card as part of
    /// `Phase::SearchingDeckForTrainerCards`.
    TakeTrainerCardFromDeck { card: CardId },
    /// Take this card as part of
    /// `Phase::SearchingDeckForPokemonOfTypeOrStadium`.
    TakePokemonOfTypeOrStadiumFromDeck { card: CardId },
    /// Stop that search before its limit is reached.
    FinishSearchingPokemonOfTypeOrStadium,
    /// Discard this card from the opponent's hand, as part of
    /// `Phase::ChoosingCardFromOpponentsHandToDiscard`.
    DiscardCardFromOpponentsHand { card: CardId },
    /// Stop `Phase::SearchingDeckForTrainerCards` before its
    /// limit is reached.
    FinishSearchingTrainerCards,
    /// Deal `Phase::ChoosingAnyOpponentPokemonDamageTarget`'s flat
    /// damage to this Pokémon, Active or Benched.
    DamageChosenOpponentPokemon { target: PokemonId },
    /// Deal `Phase::ChoosingAnyOpponentPokemonDamageTargetWeaknessIfActive`'s
    /// flat damage to this Pokémon, applying Weakness and Resistance
    /// if it turns out to be the opponent's Active.
    DamageChosenOpponentPokemonWeaknessIfActive { target: PokemonId },
    /// Pick this own Benched Pokémon as `Phase::ChoosingOwnBenchedSourceForDamageMove`'s
    /// source.
    ChooseOwnBenchedSourceForDamageMove { source: PokemonId },
    /// Pick this opponent Pokémon as `Phase::ChoosingOpponentTargetForDamageMove`'s
    /// target, moving all the source's damage counters onto it.
    MoveDamageToChosenOpponentPokemon { target: PokemonId },
    /// Pick this own Benched Pokémon as `Phase::ChoosingBenchedTargetForEnergySearch`'s
    /// target.
    ChooseBenchedTargetForEnergySearch { target: PokemonId },
    /// Take this Energy card as part of
    /// `Phase::SearchingEnergyOfTypeToAttachToChosen`.
    TakeEnergyOfTypeToAttachToChosen { card: CardId },
    /// Stop that search before its limit is spent.
    FinishSearchingEnergyOfTypeToAttachToChosen,
    /// Pick one of `Phase::ChoosingTwoOpponentPokemonDamageTargets`'s
    /// two targets — the first call reopens the same phase excluding
    /// this pick, the second resolves it.
    DamageOneOfTwoChosenOpponentPokemon { target: PokemonId },
    /// Pick one of `Phase::ChoosingThreeOpponentPokemonDamageTargets`'s
    /// remaining targets — each call reopens the same phase with this
    /// pick added to `excluding`, until 3 picks are made or none are
    /// left.
    DamageOneOfThreeChosenOpponentPokemon { target: PokemonId },
    /// Pick which of `Phase::ChoosingBenchedPokemonAttackToCopy`'s
    /// candidates, and which of its own attacks, to copy.
    CopyBenchedPokemonAttack { pokemon: PokemonId, index: usize },
    /// Pick which of `Phase::ChoosingDiscardedPokemonAttackToCopy`'s
    /// card's own attacks to copy.
    CopyDiscardedPokemonAttack { index: usize },
    /// Discard this hand card as `Phase::DiscardingHandCardThenDrawing`'s
    /// own cost, then draw its count.
    DiscardHandCardThenDraw { card: CardId },
    /// Deal `Phase::ChoosingBenchedExDamageTarget`'s flat damage to
    /// this Benched Pokémon ex.
    DamageBenchedEx { target: PokemonId },
    /// Deal `Phase::ChoosingAnyBenchedDamageTarget`'s flat damage to
    /// this Benched Pokémon.
    DamageAnyBenched { target: PokemonId },
    /// Attach `Phase::SearchingForEnergyToAttachToBenchedOfType`'s
    /// Energy to this Benched Pokémon of the matching type.
    AttachSearchedEnergyTo { target: PokemonId },
    /// Attach this Energy from hand, as part of
    /// `Phase::DecidingToUseTealDance`.
    AttachEnergyForTealDance { card: CardId },
    /// Decline it.
    DeclineTealDance,
    /// Place `Phase::DecidingCursedBlastTarget`'s damage counters on
    /// this Pokémon, and Knock Out the Ability's own carrier.
    DamageOpponentForCursedBlast { target: PokemonId },
    /// Decline it.
    DeclineCursedBlast,
    /// Take this Evolution Pokémon from the deck into hand, as
    /// part of `Phase::SearchingDeckForEvolutionPokemonOfType`.
    TakeEvolutionPokemonOfType { card: CardId },
    /// Stop that search before its limit is spent.
    FinishSearchingEvolutionPokemonOfType,
    /// Attach this Energy from the discard pile to this Pokémon, as
    /// part of `Phase::DecidingToUseSeethingSpirit`.
    AttachEnergyForSeethingSpirit { card: CardId, target: PokemonId },
    /// Take this card from the deck into hand, as part of
    /// `Phase::SearchingDeckForAnyCardAbility`.
    TakeAnyCardFromDeckForAbility { card: CardId },
    /// Decline that search.
    FinishSearchingDeckForAnyCardAbility,
    /// Decline it.
    DeclineSeethingSpirit,
    /// Attach this Basic Energy from the hand to this Pokémon, then
    /// heal it, as part of `Phase::DecidingToUseRipeningCharge`.
    AttachEnergyForRipeningCharge { card: CardId, target: PokemonId },
    /// Decline it.
    DeclineRipeningCharge,
    /// Move this attached Energy to hand, as part of
    /// `Phase::ChoosingOwnEnergyToHand`.
    MoveOwnAttachedEnergyToHand { card: CardId },
    /// Move this attached Energy to this own Benched Pokémon, as
    /// part of `Phase::ChoosingEnergyAndBenchedTargetToMove`.
    MoveEnergyToChosenBenched { card: CardId, target: PokemonId },
    /// Move this many damage counters from `source` to `target`, as
    /// part of `Phase::MovingDamageCountersFromOwnToOpponent`.
    MoveDamageCountersFromOwnToOpponent { source: PokemonId, target: PokemonId, count: u32 },
    /// Decline to move any.
    DeclineMovingDamageCounters,
    /// Take this card, seen among the top of the deck, as part of
    /// `Phase::LookingAtTopCardsToTakeOne`. The rest go to the bottom.
    TakeCardFromTopPeek { card: CardId },
    /// Attach this card, seen among the top of the deck, to this
    /// own Pokémon, as part of `Phase::ResolvingEnergyFoundInTopPeek`.
    AttachFoundEnergyTo { card: CardId, target: PokemonId },
    /// Leave this card, seen among the top of the deck, and send
    /// it to the bottom instead, as part of
    /// `Phase::ResolvingEnergyFoundInTopPeek`.
    PutFoundCardOnBottom { card: CardId },
    /// Take this Supporter, seen among the top of the deck, as
    /// part of `Phase::LookingAtTopCardsForSupporter`. The rest
    /// shuffle back.
    TakeSupporterFromTopPeek { card: CardId },
    /// Decline to take any.
    DeclineTopPeekSupporter,
    /// Accept `Phase::DecidingToUseSnowSink`'s discard.
    AcceptSnowSink,
    /// Decline it.
    DeclineSnowSink,
    /// Accept `Phase::DecidingToSwitchInForRapidVernier`'s switch.
    AcceptRapidVernierSwitch,
    /// Decline it.
    DeclineRapidVernierSwitch,
    /// Move this Energy to the switched-in Pokémon, as part of
    /// `Phase::MovingAnyEnergyForRapidVernier`.
    MoveEnergyForRapidVernier { card: CardId },
    /// Stop moving Energy.
    FinishMovingEnergyForRapidVernier,
    /// Attach `Phase::SearchingForSinisterSurgeTarget`'s Energy to
    /// this Benched Pokémon, and deal it the damage.
    AttachSinisterSurgeEnergyTo { target: PokemonId },
    /// Take this Pokémon card from the deck into hand, as part of
    /// `Phase::SearchingForFanCall`.
    TakeCardForFanCall { card: CardId },
    /// Stop that search before its limit is spent.
    FinishFanCall,
    /// Switch this Benched Pokémon in for the Active, as part of
    /// `Phase::DecidingToUseSubjugatingChains` — it is then Poisoned.
    SwitchForSubjugatingChains { target: PokemonId },
    /// Discard this attached Tool, as part of
    /// `Phase::DiscardingToolsAnywhere`.
    DiscardToolAnywhere { card: CardId },
    /// Stop before the limit is spent.
    FinishDiscardingToolsAnywhere,
}

/// Whose choice the engine is waiting for. It is not always the player whose
/// turn it is: a knockout hands the choice to the player who lost the Pokémon.
pub fn player_to_act(state: &GameState) -> Option<PlayerId> {
    match state.phase {
        Phase::Main => Some(state.current),
        Phase::Promoting { chooser, .. } => Some(chooser),
        Phase::PromotingOwnNamePrefixThenOpponent { player, .. } => Some(player),
        Phase::PromotingOpponentBasicThenConfuse { player } => Some(player),
        Phase::ChoosingProtectedFromEx { player } => Some(player),
        Phase::ChoosingToReturnToHand { player } => Some(player),
        Phase::ChoosingDamageCounterTarget { player, .. } => Some(player),
        Phase::ChoosingWhoGoesFirst { winner } => Some(winner),
        Phase::TakingBonusDraws { player, .. } => Some(player),
        Phase::PlacingActive { player } => Some(player),
        Phase::PlacingBench { player } => Some(player),
        Phase::DiscardingForRetreat { player, .. } => Some(player),
        Phase::Deciding { chooser, .. } => Some(chooser),
        Phase::Paying { player, .. } => Some(player),
        Phase::PayingToBottomOfDeck { player, .. } => Some(player),
        Phase::MovingEnergy { player } => Some(player),
        Phase::MovingEnergyFromBenchToActive { player, .. } => Some(player),
        Phase::HealingChosen { player, .. } => Some(player),
        Phase::HealingChosenIfRemainingHpAtMost { player, .. } => Some(player),
        Phase::ChoosingOneOf { player, .. } => Some(player),
        Phase::DiscardingFromHand { chooser, .. } => Some(chooser),
        Phase::HealingMegaEx { player } => Some(player),
        Phase::LookingAtBottomOfDeck { player, .. } => Some(player),
        Phase::Devolving { player, .. } => Some(player),
        Phase::SwappingIdentity { player, .. } => Some(player),
        Phase::MovingEnergyForHandheldFan { chooser, .. } => Some(chooser),
        Phase::AttachingFromDiscardForPowerglass { player } => Some(player),
        Phase::DistributingDamageCounters { player, .. } => Some(player),
        Phase::ChoosingBenchDamageTarget { player, .. } => Some(player),
        Phase::SearchingDeckForBasics { player, .. } => Some(player),
        Phase::SearchingDeckForBasicsOfType { player, .. } => Some(player),
        Phase::SearchingDeckForItem { player } => Some(player),
        Phase::SearchingDeckForAnyCards { player, .. } => Some(player),
        Phase::MovingOpponentsActiveEnergyToHand { player, .. } => Some(player),
        Phase::TakingTrainerFromDiscard { player } => Some(player),
        Phase::SearchingDeckToEvolveSelf { player, .. } => Some(player),
        Phase::TakingPokemonFromDiscard { player } => Some(player),
        Phase::DecidingToShuffleEnergyForBenchDamage { player, .. } => Some(player),
        Phase::DecidingToUseLastDitchCatch { player, .. } => Some(player),
        Phase::DecidingToUsePsychicDraw { player, .. } => Some(player),
        Phase::DecidingToUseJewelSeeker { player, .. } => Some(player),
        Phase::SearchingDeckForTrainerCards { player, .. } => Some(player),
        Phase::SearchingDeckForPokemonOfTypeOrStadium { player, .. } => Some(player),
        Phase::ChoosingCardFromOpponentsHandToDiscard { player } => Some(player),
        Phase::ChoosingAnyOpponentPokemonDamageTarget { player, .. } => Some(player),
        Phase::ChoosingAnyOpponentPokemonDamageTargetWeaknessIfActive { player, .. } => Some(player),
        Phase::ChoosingOwnBenchedSourceForDamageMove { player } => Some(player),
        Phase::ChoosingOpponentTargetForDamageMove { player, .. } => Some(player),
        Phase::ChoosingBenchedTargetForEnergySearch { player, .. } => Some(player),
        Phase::SearchingEnergyOfTypeToAttachToChosen { player, .. } => Some(player),
        Phase::ChoosingTwoOpponentPokemonDamageTargets { player, .. } => Some(player),
        Phase::ChoosingThreeOpponentPokemonDamageTargets { player, .. } => Some(player),
        Phase::ChoosingBenchedPokemonAttackToCopy { player, .. } => Some(player),
        Phase::ChoosingDiscardedPokemonAttackToCopy { player, .. } => Some(player),
        Phase::DiscardingHandCardThenDrawing { player, .. } => Some(player),
        Phase::SearchingDeckForAnyCardAbility { player, .. } => Some(player),
        Phase::ChoosingBenchedExDamageTarget { player, .. } => Some(player),
        Phase::ChoosingAnyBenchedDamageTarget { player, .. } => Some(player),
        Phase::SearchingForEnergyToAttachToBenchedOfType { player, .. } => Some(player),
        Phase::MovingDamageCountersFromOwnToOpponent { player, .. } => Some(player),
        Phase::LookingAtTopCardsToTakeOne { player, .. } => Some(player),
        Phase::ResolvingEnergyFoundInTopPeek { player, .. } => Some(player),
        Phase::LookingAtTopCardsForSupporter { player, .. } => Some(player),
        Phase::DecidingToUseTealDance { player, .. } => Some(player),
        Phase::DecidingCursedBlastTarget { player, .. } => Some(player),
        Phase::SearchingDeckForEvolutionPokemonOfType { player, .. } => Some(player),
        Phase::DecidingToUseSeethingSpirit { player, .. } => Some(player),
        Phase::DecidingToUseRipeningCharge { player, .. } => Some(player),
        Phase::ChoosingOwnEnergyToHand { player, .. } => Some(player),
        Phase::ChoosingEnergyAndBenchedTargetToMove { player, .. } => Some(player),
        Phase::DecidingToUseSnowSink { player, .. } => Some(player),
        Phase::DecidingToSwitchInForRapidVernier { player, .. } => Some(player),
        Phase::MovingAnyEnergyForRapidVernier { player, .. } => Some(player),
        Phase::SearchingForSinisterSurgeTarget { player, .. } => Some(player),
        Phase::SearchingForFanCall { player, .. } => Some(player),
        Phase::DecidingToUseSubjugatingChains { player, .. } => Some(player),
        Phase::DiscardingToolsAnywhere { player, .. } => Some(player),
        Phase::MovingOpponentsEnergy { chooser, .. } => Some(chooser),
        Phase::SearchingDiscardForNamedToBench { player, .. } => Some(player),
        Phase::ChoosingJaninesTargets { player, .. } => Some(player),
        Phase::JaninesSearch { player, .. } => Some(player),
        Phase::EvolvingWithRareCandy { player } => Some(player),
        Phase::EvolvingFromDeckNoAbility { player } => Some(player),
        Phase::DiscardingOpponentEnergy { chooser, .. } => Some(chooser),
        Phase::DiscardingOpponentSpecialEnergy { chooser, .. } => Some(chooser),
        Phase::DiscardingDefenderEnergyForAttack { chooser, .. } => Some(chooser),
        Phase::ChoosingOwnEnergyToDiscardForAttack { player, .. } => Some(player),
        Phase::DecidingToDiscardOwnEnergyForBonusDamage { player, .. } => Some(player),
        Phase::ChoosingOwnEnergyToDiscardForBonusDamage { player, .. } => Some(player),
        Phase::DiscardingAnyBasicEnergyForDamagePerCard { player, .. } => Some(player),
        Phase::DiscardingBenchDownTo { player, .. } => Some(player),
        Phase::Checkup { player } => Some(player),
        Phase::Over => None,
    }
}

pub fn legal_actions(state: &GameState) -> Vec<Action> {
    let mut actions = Vec::new();
    let Some(player) = player_to_act(state) else {
        return actions;
    };
    let side = state.player(player);

    match state.phase {
        Phase::ChoosingWhoGoesFirst { winner } => {
            // Rule 5: the winner chooses, and either seat is a legal answer.
            actions.push(Action::ChooseWhoGoesFirst { first: winner });
            actions.push(Action::ChooseWhoGoesFirst {
                first: winner.opponent(),
            });
            return actions;
        }
        Phase::TakingBonusDraws { .. } => {
            actions.push(Action::TakeBonusDraw);
            actions.push(Action::DeclineBonusDraws);
            return actions;
        }
        Phase::PlacingActive { .. } => {
            // Rule 9: the Active must be a Basic, and a hand with no Basic
            // cannot reach this phase — the mulligan rule guarantees one.
            for card in &side.hand {
                if state.def_of(*card).is_basic_pokemon() {
                    actions.push(Action::PlaceActive { card: *card });
                }
            }
            return actions;
        }
        Phase::PlacingBench { .. } => {
            if side.bench.len() < state.bench_limit(player) {
                for card in &side.hand {
                    if state.def_of(*card).is_basic_pokemon() {
                        actions.push(Action::PlaceOnBench { card: *card });
                    }
                }
            }
            actions.push(Action::FinishPlacing);
            return actions;
        }
        Phase::Checkup { player: whose } => {
            for (owner, pokemon, condition) in &state.checkup_pending {
                if *owner == whose {
                    actions.push(Action::ResolveCheckup {
                        pokemon: *pokemon,
                        condition: *condition,
                    });
                }
            }
            return actions;
        }
        Phase::DiscardingForRetreat { .. } => {
            let active = side.active.expect("a retreat starts from an Active");
            for card in &state.pokemon(active).attached {
                if state.def_of(*card).is_energy() {
                    actions.push(Action::DiscardEnergy { card: *card });
                }
            }
            return actions;
        }
        Phase::Deciding {
            chooser,
            from,
            to,
            filter,
            excludes_type_of_previous,
            remaining,
            previous,
            peek,
            ..
        } => {
            // A card bound for the Bench needs a space on it. Rule 14 caps
            // the Bench at five whatever put the Pokémon there, so a full
            // Bench offers nothing and the choice ends. A card bound to
            // attach needs some Pokémon in play the target filter admits.
            let room = match to {
                Destination::Bench => state.player(chooser).bench.len() < state.bench_limit(chooser),
                Destination::Attach(target_filter) => state
                    .player(chooser)
                    .in_play()
                    .into_iter()
                    .any(|p| state.matches_target(chooser, p, target_filter)),
                Destination::Zone(_) | Destination::TopOfDeckInOrder => true,
            };
            if remaining > 0 && room {
                let slot = crate::card::Slot {
                    filter,
                    to,
                    limit: remaining,
                    excludes_type_of_previous,
                    peek,
                };
                let zone = state.zone(chooser, from);
                // A peeked search reads only the cards nearest to being
                // drawn — the end of the Vec, since `draw` pops from
                // there — not the whole zone.
                let visible: Box<dyn Iterator<Item = &CardId>> = match peek {
                    Some(n) => Box::new(zone.iter().rev().take(n as usize)),
                    None => Box::new(zone.iter()),
                };
                for card in visible {
                    if !state.matches_slot(*card, &slot, previous) {
                        continue;
                    }
                    // A card bound for the top of its own zone is moved
                    // within it, not out of it, so the card just taken is
                    // still there to be found again. `previous` is the one
                    // to exclude — enough for the two `Ciphermaniac's
                    // Codebreaking` ever asks for, though a limit past two
                    // would need every card taken this slot remembered, not
                    // only the last.
                    if to == Destination::TopOfDeckInOrder && Some(*card) == previous {
                        continue;
                    }
                    match to {
                        Destination::Attach(target_filter) => {
                            for target in state.player(chooser).in_play() {
                                if state.matches_target(chooser, target, target_filter) {
                                    actions.push(Action::TakeCardOnto {
                                        card: *card,
                                        target,
                                    });
                                }
                            }
                        }
                        Destination::Bench
                        | Destination::Zone(_)
                        | Destination::TopOfDeckInOrder => {
                            actions.push(Action::TakeCard { card: *card });
                        }
                    }
                }
            }
            actions.push(Action::FinishDeciding);
            return actions;
        }
        Phase::Paying { .. } => {
            // The cost is the only thing the engine will take. Every card
            // still in hand may pay it; the card that demanded the cost is
            // already discarded, so no card need be excluded here.
            for card in &side.hand {
                actions.push(Action::PayWithCard { card: *card });
            }
            return actions;
        }
        Phase::PayingToBottomOfDeck { .. } => {
            for card in &side.hand {
                actions.push(Action::PayWithCardToBottomOfDeck { card: *card });
            }
            return actions;
        }
        Phase::MovingEnergy { player: whose } => {
            let in_play = state.player(whose).in_play();
            for from in &in_play {
                for card in &state.pokemon(*from).attached {
                    if !state.def_of(*card).is_energy() {
                        continue;
                    }
                    for target in &in_play {
                        if target != from {
                            actions.push(Action::MoveEnergy {
                                card: *card,
                                target: *target,
                            });
                        }
                    }
                }
            }
            return actions;
        }
        Phase::MovingEnergyFromBenchToActive { player: whose, remaining } => {
            if remaining > 0 {
                for pokemon in &state.player(whose).bench {
                    for card in &state.pokemon(*pokemon).attached {
                        if state.def_of(*card).is_energy() {
                            actions.push(Action::MoveEnergyToActive { card: *card });
                        }
                    }
                }
            }
            actions.push(Action::FinishMovingEnergyToActive);
            return actions;
        }
        Phase::HealingChosen {
            player: whose,
            of_type,
            ..
        } => {
            for target in state.player(whose).in_play() {
                if of_type.is_none_or(|t| state.pokemon_def(target).kind == t) {
                    actions.push(Action::HealTarget { target });
                }
            }
            return actions;
        }
        Phase::HealingChosenIfRemainingHpAtMost { player: whose, at_most } => {
            for target in state.player(whose).in_play() {
                if state.remaining_hp(target) <= at_most {
                    actions.push(Action::HealTarget { target });
                }
            }
            return actions;
        }
        Phase::ChoosingOneOf { .. } => {
            actions.push(Action::ChooseOption { first: true });
            actions.push(Action::ChooseOption { first: false });
            return actions;
        }
        Phase::DiscardingFromHand { of, filter, remaining, .. } => {
            if remaining > 0 {
                for card in &state.player(of).hand {
                    if state.matches_filter(*card, filter) {
                        actions.push(Action::DiscardFromHand { card: *card });
                    }
                }
            }
            actions.push(Action::FinishDiscardingFromHand);
            return actions;
        }
        Phase::HealingMegaEx { player: whose } => {
            for target in state.player(whose).in_play() {
                if state.pokemon_def(target).prizes == 3 {
                    actions.push(Action::HealMegaEx { target });
                }
            }
            return actions;
        }
        Phase::LookingAtBottomOfDeck { player: whose, count } => {
            let deck = &state.player(whose).deck;
            for card in deck.iter().take(count as usize) {
                if state.matches_filter(*card, crate::card::CardFilter::AnyPokemon) {
                    actions.push(Action::TakeFromBottomOfDeck { card: *card });
                }
            }
            actions.push(Action::DeclineBottomOfDeck);
            return actions;
        }
        Phase::Devolving { player: whose, target: None } => {
            for pokemon in state.player(whose).in_play() {
                if state.pokemon_def(pokemon).stage != crate::card::Stage::Basic {
                    actions.push(Action::ChooseDevolveTarget { target: pokemon });
                }
            }
            return actions;
        }
        Phase::Devolving { target: Some(target), .. } => {
            if state.pokemon(target).cards.len() > 1 {
                actions.push(Action::RemoveOneEvolutionCard);
            }
            actions.push(Action::FinishDevolving);
            return actions;
        }
        Phase::AttachingFromDiscardForPowerglass { player: whose } => {
            for card in &state.player(whose).discard {
                if state.matches_filter(*card, crate::card::CardFilter::BasicEnergy) {
                    actions.push(Action::AttachFromDiscardForPowerglass { card: *card });
                }
            }
            actions.push(Action::DeclinePowerglass);
            return actions;
        }
        Phase::DistributingDamageCounters { player: whose, .. } => {
            let opponent = whose.opponent();
            for pokemon in &state.player(opponent).bench {
                if !state.bench_damage_counters_blocked(whose, *pokemon)
                    && !state.bench_attack_damage_blocked(whose, *pokemon)
                    && !state.bench_attack_effect_blocked(whose, *pokemon)
                {
                    actions.push(Action::PlaceDamageCounter { target: *pokemon });
                }
            }
            return actions;
        }
        Phase::ChoosingBenchDamageTarget { player: whose, .. } => {
            let opponent = whose.opponent();
            for pokemon in &state.player(opponent).bench {
                actions.push(Action::DamageBenchedPokemon { target: *pokemon });
            }
            return actions;
        }
        Phase::SearchingDeckForBasics { player: whose, .. } => {
            for card in &state.player(whose).deck {
                if state.matches_filter(*card, crate::card::CardFilter::PokemonOfStage(crate::card::Stage::Basic)) {
                    actions.push(Action::TakeBasicPokemonForCallForFamily { card: *card });
                }
            }
            actions.push(Action::FinishCallForFamily);
            return actions;
        }
        Phase::SearchingDeckForBasicsOfType { player: whose, kind, .. } => {
            for card in &state.player(whose).deck {
                if state.matches_filter(*card, crate::card::CardFilter::BasicPokemonOfType(kind)) {
                    actions.push(Action::TakeBasicPokemonOfTypeForEnergyAttach { card: *card });
                }
            }
            actions.push(Action::FinishSearchingBasicsOfType);
            return actions;
        }
        Phase::SearchingDeckForItem { player: whose } => {
            for card in &state.player(whose).deck {
                if state.matches_filter(*card, crate::card::CardFilter::TrainerOfKind(TrainerKind::Item)) {
                    actions.push(Action::TakeItemFromDeck { card: *card });
                }
            }
            return actions;
        }
        Phase::SearchingDeckForAnyCards { player: whose, .. } => {
            for card in &state.player(whose).deck {
                actions.push(Action::TakeAnyCardFromDeck { card: *card });
            }
            actions.push(Action::FinishSearchingAnyCards);
            return actions;
        }
        Phase::MovingOpponentsActiveEnergyToHand { player: whose, .. } => {
            let opponent = whose.opponent();
            let active = state.player(opponent).active.expect("this effect needs an Active to read");
            for card in &state.pokemon(active).attached {
                if state.def_of(*card).is_energy() {
                    actions.push(Action::MoveOpponentsActiveEnergyToHand { card: *card });
                }
            }
            actions.push(Action::FinishMovingOpponentsActiveEnergyToHand);
            return actions;
        }
        Phase::TakingTrainerFromDiscard { player: whose } => {
            for card in &state.player(whose).discard {
                if state.matches_filter(*card, crate::card::CardFilter::AnyTrainer) {
                    actions.push(Action::TakeTrainerFromDiscard { card: *card });
                }
            }
            return actions;
        }
        Phase::TakingPokemonFromDiscard { player: whose } => {
            for card in &state.player(whose).discard {
                if state.matches_filter(*card, crate::card::CardFilter::AnyPokemon) {
                    actions.push(Action::TakePokemonFromDiscard { card: *card });
                }
            }
            return actions;
        }
        Phase::DecidingToShuffleEnergyForBenchDamage { .. } => {
            actions.push(Action::AcceptShuffleEnergyForBenchDamage);
            actions.push(Action::DeclineShuffleEnergyForBenchDamage);
            return actions;
        }
        Phase::DecidingToUseLastDitchCatch { player: whose, .. } => {
            for card in &state.player(whose).deck {
                if state.matches_filter(*card, crate::card::CardFilter::TrainerOfKind(TrainerKind::Supporter)) {
                    actions.push(Action::TakeSupporterForLastDitchCatch { card: *card });
                }
            }
            actions.push(Action::DeclineLastDitchCatch);
            return actions;
        }
        Phase::DecidingToUsePsychicDraw { .. } => {
            actions.push(Action::AcceptPsychicDraw);
            actions.push(Action::DeclinePsychicDraw);
            return actions;
        }
        Phase::DecidingToUseJewelSeeker { .. } => {
            actions.push(Action::AcceptJewelSeeker);
            actions.push(Action::DeclineJewelSeeker);
            return actions;
        }
        Phase::SearchingDeckForTrainerCards { player: whose, .. } => {
            for card in &state.player(whose).deck {
                if state.matches_filter(*card, crate::card::CardFilter::AnyTrainer) {
                    actions.push(Action::TakeTrainerCardFromDeck { card: *card });
                }
            }
            actions.push(Action::FinishSearchingTrainerCards);
            return actions;
        }
        Phase::SearchingDeckForPokemonOfTypeOrStadium { player: whose, kind, .. } => {
            for card in &state.player(whose).deck {
                if state.matches_filter(*card, crate::card::CardFilter::PokemonOfTypeOrStadium(kind)) {
                    actions.push(Action::TakePokemonOfTypeOrStadiumFromDeck { card: *card });
                }
            }
            actions.push(Action::FinishSearchingPokemonOfTypeOrStadium);
            return actions;
        }
        Phase::ChoosingCardFromOpponentsHandToDiscard { player: whose } => {
            for card in &state.player(whose.opponent()).hand {
                actions.push(Action::DiscardCardFromOpponentsHand { card: *card });
            }
            return actions;
        }
        Phase::ChoosingAnyOpponentPokemonDamageTarget { player: whose, .. } => {
            for pokemon in state.player(whose.opponent()).in_play() {
                actions.push(Action::DamageChosenOpponentPokemon { target: pokemon });
            }
            return actions;
        }
        Phase::ChoosingAnyOpponentPokemonDamageTargetWeaknessIfActive { player: whose, .. } => {
            for pokemon in state.player(whose.opponent()).in_play() {
                actions.push(Action::DamageChosenOpponentPokemonWeaknessIfActive { target: pokemon });
            }
            return actions;
        }
        Phase::ChoosingOwnBenchedSourceForDamageMove { player: whose } => {
            for pokemon in &state.player(whose).bench {
                actions.push(Action::ChooseOwnBenchedSourceForDamageMove { source: *pokemon });
            }
            return actions;
        }
        Phase::ChoosingOpponentTargetForDamageMove { player: whose, .. } => {
            for pokemon in state.player(whose.opponent()).in_play() {
                actions.push(Action::MoveDamageToChosenOpponentPokemon { target: pokemon });
            }
            return actions;
        }
        Phase::ChoosingBenchedTargetForEnergySearch { player: whose, .. } => {
            for pokemon in &state.player(whose).bench {
                actions.push(Action::ChooseBenchedTargetForEnergySearch { target: *pokemon });
            }
            return actions;
        }
        Phase::SearchingEnergyOfTypeToAttachToChosen { player: whose, kind, .. } => {
            for card in &state.player(whose).deck {
                if state.matches_filter(*card, crate::card::CardFilter::BasicEnergyOfType(kind)) {
                    actions.push(Action::TakeEnergyOfTypeToAttachToChosen { card: *card });
                }
            }
            actions.push(Action::FinishSearchingEnergyOfTypeToAttachToChosen);
            return actions;
        }
        Phase::ChoosingTwoOpponentPokemonDamageTargets { player: whose, excluding, .. } => {
            for pokemon in state.player(whose.opponent()).in_play() {
                if Some(pokemon) != excluding {
                    actions.push(Action::DamageOneOfTwoChosenOpponentPokemon { target: pokemon });
                }
            }
            return actions;
        }
        Phase::ChoosingThreeOpponentPokemonDamageTargets { player: whose, excluding, .. } => {
            for pokemon in state.player(whose.opponent()).in_play() {
                if !excluding.contains(&Some(pokemon)) {
                    actions.push(Action::DamageOneOfThreeChosenOpponentPokemon { target: pokemon });
                }
            }
            return actions;
        }
        Phase::ChoosingBenchedPokemonAttackToCopy { player: whose, prefix } => {
            for pokemon in &state.player(whose).bench {
                if !state.pokemon_def(*pokemon).name.starts_with(prefix) {
                    continue;
                }
                for index in 0..state.pokemon_def(*pokemon).attacks.len() {
                    actions.push(Action::CopyBenchedPokemonAttack { pokemon: *pokemon, index });
                }
            }
            return actions;
        }
        Phase::DiscardingHandCardThenDrawing { player: whose, .. } => {
            for card in &state.player(whose).hand {
                actions.push(Action::DiscardHandCardThenDraw { card: *card });
            }
            return actions;
        }
        Phase::SearchingDeckForAnyCardAbility { player: whose, .. } => {
            for card in &state.player(whose).deck {
                actions.push(Action::TakeAnyCardFromDeckForAbility { card: *card });
            }
            actions.push(Action::FinishSearchingDeckForAnyCardAbility);
            return actions;
        }
        Phase::ChoosingDiscardedPokemonAttackToCopy { card, .. } => {
            let attacks = state.def_of(card).as_pokemon().expect("only opened for a Pokémon").attacks.len();
            for index in 0..attacks {
                actions.push(Action::CopyDiscardedPokemonAttack { index });
            }
            return actions;
        }
        Phase::ChoosingBenchedExDamageTarget { player: whose, .. } => {
            for pokemon in &state.player(whose.opponent()).bench {
                if state.pokemon_def(*pokemon).prizes > 1 {
                    actions.push(Action::DamageBenchedEx { target: *pokemon });
                }
            }
            return actions;
        }
        Phase::ChoosingAnyBenchedDamageTarget { player: whose, .. } => {
            for pokemon in &state.player(whose.opponent()).bench {
                actions.push(Action::DamageAnyBenched { target: *pokemon });
            }
            return actions;
        }
        Phase::SearchingForEnergyToAttachToBenchedOfType { player: whose, kind } => {
            let has_energy =
                state.player(whose).deck.iter().any(|c| state.def_of(*c).is_energy());
            if has_energy {
                for target in &state.player(whose).bench {
                    if state.pokemon_def(*target).kind == kind {
                        actions.push(Action::AttachSearchedEnergyTo { target: *target });
                    }
                }
            }
            return actions;
        }
        Phase::DecidingToUseTealDance { player: whose, pokemon } => {
            let crate::card::AbilityEffect::OncePerTurnMayAttachBasicEnergyOfTypeThenDraw(kind) =
                state.pokemon_def(pokemon).ability.expect("named only when carried").effect
            else {
                unreachable!("this phase only ever opens for this effect");
            };
            for card in &state.player(whose).hand {
                if state.matches_filter(*card, crate::card::CardFilter::BasicEnergyOfType(kind)) {
                    actions.push(Action::AttachEnergyForTealDance { card: *card });
                }
            }
            actions.push(Action::DeclineTealDance);
            return actions;
        }
        Phase::DecidingCursedBlastTarget { player: whose, .. } => {
            for pokemon in state.player(whose.opponent()).in_play() {
                actions.push(Action::DamageOpponentForCursedBlast { target: pokemon });
            }
            actions.push(Action::DeclineCursedBlast);
            return actions;
        }
        Phase::SearchingDeckForEvolutionPokemonOfType { player: whose, kind, .. } => {
            for card in &state.player(whose).deck {
                if state.matches_filter(*card, crate::card::CardFilter::EvolutionPokemonOfType(kind)) {
                    actions.push(Action::TakeEvolutionPokemonOfType { card: *card });
                }
            }
            actions.push(Action::FinishSearchingEvolutionPokemonOfType);
            return actions;
        }
        Phase::DecidingToUseSeethingSpirit { player: whose, .. } => {
            let side = state.player(whose);
            for card in &side.discard {
                if !state.def_of(*card).is_energy() {
                    continue;
                }
                for target in side.in_play() {
                    actions.push(Action::AttachEnergyForSeethingSpirit { card: *card, target });
                }
            }
            actions.push(Action::DeclineSeethingSpirit);
            return actions;
        }
        Phase::DecidingToUseRipeningCharge { player: whose, pokemon } => {
            let crate::card::AbilityEffect::OncePerTurnMayAttachBasicEnergyOfTypeFromHandToChosenThenHeal(
                kind,
                _,
            ) = state.pokemon_def(pokemon).ability.expect("named only when carried").effect
            else {
                unreachable!("this phase only ever opens for this effect");
            };
            let side = state.player(whose);
            for card in &side.hand {
                if !state.matches_filter(*card, crate::card::CardFilter::BasicEnergyOfType(kind)) {
                    continue;
                }
                for target in side.in_play() {
                    actions.push(Action::AttachEnergyForRipeningCharge { card: *card, target });
                }
            }
            actions.push(Action::DeclineRipeningCharge);
            return actions;
        }
        Phase::ChoosingOwnEnergyToHand { attacker, .. } => {
            for card in &state.pokemon(attacker).attached {
                if state.def_of(*card).is_energy() {
                    actions.push(Action::MoveOwnAttachedEnergyToHand { card: *card });
                }
            }
            return actions;
        }
        Phase::ChoosingEnergyAndBenchedTargetToMove { player: whose, attacker } => {
            for card in &state.pokemon(attacker).attached {
                if state.def_of(*card).is_energy() {
                    for target in &state.player(whose).bench {
                        actions.push(Action::MoveEnergyToChosenBenched { card: *card, target: *target });
                    }
                }
            }
            return actions;
        }
        Phase::MovingDamageCountersFromOwnToOpponent { player: whose, limit, .. } => {
            for source in state.player(whose).in_play() {
                let damage = state.pokemon(source).damage;
                if damage == 0 {
                    continue;
                }
                let max = limit.min(damage / 10);
                // `Battle Cage` never removes this choice: the move still
                // starts by taking the counters off `source`. It only
                // stops them landing on a protected Bench target, so a
                // blocked target stays offered — `apply` is the one that
                // reads the block, and lets the counters vanish instead
                // of arriving.
                for target in state.player(whose.opponent()).in_play() {
                    for tens in 1..=max {
                        actions.push(Action::MoveDamageCountersFromOwnToOpponent {
                            source,
                            target,
                            count: tens * 10,
                        });
                    }
                }
            }
            actions.push(Action::DeclineMovingDamageCounters);
            return actions;
        }
        Phase::LookingAtTopCardsToTakeOne { player: whose, count, .. } => {
            let deck = &state.player(whose).deck;
            let seen = count.min(deck.len() as u32) as usize;
            for card in &deck[deck.len() - seen..] {
                actions.push(Action::TakeCardFromTopPeek { card: *card });
            }
            return actions;
        }
        Phase::ResolvingEnergyFoundInTopPeek { player: whose, kind, remaining, .. } => {
            let deck = &state.player(whose).deck;
            let seen = remaining.min(deck.len() as u32) as usize;
            for card in &deck[deck.len() - seen..] {
                actions.push(Action::PutFoundCardOnBottom { card: *card });
                if state.matches_filter(*card, crate::card::CardFilter::BasicEnergyOfType(kind)) {
                    for target in state.player(whose).in_play() {
                        actions.push(Action::AttachFoundEnergyTo { card: *card, target });
                    }
                }
            }
            return actions;
        }
        Phase::LookingAtTopCardsForSupporter { player: whose, count, .. } => {
            let deck = &state.player(whose).deck;
            let seen = count.min(deck.len() as u32) as usize;
            for card in &deck[deck.len() - seen..] {
                if state.matches_filter(*card, crate::card::CardFilter::TrainerOfKind(TrainerKind::Supporter))
                {
                    actions.push(Action::TakeSupporterFromTopPeek { card: *card });
                }
            }
            actions.push(Action::DeclineTopPeekSupporter);
            return actions;
        }
        Phase::DecidingToUseSnowSink { .. } => {
            actions.push(Action::AcceptSnowSink);
            actions.push(Action::DeclineSnowSink);
            return actions;
        }
        Phase::DecidingToSwitchInForRapidVernier { .. } => {
            actions.push(Action::AcceptRapidVernierSwitch);
            actions.push(Action::DeclineRapidVernierSwitch);
            return actions;
        }
        Phase::MovingAnyEnergyForRapidVernier { player: whose, pokemon } => {
            for source in state.player(whose).in_play() {
                if source == pokemon {
                    continue;
                }
                for card in &state.pokemon(source).attached {
                    if state.def_of(*card).is_energy() {
                        actions.push(Action::MoveEnergyForRapidVernier { card: *card });
                    }
                }
            }
            actions.push(Action::FinishMovingEnergyForRapidVernier);
            return actions;
        }
        Phase::SearchingForSinisterSurgeTarget { player: whose, kind, .. } => {
            for target in &state.player(whose).bench {
                if state.pokemon_def(*target).kind == kind {
                    actions.push(Action::AttachSinisterSurgeEnergyTo { target: *target });
                }
            }
            return actions;
        }
        Phase::SearchingForFanCall { player: whose, kind, hp, .. } => {
            for card in &state.player(whose).deck {
                if state.matches_filter(*card, crate::card::CardFilter::PokemonOfTypeWithHpAtMost(kind, hp)) {
                    actions.push(Action::TakeCardForFanCall { card: *card });
                }
            }
            actions.push(Action::FinishFanCall);
            return actions;
        }
        Phase::DiscardingToolsAnywhere { .. } => {
            for side_player in [PlayerId::One, PlayerId::Two] {
                for pokemon in state.player(side_player).in_play() {
                    for card in &state.pokemon(pokemon).attached {
                        if state.def_of(*card).as_trainer().is_some_and(|t| t.kind == TrainerKind::Tool) {
                            actions.push(Action::DiscardToolAnywhere { card: *card });
                        }
                    }
                }
            }
            actions.push(Action::FinishDiscardingToolsAnywhere);
            return actions;
        }
        Phase::DecidingToUseSubjugatingChains { player: whose, kind, excluding, .. } => {
            for target in &state.player(whose).bench {
                let def = state.pokemon_def(*target);
                if def.kind == kind && def.name != excluding {
                    actions.push(Action::SwitchForSubjugatingChains { target: *target });
                }
            }
            return actions;
        }
        Phase::MovingOpponentsEnergy { chooser, of } => {
            let in_play = state.player(of).in_play();
            for from in &in_play {
                if state.bench_attack_effect_blocked(chooser, *from) {
                    continue;
                }
                for card in &state.pokemon(*from).attached {
                    if !state.def_of(*card).is_energy() {
                        continue;
                    }
                    for target in &in_play {
                        if target != from && !state.bench_attack_effect_blocked(chooser, *target) {
                            actions.push(Action::MoveOpponentsEnergy {
                                card: *card,
                                target: *target,
                            });
                        }
                    }
                }
            }
            return actions;
        }
        Phase::SearchingDiscardForNamedToBench { player: whose, name, .. } => {
            for card in &state.player(whose).discard {
                if state.matches_filter(*card, crate::card::CardFilter::PokemonNamed(name)) {
                    actions.push(Action::TakeNamedFromDiscardToBench { card: *card });
                }
            }
            actions.push(Action::FinishSearchingDiscardForNamedToBench);
            return actions;
        }
        Phase::SearchingDeckToEvolveSelf { player: whose, target } => {
            let from = state.pokemon_def(target).name;
            for card in &state.player(whose).deck {
                if state.def_of(*card).as_pokemon().is_some_and(|p| p.evolve_from == Some(from)) {
                    actions.push(Action::EvolveWithAscension { card: *card });
                }
            }
            return actions;
        }
        Phase::MovingEnergyForHandheldFan { attacker, .. } => {
            let owner = state.pokemon(attacker).owner;
            for card in &state.pokemon(attacker).attached {
                if !state.def_of(*card).is_energy() {
                    continue;
                }
                for target in &state.player(owner).bench {
                    actions.push(Action::MoveEnergyForHandheldFan {
                        card: *card,
                        target: *target,
                    });
                }
            }
            return actions;
        }
        Phase::SwappingIdentity { player: whose, target: None } => {
            for pokemon in state.player(whose).in_play() {
                if state.pokemon_def(pokemon).stage == crate::card::Stage::Basic {
                    actions.push(Action::ChooseIdentitySwapTarget { target: pokemon });
                }
            }
            return actions;
        }
        Phase::SwappingIdentity { player: whose, target: Some(_) } => {
            for card in &state.player(whose).discard {
                if state.matches_filter(
                    *card,
                    crate::card::CardFilter::PokemonOfStage(crate::card::Stage::Basic),
                ) {
                    actions.push(Action::SwapIdentityWithDiscarded { card: *card });
                }
            }
            return actions;
        }
        Phase::ChoosingJaninesTargets { player: whose, remaining, chosen } => {
            if remaining > 0 {
                for target in state.player(whose).in_play() {
                    let already_chosen = chosen.contains(&Some(target));
                    if !already_chosen
                        && state.pokemon_def(target).kind == crate::card::Type::Darkness
                    {
                        actions.push(Action::ChooseJaninesTarget { target });
                    }
                }
            }
            actions.push(Action::FinishChoosingJaninesTargets);
            return actions;
        }
        Phase::JaninesSearch { player: whose, targets, index, .. } => {
            if targets[index as usize].is_some() {
                for card in state.player(whose).deck.iter() {
                    if state.matches_filter(
                        *card,
                        crate::card::CardFilter::BasicEnergyOfType(crate::card::Type::Darkness),
                    ) {
                        actions.push(Action::TakeEnergyForJanine { card: *card });
                    }
                }
            }
            actions.push(Action::FinishJaninesSearch);
            return actions;
        }
        Phase::DiscardingOpponentEnergy { of, .. } => {
            for pokemon in state.player(of).in_play() {
                for card in &state.pokemon(pokemon).attached {
                    if state.def_of(*card).is_energy() {
                        actions.push(Action::DiscardOpponentEnergy { card: *card });
                    }
                }
            }
            return actions;
        }
        Phase::DiscardingOpponentSpecialEnergy { of, .. } => {
            for pokemon in state.player(of).in_play() {
                for card in &state.pokemon(pokemon).attached {
                    if state.def_of(*card).as_energy().is_some_and(|e| e.effect.is_some()) {
                        actions.push(Action::DiscardOpponentSpecialEnergy { card: *card });
                    }
                }
            }
            return actions;
        }
        Phase::DiscardingDefenderEnergyForAttack { target, .. } => {
            for card in &state.pokemon(target).attached {
                if state.def_of(*card).is_energy() {
                    actions.push(Action::DiscardDefenderEnergyForAttack { card: *card });
                }
            }
            return actions;
        }
        Phase::ChoosingOwnEnergyToDiscardForAttack { attacker, .. } => {
            for card in &state.pokemon(attacker).attached {
                if state.def_of(*card).is_energy() {
                    actions.push(Action::DiscardOwnEnergyForAttack { card: *card });
                }
            }
            return actions;
        }
        Phase::DecidingToDiscardOwnEnergyForBonusDamage { .. } => {
            actions.push(Action::AcceptDiscardOwnEnergyForBonusDamage);
            actions.push(Action::DeclineDiscardOwnEnergyForBonusDamage);
            return actions;
        }
        Phase::ChoosingOwnEnergyToDiscardForBonusDamage { attacker, kind, .. } => {
            for card in &state.pokemon(attacker).attached {
                if state.matches_filter(*card, crate::card::CardFilter::BasicEnergyOfType(kind)) {
                    actions.push(Action::DiscardOwnEnergyForBonusDamage { card: *card });
                }
            }
            actions.push(Action::FinishDiscardingOwnEnergyForBonusDamage);
            return actions;
        }
        Phase::DiscardingAnyBasicEnergyForDamagePerCard { player: whose, .. } => {
            for pokemon in state.player(whose).in_play() {
                for card in &state.pokemon(pokemon).attached {
                    if state.matches_filter(*card, crate::card::CardFilter::BasicEnergy) {
                        actions.push(Action::DiscardBasicEnergyForDamagePerCard { card: *card });
                    }
                }
            }
            actions.push(Action::FinishDiscardingBasicEnergyForDamagePerCard);
            return actions;
        }
        Phase::DiscardingBenchDownTo { player: whose, .. } => {
            for pokemon in &state.player(whose).bench {
                actions.push(Action::DiscardBenchedPokemon { pokemon: *pokemon });
            }
            return actions;
        }
        Phase::EvolvingWithRareCandy { player: whose } => {
            for (card, target) in rare_candy_pairs(state, whose) {
                actions.push(Action::EvolveSkippingOneStage { card, target });
            }
            return actions;
        }
        Phase::EvolvingFromDeckNoAbility { player: whose } => {
            for (card, target) in salvatore_pairs(state, whose) {
                actions.push(Action::EvolveFromDeck { card, target });
            }
            return actions;
        }
        _ => {}
    }

    if let Phase::Promoting { of, .. } = state.phase {
        for pokemon in &state.player(of).bench {
            actions.push(Action::Promote { pokemon: *pokemon });
        }
        return actions;
    }

    if let Phase::PromotingOwnNamePrefixThenOpponent { player, prefix } = state.phase {
        for pokemon in &state.player(player).bench {
            if state.pokemon_def(*pokemon).name.starts_with(prefix) {
                actions.push(Action::Promote { pokemon: *pokemon });
            }
        }
        return actions;
    }

    if let Phase::ChoosingDamageCounterTarget { player, .. } = state.phase {
        for target in state.player(player.opponent()).in_play() {
            actions.push(Action::PlaceDamageCountersOn { target });
        }
        return actions;
    }

    if let Phase::ChoosingToReturnToHand { player } = state.phase {
        for target in state.player(player).in_play() {
            actions.push(Action::ReturnToHand { target });
        }
        return actions;
    }

    if let Phase::ChoosingProtectedFromEx { player } = state.phase {
        for target in state.player(player).in_play() {
            actions.push(Action::ProtectFromEx { target });
        }
        return actions;
    }

    if let Phase::PromotingOpponentBasicThenConfuse { player } = state.phase {
        for pokemon in &state.player(player.opponent()).bench {
            if state.pokemon_def(*pokemon).stage == crate::card::Stage::Basic {
                actions.push(Action::Promote { pokemon: *pokemon });
            }
        }
        return actions;
    }

    // A Stadium's own once-a-turn action — not dispatched through
    // PlayTrainer, since the Stadium is already in play; offered
    // directly, the way AttachEnergy and PlayTool are.
    if state.stadium_effect() == Some(crate::card::TrainerEffect::MayPutHandCardOnTopOfDeck)
        && !state.is_spent(Limit::StadiumEffectUsed(player))
    {
        for card in &side.hand {
            actions.push(Action::PutOnTopOfDeckForAcademyAtNight { card: *card });
        }
    }
    if state.stadium_effect() == Some(crate::card::TrainerEffect::MayDrawTwoIfPlayedTeamRocketSupporter)
        && !state.is_spent(Limit::StadiumEffectUsed(player))
        && state.played_a_team_rocket_supporter_this_turn[player.index()]
    {
        actions.push(Action::DrawTwoForTeamRocketsFactory);
    }
    if state.stadium_effect()
        == Some(crate::card::TrainerEffect::MaySearchBasicToBenchThenMaybeEndTurn)
        && !state.is_spent(Limit::StadiumEffectUsed(player))
    {
        actions.push(Action::UseLumioseCity);
    }
    if state.stadium_effect() == Some(crate::card::TrainerEffect::StadiumMayDiscardTwoToDrawOne)
        && !state.is_spent(Limit::StadiumEffectUsed(player))
        && side.hand.len() >= 2
    {
        actions.push(Action::UsePrismTower);
    }
    if let Some(crate::card::TrainerEffect::StadiumMayHealAllIfPlayedSupporter(_)) =
        state.stadium_effect()
        && !state.is_spent(Limit::StadiumEffectUsed(player))
        && state.is_spent(Limit::SupporterPlayed(player))
    {
        actions.push(Action::UseCommunityCenter);
    }
    if let Some(crate::card::TrainerEffect::StadiumMayReturnEnergyOfTypeFromDiscard(kind, _)) =
        state.stadium_effect()
        && !state.is_spent(Limit::StadiumEffectUsed(player))
        && side
            .discard
            .iter()
            .any(|c| state.matches_filter(*c, crate::card::CardFilter::BasicEnergyOfType(kind)))
    {
        actions.push(Action::UseLevincia);
    }
    if let Some(crate::card::TrainerEffect::StadiumMaySearchForNameToHand(word)) =
        state.stadium_effect()
        && !state.is_spent(Limit::StadiumEffectUsed(player))
        && side
            .deck
            .iter()
            .any(|c| state.matches_filter(*c, crate::card::CardFilter::PokemonNameContains(word)))
    {
        actions.push(Action::UseSpikemuthGym);
    }
    if let Some(crate::card::TrainerEffect::StadiumMayDiscardEnergyToDrawUpToTypeCount(_)) =
        state.stadium_effect()
        && !state.is_spent(Limit::StadiumEffectUsed(player))
        && side
            .hand
            .iter()
            .any(|c| state.matches_filter(*c, crate::card::CardFilter::AnyEnergy))
    {
        actions.push(Action::UseMysteryGarden);
    }
    if let Some(crate::card::TrainerEffect::StadiumMaySwitchActiveOfType(kind)) =
        state.stadium_effect()
        && !state.is_spent(Limit::StadiumEffectUsed(player))
        && side.active.is_some_and(|a| state.pokemon_def(a).kind == kind)
    {
        for target in &side.bench {
            if state.pokemon_def(*target).kind == kind {
                actions.push(Action::SwitchForSurfingBeach { target: *target });
            }
        }
    }

    for card in &side.hand {
        let def = state.def_of(*card);
        if def.is_basic_pokemon() && side.bench.len() < state.bench_limit(player) {
            actions.push(Action::PlayBasic { card: *card });
        }
        // A Tool attaches like Energy does — immediately, with a target —
        // not through PlayTrainer, which never names one. Unlike Energy,
        // a Pokémon carries at most one: rule text every Tool print
        // shares, not read from any one card's own effect.
        if def.as_trainer().is_some_and(|t| t.kind == TrainerKind::Tool)
            && !(crate::import::is_ace_spec(def.print_id())
                && state.opponent_ace_specs_blocked(player))
        {
            for target in side.in_play() {
                let carries_a_tool = state.pokemon(target).attached.iter().any(|c| {
                    state
                        .def_of(*c)
                        .as_trainer()
                        .is_some_and(|t| t.kind == TrainerKind::Tool)
                });
                if !carries_a_tool {
                    actions.push(Action::PlayTool {
                        card: *card,
                        target,
                    });
                }
            }
        }
        if def.is_energy()
            && !state.is_spent(Limit::EnergyAttached(player))
            && !(crate::import::is_ace_spec(def.print_id())
                && state.opponent_ace_specs_blocked(player))
        {
            for target in side.in_play() {
                actions.push(Action::AttachEnergy {
                    card: *card,
                    target,
                });
            }
        }
        // Rules 18-20: not on the acting player's own first turn of the
        // game, only onto the Pokémon this card names, only if it has
        // been in play since the start of the turn, and only once per
        // Pokémon per turn.
        if let Some(from) = def.as_pokemon().and_then(|p| p.evolve_from)
            && !state.is_players_first_turn()
        {
            for target in side.in_play() {
                let evolution = def.as_pokemon().expect("this arm only runs for a Pokémon card");
                let eligible = state.pokemon_def(target).name == from
                    && (state.pokemon(target).played_on_turn < state.turn_number
                        || state.forest_of_vitality_applies(target, evolution))
                    && !state.is_spent(Limit::Evolved(target))
                    && !state.pokemon(target).cannot_evolve_this_turn;
                if eligible {
                    actions.push(Action::Evolve {
                        card: *card,
                        target,
                    });
                }
            }
        }
        // Rule 13: an Item any number of times; a Supporter or a Stadium
        // once a turn. A Tool is handled above, by `PlayTool` — it always
        // names a target, which `PlayTrainer` never does.
        if let Some(trainer) = def.as_trainer().filter(|t| t.kind != TrainerKind::Tool) {
            let cannot_play_items = matches!(
                state.opponent_next_turn_restriction,
                Some((target, crate::card::AttackEffect::OpponentCannotPlayItemsNextTurn, _))
                    if state.pokemon(target).owner == player
            );
            let cannot_play_stadiums = matches!(
                state.opponent_next_turn_restriction,
                Some((
                    target,
                    crate::card::AttackEffect::DiscardsOpponentsStadiumThenOpponentCannotPlayStadiumsNextTurn,
                    _
                )) if state.pokemon(target).owner == player
            );
            let timing = match trainer.kind {
                TrainerKind::Item => !cannot_play_items,
                TrainerKind::Tool => unreachable!("filtered out above"),
                TrainerKind::Supporter => {
                    !state.is_spent(Limit::SupporterPlayed(player))
                        && !state.is_first_turn_of_game()
                }
                TrainerKind::Stadium => {
                    !state.is_spent(Limit::StadiumPlayed(player)) && !cannot_play_stadiums
                }
            } && !(crate::import::is_ace_spec(def.print_id())
                && state.opponent_ace_specs_blocked(player));
            // A card that switches the opponent's Active needs somewhere to
            // switch to; every other effect built so far can always be
            // attempted, even where it turns up nothing to move.
            let has_a_target = match trainer.effect {
                TrainerEffect::SwitchOpponentActive
                | TrainerEffect::SwitchOutOpponentActive => {
                    !state.player(player.opponent()).bench.is_empty()
                }
                TrainerEffect::SwitchOwnActive | TrainerEffect::SwitchOwnActiveWithFollowUp(_) => {
                    !side.bench.is_empty()
                }
                TrainerEffect::SwitchOpponentActiveThenOwn => {
                    !state.player(player.opponent()).bench.is_empty()
                }
                TrainerEffect::SwitchOpponentActiveBasicThenConfuse => state
                    .player(player.opponent())
                    .bench
                    .iter()
                    .any(|p| state.pokemon_def(*p).stage == crate::card::Stage::Basic),
                // A Pokémon that has actually evolved, to devolve.
                TrainerEffect::DevolveChosen => side
                    .in_play()
                    .iter()
                    .any(|p| state.pokemon_def(*p).stage != crate::card::Stage::Basic),
                // A Basic in play to swap out, and a Basic in discard to
                // swap in.
                TrainerEffect::SwapBasicWithDiscard => {
                    side.in_play()
                        .iter()
                        .any(|p| state.pokemon_def(*p).stage == crate::card::Stage::Basic)
                        && side.discard.iter().any(|c| {
                            state.matches_filter(
                                *c,
                                crate::card::CardFilter::PokemonOfStage(crate::card::Stage::Basic),
                            )
                        })
                }
                // An Energy to move, and a second Pokémon to move it to.
                TrainerEffect::MoveAttachedEnergy => {
                    let in_play = side.in_play();
                    in_play.len() > 1
                        && in_play.iter().any(|p| {
                            state
                                .pokemon(*p)
                                .attached
                                .iter()
                                .any(|c| state.def_of(*c).is_energy())
                        })
                }
                // An Energy on some Benched Pokémon to move onto the
                // Active. `N's Plan` is only ever "up to" a limit, so a
                // Bench with nothing on it still leaves nothing to offer.
                TrainerEffect::MoveEnergyFromBenchToActive { .. } => side.bench.iter().any(|p| {
                    state
                        .pokemon(*p)
                        .attached
                        .iter()
                        .any(|c| state.def_of(*c).is_energy())
                }),
                // A Mega Evolution ex the player controls.
                TrainerEffect::HealMegaExAndTakeEnergyIfHealed => side
                    .in_play()
                    .iter()
                    .any(|p| state.pokemon_def(*p).prizes == 3),
                // "Up to 2" — legal even holding no Darkness Pokémon at all.
                TrainerEffect::JaninesSecretArt => true,
                // An Energy on the opponent's board to discard. Only the
                // no-coin cards are gated: `Crushing Hammer` still flips
                // with nothing to hit, so it stays playable.
                TrainerEffect::DiscardOpponentEnergy => {
                    state.has_energy_in_play(player.opponent())
                }
                TrainerEffect::DiscardOpponentSpecialEnergy => {
                    state.has_special_energy_in_play(player.opponent())
                }
                // A Stage 2 in hand, and a Basic under it in play. Rare
                // Candy is only playable at all where the pair already
                // exists — nothing in its phase ever declines.
                TrainerEffect::EvolveSkippingOneStage => {
                    !rare_candy_pairs(state, player).is_empty()
                }
                _ => true,
            };
            // A requirement gates the card before anything else does.
            let requirement_met = match trainer.requirement {
                None => true,
                Some(Requirement::DiscardOtherCardsFromHand(count)) => {
                    side.hand.len() as u32 > count
                }
                Some(Requirement::OpponentPrizesAtMost(most)) => {
                    state.player(player.opponent()).prizes.len() <= most
                }
                Some(Requirement::KnockedOutDuringOpponentsLastTurn) => {
                    state.knocked_out_last_turn[player.index()]
                }
                Some(Requirement::ActiveHasAtLeastEnergy(least)) => side.active.is_some_and(|a| {
                    state
                        .pokemon(a)
                        .attached
                        .iter()
                        .filter(|c| state.def_of(**c).is_energy())
                        .count() as u32
                        >= least
                }),
                Some(Requirement::MorePrizesThanOpponent) => {
                    side.prizes.len() > state.player(player.opponent()).prizes.len()
                }
                Some(Requirement::HandSizeIs(count)) => side.hand.len() as u32 == count,
                Some(Requirement::OpponentPrizesExactly(exactly)) => {
                    state.player(player.opponent()).prizes.len() == exactly
                }
                Some(Requirement::OwnTeraPokemonInPlay) => side
                    .in_play()
                    .iter()
                    .any(|p| state.pokemon_def(*p).markers.contains(&crate::card::Marker::Tera)),
                Some(Requirement::ActiveNamePrefix(prefix)) => {
                    side.active.is_some_and(|a| state.pokemon_def(a).name.starts_with(prefix))
                        && side
                            .bench
                            .iter()
                            .any(|b| state.pokemon_def(*b).name.starts_with(prefix))
                }
                Some(Requirement::SecondCopyOfThisInHand) => {
                    let def = state.cards[card.index()].def;
                    side.hand
                        .iter()
                        .filter(|c| state.cards[c.index()].def == def)
                        .count()
                        > 1
                }
                Some(Requirement::PutOtherCardsOnBottomOfDeck(count)) => {
                    side.hand.len() as u32 > count
                }
            };
            // Rule 59: not a Stadium whose name is already in play.
            let name_is_free = trainer.kind != TrainerKind::Stadium
                || state
                    .stadium
                    .is_none_or(|(_, in_play)| state.def_of(in_play).name() != trainer.name);
            if timing && has_a_target && name_is_free && requirement_met {
                actions.push(Action::PlayTrainer { card: *card });
            }
        }
    }

    // Rules 50-51: Asleep and Paralyzed stop both an attack and a retreat.
    // Confused stops neither; it flips when the attack happens.
    let held = |active| {
        state.has_condition(active, Condition::Asleep)
            || state.has_condition(active, Condition::Paralyzed)
    };

    // Rules 23-24: once per turn, pay the Retreat Cost in Energy, and only
    // with somewhere to retreat to.
    if let Some(active) = side.active {
        let cost = state.effective_retreat_cost(active);
        let cannot_retreat = matches!(
            state.opponent_next_turn_restriction,
            Some((target, crate::card::AttackEffect::DefenderCannotRetreatNextTurn, _))
                if target == active
        ) || matches!(
            state.opponent_next_turn_restriction,
            Some((
                target,
                crate::card::AttackEffect::InflictsConditionAndDefenderCannotRetreatNextTurn(_),
                _
            )) if target == active
        ) || matches!(
            state.opponent_next_turn_restriction,
            Some((
                target,
                crate::card::AttackEffect::DefenderCannotRetreatAndTakesMoreDamageNextTurn(_),
                _
            )) if target == active
        ) || matches!(
            state.side_shield_next_turn,
            Some((granted_by, crate::card::SideShield::OpponentPoisonedCannotRetreat))
                if granted_by != player
        ) && state.has_condition(active, Condition::Poisoned);
        if !state.is_spent(Limit::Retreated(player))
            && !held(active)
            && !cannot_retreat
            && state.energy_attached(active) as u32 >= cost
        {
            for pokemon in &side.bench {
                actions.push(Action::Retreat { to: *pokemon });
            }
        }

        // Rule 17: the player going first skips their attack step.
        let cannot_attack_at_all = matches!(
            state.own_next_turn_restriction,
            Some((target, crate::card::AttackEffect::AttackerCannotAttackNextTurn, true))
                if target == active
        );
        if !state.is_first_turn_of_game() && !held(active) && !cannot_attack_at_all {
            let tera_surcharge = state.pokemon_def(active).markers.contains(&crate::card::Marker::Tera)
                && state.stadium_effect() == Some(crate::card::TrainerEffect::TeraAttacksCostMore);
            let locked_attack_name = matches!(
                state.locked_attack_next_turn,
                Some((target, _, true)) if target == active
            )
            .then(|| state.locked_attack_next_turn.unwrap().1);
            let discount = state.pokemon_def(active).ability.and_then(|a| match a.effect {
                crate::card::AbilityEffect::PassiveNamedAttackCostsLessPerOpponentPrizeTaken(name) => {
                    Some((name, 6 - state.player(player.opponent()).prizes.len()))
                }
                _ => None,
            });
            let tool_any_type_discount: u32 = state
                .pokemon(active)
                .attached
                .iter()
                .filter_map(|c| state.def_of(*c).as_trainer())
                .map(|t| match t.effect {
                    crate::card::TrainerEffect::ReducesAttackCostByAnyTypeIfCarrierMarked(marker, amount)
                        if state.pokemon_def(active).markers.contains(&marker) =>
                    {
                        amount
                    }
                    _ => 0,
                })
                .sum();
            let tool_colorless_discount: u32 = state
                .pokemon(active)
                .attached
                .iter()
                .filter_map(|c| state.def_of(*c).as_trainer())
                .map(|t| match t.effect {
                    crate::card::TrainerEffect::ReducesAttackCostIfMorePrizesRemaining(amount)
                        if state.player(player).prizes.len() > state.player(player.opponent()).prizes.len() =>
                    {
                        amount
                    }
                    _ => 0,
                })
                .sum();
            let colorless_override = state.pokemon_def(active).ability.and_then(|a| match a.effect {
                crate::card::AbilityEffect::PassiveNamedAttackCostsJustColorlessIfOpponentDiscardNameContains(
                    attack_name,
                    word,
                ) if state
                    .player(player.opponent())
                    .discard
                    .iter()
                    .any(|c| state.def_of(*c).name().contains(word)) =>
                {
                    Some(attack_name)
                }
                _ => None,
            });
            for (index, attack) in state.pokemon_def(active).attacks.iter().enumerate() {
                if Some(attack.name) == locked_attack_name {
                    continue;
                }
                let mut cost = attack.cost.clone();
                if Some(attack.name) == colorless_override {
                    cost = vec![crate::card::Type::Colorless];
                }
                if tera_surcharge {
                    cost.push(crate::card::Type::Colorless);
                }
                if let Some((discounted_name, prizes_taken)) = discount
                    && attack.name == discounted_name
                {
                    for _ in 0..prizes_taken {
                        if let Some(pos) = cost.iter().rposition(|t| *t == crate::card::Type::Colorless) {
                            cost.remove(pos);
                        } else {
                            break;
                        }
                    }
                }
                for _ in 0..tool_any_type_discount {
                    if cost.is_empty() {
                        break;
                    }
                    cost.remove(cost.len() - 1);
                }
                for _ in 0..tool_colorless_discount {
                    if let Some(pos) = cost.iter().rposition(|t| *t == crate::card::Type::Colorless) {
                        cost.remove(pos);
                    } else {
                        break;
                    }
                }
                if state.pays_cost(active, &cost) {
                    actions.push(Action::Attack { index });
                }
            }
        }
    }

    // An Ability, offered once its own gates all hold: whose turn it
    // is (already true — `legal_actions` only ever builds this list
    // for `player_to_act`), which Pokémon it demands, and whether the
    // player has already used one with this name this turn. Skipped
    // outright under `Team Rocket's Watchtower`, but only for a `{C}`
    // Pokémon — the card's own text names only Colorless.
    for pokemon in side.in_play() {
        let Some(ability) = state.pokemon_def(pokemon).ability else {
            continue;
        };
        if state.abilities_disabled_for(pokemon) {
            continue;
        }
        if state.is_spent(Limit::for_ability_use(player, pokemon, ability.name)) {
            continue;
        }
        let eligible = match ability.effect {
            crate::card::AbilityEffect::OncePerTurnWhileActiveMayDrawCards(_) => {
                side.active == Some(pokemon)
            }
            crate::card::AbilityEffect::OncePerTurnIfKnockedOutLastTurnMayDrawCards(_) => {
                state.knocked_out_last_turn[player.index()]
            }
            crate::card::AbilityEffect::OncePerTurnMayAttachBasicEnergyOfTypeThenDraw(kind) => {
                side.hand.iter().any(|c| {
                    state.matches_filter(*c, crate::card::CardFilter::BasicEnergyOfType(kind))
                })
            }
            // Works the same from the Active Spot or the Bench.
            crate::card::AbilityEffect::OncePerTurnMayDrawThenShuffleSelfIntoDeck(_) => true,
            crate::card::AbilityEffect::OncePerTurnWhileActiveMayShuffleSelfIntoDeck => {
                side.active == Some(pokemon)
            }
            // Works from the Active Spot or the Bench alike.
            crate::card::AbilityEffect::OncePerTurnMayDamageOpponentThenKnockOutSelf(_) => {
                !state.self_knockout_abilities_disabled()
            }
            crate::card::AbilityEffect::OncePerTurnMaySearchEvolutionPokemonOfType(kind, _) => {
                side.deck.iter().any(|c| {
                    state.matches_filter(*c, crate::card::CardFilter::EvolutionPokemonOfType(kind))
                })
            }
            crate::card::AbilityEffect::OncePerTurnMayAttachBasicEnergyFromDiscardToChosen => {
                side.discard.iter().any(|c| state.def_of(*c).is_energy())
            }
            crate::card::AbilityEffect::OncePerTurnMayDiscardFromHandThenDrawCards(_) => {
                !side.hand.is_empty()
            }
            crate::card::AbilityEffect::OncePerTurnMayAttachBasicEnergyOfTypeFromHandToChosenThenHeal(
                kind,
                _,
            ) => side
                .hand
                .iter()
                .any(|c| state.matches_filter(*c, crate::card::CardFilter::BasicEnergyOfType(kind))),
            crate::card::AbilityEffect::OncePerTurnMaySearchBasicEnergyOfTypeAttachToBenchedThenDamage(
                kind,
                _,
            ) => {
                let has_energy = side.deck.iter().any(|c| {
                    state.matches_filter(*c, crate::card::CardFilter::BasicEnergyOfType(kind))
                });
                let has_target = side.bench.iter().any(|p| state.pokemon_def(*p).kind == kind);
                has_energy && has_target
            }
            crate::card::AbilityEffect::OnceDuringFirstTurnMaySearchPokemonOfTypeWithHpAtMost(
                kind,
                hp,
                _,
            ) => {
                // "Your first turn": turn 0 for whoever goes first, turn 1
                // for whoever goes second — each player's own first turn,
                // not only the game's very first.
                state.turn_number <= 1
                    && side.deck.iter().any(|c| {
                        state.matches_filter(*c, crate::card::CardFilter::PokemonOfTypeWithHpAtMost(kind, hp))
                    })
            }
            crate::card::AbilityEffect::OncePerTurnMaySwitchBenchedOfTypeExcludingNamedThenPoison(
                kind,
                excluding,
            ) => side.bench.iter().any(|p| {
                let def = state.pokemon_def(*p);
                def.kind == kind && def.name != excluding
            }),
            // Triggered the moment this Pokémon is played from hand
            // (`trigger_last_ditch_catch`), never a standing choice.
            crate::card::AbilityEffect::WhenBenchedFromHandMaySearchSupporter => false,
            // Triggered the moment this Pokémon evolves from hand
            // (`trigger_psychic_draw`), never a standing choice.
            crate::card::AbilityEffect::WhenEvolvedFromHandMayDrawCards(_) => false,
            crate::card::AbilityEffect::WhenEvolvedFromHandMaySearchTrainersIfOwnTeraInPlay(_) => false,
            // Triggered the moment this Pokémon is played from hand
            // (`trigger_snow_sink`), never a standing choice.
            crate::card::AbilityEffect::WhenBenchedFromHandMayDiscardStadium => false,
            // Triggered the moment this Pokémon is played from hand
            // (`trigger_rapid_vernier`), never a standing choice.
            crate::card::AbilityEffect::WhenBenchedFromHandMaySwitchThenMoveAnyEnergy => false,
            // A standing effect read directly by `effective_retreat_cost`,
            // never a standing choice.
            crate::card::AbilityEffect::PassiveOwnBasicPokemonHaveNoRetreatCost => false,
            crate::card::AbilityEffect::PassiveImmuneToDamageFromOpponentEx => false,
            crate::card::AbilityEffect::PassiveBlocksDamageCounterMovement => false,
            crate::card::AbilityEffect::PassiveDisablesSelfKnockOutAbilities => false,
            crate::card::AbilityEffect::PassiveSetsOpponentTypeWeaknessTo(..) => false,
            crate::card::AbilityEffect::PassivePreventsAttackDamageToNonRuleBoxBench => false,
            crate::card::AbilityEffect::PassivePreventsAttackEffectsOnBench => false,
            crate::card::AbilityEffect::PassiveFutureAttacksDoBonusDamageToActiveExceptNamed(_) => false,
            crate::card::AbilityEffect::PassiveBonusDamageToActiveIfSelfDamaged(_) => false,
            crate::card::AbilityEffect::PassiveImmuneToAsleep => false,
            crate::card::AbilityEffect::PassiveDisablesOpponentActiveAbilityExceptSelf => false,
            crate::card::AbilityEffect::PassiveBlocksOpponentAceSpecPlaysIfSelfHasTool => false,
            crate::card::AbilityEffect::PassiveNamedAttackCostsJustColorlessIfOpponentDiscardNameContains(
                ..,
            ) => false,
            crate::card::AbilityEffect::PassiveCoinFlipPreventsAttackKnockOutAtTenHp => false,
            crate::card::AbilityEffect::PassiveFestivalLead => false,
            crate::card::AbilityEffect::PassiveDoublesBasicGrassEnergyForCost => false,
            crate::card::AbilityEffect::PassiveBonusCheckupDamageToOpponentsPoisonedWhileActive(_) => {
                false
            }
            crate::card::AbilityEffect::OncePerTurnMaySearchAnyCardIfActiveHasNamedAbility(name) => {
                side.active.is_some_and(|a| {
                    state.pokemon_def(a).ability.is_some_and(|active_ability| active_ability.name == name)
                })
            }
            crate::card::AbilityEffect::PassiveNamedAttackCostsLessPerOpponentPrizeTaken(_) => false,
            crate::card::AbilityEffect::OncePerTurnIfEnergyOfTypeAttachedMayMoveDamageCountersToOpponent(
                kind,
                _,
            ) => {
                !state.damage_counter_movement_blocked()
                    && state
                        .pokemon(pokemon)
                        .attached
                        .iter()
                        .any(|c| state.matches_filter(*c, crate::card::CardFilter::BasicEnergyOfType(kind)))
                    && side.in_play().iter().any(|p| state.pokemon(*p).damage > 0)
            }
            crate::card::AbilityEffect::OncePerTurnMayLookAtTopCardsTakeOneRestToBottom(_) => {
                !side.deck.is_empty()
            }
            crate::card::AbilityEffect::OncePerTurnMayLookAtTopCardsAttachFoundBasicEnergyOfType(
                _,
                _,
            ) => !side.deck.is_empty(),
            crate::card::AbilityEffect::OncePerTurnWhileActiveMayLookAtTopCardsTakeASupporter(count) => {
                side.active == Some(pokemon) && {
                    let seen = (count as usize).min(side.deck.len());
                    side.deck[side.deck.len() - seen..].iter().any(|c| {
                        state.matches_filter(*c, crate::card::CardFilter::TrainerOfKind(TrainerKind::Supporter))
                    })
                }
            }
        };
        if eligible {
            actions.push(Action::UseAbility { pokemon });
        }
    }

    actions.push(Action::EndTurn);
    actions
}

/// Every Stage 2 in hand and Basic in play that `Rare Candy` may pair: the
/// same rules 18-20 an ordinary evolution reads — not the acting player's
/// own first turn of the game, the target in play since before this turn,
/// not yet evolved this turn — matched by `evolves_from_basic` two links
/// down rather than by `evolve_from` one link up.
/// Every (card in the deck, target in play) pair `Salvatore` could
/// evolve — a no-Ability card whose `evolve_from` names an in-play
/// Pokémon of the player's own, with no restriction on the turn either
/// one came into play, unlike `rare_candy_pairs`.
fn salvatore_pairs(state: &GameState, player: PlayerId) -> Vec<(CardId, PokemonId)> {
    let side = state.player(player);
    let mut pairs = Vec::new();
    for card in &side.deck {
        let Some(evolution) = state.def_of(*card).as_pokemon() else {
            continue;
        };
        let Some(from) = evolution.evolve_from else {
            continue;
        };
        if evolution.ability.is_some() {
            continue;
        }
        for target in side.in_play() {
            let eligible = state.pokemon_def(target).name == from
                && !state.is_spent(Limit::Evolved(target))
                && !state.pokemon(target).cannot_evolve_this_turn;
            if eligible {
                pairs.push((*card, target));
            }
        }
    }
    pairs
}

fn rare_candy_pairs(state: &GameState, player: PlayerId) -> Vec<(CardId, PokemonId)> {
    if state.is_players_first_turn() {
        return Vec::new();
    }
    let side = state.player(player);
    let mut pairs = Vec::new();
    for card in &side.hand {
        let Some(from) = state
            .def_of(*card)
            .as_pokemon()
            .and_then(|p| p.evolves_from_basic)
        else {
            continue;
        };
        for target in side.in_play() {
            let evolution = state
                .def_of(*card)
                .as_pokemon()
                .expect("this arm only runs for a Pokémon card");
            let eligible = state.pokemon_def(target).name == from
                && (state.pokemon(target).played_on_turn < state.turn_number
                    || state.forest_of_vitality_applies(target, evolution))
                && !state.is_spent(Limit::Evolved(target))
                && !state.pokemon(target).cannot_evolve_this_turn;
            if eligible {
                pairs.push((*card, target));
            }
        }
    }
    pairs
}

/// Render an action the way the text interface shows it.
pub fn describe(state: &GameState, action: Action) -> String {
    match action {
        Action::PlayBasic { card } => {
            format!("Bench {}", state.def_of(card).name())
        }
        Action::Evolve { card, target } => format!(
            "Evolve {} into {}",
            state.pokemon_def(target).name,
            state.def_of(card).name()
        ),
        Action::AttachEnergy { card, target } => format!(
            "Attach {} to {}",
            state.def_of(card).name(),
            state.pokemon_def(target).name
        ),
        Action::PlayTool { card, target } => format!(
            "Attach {} to {}",
            state.def_of(card).name(),
            state.pokemon_def(target).name
        ),
        Action::Retreat { to } => {
            format!("Retreat, promoting {}", state.pokemon_def(to).name)
        }
        Action::Attack { index } => {
            let active = state
                .player(state.current)
                .active
                .expect("attacking needs an Active");
            let attack = &state.pokemon_def(active).attacks[index];
            format!("Attack: {} ({} damage)", attack.name, attack.base_damage)
        }
        Action::EndTurn => "End turn".to_string(),
        Action::Promote { pokemon } => {
            format!("Promote {}", state.pokemon_def(pokemon).name)
        }
        Action::PlayTrainer { card } => format!("Play {}", state.def_of(card).name()),
        Action::TakeCard { card } => format!("Take {}", state.def_of(card).name()),
        Action::TakeCardOnto { card, target } => format!(
            "Take {} and attach it to {}",
            state.def_of(card).name(),
            state.pokemon_def(target).name
        ),
        Action::FinishDeciding => "Stop taking cards".to_string(),
        Action::PayWithCard { card } => {
            format!("Discard {} to pay for the card", state.def_of(card).name())
        }
        Action::PayWithCardToBottomOfDeck { card } => {
            format!("Put {} on the bottom of the deck to pay", state.def_of(card).name())
        }
        Action::MoveEnergy { card, target } => format!(
            "Move {} to {}",
            state.def_of(card).name(),
            state.pokemon_def(target).name
        ),
        Action::MoveEnergyToActive { card } => {
            format!("Move {} to the Active", state.def_of(card).name())
        }
        Action::FinishMovingEnergyToActive => "Stop moving Energy".to_string(),
        Action::HealTarget { target } => format!("Heal {}", state.pokemon_def(target).name),
        Action::ChooseOption { first } => {
            format!("Choose the {} option", if first { "first" } else { "second" })
        }
        Action::DiscardFromHand { card } => {
            format!("Discard {} from that hand", state.def_of(card).name())
        }
        Action::FinishDiscardingFromHand => "Stop discarding from that hand".to_string(),
        Action::HealMegaEx { target } => format!("Heal {} fully", state.pokemon_def(target).name),
        Action::TakeFromBottomOfDeck { card } => {
            format!("Take {} from the bottom of the deck", state.def_of(card).name())
        }
        Action::DeclineBottomOfDeck => "Decline the bottom of the deck".to_string(),
        Action::ChooseDevolveTarget { target } => {
            format!("Devolve {}", state.pokemon_def(target).name)
        }
        Action::RemoveOneEvolutionCard => "Remove one evolution card".to_string(),
        Action::FinishDevolving => "Stop devolving".to_string(),
        Action::ChooseIdentitySwapTarget { target } => {
            format!("Swap out {}", state.pokemon_def(target).name)
        }
        Action::SwapIdentityWithDiscarded { card } => {
            format!("Swap in {}", state.def_of(card).name())
        }
        Action::MoveEnergyForHandheldFan { card, target } => format!(
            "Move {} to {}",
            state.def_of(card).name(),
            state.pokemon_def(target).name
        ),
        Action::AttachFromDiscardForPowerglass { card } => {
            format!("Attach {} from discard", state.def_of(card).name())
        }
        Action::DeclinePowerglass => "Decline Powerglass".to_string(),
        Action::PutOnTopOfDeckForAcademyAtNight { card } => {
            format!("Put {} on top of the deck", state.def_of(card).name())
        }
        Action::DrawTwoForTeamRocketsFactory => "Draw 2 (Team Rocket's Factory)".to_string(),
        Action::UseLumioseCity => "Search for a Basic Pokémon (Lumiose City)".to_string(),
        Action::PlaceDamageCounter { target } => {
            format!("Place a damage counter on {}", state.pokemon_def(target).name)
        }
        Action::DamageBenchedPokemon { target } => {
            format!("Damage {}", state.pokemon_def(target).name)
        }
        Action::TakeBasicPokemonForCallForFamily { card } => {
            format!("Bench {}", state.def_of(card).name())
        }
        Action::FinishCallForFamily => "Stop searching".to_string(),
        Action::TakeBasicPokemonOfTypeForEnergyAttach { card } => {
            format!("Bench {}", state.def_of(card).name())
        }
        Action::FinishSearchingBasicsOfType => "Stop searching".to_string(),
        Action::TakeItemFromDeck { card } => format!("Take {}", state.def_of(card).name()),
        Action::TakeAnyCardFromDeck { card } => format!("Take {}", state.def_of(card).name()),
        Action::FinishSearchingAnyCards => "Stop searching".to_string(),
        Action::MoveOpponentsActiveEnergyToHand { card } => {
            format!("Move {} to their hand", state.def_of(card).name())
        }
        Action::FinishMovingOpponentsActiveEnergyToHand => "Stop moving Energy".to_string(),
        Action::TakeTrainerFromDiscard { card } => {
            format!("Take {} from discard", state.def_of(card).name())
        }
        Action::EvolveWithAscension { card } => {
            format!("Evolve into {}", state.def_of(card).name())
        }
        Action::TakePokemonFromDiscard { card } => {
            format!("Take {} from discard", state.def_of(card).name())
        }
        Action::AcceptShuffleEnergyForBenchDamage => "Shuffle Energy for bench damage".to_string(),
        Action::DeclineShuffleEnergyForBenchDamage => "Decline".to_string(),
        Action::MoveOpponentsEnergy { card, target } => format!(
            "Move {} to {}",
            state.def_of(card).name(),
            state.pokemon_def(target).name
        ),
        Action::TakeNamedFromDiscardToBench { card } => {
            format!("Bench {} from discard", state.def_of(card).name())
        }
        Action::FinishSearchingDiscardForNamedToBench => "Stop searching".to_string(),
        Action::ChooseJaninesTarget { target } => {
            format!("Choose {}", state.pokemon_def(target).name)
        }
        Action::FinishChoosingJaninesTargets => "Stop choosing targets".to_string(),
        Action::TakeEnergyForJanine { card } => {
            format!("Attach {}", state.def_of(card).name())
        }
        Action::FinishJaninesSearch => "Move on".to_string(),
        Action::EvolveSkippingOneStage { card, target } => format!(
            "Use Rare Candy: evolve {} into {}",
            state.pokemon_def(target).name,
            state.def_of(card).name()
        ),
        Action::UsePrismTower => "Discard 2 to draw 1 (Prism Tower)".to_string(),
        Action::UseCommunityCenter => "Heal every Pokemon (Community Center)".to_string(),
        Action::UseLevincia => "Return Energy from discard (Levincia)".to_string(),
        Action::UseSpikemuthGym => "Search for a Marnie's Pokemon (Spikemuth Gym)".to_string(),
        Action::UseMysteryGarden => "Discard Energy to draw (Mystery Garden)".to_string(),
        Action::SwitchForSurfingBeach { target } => {
            format!("Switch in {} (Surfing Beach)", state.pokemon_def(target).name)
        }
        Action::PlaceDamageCountersOn { target } => {
            format!("Place damage counters on {}", state.pokemon_def(target).name)
        }
        Action::ReturnToHand { target } => {
            format!("Return {} to hand", state.pokemon_def(target).name)
        }
        Action::ProtectFromEx { target } => {
            format!("Protect {} from an ex next turn", state.pokemon_def(target).name)
        }
        Action::EvolveFromDeck { card, target } => format!(
            "Use Salvatore: evolve {} into {}",
            state.pokemon_def(target).name,
            state.def_of(card).name()
        ),
        Action::DiscardOpponentEnergy { card } => {
            format!("Discard the opponent's {}", state.def_of(card).name())
        }
        Action::DiscardOpponentSpecialEnergy { card } => {
            format!("Discard the opponent's {}", state.def_of(card).name())
        }
        Action::DiscardDefenderEnergyForAttack { card } => {
            format!("Discard the defender's {}", state.def_of(card).name())
        }
        Action::DiscardOwnEnergyForAttack { card } => {
            format!("Discard {}", state.def_of(card).name())
        }
        Action::AcceptDiscardOwnEnergyForBonusDamage => "Discard Energy for bonus damage".to_string(),
        Action::DeclineDiscardOwnEnergyForBonusDamage => "Decline the bonus damage".to_string(),
        Action::DiscardOwnEnergyForBonusDamage { card } => {
            format!("Discard {}", state.def_of(card).name())
        }
        Action::FinishDiscardingOwnEnergyForBonusDamage => "Stop discarding".to_string(),
        Action::DiscardBasicEnergyForDamagePerCard { card } => {
            format!("Discard {}", state.def_of(card).name())
        }
        Action::FinishDiscardingBasicEnergyForDamagePerCard => "Stop discarding".to_string(),
        Action::DiscardBenchedPokemon { pokemon } => {
            format!("Discard {} from the Bench", state.pokemon_def(pokemon).name)
        }
        Action::ChooseWhoGoesFirst { first } => format!("{first:?} takes the first turn"),
        Action::TakeBonusDraw => "Take a bonus card".to_string(),
        Action::DeclineBonusDraws => "Take no more bonus cards".to_string(),
        Action::PlaceActive { card } => {
            format!("Place {} as your Active", state.def_of(card).name())
        }
        Action::PlaceOnBench { card } => {
            format!("Place {} on your Bench", state.def_of(card).name())
        }
        Action::FinishPlacing => "Finish placing".to_string(),
        Action::DiscardEnergy { card } => {
            format!("Discard {} to retreat", state.def_of(card).name())
        }
        Action::ResolveCheckup { pokemon, condition } => format!(
            "Resolve {condition:?} on {}",
            state.pokemon_def(pokemon).name
        ),
        Action::UseAbility { pokemon } => {
            let ability = state.pokemon_def(pokemon).ability.expect("named only when carried");
            format!("Use {}'s {}", state.pokemon_def(pokemon).name, ability.name)
        }
        Action::TakeSupporterForLastDitchCatch { card } => {
            format!("Take {} (Last-Ditch Catch)", state.def_of(card).name())
        }
        Action::DeclineLastDitchCatch => "Decline Last-Ditch Catch".to_string(),
        Action::AcceptPsychicDraw => "Use Psychic Draw".to_string(),
        Action::DeclinePsychicDraw => "Decline Psychic Draw".to_string(),
        Action::AcceptJewelSeeker => "Use Jewel Seeker".to_string(),
        Action::DeclineJewelSeeker => "Decline Jewel Seeker".to_string(),
        Action::TakeTrainerCardFromDeck { card } => format!("Take {}", state.def_of(card).name()),
        Action::FinishSearchingTrainerCards => "Stop searching".to_string(),
        Action::TakePokemonOfTypeOrStadiumFromDeck { card } => {
            format!("Take {}", state.def_of(card).name())
        }
        Action::FinishSearchingPokemonOfTypeOrStadium => "Stop searching".to_string(),
        Action::DiscardCardFromOpponentsHand { card } => {
            format!("Discard {} from the opponent's hand", state.def_of(card).name())
        }
        Action::DamageChosenOpponentPokemon { target } => {
            format!("Damage {}", state.pokemon_def(target).name)
        }
        Action::DamageChosenOpponentPokemonWeaknessIfActive { target } => {
            format!("Damage {}", state.pokemon_def(target).name)
        }
        Action::ChooseOwnBenchedSourceForDamageMove { source } => {
            format!("Move damage from {}", state.pokemon_def(source).name)
        }
        Action::MoveDamageToChosenOpponentPokemon { target } => {
            format!("Move damage to {}", state.pokemon_def(target).name)
        }
        Action::ChooseBenchedTargetForEnergySearch { target } => {
            format!("Search Energy to attach to {}", state.pokemon_def(target).name)
        }
        Action::TakeEnergyOfTypeToAttachToChosen { card } => {
            format!("Take {}", state.def_of(card).name())
        }
        Action::FinishSearchingEnergyOfTypeToAttachToChosen => "Stop searching".to_string(),
        Action::DamageOneOfTwoChosenOpponentPokemon { target } => {
            format!("Damage {}", state.pokemon_def(target).name)
        }
        Action::DamageOneOfThreeChosenOpponentPokemon { target } => {
            format!("Damage {}", state.pokemon_def(target).name)
        }
        Action::CopyBenchedPokemonAttack { pokemon, index } => format!(
            "Copy {}'s {}",
            state.pokemon_def(pokemon).name,
            state.pokemon_def(pokemon).attacks[index].name
        ),
        Action::DiscardHandCardThenDraw { card } => {
            format!("Discard {}", state.def_of(card).name())
        }
        Action::TakeAnyCardFromDeckForAbility { card } => {
            format!("Take {}", state.def_of(card).name())
        }
        Action::FinishSearchingDeckForAnyCardAbility => "Stop searching".to_string(),
        Action::CopyDiscardedPokemonAttack { index } => {
            let card = match state.phase {
                Phase::ChoosingDiscardedPokemonAttackToCopy { card, .. } => card,
                _ => return "Copy an attack".to_string(),
            };
            let def = state.def_of(card).as_pokemon().expect("only opened for a Pokémon");
            format!("Copy {}'s {}", def.name, def.attacks[index].name)
        }
        Action::DamageBenchedEx { target } => {
            format!("Damage {}", state.pokemon_def(target).name)
        }
        Action::DamageAnyBenched { target } => {
            format!("Damage {}", state.pokemon_def(target).name)
        }
        Action::AttachSearchedEnergyTo { target } => {
            format!("Attach Energy to {}", state.pokemon_def(target).name)
        }
        Action::MoveDamageCountersFromOwnToOpponent { source, target, count } => format!(
            "Move {count} damage from {} to {}",
            state.pokemon_def(source).name,
            state.pokemon_def(target).name
        ),
        Action::DeclineMovingDamageCounters => "Decline moving damage counters".to_string(),
        Action::TakeCardFromTopPeek { card } => {
            format!("Take {} (Recon Directive)", state.def_of(card).name())
        }
        Action::AttachFoundEnergyTo { card, target } => format!(
            "Attach {} to {} (Metal Maker)",
            state.def_of(card).name(),
            state.pokemon_def(target).name
        ),
        Action::PutFoundCardOnBottom { card } => {
            format!("Bury {} (Metal Maker)", state.def_of(card).name())
        }
        Action::TakeSupporterFromTopPeek { card } => {
            format!("Take {} (Attract Customers)", state.def_of(card).name())
        }
        Action::DeclineTopPeekSupporter => "Decline Attract Customers".to_string(),
        Action::AttachEnergyForTealDance { card } => {
            format!("Attach {} (Teal Dance)", state.def_of(card).name())
        }
        Action::DeclineTealDance => "Decline Teal Dance".to_string(),
        Action::DamageOpponentForCursedBlast { target } => {
            format!("Damage {} (Cursed Blast)", state.pokemon_def(target).name)
        }
        Action::DeclineCursedBlast => "Decline Cursed Blast".to_string(),
        Action::TakeEvolutionPokemonOfType { card } => {
            format!("Take {}", state.def_of(card).name())
        }
        Action::FinishSearchingEvolutionPokemonOfType => "Stop searching".to_string(),
        Action::AttachEnergyForSeethingSpirit { card, target } => format!(
            "Attach {} to {} (Seething Spirit)",
            state.def_of(card).name(),
            state.pokemon_def(target).name
        ),
        Action::DeclineSeethingSpirit => "Decline Seething Spirit".to_string(),
        Action::AttachEnergyForRipeningCharge { card, target } => format!(
            "Attach {} to {} (Ripening Charge)",
            state.def_of(card).name(),
            state.pokemon_def(target).name
        ),
        Action::DeclineRipeningCharge => "Decline Ripening Charge".to_string(),
        Action::MoveOwnAttachedEnergyToHand { card } => {
            format!("Move {} to hand", state.def_of(card).name())
        }
        Action::MoveEnergyToChosenBenched { card, target } => format!(
            "Move {} to {}",
            state.def_of(card).name(),
            state.pokemon_def(target).name
        ),
        Action::AcceptSnowSink => "Discard the Stadium (Snow Sink)".to_string(),
        Action::DeclineSnowSink => "Decline Snow Sink".to_string(),
        Action::AcceptRapidVernierSwitch => "Switch in (Rapid Vernier)".to_string(),
        Action::DeclineRapidVernierSwitch => "Decline Rapid Vernier".to_string(),
        Action::MoveEnergyForRapidVernier { card } => {
            format!("Move {} (Rapid Vernier)", state.def_of(card).name())
        }
        Action::FinishMovingEnergyForRapidVernier => "Stop moving Energy".to_string(),
        Action::AttachSinisterSurgeEnergyTo { target } => {
            format!("Attach Energy to {} (Sinister Surge)", state.pokemon_def(target).name)
        }
        Action::TakeCardForFanCall { card } => format!("Take {} (Fan Call)", state.def_of(card).name()),
        Action::FinishFanCall => "Stop searching".to_string(),
        Action::SwitchForSubjugatingChains { target } => {
            format!("Switch in {} (Subjugating Chains)", state.pokemon_def(target).name)
        }
        Action::DiscardToolAnywhere { card } => {
            format!("Discard {} (Tool Scrapper)", state.def_of(card).name())
        }
        Action::FinishDiscardingToolsAnywhere => "Stop discarding Tools".to_string(),
    }
}
