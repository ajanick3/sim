//! Applying an action to the state.

use crate::action::{Action, legal_actions};
use crate::card::{CardDb, Condition, Destination, Requirement, TrainerEffect, TrainerKind, Zone};
use crate::ids::{CardDefId, CardId, PlayerId, PokemonId};
use crate::rng::{Rng, shuffle};
use crate::state::{GameState, Limit, Outcome, Phase, WinReason};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IllegalAction;

/// Apply one action. An action outside [`legal_actions`] is refused and the
/// state does not move.
pub fn apply(state: &mut GameState, action: Action) -> Result<(), IllegalAction> {
    if !legal_actions(state).contains(&action) {
        return Err(IllegalAction);
    }

    match action {
        Action::ChooseWhoGoesFirst { first } => {
            state.current = first;
            state.log.push(format!("{first:?} takes the first turn."));
            advance_setup(state);
        }

        Action::TakeBonusDraw => {
            let player = setup_player(state)?;
            state.draw(player);
            state.bonus_draws[player.index()] -= 1;
            state.log.push(format!("{player:?} takes a bonus card."));
            advance_setup(state);
        }

        Action::DeclineBonusDraws => {
            let player = setup_player(state)?;
            state.bonus_draws[player.index()] = 0;
            advance_setup(state);
        }

        Action::PlaceActive { card } => {
            let player = setup_player(state)?;
            state.remove_from_hand(player, card);
            let pokemon = state.put_into_play(player, card);
            state.players[player.index()].active = Some(pokemon);
            advance_setup(state);
        }

        Action::PlaceOnBench { card } => {
            let player = setup_player(state)?;
            state.remove_from_hand(player, card);
            let pokemon = state.put_into_play(player, card);
            state.players[player.index()].bench.push(pokemon);
            advance_setup(state);
        }

        Action::FinishPlacing => {
            let player = setup_player(state)?;
            state.bench_placed[player.index()] = true;
            advance_setup(state);
        }

        Action::PlayBasic { card } => {
            let player = state.current;
            state.remove_from_hand(player, card);
            let pokemon = state.put_into_play(player, card);
            state.players[player.index()].bench.push(pokemon);
            let name = state.def_of(card).name();
            state.log.push(format!("{player:?} benches {name}."));
            apply_risky_ruins(state, pokemon);
        }

        Action::Evolve { card, target } => {
            let player = state.current;
            state.remove_from_hand(player, card);
            state.pokemon[target.index()].cards.push(card);
            state.spend(Limit::Evolved(target));
            // Rule 22: evolving clears every Special Condition. Damage and
            // attachments are untouched — nothing here moves them.
            state.clear_conditions(target);
            let name = state.pokemon_def(target).name;
            state.log.push(format!("{player:?} evolves into {name}."));
        }

        Action::PlayTrainer { card } => {
            let player = state.current;
            let trainer = state
                .def_of(card)
                .as_trainer()
                .expect("legal_actions offers PlayTrainer only for a Trainer")
                .clone();
            state.remove_from_hand(player, card);
            match trainer.kind {
                TrainerKind::Supporter => {
                    state.spend(Limit::SupporterPlayed(player));
                    state.players[player.index()].discard.push(card);
                    // `Team Rocket's Factory` reads this the same turn.
                    if trainer.name.contains("Team Rocket") {
                        state.played_a_team_rocket_supporter_this_turn[player.index()] = true;
                    }
                }
                // Rule 58: a Stadium stays in play, and the one already
                // there goes to its own owner's discard, not to this
                // player's.
                TrainerKind::Stadium => {
                    state.spend(Limit::StadiumPlayed(player));
                    if let Some((owner, old)) = state.stadium {
                        state.players[owner.index()].discard.push(old);
                    }
                    state.stadium = Some((player, card));
                }
                TrainerKind::Tool => {
                    unreachable!("legal_actions offers a Tool only through PlayTool")
                }
                TrainerKind::Item => {
                    state.players[player.index()].discard.push(card);
                }
            }
            let name = trainer.name;
            state.log.push(format!("{player:?} plays {name}."));
            // A cost is paid before the effect runs. The phase names the
            // card, so the effect is read back from it when the cost is met.
            match trainer.requirement {
                Some(Requirement::DiscardOtherCardsFromHand(count)) => {
                    state.phase = Phase::Paying {
                        player,
                        card,
                        remaining: count,
                    };
                }
                // A requirement read from the board, or from history,
                // costs nothing, and `legal_actions` has already checked it.
                None
                | Some(
                    Requirement::OpponentPrizesAtMost(_)
                    | Requirement::KnockedOutDuringOpponentsLastTurn
                    | Requirement::ActiveHasAtLeastEnergy(_)
                    | Requirement::MorePrizesThanOpponent
                    | Requirement::HandSizeIs(_),
                ) => {
                    resolve_trainer(state, player, card, trainer.effect);
                }
                // The cost is a second physical copy of this same card,
                // not a choice among other cards — `legal_actions` already
                // confirmed one sits in hand, so there is nothing to ask.
                Some(Requirement::SecondCopyOfThisInHand) => {
                    let def = state.cards[card.index()].def;
                    let second = *state.players[player.index()]
                        .hand
                        .iter()
                        .find(|c| state.cards[c.index()].def == def)
                        .expect("legal_actions confirmed a second copy");
                    state.players[player.index()].hand.retain(|c| *c != second);
                    state.players[player.index()].discard.push(second);
                    resolve_trainer(state, player, card, trainer.effect);
                }
            }
        }

        Action::AttachEnergy { card, target } => {
            let player = state.current;
            state.remove_from_hand(player, card);
            state.pokemon[target.index()].attached.push(card);
            state.spend(Limit::EnergyAttached(player));
            let energy = state.def_of(card).name();
            let name = state.pokemon_def(target).name;
            state
                .log
                .push(format!("{player:?} attaches {energy} to {name}."));
            if state.immune_under_festival_grounds(target) {
                state.clear_conditions(target);
            }
        }

        Action::PlayTool { card, target } => {
            let player = state.current;
            state.remove_from_hand(player, card);
            state.pokemon[target.index()].attached.push(card);
            let tool = state.def_of(card).name();
            let name = state.pokemon_def(target).name;
            state
                .log
                .push(format!("{player:?} attaches {tool} to {name}."));
        }

        Action::Retreat { to } => retreat(state, to),

        Action::DiscardEnergy { card } => {
            let Phase::DiscardingForRetreat {
                player,
                to,
                remaining,
            } = state.phase
            else {
                return Err(IllegalAction);
            };
            let active = state.players[player.index()]
                .active
                .expect("a retreat starts from an Active");
            state.pokemon[active.index()]
                .attached
                .retain(|c| *c != card);
            state.players[player.index()].discard.push(card);

            if remaining > 1 {
                state.phase = Phase::DiscardingForRetreat {
                    player,
                    to,
                    remaining: remaining - 1,
                };
            } else {
                promote_from_retreat(state, player, to);
            }
        }

        Action::Attack { index } => {
            attack(state, index);
            state.pending_end_turn = true;
            // Handheld Fan can open a phase of its own, mid-attack, that
            // needs the chooser's own action before anything else moves
            // on — settle waits for that the same way it already waits
            // out any other phase a card opens.
            if state.phase == Phase::Main {
                settle(state);
            }
        }

        Action::EndTurn => {
            state.pending_end_turn = true;
            settle(state);
        }

        Action::ResolveCheckup { pokemon, condition } => {
            resolve_checkup(state, pokemon, condition);
            settle(state);
        }

        Action::Promote { pokemon } => {
            let (of, chooser, then) = match state.phase {
                Phase::Promoting { of, chooser, then } => (of, chooser, then),
                _ => return Err(IllegalAction),
            };
            let side = &mut state.players[of.index()];
            side.bench.retain(|p| *p != pokemon);
            // A knockout already cleared the old Active to `None` before
            // this phase opened, so there is nothing here to lose. A live
            // switch — `Switch`, `Boss's Orders` — has not: its Active is
            // displaced, not gone, and belongs back on the Bench.
            let displaced = side.active.replace(pokemon);
            if let Some(displaced) = displaced {
                side.bench.push(displaced);
            }
            state.phase = Phase::Main;
            let name = state.pokemon_def(pokemon).name;
            if of == chooser {
                state.log.push(format!("{of:?} promotes {name}."));
            } else {
                state
                    .log
                    .push(format!("{chooser:?} sends up {name} for {of:?}."));
            }
            // A live switch's own follow-up reads what was just displaced —
            // never present for a knockout or `Boss's Orders`.
            match then {
                Some(crate::card::PromoteFollowUp::HealDisplacedIfEx(amount)) => {
                    if let Some(displaced) = displaced
                        && state.pokemon_def(displaced).prizes > 1
                    {
                        state.pokemon[displaced.index()].damage =
                            state.pokemon(displaced).damage.saturating_sub(amount);
                    }
                }
                Some(crate::card::PromoteFollowUp::DrawUpTo(target)) => {
                    while state.player(chooser).hand.len() < target as usize {
                        if !state.draw(chooser) {
                            break;
                        }
                    }
                }
                Some(crate::card::PromoteFollowUp::AlsoSwitchOwnActive) => {
                    if !state.player(chooser).bench.is_empty() {
                        state.phase = Phase::Promoting {
                            of: chooser,
                            chooser,
                            then: None,
                        };
                        // `settle` resets an unclaimed phase back to Main,
                        // the same trap ADR 0030 fixed for a paid cost's
                        // effect — skip it here, the same way, so the
                        // follow-up switch this just opened survives.
                        return Ok(());
                    }
                }
                None => {}
            }
            settle(state);
        }

        Action::TakeCard { card } => {
            let (chooser, played, step, from, to, filter, excludes, peek, remaining, moved, then) =
                match state.phase {
                    Phase::Deciding {
                        chooser,
                        card: played,
                        step,
                        from,
                        to,
                        filter,
                        excludes_type_of_previous,
                        peek,
                        remaining,
                        moved,
                        then,
                        ..
                    } => (
                        chooser,
                        played,
                        step,
                        from,
                        to,
                        filter,
                        excludes_type_of_previous,
                        peek,
                        remaining,
                        moved,
                        then,
                    ),
                    _ => return Err(IllegalAction),
                };
            let name = state.def_of(card).name();
            match to {
                Destination::Zone(zone) => {
                    state.move_card(chooser, card, from, zone);
                    state.log.push(format!("{chooser:?} takes {name}."));
                }
                Destination::Bench => {
                    // The Bench holds Pokémon, not cards, so the card leaves
                    // its zone and comes into play the same way a Basic
                    // played from hand does.
                    state.zone_mut(chooser, from).retain(|c| *c != card);
                    let pokemon = state.put_into_play(chooser, card);
                    state.players[chooser.index()].bench.push(pokemon);
                    state.log.push(format!("{chooser:?} benches {name}."));
                    apply_risky_ruins(state, pokemon);
                }
                // `legal_actions` never offers `TakeCard` for a slot bound
                // to attach: that needs a target, which is `TakeCardOnto`.
                Destination::Attach(_) => unreachable!("Attach is taken with a target"),
                Destination::TopOfLibraryInOrder => {
                    // The library was already shuffled when this slot
                    // opened; each card taken lands right back on top,
                    // undisturbed, in the order it was taken.
                    state.zone_mut(chooser, from).retain(|c| *c != card);
                    state.players[chooser.index()].library.push(card);
                    state
                        .log
                        .push(format!("{chooser:?} puts {name} on top of the deck."));
                }
            }
            // A slot whose limit runs out does not move the search on by
            // itself. ADR 0012 keeps the choice to stop with the player, and
            // that holds slot by slot: `FinishDeciding` ends the slot, and
            // the search goes to the next one there.
            state.phase = Phase::Deciding {
                chooser,
                card: played,
                step,
                from,
                to,
                filter,
                excludes_type_of_previous: excludes,
                peek,
                remaining: remaining - 1,
                moved: moved + 1,
                previous: Some(card),
                then,
            };
        }

        Action::TakeCardOnto { card, target } => {
            let (chooser, played, step, from, to, filter, excludes, peek, remaining, moved, then) =
                match state.phase {
                    Phase::Deciding {
                        chooser,
                        card: played,
                        step,
                        from,
                        to: to @ Destination::Attach(_),
                        filter,
                        excludes_type_of_previous,
                        peek,
                        remaining,
                        moved,
                        then,
                        ..
                    } => (
                        chooser,
                        played,
                        step,
                        from,
                        to,
                        filter,
                        excludes_type_of_previous,
                        peek,
                        remaining,
                        moved,
                        then,
                    ),
                    _ => return Err(IllegalAction),
                };
            state.zone_mut(chooser, from).retain(|c| *c != card);
            state.pokemon[target.index()].attached.push(card);
            let energy = state.def_of(card).name();
            let name = state.pokemon_def(target).name;
            state
                .log
                .push(format!("{chooser:?} attaches {energy} to {name}."));
            state.phase = Phase::Deciding {
                chooser,
                card: played,
                step,
                from,
                to,
                filter,
                excludes_type_of_previous: excludes,
                peek,
                remaining: remaining - 1,
                moved: moved + 1,
                previous: Some(card),
                then,
            };
        }

        Action::FinishDeciding => {
            // Declining a slot moves the search on rather than ending it: a
            // card that asks for one of each does not demand that the deck
            // holds every one.
            let (chooser, played, step, from, moved, previous, then) = match state.phase {
                Phase::Deciding {
                    chooser,
                    card: played,
                    step,
                    from,
                    moved,
                    previous,
                    then,
                    ..
                } => (chooser, played, step, from, moved, previous, then),
                _ => return Err(IllegalAction),
            };
            enter_slot(
                state,
                chooser,
                played,
                step + 1,
                from,
                then,
                Progress { moved, previous },
            );
        }

        Action::PayWithCard { card } => {
            let (player, played, remaining) = match state.phase {
                Phase::Paying {
                    player,
                    card: played,
                    remaining,
                } => (player, played, remaining),
                _ => return Err(IllegalAction),
            };
            state.remove_from_hand(player, card);
            state.players[player.index()].discard.push(card);
            let name = state.def_of(card).name();
            state
                .log
                .push(format!("{player:?} discards {name} to pay."));
            if remaining > 1 {
                state.phase = Phase::Paying {
                    player,
                    card: played,
                    remaining: remaining - 1,
                };
            } else {
                let effect = state
                    .def_of(played)
                    .as_trainer()
                    .expect("a cost is only ever paid for a Trainer")
                    .effect
                    .clone();
                // The cost is paid; `Phase::Paying` has nothing left to
                // say. Clear it before the effect runs, the same as
                // `PlayTrainer` leaves `Main` behind it: an effect that
                // opens its own phase overwrites this in the same step,
                // and one that does not — `Morty's Conviction` is the
                // first — would otherwise leave `Paying` stale forever.
                state.phase = Phase::Main;
                // No `settle` here, the same as playing the card itself:
                // the effect either opened a phase of its own or finished,
                // and both are settled where the phase ends.
                resolve_trainer(state, player, played, effect);
            }
        }

        Action::MoveEnergy { card, target } => {
            let player = match state.phase {
                Phase::MovingEnergy { player } => player,
                _ => return Err(IllegalAction),
            };
            for pokemon in state.player(player).in_play() {
                state.pokemon[pokemon.index()]
                    .attached
                    .retain(|c| *c != card);
            }
            state.pokemon[target.index()].attached.push(card);
            let energy = state.def_of(card).name();
            let name = state.pokemon_def(target).name;
            state.log.push(format!("{energy} moves to {name}."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::MoveEnergyToActive { card } => {
            let (player, remaining) = match state.phase {
                Phase::MovingEnergyFromBenchToActive { player, remaining } => (player, remaining),
                _ => return Err(IllegalAction),
            };
            let active = state
                .player(player)
                .active
                .expect("a move onto the Active needs one in play");
            for pokemon in state.player(player).bench.clone() {
                state.pokemon[pokemon.index()].attached.retain(|c| *c != card);
            }
            state.pokemon[active.index()].attached.push(card);
            let energy = state.def_of(card).name();
            state.log.push(format!("{energy} moves to the Active."));
            // ADR 0012 keeps the choice to stop with the player, even at
            // the limit: `FinishMovingEnergyToActive` ends the phase.
            state.phase = Phase::MovingEnergyFromBenchToActive {
                player,
                remaining: remaining - 1,
            };
        }

        Action::FinishMovingEnergyToActive => {
            match state.phase {
                Phase::MovingEnergyFromBenchToActive { .. } => {}
                _ => return Err(IllegalAction),
            }
            state.phase = Phase::Main;
            settle(state);
        }

        Action::HealTarget { target } => {
            let amount = match state.phase {
                Phase::HealingChosen { amount, .. } => amount,
                _ => return Err(IllegalAction),
            };
            state.pokemon[target.index()].damage =
                state.pokemon(target).damage.saturating_sub(amount);
            state.clear_conditions(target);
            let name = state.pokemon_def(target).name;
            state.log.push(format!("{name} is healed."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::ChooseOption { first } => {
            let (player, played) = match state.phase {
                Phase::ChoosingOneOf { player, card } => (player, card),
                _ => return Err(IllegalAction),
            };
            let chosen = match state
                .def_of(played)
                .as_trainer()
                .expect("a choice is only ever a Trainer's effect")
                .effect
                .clone()
            {
                TrainerEffect::ChooseOneOf(a, b) => {
                    if first {
                        *a
                    } else {
                        *b
                    }
                }
                _ => unreachable!("Phase::ChoosingOneOf only ever names a ChooseOneOf card"),
            };
            // The phase clears before the chosen branch runs, the same
            // as playing the card itself: a branch with no phase of its
            // own leaves Main behind it, and one that opens a phase
            // overwrites this in the same step.
            state.phase = Phase::Main;
            resolve_trainer(state, player, played, chosen);
        }

        Action::DiscardFromHand { card } => {
            let (chooser, of, filter, remaining, then) = match state.phase {
                Phase::DiscardingFromHand { chooser, of, filter, remaining, then } => {
                    (chooser, of, filter, remaining, then)
                }
                _ => return Err(IllegalAction),
            };
            state.players[of.index()].hand.retain(|c| *c != card);
            state.players[of.index()].discard.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{chooser:?} discards {name}."));
            state.phase = Phase::DiscardingFromHand {
                chooser,
                of,
                filter,
                remaining: remaining - 1,
                then,
            };
        }

        Action::FinishDiscardingFromHand => {
            let (chooser, then) = match state.phase {
                Phase::DiscardingFromHand { chooser, then, .. } => (chooser, then),
                _ => return Err(IllegalAction),
            };
            match then {
                None => {
                    state.phase = Phase::Main;
                    settle(state);
                }
                Some(crate::card::DiscardFollowUp::AlsoDiscardOwnHandDownTo(target)) => {
                    // `chooser` here is still the player whose discard just
                    // ended (the opponent, for `Hand Trimmer`); the follow-up
                    // is the other player trimming their own hand next.
                    let next = chooser.opponent();
                    let hand_len = state.player(next).hand.len() as u32;
                    state.phase = Phase::DiscardingFromHand {
                        chooser: next,
                        of: next,
                        filter: crate::card::CardFilter::AnyCard,
                        remaining: hand_len.saturating_sub(target),
                        then: None,
                    };
                }
            }
        }

        Action::HealMegaEx { target } => {
            let player = match state.phase {
                Phase::HealingMegaEx { player } => player,
                _ => return Err(IllegalAction),
            };
            let healed = state.pokemon(target).damage > 0;
            state.pokemon[target.index()].damage = 0;
            if healed {
                // "All Energy attached to it" — not a Tool sitting in the
                // same `attached` list. `std::mem::take` cannot pick and
                // choose, so partition instead of clearing outright.
                let taken = std::mem::take(&mut state.pokemon[target.index()].attached);
                let (energy, kept): (Vec<_>, Vec<_>) =
                    taken.into_iter().partition(|c| state.def_of(*c).is_energy());
                state.pokemon[target.index()].attached = kept;
                state.players[player.index()].hand.extend(energy);
            }
            let name = state.pokemon_def(target).name;
            state.log.push(format!("{name} is healed fully."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::TakeFromBottomOfLibrary { card } => {
            let player = match state.phase {
                Phase::LookingAtBottomOfLibrary { player, .. } => player,
                _ => return Err(IllegalAction),
            };
            state.players[player.index()].library.retain(|c| *c != card);
            state.players[player.index()].hand.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{player:?} takes {name} from the bottom of the library."));
            let library = &mut state.players[player.index()].library;
            shuffle(state.rng.as_mut(), library);
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DeclineBottomOfLibrary => {
            let player = match state.phase {
                Phase::LookingAtBottomOfLibrary { player, .. } => player,
                _ => return Err(IllegalAction),
            };
            let library = &mut state.players[player.index()].library;
            shuffle(state.rng.as_mut(), library);
            state.phase = Phase::Main;
            settle(state);
        }

        Action::ChooseDevolveTarget { target } => {
            let player = match state.phase {
                Phase::Devolving { player, target: None } => player,
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Devolving { player, target: Some(target) };
        }

        Action::RemoveOneEvolutionCard => {
            let (player, target) = match state.phase {
                Phase::Devolving { player, target: Some(target) } => (player, target),
                _ => return Err(IllegalAction),
            };
            let removed = state.pokemon[target.index()]
                .cards
                .pop()
                .expect("a Pokémon in play always keeps its Basic");
            state.players[player.index()].hand.push(removed);
            state.pokemon[target.index()].cannot_evolve_this_turn = true;
            let name = state.pokemon_def(target).name;
            state.log.push(format!("{name} devolves."));
            // Still in Devolving: `legal_actions` offers another card to
            // remove, or FinishDevolving, depending on what is left.
        }

        Action::FinishDevolving => {
            match state.phase {
                Phase::Devolving { target: Some(_), .. } => {}
                _ => return Err(IllegalAction),
            }
            state.phase = Phase::Main;
            settle(state);
        }

        Action::ChooseIdentitySwapTarget { target } => {
            let player = match state.phase {
                Phase::SwappingIdentity { player, target: None } => player,
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::SwappingIdentity { player, target: Some(target) };
        }

        Action::SwapIdentityWithDiscarded { card } => {
            let (player, target) = match state.phase {
                Phase::SwappingIdentity { player, target: Some(target) } => (player, target),
                _ => return Err(IllegalAction),
            };
            state.players[player.index()].discard.retain(|c| *c != card);
            // The Pokémon in play keeps its damage, attachments,
            // conditions, and played_on_turn — only which card it is
            // changes. Its old card takes the discarded one's place.
            let old = std::mem::replace(&mut state.pokemon[target.index()].cards, vec![card]);
            state.players[player.index()].discard.extend(old);
            let name = state.pokemon_def(target).name;
            state.log.push(format!("{name} takes the place of what was there."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::MoveEnergyForHandheldFan { card, target } => {
            let attacker = match state.phase {
                Phase::MovingEnergyForHandheldFan { attacker, .. } => attacker,
                _ => return Err(IllegalAction),
            };
            state.pokemon[attacker.index()].attached.retain(|c| *c != card);
            state.pokemon[target.index()].attached.push(card);
            let energy = state.def_of(card).name();
            let name = state.pokemon_def(target).name;
            state.log.push(format!("{energy} moves to {name}."));
            state.phase = Phase::Main;
            // The attack this interrupted never got to settle — this is
            // where the deferred knockout check finally runs.
            settle(state);
        }

        Action::AttachFromDiscardForPowerglass { card } => {
            let player = match state.phase {
                Phase::AttachingFromDiscardForPowerglass { player } => player,
                _ => return Err(IllegalAction),
            };
            let active = state.player(player).active.expect("Powerglass named this Active");
            state.players[player.index()].discard.retain(|c| *c != card);
            state.pokemon[active.index()].attached.push(card);
            let energy = state.def_of(card).name();
            state.log.push(format!("Powerglass attaches {energy} from discard."));
            end_the_turn(state);
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DeclinePowerglass => {
            match state.phase {
                Phase::AttachingFromDiscardForPowerglass { .. } => {}
                _ => return Err(IllegalAction),
            }
            end_the_turn(state);
            state.phase = Phase::Main;
            settle(state);
        }

        Action::PutOnTopOfDeckForAcademyAtNight { card } => {
            let player = state.current;
            state.remove_from_hand(player, card);
            state.players[player.index()].library.push(card);
            state.spend(Limit::StadiumEffectUsed(player));
            let name = state.def_of(card).name();
            state.log.push(format!("{player:?} puts {name} on top of the deck."));
        }

        Action::DrawTwoForTeamRocketsFactory => {
            let player = state.current;
            state.spend(Limit::StadiumEffectUsed(player));
            state.draw(player);
            state.draw(player);
            state.log.push(format!("{player:?} draws 2 (Team Rocket's Factory)."));
        }

        Action::UseLumioseCity => {
            let player = state.current;
            let (_, stadium_card) = state.stadium.expect("Lumiose City is in play to offer this");
            state.spend(Limit::StadiumEffectUsed(player));
            enter_slot(
                state,
                player,
                stadium_card,
                0,
                crate::card::Zone::Library,
                Some(crate::card::Then::EndTurnIfMoved),
                Progress { moved: 0, previous: None },
            );
        }

        Action::PlaceDamageCounter { target } => {
            let (player, remaining) = match state.phase {
                Phase::DistributingDamageCounters { player, remaining } => (player, remaining),
                _ => return Err(IllegalAction),
            };
            state.pokemon[target.index()].damage += 10;
            let name = state.pokemon_def(target).name;
            state.log.push(format!("{name} takes a damage counter."));
            if remaining <= 1 {
                state.phase = Phase::Main;
                settle(state);
            } else {
                state.phase = Phase::DistributingDamageCounters {
                    player,
                    remaining: remaining - 1,
                };
            }
        }

        Action::ChooseJaninesTarget { target } => {
            let (player, remaining, mut chosen) = match state.phase {
                Phase::ChoosingJaninesTargets { player, remaining, chosen } => {
                    (player, remaining, chosen)
                }
                _ => return Err(IllegalAction),
            };
            let slot = chosen
                .iter_mut()
                .find(|c| c.is_none())
                .expect("legal_actions never offers a third choice");
            *slot = Some(target);
            let name = state.pokemon_def(target).name;
            state.log.push(format!("{player:?} chooses {name}."));
            state.phase = Phase::ChoosingJaninesTargets {
                player,
                remaining: remaining - 1,
                chosen,
            };
        }

        Action::FinishChoosingJaninesTargets => {
            let (player, chosen) = match state.phase {
                Phase::ChoosingJaninesTargets { player, chosen, .. } => (player, chosen),
                _ => return Err(IllegalAction),
            };
            if chosen[0].is_none() {
                // Nothing chosen, nothing to search: the card resolved
                // without doing anything more.
                state.phase = Phase::Main;
                settle(state);
            } else {
                state.phase = Phase::JaninesSearch {
                    player,
                    targets: chosen,
                    index: 0,
                    attached_to_active: false,
                };
            }
        }

        Action::TakeEnergyForJanine { card } => {
            let (player, targets, index, mut attached_to_active) = match state.phase {
                Phase::JaninesSearch { player, targets, index, attached_to_active } => {
                    (player, targets, index, attached_to_active)
                }
                _ => return Err(IllegalAction),
            };
            let target = targets[index as usize].expect("this search always names a target");
            state.players[player.index()].library.retain(|c| *c != card);
            state.pokemon[target.index()].attached.push(card);
            if state.player(player).active == Some(target) {
                attached_to_active = true;
            }
            let energy = state.def_of(card).name();
            let name = state.pokemon_def(target).name;
            state.log.push(format!("{energy} moves to {name}."));
            advance_janines_search(state, player, targets, index, attached_to_active);
        }

        Action::FinishJaninesSearch => {
            let (player, targets, index, attached_to_active) = match state.phase {
                Phase::JaninesSearch { player, targets, index, attached_to_active } => {
                    (player, targets, index, attached_to_active)
                }
                _ => return Err(IllegalAction),
            };
            advance_janines_search(state, player, targets, index, attached_to_active);
        }

        Action::EvolveSkippingOneStage { card, target } => {
            let player = match state.phase {
                Phase::EvolvingWithRareCandy { player } => player,
                _ => return Err(IllegalAction),
            };
            state.remove_from_hand(player, card);
            state.pokemon[target.index()].cards.push(card);
            state.spend(Limit::Evolved(target));
            // Rule 22: evolving clears every Special Condition. Damage and
            // attachments are untouched — nothing here moves them.
            state.clear_conditions(target);
            let name = state.pokemon_def(target).name;
            state
                .log
                .push(format!("{player:?} uses Rare Candy: evolves into {name}."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DiscardOpponentEnergy { card } => {
            let of = match state.phase {
                Phase::DiscardingOpponentEnergy { of, .. } => of,
                _ => return Err(IllegalAction),
            };
            for pokemon in state.player(of).in_play() {
                state.pokemon[pokemon.index()]
                    .attached
                    .retain(|c| *c != card);
            }
            state.players[of.index()].discard.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} is discarded."));
            state.phase = Phase::Main;
            settle(state);
        }
    }

    // Recorded here, at the end: an arm that refused part-way through
    // returned before this, and a refusal has nothing to replay.
    state.history.push(action);
    Ok(())
}

/// Play a recorded game again.
///
/// The engine is a pure function of its cards, its seed, and its actions, so
/// the same three give the same game. The caller supplies the first two: a
/// state does not keep its seed, only the generator it has already advanced.
///
/// An action that does not fit the game being rebuilt is refused, the same
/// way [`apply`] refuses it, and the replay stops there. A log this engine
/// wrote always fits; one from a different deal, or from an older set of
/// rules, is exactly what this catches.
pub fn replay(
    db: CardDb,
    decklists: [Vec<CardDefId>; 2],
    rng: Box<dyn Rng>,
    history: &[Action],
) -> Result<GameState, IllegalAction> {
    let mut state = GameState::new(db, decklists, rng);
    for action in history {
        apply(&mut state, *action)?;
    }
    Ok(state)
}

/// Take back the last action, by rebuilding the game without it.
///
/// A shuffle cannot be reversed — the generator moved on and the order it
/// produced is not written down anywhere — so an undo cannot walk the state
/// backwards. It replays instead, which is correct for every action and
/// costs one replay. `history` is the log to undo the last entry of; with an
/// empty log there is nothing to take back, and the deal itself is the
/// position before any action.
pub fn undo(
    db: CardDb,
    decklists: [Vec<CardDefId>; 2],
    rng: Box<dyn Rng>,
    history: &[Action],
) -> Result<GameState, IllegalAction> {
    let keep = history.len().saturating_sub(1);
    replay(db, decklists, rng, &history[..keep])
}

/// Open the search's `step`th slot, or end the search when it has none.
///
/// The phase names the Trainer rather than carrying its slots, so this reads
/// the next slot back from the card. A search ends where its last slot ends:
/// the deck is shuffled if the search read it or put a card back into it,
/// `then` runs, and the turn goes on.
/// What a search has accumulated so far, across every slot it has run.
struct Progress {
    moved: u32,
    /// The last card any slot took, or `None` before the first. Only a slot
    /// marked `excludes_type_of_previous` reads it.
    previous: Option<CardId>,
}

fn enter_slot(
    state: &mut GameState,
    chooser: PlayerId,
    card: CardId,
    step: u32,
    from: Zone,
    then: Option<crate::card::Then>,
    progress: Progress,
) {
    let Progress { moved, previous } = progress;
    let slot = state
        .def_of(card)
        .as_trainer()
        .expect("a search is only ever a Trainer's effect")
        .slots()
        .get(step as usize)
        .cloned();

    let Some(slot) = slot else {
        // The deck is shuffled once the whole search ends, not after each
        // slot. Either end of a move calls for it: a card put back into the
        // deck is shuffled in, and a deck that was searched is shuffled
        // because the player has seen the order of it.
        let slots = state
            .def_of(card)
            .as_trainer()
            .expect("a search is only ever a Trainer's effect")
            .slots();
        let puts_back = slots.iter().any(|s| s.to == Destination::Zone(Zone::Library));
        // A search that places its cards on top in order already shuffled
        // the rest of the deck when that slot opened, and placed its cards
        // afterward. Shuffling again here would scramble them right back in.
        let already_ordered_on_top = slots
            .iter()
            .any(|s| s.to == Destination::TopOfLibraryInOrder);
        if (from == Zone::Library || puts_back) && !already_ordered_on_top {
            let library = &mut state.players[chooser.index()].library;
            shuffle(state.rng.as_mut(), library);
        }
        match then {
            Some(crate::card::Then::DrawPerCardMoved(per_card)) => {
                for _ in 0..(moved * per_card) {
                    state.draw(chooser);
                }
            }
            Some(crate::card::Then::EndTurnIfMoved) if moved > 0 => {
                state.pending_end_turn = true;
            }
            Some(crate::card::Then::EndTurnIfMoved) | None => {}
        }
        state.phase = Phase::Main;
        settle(state);
        return;
    };

    // "Shuffle your deck, then put those cards on top of it": the shuffle
    // happens before anything is placed, on whatever the search has not
    // yet taken. Cards taken during this slot are pushed onto the end of
    // the Library one at a time, undisturbed by any shuffle after this one.
    if slot.to == Destination::TopOfLibraryInOrder {
        let library = &mut state.players[chooser.index()].library;
        shuffle(state.rng.as_mut(), library);
    }

    state.phase = Phase::Deciding {
        chooser,
        card,
        step,
        from,
        to: slot.to,
        filter: slot.filter,
        excludes_type_of_previous: slot.excludes_type_of_previous,
        peek: slot.peek,
        remaining: slot.limit,
        moved,
        // A new slot keeps what the search has taken so far: the constraint
        // reads across slots, not within one.
        previous,
        then,
    };
}

/// Run a Trainer's effect once it has been played and discarded.
///
/// `player` is who played it, and `card` is the card itself: a search names
/// it so the next slot can be read back from it. A `Decide` or
/// `SwitchOpponentActive` opens a phase and waits; everything else has no
/// choice left in it and finishes here.
fn resolve_trainer(state: &mut GameState, player: PlayerId, card: CardId, effect: TrainerEffect) {
    match effect {
        TrainerEffect::Decide { from, then, .. } => {
            enter_slot(
                state,
                player,
                card,
                0,
                from,
                then,
                Progress {
                    moved: 0,
                    previous: None,
                },
            );
        }

        TrainerEffect::MoveAttachedEnergy => {
            state.phase = Phase::MovingEnergy { player };
        }

        TrainerEffect::MoveEnergyFromBenchToActive { limit } => {
            state.phase = Phase::MovingEnergyFromBenchToActive {
                player,
                remaining: limit,
            };
        }

        TrainerEffect::EvolveSkippingOneStage => {
            state.phase = Phase::EvolvingWithRareCandy { player };
        }

        TrainerEffect::SwitchOpponentActive => {
            state.phase = Phase::Promoting {
                of: player.opponent(),
                chooser: player,
                then: None,
            };
        }

        TrainerEffect::SwitchOwnActive => {
            state.phase = Phase::Promoting {
                of: player,
                chooser: player,
                then: None,
            };
        }

        TrainerEffect::SwitchOwnActiveWithFollowUp(follow_up) => {
            state.phase = Phase::Promoting {
                of: player,
                chooser: player,
                then: Some(follow_up),
            };
        }

        TrainerEffect::SwitchOpponentActiveThenOwn => {
            state.phase = Phase::Promoting {
                of: player.opponent(),
                chooser: player,
                then: Some(crate::card::PromoteFollowUp::AlsoSwitchOwnActive),
            };
        }

        TrainerEffect::ShuffleHandThenDraw {
            normal,
            at_six_prizes,
        } => {
            let count = if state.player(player).prizes.len() == 6 {
                at_six_prizes
            } else {
                normal
            };
            shuffle_hand_into_library(state, player);
            for _ in 0..count {
                state.draw(player);
            }
        }

        TrainerEffect::OpponentHandToBottomThenDraw { count } => {
            let opponent = player.opponent();
            let mut hand = std::mem::take(&mut state.players[opponent.index()].hand);
            if hand.is_empty() {
                return;
            }
            // The hand is shuffled before it goes under the deck, so neither
            // player knows the order it lands in.
            shuffle(state.rng.as_mut(), &mut hand);
            let library = &mut state.players[opponent.index()].library;
            // A draw takes from the end, so the bottom of the deck is the
            // front of this list.
            for card in hand.into_iter().rev() {
                library.insert(0, card);
            }
            for _ in 0..count {
                state.draw(opponent);
            }
        }

        TrainerEffect::BothShuffleHandThenDraw { you, opponent } => {
            for (whose, count) in [(player, you), (player.opponent(), opponent)] {
                shuffle_hand_into_library(state, whose);
                for _ in 0..count {
                    state.draw(whose);
                }
            }
        }

        // The card's own placement was the whole effect.
        TrainerEffect::Nothing => {}

        TrainerEffect::HealActive(amount) => {
            let active = state
                .player(player)
                .active
                .expect("the requirement already confirmed an Active");
            state.pokemon[active.index()].damage =
                state.pokemon(active).damage.saturating_sub(amount);
        }

        TrainerEffect::HealChosen(amount) => {
            state.phase = Phase::HealingChosen { player, amount };
        }

        TrainerEffect::BonusDamageThisTurn(amount, target) => {
            state.turn_bonus = Some((amount, target));
        }

        TrainerEffect::ChooseOneOf(..) => {
            state.phase = Phase::ChoosingOneOf { player, card };
        }

        TrainerEffect::DrawPerOpponentBenched => {
            let count = state.player(player.opponent()).bench.len();
            for _ in 0..count {
                state.draw(player);
            }
        }

        TrainerEffect::OpponentDiscardsDownTo(target) => {
            let opponent = player.opponent();
            let hand_len = state.player(opponent).hand.len() as u32;
            state.phase = Phase::DiscardingFromHand {
                chooser: opponent,
                of: opponent,
                filter: crate::card::CardFilter::AnyCard,
                remaining: hand_len.saturating_sub(target),
                then: None,
            };
        }

        TrainerEffect::DiscardFromOpponentsHand { filter, limit } => {
            state.phase = Phase::DiscardingFromHand {
                chooser: player,
                of: player.opponent(),
                filter,
                remaining: limit,
                then: None,
            };
        }

        TrainerEffect::BothDiscardDownTo(target) => {
            let opponent = player.opponent();
            let hand_len = state.player(opponent).hand.len() as u32;
            state.phase = Phase::DiscardingFromHand {
                chooser: opponent,
                of: opponent,
                filter: crate::card::CardFilter::AnyCard,
                remaining: hand_len.saturating_sub(target),
                then: Some(crate::card::DiscardFollowUp::AlsoDiscardOwnHandDownTo(target)),
            };
        }

        TrainerEffect::HealMegaExAndTakeEnergyIfHealed => {
            state.phase = Phase::HealingMegaEx { player };
        }

        TrainerEffect::LookAtBottomOfLibrary { count } => {
            state.phase = Phase::LookingAtBottomOfLibrary { player, count };
        }

        TrainerEffect::DevolveChosen => {
            state.phase = Phase::Devolving { player, target: None };
        }

        TrainerEffect::SwapBasicWithDiscard => {
            state.phase = Phase::SwappingIdentity { player, target: None };
        }

        TrainerEffect::ReducesRetreatCost(_)
        | TrainerEffect::IncreasesHp(_)
        | TrainerEffect::BonusDamageWithoutRuleBoxVsEx(_)
        | TrainerEffect::BonusDamageIfPoisonedVsActive(_)
        | TrainerEffect::FewerPrizeIfLilliesKnockedOutByAttack
        | TrainerEffect::DamagesAttackerWhenDefenderIsHit(_)
        | TrainerEffect::DrawsWhenDefenderIsHit(_)
        | TrainerEffect::MovesEnergyFromAttackerToTheirBench
        | TrainerEffect::MayAttachBasicEnergyFromDiscardAtTurnEnd => {
            unreachable!(
                "a static effect is read wherever it applies, never dispatched \
                 at play time — a Tool never reaches resolve_trainer at all"
            )
        }

        // A Stadium's static effect, unlike a Tool's, does reach here —
        // playing a Stadium is an ordinary PlayTrainer, since nothing
        // skips it the way PlayTool skips a Tool. There is still nothing
        // to do at play time: `state.stadium` already names the card,
        // and every reader (`effective_hp`, `effective_retreat_cost`, …)
        // reads the effect from there, not from this dispatch.
        TrainerEffect::ReducesHpForStage(..)
        | TrainerEffect::RemovesRetreatCostForNamePrefix(_)
        | TrainerEffect::MayPutHandCardOnTopOfDeck
        | TrainerEffect::MayDrawTwoIfPlayedTeamRocketSupporter
        | TrainerEffect::MaySearchBasicToBenchThenMaybeEndTurn
        | TrainerEffect::ToolsHaveNoEffect
        | TrainerEffect::DamagesNonDarknessBasicBenched(_)
        | TrainerEffect::GrassCanEvolveTheTurnItIsPlayed => {}

        // "Recovers from all Special Conditions" reads as an immediate
        // sweep at the moment this becomes true for a Pokémon — playing
        // the Stadium is the first such moment (`AttachEnergy`'s own
        // handler is the other).
        TrainerEffect::EnergizedPokemonImmuneToSpecialConditions => {
            for pokemon_id in (0..state.pokemon.len()).map(|i| PokemonId(i as u32)) {
                if state.immune_under_festival_grounds(pokemon_id) {
                    state.clear_conditions(pokemon_id);
                }
            }
        }

        TrainerEffect::JaninesSecretArt => {
            state.phase = Phase::ChoosingJaninesTargets {
                player,
                remaining: 2,
                chosen: [None, None],
            };
        }

        TrainerEffect::CoinFlipDiscardOpponentEnergy => {
            if state.rng.flip() {
                state.phase = Phase::DiscardingOpponentEnergy {
                    chooser: player,
                    of: player.opponent(),
                };
            }
        }

        TrainerEffect::DiscardOpponentEnergy => {
            state.phase = Phase::DiscardingOpponentEnergy {
                chooser: player,
                of: player.opponent(),
            };
        }
    }
}

/// Shuffle a player's hand into their Library. Several Supporters start this
/// way before drawing a fresh hand.
fn shuffle_hand_into_library(state: &mut GameState, player: PlayerId) {
    let hand = std::mem::take(&mut state.players[player.index()].hand);
    state.players[player.index()].library.extend(hand);
    let library = &mut state.players[player.index()].library;
    shuffle(state.rng.as_mut(), library);
}

/// Move Janine's Secret Art on to its next step: the second target's search,
/// or the end of the effect. Both `Action::TakeEnergyForJanine` (a card was
/// found) and `Action::FinishJaninesSearch` (the player declines to search
/// further for this target) land here.
fn advance_janines_search(
    state: &mut GameState,
    player: PlayerId,
    targets: [Option<PokemonId>; 2],
    index: u8,
    attached_to_active: bool,
) {
    if index == 0 && targets[1].is_some() {
        state.phase = Phase::JaninesSearch {
            player,
            targets,
            index: 1,
            attached_to_active,
        };
        return;
    }
    let library = &mut state.players[player.index()].library;
    shuffle(state.rng.as_mut(), library);
    state.log.push(format!("{player:?} shuffles their library."));
    if attached_to_active
        && let Some(active) = state.player(player).active
    {
        state.inflict(active, Condition::Poisoned);
    }
    state.phase = Phase::Main;
    settle(state);
}

fn setup_player(state: &GameState) -> Result<PlayerId, IllegalAction> {
    match state.phase {
        Phase::TakingBonusDraws { player, .. }
        | Phase::PlacingActive { player }
        | Phase::PlacingBench { player } => Ok(player),
        _ => Err(IllegalAction),
    }
}

/// Move setup to the next choice it owes, or start the game.
///
/// The rulebook has both players set up at once. Nothing at setup is visible
/// to the opponent, so the engine asks one player for all of it — the bonus
/// draws, the Active, the Bench — before it turns to the other.
fn advance_setup(state: &mut GameState) {
    for player in [PlayerId::One, PlayerId::Two] {
        let remaining = state.bonus_draws[player.index()];
        if remaining > 0 {
            state.phase = Phase::TakingBonusDraws { player, remaining };
            return;
        }
        if state.player(player).active.is_none() {
            state.phase = Phase::PlacingActive { player };
            return;
        }
        if !state.bench_placed[player.index()] {
            state.phase = Phase::PlacingBench { player };
            return;
        }
    }

    // Rule 10: the Prizes come off the top after the Pokémon are down.
    for player in [PlayerId::One, PlayerId::Two] {
        state.set_prizes(player);
    }
    state.phase = Phase::Main;
    state.begin_turn();
    state
        .log
        .push(format!("Turn {} begins.", state.turn_number + 1));

    // Rule 15: the player going first draws, they only skip the attack.
    if !state.draw(state.current) {
        win(state, state.current.opponent(), WinReason::CouldNotDraw);
        state.phase = Phase::Over;
    }
}

/// Rule 23: retreating discards Energy equal to the Retreat Cost. Which Energy
/// is the player's choice, so a cost above zero opens a phase.
fn retreat(state: &mut GameState, to: PokemonId) {
    let player = state.current;
    let active = state.players[player.index()]
        .active
        .expect("retreating needs an Active");
    let cost = state.effective_retreat_cost(active);

    if cost == 0 {
        promote_from_retreat(state, player, to);
        return;
    }
    state.phase = Phase::DiscardingForRetreat {
        player,
        to,
        remaining: cost as u8,
    };
}

fn promote_from_retreat(state: &mut GameState, player: PlayerId, to: PokemonId) {
    let active = state.players[player.index()]
        .active
        .expect("retreating needs an Active");
    // Rule 27: moving to the Bench removes all Special Conditions.
    state.clear_conditions(active);
    let side = &mut state.players[player.index()];
    side.bench.retain(|p| *p != to);
    side.bench.push(active);
    side.active = Some(to);
    state.spend(Limit::Retreated(player));
    state.phase = Phase::Main;

    let name = state.pokemon_def(to).name;
    state.log.push(format!("{player:?} retreats to {name}."));
}

/// Rules 29-37, in the order the rulebook sets out.
fn attack(state: &mut GameState, index: usize) {
    let player = state.current;
    let attacker = state.players[player.index()]
        .active
        .expect("attacking needs an Active");
    let Some(defender) = state.players[player.opponent().index()].active else {
        return;
    };

    // Rule 30: Confusion flips before the attack happens. Rule 52: on tails
    // the attack does not happen and 3 damage counters go on your own Pokémon.
    if state.has_condition(attacker, Condition::Confused) && !state.rng.flip() {
        state.pokemon[attacker.index()].damage += 30;
        let name = state.pokemon_def(attacker).name;
        state
            .log
            .push(format!("{name} is Confused and hurts itself."));
        return;
    }

    let attack = state.pokemon_def(attacker).attacks[index].clone();
    let base = match attack.effect {
        Some(crate::card::AttackEffect::DamagePerCount(count, per_unit)) => {
            count_for_attack(state, attacker, count) * per_unit
        }
        Some(crate::card::AttackEffect::CoinFlipBonusDamage(bonus)) => {
            if state.rng.flip() {
                attack.base_damage + bonus
            } else {
                attack.base_damage
            }
        }
        _ => attack.base_damage,
    };
    let ignore_defenders_effects =
        matches!(attack.effect, Some(crate::card::AttackEffect::IgnoresDefendersEffects));
    let damage = damage_dealt_with(state, attacker, defender, base, ignore_defenders_effects);

    state.pokemon[defender.index()].damage += damage;
    // `Lillie's Pearl` tells this knockout apart from one a checkup
    // causes; `settle`'s knockout check consumes this, every call.
    state.attacking_defender = Some(defender);
    if damage > 0 {
        trigger_defenders_tool(state, attacker, defender);
    }
    if let Some(condition) = attack.inflicts {
        state.inflict(defender, condition);
        let name = state.pokemon_def(defender).name;
        state.log.push(format!("{name} is now {condition:?}."));
    }
    if let Some(effect) = attack.effect {
        resolve_attack_effect(state, attacker, defender, effect);
    }

    let attacker_name = state.pokemon_def(attacker).name;
    let defender_name = state.pokemon_def(defender).name;
    state.log.push(format!(
        "{attacker_name} uses {} on {defender_name} for {damage}.",
        attack.name
    ));
}

/// An attack's own printed effect, read once the attack's plain damage
/// and `inflicts` have already landed. Milestone 11's dispatch, the
/// counterpart to `resolve_trainer` — a separate function since an
/// attack and a Trainer effect are read from different call sites,
/// with different arguments (attacker and defender, not a player and a
/// played card).
fn resolve_attack_effect(
    state: &mut GameState,
    attacker: PokemonId,
    defender: PokemonId,
    effect: crate::card::AttackEffect,
) {
    match effect {
        crate::card::AttackEffect::Recoil(amount) => {
            state.pokemon[attacker.index()].damage += amount;
            let name = state.pokemon_def(attacker).name;
            state.log.push(format!("{name} also takes {amount} damage."));
        }
        // Already spent, before `damage_dealt` ran — see `attack`'s own
        // `base` computation.
        crate::card::AttackEffect::DamagePerCount(..) => {}
        // Already spent, before `damage_dealt_with` ran.
        crate::card::AttackEffect::IgnoresDefendersEffects => {}
        crate::card::AttackEffect::InflictsCondition(condition) => {
            state.inflict(defender, condition);
            let name = state.pokemon_def(defender).name;
            state.log.push(format!("{name} is now {condition:?}."));
        }
        crate::card::AttackEffect::CoinFlipInflicts(condition) => {
            if state.rng.flip() {
                state.inflict(defender, condition);
                let name = state.pokemon_def(defender).name;
                state.log.push(format!("{name} is now {condition:?}."));
            }
        }
        // Already spent, before `damage_dealt_with` ran — see `attack`'s
        // own `base` computation.
        crate::card::AttackEffect::CoinFlipBonusDamage(_) => {}
        crate::card::AttackEffect::DefenderCannotRetreatNextTurn => {
            state.opponent_next_turn_restriction = Some((defender, effect));
            let name = state.pokemon_def(defender).name;
            state.log.push(format!("{name} cannot retreat next turn."));
        }
        crate::card::AttackEffect::AttackerCannotAttackNextTurn => {
            state.own_next_turn_restriction = Some((attacker, effect, false));
            let name = state.pokemon_def(attacker).name;
            state.log.push(format!("{name} cannot attack next turn."));
        }
        crate::card::AttackEffect::DamageCountersToOpponentBenchAnyWay(count) => {
            let owner = state.pokemon(attacker).owner;
            if !state.player(owner.opponent()).bench.is_empty() {
                state.phase = Phase::DistributingDamageCounters {
                    player: owner,
                    remaining: count,
                };
            }
        }
    }
}

/// What `AttackEffect::DamagePerCount` reads for this attack, counted
/// fresh from the board.
fn count_for_attack(state: &GameState, attacker: PokemonId, count: crate::card::Count) -> u32 {
    let owner = state.pokemon(attacker).owner;
    let opponent = owner.opponent();
    match count {
        crate::card::Count::OwnDamageCounters => state.pokemon(attacker).damage / 10,
        crate::card::Count::OpponentBasicEnergyInDiscard => state
            .player(opponent)
            .discard
            .iter()
            .filter(|c| state.def_of(**c).is_energy())
            .count() as u32,
        crate::card::Count::OpponentPokemonExInPlay => state
            .player(opponent)
            .in_play()
            .iter()
            .filter(|p| state.pokemon_def(**p).prizes > 1)
            .count() as u32,
        crate::card::Count::OwnBasicPokemonInPlay => state
            .player(owner)
            .in_play()
            .iter()
            .filter(|p| state.pokemon_def(**p).stage == crate::card::Stage::Basic)
            .count() as u32,
        crate::card::Count::OwnDamagedWithNamePrefix(prefix) => state
            .player(owner)
            .in_play()
            .iter()
            .filter(|p| state.pokemon_def(**p).name.starts_with(prefix) && state.pokemon(**p).damage > 0)
            .count() as u32,
    }
}

/// A Tool on the Active that just took attack damage — Punk Helmet,
/// Handheld Fan, Lucky Helmet. "Even if Knocked Out" means this runs
/// before `settle` decides that; nothing here reads whether the
/// defender survives.
fn trigger_defenders_tool(state: &mut GameState, attacker: PokemonId, defender: PokemonId) {
    if state.tools_disabled() {
        return;
    }
    let tools: Vec<CardId> = state
        .pokemon(defender)
        .attached
        .iter()
        .copied()
        .filter(|c| {
            state
                .def_of(*c)
                .as_trainer()
                .is_some_and(|t| t.kind == TrainerKind::Tool)
        })
        .collect();
    for tool in tools {
        let Some(effect) = state.def_of(tool).as_trainer().map(|t| t.effect.clone()) else {
            continue;
        };
        match effect {
            TrainerEffect::DamagesAttackerWhenDefenderIsHit(amount) => {
                state.pokemon[attacker.index()].damage += amount;
                let name = state.pokemon_def(attacker).name;
                state
                    .log
                    .push(format!("{name} takes {amount} in return."));
            }
            TrainerEffect::DrawsWhenDefenderIsHit(count) => {
                let owner = state.pokemon(defender).owner;
                for _ in 0..count {
                    state.draw(owner);
                }
            }
            TrainerEffect::MovesEnergyFromAttackerToTheirBench => {
                let attacker_owner = state.pokemon(attacker).owner;
                let has_energy = state
                    .pokemon(attacker)
                    .attached
                    .iter()
                    .any(|c| state.def_of(*c).is_energy());
                let has_bench = !state.player(attacker_owner).bench.is_empty();
                if has_energy && has_bench {
                    let chooser = state.pokemon(defender).owner;
                    state.phase = Phase::MovingEnergyForHandheldFan { chooser, attacker };
                    return;
                }
            }
            _ => {}
        }
    }
}

/// Step 31 to 35 of the damage order. Milestone 1 has no effects that change
/// damage, so steps 32 and 34 are the identity — the shape is here so a card
/// that does have one has a place to act.
pub fn damage_dealt(state: &GameState, attacker: PokemonId, defender: PokemonId, base: u32) -> u32 {
    damage_dealt_with(state, attacker, defender, base, false)
}

/// `damage_dealt`'s own order, with step 33 (Weakness, Resistance, and
/// any other effect on the defender) skippable — `AttackEffect::IgnoresDefendersEffects`'s
/// own read, kept out of the public `damage_dealt` so every existing
/// caller, including several tests, keeps reading the ordinary order.
fn damage_dealt_with(
    state: &GameState,
    attacker: PokemonId,
    defender: PokemonId,
    base: u32,
    ignore_defenders_effects: bool,
) -> u32 {
    let mut damage = base;

    // Step 32: effects on the attacking player's Pokémon. Stop at 0.
    if damage == 0 {
        return 0;
    }
    if let Some((bonus, target)) = state.turn_bonus
        && state.pokemon(attacker).owner == state.current
        && state.player(state.pokemon(attacker).owner).active == Some(attacker)
    {
        let defender_def = state.pokemon_def(defender);
        let restricted_to_defender = match target {
            crate::card::TurnBonusTarget::OpponentActiveEx => defender_def.prizes > 1,
            crate::card::TurnBonusTarget::OpponentActiveWithoutRuleBox => defender_def.prizes == 1,
        };
        let opponent_active = state
            .player(state.pokemon(attacker).owner.opponent())
            .active;
        if restricted_to_defender && opponent_active == Some(defender) {
            damage += bonus;
        }
    }

    // Step 32b: a Tool attached to the attacker, unconditioned on whose
    // turn it is — unlike `turn_bonus`, this reads every attack, not only
    // "this turn." Both built so far restrict to the opponent's Active,
    // which the single-Active format makes the defender always is.
    // `Jamming Tower` turns this step off entirely.
    let attacker_tools: &[CardId] = if state.tools_disabled() {
        &[]
    } else {
        &state.pokemon(attacker).attached
    };
    for tool in attacker_tools {
        let Some(effect) = state.def_of(*tool).as_trainer().map(|t| &t.effect) else {
            continue;
        };
        match effect {
            crate::card::TrainerEffect::BonusDamageWithoutRuleBoxVsEx(bonus) => {
                let attacker_has_no_rule_box = state.pokemon_def(attacker).prizes == 1;
                let defender_is_ex = state.pokemon_def(defender).prizes > 1;
                if attacker_has_no_rule_box && defender_is_ex {
                    damage += bonus;
                }
            }
            crate::card::TrainerEffect::BonusDamageIfPoisonedVsActive(bonus) => {
                if state.has_condition(attacker, Condition::Poisoned) {
                    damage += bonus;
                }
            }
            _ => {}
        }
    }

    // Step 33: Weakness, then Resistance. Both read the attacker's type.
    // `AttackEffect::IgnoresDefendersEffects` skips this step outright —
    // "isn't affected by any effects on your opponent's Active Pokémon"
    // reads as including Weakness and Resistance, not only a Tool or
    // Stadium bonus.
    if !ignore_defenders_effects {
        let attacker_type = state.pokemon_def(attacker).kind;
        let defender_def = state.pokemon_def(defender);
        if defender_def.weakness == Some(attacker_type) {
            damage *= 2;
        }
        if defender_def.resistance == Some(attacker_type) {
            damage = damage.saturating_sub(30);
        }
    }

    // Step 34: effects on the defending Pokémon.
    // Step 35: 1 counter per 10 damage, so damage lands in tens.
    damage - damage % 10
}

/// Carry the state forward until it is waiting on a player again: settle
/// knockouts, check for a winner, ask for a promotion, run the checkup, and
/// start the next turn when one is owed.
fn settle(state: &mut GameState) {
    loop {
        if state.is_over() {
            state.phase = Phase::Over;
            return;
        }

        knock_out_the_dead(state);
        if state.is_over() {
            state.phase = Phase::Over;
            return;
        }

        // Rule 40: the player whose Active was knocked out chooses the next one.
        for player in [PlayerId::One, PlayerId::Two] {
            if state.player(player).active.is_none() {
                if state.player(player).bench.is_empty() {
                    // Rule 42: no Pokémon to promote loses the game.
                    win(state, player.opponent(), WinReason::NoPokemonInPlay);
                    state.phase = Phase::Over;
                    return;
                }
                state.phase = Phase::Promoting {
                    of: player,
                    chooser: player,
                    then: None,
                };
                return;
            }
        }

        // Rule 45: the checkup runs after a turn ends and before the next one.
        // `Powerglass` triggers first — "at the end of your turn" — and, if
        // it opens a phase, `end_the_turn` runs again once that resolves,
        // called directly rather than through this loop.
        if state.pending_end_turn {
            state.pending_end_turn = false;
            if let Some(player) = powerglass_owner(state) {
                state.phase = Phase::AttachingFromDiscardForPowerglass { player };
                return;
            }
            end_the_turn(state);
        }

        if let Some(player) = next_checkup_player(state) {
            state.phase = Phase::Checkup { player };
            return;
        }

        if !state.pending_turn_start {
            state.phase = Phase::Main;
            return;
        }

        // Rule 48: the knockouts above have settled, so the next turn starts.
        state.pending_turn_start = false;
        start_next_turn(state);
        if state.is_over() {
            state.phase = Phase::Over;
            return;
        }
    }
}

/// The rest of what a turn ending owes, once `Powerglass` (if any) is
/// out of the way: queue the checkup, clear Paralysis, and mark the next
/// turn owed. Called from `settle`'s own loop, and directly by whichever
/// action resolves `Phase::AttachingFromDiscardForPowerglass`.
fn end_the_turn(state: &mut GameState) {
    fill_checkup(state);
    clear_paralysis(state);
    state.pending_turn_start = true;
}

/// The Active of the player whose turn is ending, if it carries
/// `Powerglass` — the one Tool so far that triggers on the turn ending
/// itself, rather than on being attacked.
/// `Risky Ruins`: a Basic of any type but Darkness takes damage the
/// moment it lands on a Bench, from either arrival site — `PlayBasic`
/// and a search's own `Destination::Bench` alike.
fn apply_risky_ruins(state: &mut GameState, pokemon: PokemonId) {
    let Some(TrainerEffect::DamagesNonDarknessBasicBenched(amount)) = state.stadium_effect()
    else {
        return;
    };
    let def = state.pokemon_def(pokemon);
    if def.stage != crate::card::Stage::Basic || def.kind == crate::card::Type::Darkness {
        return;
    }
    state.pokemon[pokemon.index()].damage += amount;
    let name = state.pokemon_def(pokemon).name;
    state.log.push(format!("{name} takes {amount} (Risky Ruins)."));
}

fn powerglass_owner(state: &GameState) -> Option<PlayerId> {
    if state.tools_disabled() {
        return None;
    }
    let player = state.current;
    let active = state.player(player).active?;
    let carries_powerglass = state.pokemon(active).attached.iter().any(|c| {
        state
            .def_of(*c)
            .as_trainer()
            .is_some_and(|t| t.effect == TrainerEffect::MayAttachBasicEnergyFromDiscardAtTurnEnd)
    });
    carries_powerglass.then_some(player)
}

/// Queue every between-turn effect the conditions owe. Rule 49 keeps a
/// condition on the Active, so nothing on the Bench is queued.
fn fill_checkup(state: &mut GameState) {
    for player in [PlayerId::One, PlayerId::Two] {
        let Some(active) = state.player(player).active else {
            continue;
        };
        for condition in state.pokemon(active).conditions.clone() {
            if has_checkup_effect(condition) {
                state.checkup_pending.push((player, active, condition));
            }
        }
    }
}

/// Rule 51: Paralysis recovers at the checkup after its owner's next turn.
/// The owner has just taken that turn, so this is that checkup.
fn clear_paralysis(state: &mut GameState) {
    let owner = state.current;
    let Some(active) = state.player(owner).active else {
        return;
    };
    if state.has_condition(active, Condition::Paralyzed) {
        state.remove_condition(active, Condition::Paralyzed);
        let name = state.pokemon_def(active).name;
        state.log.push(format!("{name} is no longer Paralyzed."));
    }
}

/// Whether a condition does anything at the checkup.
fn has_checkup_effect(condition: Condition) -> bool {
    match condition {
        Condition::Poisoned | Condition::Burned | Condition::Asleep => true,
        Condition::Paralyzed | Condition::Confused => false,
    }
}

/// The damage a condition puts on at the checkup.
fn checkup_damage(condition: Condition) -> u32 {
    match condition {
        // Rule 54: 1 damage counter.
        Condition::Poisoned => 10,
        // Rule 53: 2 damage counters, then a flip.
        Condition::Burned => 20,
        Condition::Asleep | Condition::Paralyzed | Condition::Confused => 0,
    }
}

/// Whose effects the checkup is waiting on. The player whose turn just ended
/// resolves their own first.
fn next_checkup_player(state: &GameState) -> Option<PlayerId> {
    let owed = |player: PlayerId| state.checkup_pending.iter().any(|(p, _, _)| *p == player);
    if owed(state.current) {
        Some(state.current)
    } else if owed(state.current.opponent()) {
        Some(state.current.opponent())
    } else {
        None
    }
}

fn resolve_checkup(state: &mut GameState, pokemon: PokemonId, condition: Condition) {
    let damage = checkup_damage(condition);
    if damage > 0 {
        state.pokemon[pokemon.index()].damage += damage;
        let name = state.pokemon_def(pokemon).name;
        state
            .log
            .push(format!("{name} takes {damage} from {condition:?}."));
    }

    // Rules 50 and 53: Asleep and Burned each flip, and heads removes them.
    if matches!(condition, Condition::Burned | Condition::Asleep) && state.rng.flip() {
        state.remove_condition(pokemon, condition);
        let name = state.pokemon_def(pokemon).name;
        state
            .log
            .push(format!("{name} is no longer {condition:?}."));
    }
    if let Some(at) = state
        .checkup_pending
        .iter()
        .position(|(_, p, c)| *p == pokemon && *c == condition)
    {
        state.checkup_pending.remove(at);
    }
}

fn knock_out_the_dead(state: &mut GameState) {
    // Scoped to this call only, whether or not it matches anything below —
    // see the field's own doc comment.
    let attacking_defender = state.attacking_defender.take();
    for player in [PlayerId::One, PlayerId::Two] {
        for pokemon in state.player(player).in_play() {
            if state.remaining_hp(pokemon) > 0 {
                continue;
            }
            // Rule 39: the opponent of the knocked-out player takes a Prize.
            // The count starts at what the card is worth, and a card
            // effect adjusts it. `Lillie's Pearl` is the only one that
            // does, and only when an attack — not a checkup — caused
            // this exact knockout.
            let mut count = state.pokemon_def(pokemon).prizes as usize;
            if attacking_defender == Some(pokemon)
                && state.pokemon_def(pokemon).name.starts_with("Lillie's")
                && state.pokemon(pokemon).attached.iter().any(|c| {
                    state.def_of(*c).as_trainer().is_some_and(|t| {
                        t.effect == crate::card::TrainerEffect::FewerPrizeIfLilliesKnockedOutByAttack
                    })
                })
            {
                count = count.saturating_sub(1);
            }
            knock_out(state, pokemon);
            take_prizes(state, player.opponent(), count);
            if state.is_over() {
                return;
            }
        }
    }
}

fn knock_out(state: &mut GameState, pokemon: PokemonId) {
    let owner = state.pokemon(pokemon).owner;
    let name = state.pokemon_def(pokemon).name;

    // Rule 38: the Pokémon and everything attached go to its owner's discard.
    // Every stage it evolved through goes with it (rule 22 keeps the stack).
    let cards = std::mem::take(&mut state.pokemon[pokemon.index()].cards);
    let attached = std::mem::take(&mut state.pokemon[pokemon.index()].attached);
    let side = &mut state.players[owner.index()];
    side.discard.extend(cards);
    side.discard.extend(attached);
    if side.active == Some(pokemon) {
        side.active = None;
    }
    side.bench.retain(|p| *p != pokemon);

    state.pokemon[pokemon.index()].knocked_out = true;
    state.knocked_out_last_turn[owner.index()] = true;
    state.log.push(format!("{name} is Knocked Out."));
}

fn take_prizes(state: &mut GameState, player: PlayerId, count: usize) {
    for _ in 0..count {
        match state.players[player.index()].prizes.pop() {
            Some(card) => state.players[player.index()].hand.push(card),
            None => break,
        }
    }
    state.log.push(format!("{player:?} takes {count} Prize."));
    // Rule 41, and rule 44: this check comes before any other, so taking the
    // last Prize wins even when a knockout would end the game another way.
    if state.player(player).prizes.is_empty() {
        win(state, player, WinReason::AllPrizesTaken);
    }
}

fn start_next_turn(state: &mut GameState) {
    // The turn that just ended is no longer "last turn" for the player
    // whose turn it was: whatever it Knocked Out of theirs, `Unfair Stamp`
    // has had its one turn to read. Clearing it here, not when their own
    // turn opens, is what leaves it true for that whole turn.
    state.knocked_out_last_turn[state.current.index()] = false;
    state.current = state.current.opponent();
    state.turn_number += 1;
    state.begin_turn();
    state
        .log
        .push(format!("Turn {} begins.", state.turn_number + 1));

    // Rule 12: draw, and lose if you cannot.
    if !state.draw(state.current) {
        win(state, state.current.opponent(), WinReason::CouldNotDraw);
    }
}

fn win(state: &mut GameState, winner: PlayerId, reason: WinReason) {
    if state.outcome.is_none() {
        state.outcome = Some(Outcome { winner, reason });
        state.log.push(format!("{winner:?} wins: {reason:?}."));
    }
}
