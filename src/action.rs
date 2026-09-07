//! What a player may do, and what the engine will accept.
//!
//! [`legal_actions`] is the engine's primary interface. A human interface
//! numbers the list and reads a choice; a bot is
//! `(state, legal_actions) -> Action`. Neither can reach a state the other
//! cannot, and neither can cheat by acting outside the list.

use crate::card::{Condition, Destination, Requirement, TrainerEffect, TrainerKind};
use crate::ids::{CardId, PlayerId, PokemonId};
use crate::state::{BENCH_LIMIT, GameState, Limit, Phase};

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
    /// Discard one card from hand toward what a card demanded to be played.
    PayWithCard { card: CardId },
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
    /// Take this Pokémon found at the bottom of the Library.
    /// `Phase::LookingAtBottomOfLibrary` names the search it ends.
    TakeFromBottomOfLibrary { card: CardId },
    /// Decline every Pokémon `Phase::LookingAtBottomOfLibrary` found; the
    /// Library still shuffles.
    DeclineBottomOfLibrary,
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
    /// on top of the Library.
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
    /// Put one damage counter on this Benched Pokémon, as part of
    /// `Phase::DistributingDamageCounters`.
    PlaceDamageCounter { target: PokemonId },
    /// Deal `Phase::ChoosingBenchDamageTarget`'s flat damage to this
    /// Benched Pokémon.
    DamageBenchedPokemon { target: PokemonId },
    /// Take this Basic Pokémon from the library onto the Bench, as part
    /// of `Phase::SearchingLibraryForBasics`.
    TakeBasicPokemonForCallForFamily { card: CardId },
    /// Stop `Phase::SearchingLibraryForBasics` before its limit is
    /// spent.
    FinishCallForFamily,
    /// Move this Energy from the opponent's Active into their hand, as
    /// part of `Phase::MovingOpponentsActiveEnergyToHand`.
    MoveOpponentsActiveEnergyToHand { card: CardId },
    /// Stop `Phase::MovingOpponentsActiveEnergyToHand` before its
    /// limit is spent — the effect is optional ("may").
    FinishMovingOpponentsActiveEnergyToHand,
    /// Take this Trainer card from the discard pile into hand, as part
    /// of `Phase::TakingTrainerFromDiscard`.
    TakeTrainerFromDiscard { card: CardId },
    /// Evolve into this card from the library, as part of
    /// `Phase::SearchingLibraryToEvolveSelf`.
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
    /// Take this Supporter card from the library into hand, as part
    /// of `Phase::DecidingToUseLastDitchCatch`.
    TakeSupporterForLastDitchCatch { card: CardId },
    /// Decline it — nothing else about this play changes.
    DeclineLastDitchCatch,
    /// Accept `Phase::DecidingToUsePsychicDraw`'s draw.
    AcceptPsychicDraw,
    /// Decline it.
    DeclinePsychicDraw,
    /// Deal `Phase::ChoosingAnyOpponentPokemonDamageTarget`'s flat
    /// damage to this Pokémon, Active or Benched.
    DamageChosenOpponentPokemon { target: PokemonId },
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
    /// Take this Evolution Pokémon from the library into hand, as
    /// part of `Phase::SearchingLibraryForEvolutionPokemonOfType`.
    TakeEvolutionPokemonOfType { card: CardId },
    /// Stop that search before its limit is spent.
    FinishSearchingEvolutionPokemonOfType,
    /// Attach this Energy from the discard pile to this Pokémon, as
    /// part of `Phase::DecidingToUseSeethingSpirit`.
    AttachEnergyForSeethingSpirit { card: CardId, target: PokemonId },
    /// Decline it.
    DeclineSeethingSpirit,
    /// Move this attached Energy to hand, as part of
    /// `Phase::ChoosingOwnEnergyToHand`.
    MoveOwnAttachedEnergyToHand { card: CardId },
    /// Accept `Phase::DecidingToUseSnowSink`'s discard.
    AcceptSnowSink,
    /// Decline it.
    DeclineSnowSink,
}

/// Whose choice the engine is waiting for. It is not always the player whose
/// turn it is: a knockout hands the choice to the player who lost the Pokémon.
pub fn player_to_act(state: &GameState) -> Option<PlayerId> {
    match state.phase {
        Phase::Main => Some(state.current),
        Phase::Promoting { chooser, .. } => Some(chooser),
        Phase::ChoosingWhoGoesFirst { winner } => Some(winner),
        Phase::TakingBonusDraws { player, .. } => Some(player),
        Phase::PlacingActive { player } => Some(player),
        Phase::PlacingBench { player } => Some(player),
        Phase::DiscardingForRetreat { player, .. } => Some(player),
        Phase::Deciding { chooser, .. } => Some(chooser),
        Phase::Paying { player, .. } => Some(player),
        Phase::MovingEnergy { player } => Some(player),
        Phase::MovingEnergyFromBenchToActive { player, .. } => Some(player),
        Phase::HealingChosen { player, .. } => Some(player),
        Phase::ChoosingOneOf { player, .. } => Some(player),
        Phase::DiscardingFromHand { chooser, .. } => Some(chooser),
        Phase::HealingMegaEx { player } => Some(player),
        Phase::LookingAtBottomOfLibrary { player, .. } => Some(player),
        Phase::Devolving { player, .. } => Some(player),
        Phase::SwappingIdentity { player, .. } => Some(player),
        Phase::MovingEnergyForHandheldFan { chooser, .. } => Some(chooser),
        Phase::AttachingFromDiscardForPowerglass { player } => Some(player),
        Phase::DistributingDamageCounters { player, .. } => Some(player),
        Phase::ChoosingBenchDamageTarget { player, .. } => Some(player),
        Phase::SearchingLibraryForBasics { player, .. } => Some(player),
        Phase::MovingOpponentsActiveEnergyToHand { player, .. } => Some(player),
        Phase::TakingTrainerFromDiscard { player } => Some(player),
        Phase::SearchingLibraryToEvolveSelf { player, .. } => Some(player),
        Phase::TakingPokemonFromDiscard { player } => Some(player),
        Phase::DecidingToShuffleEnergyForBenchDamage { player, .. } => Some(player),
        Phase::DecidingToUseLastDitchCatch { player, .. } => Some(player),
        Phase::DecidingToUsePsychicDraw { player, .. } => Some(player),
        Phase::ChoosingAnyOpponentPokemonDamageTarget { player, .. } => Some(player),
        Phase::DecidingToUseTealDance { player, .. } => Some(player),
        Phase::DecidingCursedBlastTarget { player, .. } => Some(player),
        Phase::SearchingLibraryForEvolutionPokemonOfType { player, .. } => Some(player),
        Phase::DecidingToUseSeethingSpirit { player, .. } => Some(player),
        Phase::ChoosingOwnEnergyToHand { player, .. } => Some(player),
        Phase::DecidingToUseSnowSink { player, .. } => Some(player),
        Phase::MovingOpponentsEnergy { chooser, .. } => Some(chooser),
        Phase::SearchingDiscardForNamedToBench { player, .. } => Some(player),
        Phase::ChoosingJaninesTargets { player, .. } => Some(player),
        Phase::JaninesSearch { player, .. } => Some(player),
        Phase::EvolvingWithRareCandy { player } => Some(player),
        Phase::DiscardingOpponentEnergy { chooser, .. } => Some(chooser),
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
            if side.bench.len() < BENCH_LIMIT {
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
                Destination::Bench => state.player(chooser).bench.len() < BENCH_LIMIT,
                Destination::Attach(target_filter) => state
                    .player(chooser)
                    .in_play()
                    .into_iter()
                    .any(|p| state.matches_target(chooser, p, target_filter)),
                Destination::Zone(_) | Destination::TopOfLibraryInOrder => true,
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
                    if to == Destination::TopOfLibraryInOrder && Some(*card) == previous {
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
                        | Destination::TopOfLibraryInOrder => {
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
        Phase::HealingChosen { player: whose, .. } => {
            for target in state.player(whose).in_play() {
                actions.push(Action::HealTarget { target });
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
        Phase::LookingAtBottomOfLibrary { player: whose, count } => {
            let library = &state.player(whose).library;
            for card in library.iter().take(count as usize) {
                if state.matches_filter(*card, crate::card::CardFilter::AnyPokemon) {
                    actions.push(Action::TakeFromBottomOfLibrary { card: *card });
                }
            }
            actions.push(Action::DeclineBottomOfLibrary);
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
                actions.push(Action::PlaceDamageCounter { target: *pokemon });
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
        Phase::SearchingLibraryForBasics { player: whose, .. } => {
            for card in &state.player(whose).library {
                if state.matches_filter(*card, crate::card::CardFilter::PokemonOfStage(crate::card::Stage::Basic)) {
                    actions.push(Action::TakeBasicPokemonForCallForFamily { card: *card });
                }
            }
            actions.push(Action::FinishCallForFamily);
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
            for card in &state.player(whose).library {
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
        Phase::ChoosingAnyOpponentPokemonDamageTarget { player: whose, .. } => {
            for pokemon in state.player(whose.opponent()).in_play() {
                actions.push(Action::DamageChosenOpponentPokemon { target: pokemon });
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
        Phase::SearchingLibraryForEvolutionPokemonOfType { player: whose, kind, .. } => {
            for card in &state.player(whose).library {
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
        Phase::ChoosingOwnEnergyToHand { attacker, .. } => {
            for card in &state.pokemon(attacker).attached {
                if state.def_of(*card).is_energy() {
                    actions.push(Action::MoveOwnAttachedEnergyToHand { card: *card });
                }
            }
            return actions;
        }
        Phase::DecidingToUseSnowSink { .. } => {
            actions.push(Action::AcceptSnowSink);
            actions.push(Action::DeclineSnowSink);
            return actions;
        }
        Phase::MovingOpponentsEnergy { of, .. } => {
            let in_play = state.player(of).in_play();
            for from in &in_play {
                for card in &state.pokemon(*from).attached {
                    if !state.def_of(*card).is_energy() {
                        continue;
                    }
                    for target in &in_play {
                        if target != from {
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
        Phase::SearchingLibraryToEvolveSelf { player: whose, target } => {
            let from = state.pokemon_def(target).name;
            for card in &state.player(whose).library {
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
                for card in state.player(whose).library.iter() {
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
        Phase::EvolvingWithRareCandy { player: whose } => {
            for (card, target) in rare_candy_pairs(state, whose) {
                actions.push(Action::EvolveSkippingOneStage { card, target });
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

    for card in &side.hand {
        let def = state.def_of(*card);
        if def.is_basic_pokemon() && side.bench.len() < BENCH_LIMIT {
            actions.push(Action::PlayBasic { card: *card });
        }
        // A Tool attaches like Energy does — immediately, with a target —
        // not through PlayTrainer, which never names one. Unlike Energy,
        // a Pokémon carries at most one: rule text every Tool print
        // shares, not read from any one card's own effect.
        if def.as_trainer().is_some_and(|t| t.kind == TrainerKind::Tool) {
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
        if def.is_energy() && !state.is_spent(Limit::EnergyAttached(player)) {
            for target in side.in_play() {
                actions.push(Action::AttachEnergy {
                    card: *card,
                    target,
                });
            }
        }
        // Rules 18-20: not on the first turn of the game, only onto the
        // Pokémon this card names, only if it has been in play since the
        // start of the turn, and only once per Pokémon per turn.
        if let Some(from) = def.as_pokemon().and_then(|p| p.evolve_from)
            && !state.is_first_turn_of_game()
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
            let timing = match trainer.kind {
                TrainerKind::Item => !cannot_play_items,
                TrainerKind::Tool => unreachable!("filtered out above"),
                TrainerKind::Supporter => {
                    !state.is_spent(Limit::SupporterPlayed(player))
                        && !state.is_first_turn_of_game()
                }
                TrainerKind::Stadium => !state.is_spent(Limit::StadiumPlayed(player)),
            };
            // A card that switches the opponent's Active needs somewhere to
            // switch to; every other effect built so far can always be
            // attempted, even where it turns up nothing to move.
            let has_a_target = match trainer.effect {
                TrainerEffect::SwitchOpponentActive => {
                    !state.player(player.opponent()).bench.is_empty()
                }
                TrainerEffect::SwitchOwnActive | TrainerEffect::SwitchOwnActiveWithFollowUp(_) => {
                    !side.bench.is_empty()
                }
                TrainerEffect::SwitchOpponentActiveThenOwn => {
                    !state.player(player.opponent()).bench.is_empty()
                }
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
                Some(Requirement::SecondCopyOfThisInHand) => {
                    let def = state.cards[card.index()].def;
                    side.hand
                        .iter()
                        .filter(|c| state.cards[c.index()].def == def)
                        .count()
                        > 1
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
        );
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
            for (index, attack) in state.pokemon_def(active).attacks.iter().enumerate() {
                if state.pays_cost(active, &attack.cost) {
                    actions.push(Action::Attack { index });
                }
            }
        }
    }

    // An Ability, offered once its own gates all hold: whose turn it
    // is (already true — `legal_actions` only ever builds this list
    // for `player_to_act`), which Pokémon it demands, and whether the
    // player has already used one with this name this turn.
    for pokemon in side.in_play() {
        let Some(ability) = state.pokemon_def(pokemon).ability else {
            continue;
        };
        if state.is_spent(Limit::AbilityUsed(player, ability.name)) {
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
            crate::card::AbilityEffect::OncePerTurnMayDamageOpponentThenKnockOutSelf(_) => true,
            crate::card::AbilityEffect::OncePerTurnMaySearchEvolutionPokemonOfType(kind, _) => {
                side.library.iter().any(|c| {
                    state.matches_filter(*c, crate::card::CardFilter::EvolutionPokemonOfType(kind))
                })
            }
            crate::card::AbilityEffect::OncePerTurnMayAttachBasicEnergyFromDiscardToChosen => {
                side.discard.iter().any(|c| state.def_of(*c).is_energy())
            }
            // Triggered the moment this Pokémon is played from hand
            // (`trigger_last_ditch_catch`), never a standing choice.
            crate::card::AbilityEffect::WhenBenchedFromHandMaySearchSupporter => false,
            // Triggered the moment this Pokémon evolves from hand
            // (`trigger_psychic_draw`), never a standing choice.
            crate::card::AbilityEffect::WhenEvolvedFromHandMayDrawCards(_) => false,
            // Triggered the moment this Pokémon is played from hand
            // (`trigger_snow_sink`), never a standing choice.
            crate::card::AbilityEffect::WhenBenchedFromHandMayDiscardStadium => false,
        };
        if eligible {
            actions.push(Action::UseAbility { pokemon });
        }
    }

    actions.push(Action::EndTurn);
    actions
}

/// Every Stage 2 in hand and Basic in play that `Rare Candy` may pair: the
/// same rules 18-20 an ordinary evolution reads — not the first turn of the
/// game, the target in play since before this turn, not yet evolved this
/// turn — matched by `evolves_from_basic` two links down rather than by
/// `evolve_from` one link up.
fn rare_candy_pairs(state: &GameState, player: PlayerId) -> Vec<(CardId, PokemonId)> {
    if state.is_first_turn_of_game() {
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
        Action::TakeFromBottomOfLibrary { card } => {
            format!("Take {} from the bottom of the library", state.def_of(card).name())
        }
        Action::DeclineBottomOfLibrary => "Decline the bottom of the library".to_string(),
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
        Action::DiscardOpponentEnergy { card } => {
            format!("Discard the opponent's {}", state.def_of(card).name())
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
        Action::DamageChosenOpponentPokemon { target } => {
            format!("Damage {}", state.pokemon_def(target).name)
        }
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
        Action::MoveOwnAttachedEnergyToHand { card } => {
            format!("Move {} to hand", state.def_of(card).name())
        }
        Action::AcceptSnowSink => "Discard the Stadium (Snow Sink)".to_string(),
        Action::DeclineSnowSink => "Decline Snow Sink".to_string(),
    }
}
