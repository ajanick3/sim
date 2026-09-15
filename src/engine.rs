//! Applying an action to the state.

use crate::action::{Action, legal_actions};
use crate::card::{CardDb, Condition, Destination, Requirement, TrainerEffect, TrainerKind, Zone};
use crate::ids::{CardDefId, CardId, PlayerId, PokemonId};
use crate::rng::{Rng, shuffle};
use crate::state::{BENCH_LIMIT, GameState, Limit, Outcome, Phase, WinReason};

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
            trigger_last_ditch_catch(state, player, pokemon);
            trigger_snow_sink(state, player, pokemon);
            trigger_rapid_vernier(state, player, pokemon);
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
            // The ruling on Jewel Seeker: if the evolution's own carried-
            // over damage now exceeds its new HP, the Knockout takes
            // effect before any evolve-triggered Ability could fire —
            // settle() here, checked before either trigger, is what
            // gives that Knockout its own chance to happen first.
            settle(state);
            if !state.pokemon(target).knocked_out {
                trigger_psychic_draw(state, player, target);
                trigger_jewel_seeker(state, player, target);
            }
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
                        open_discard_bench_down_to_if_stadium_left(state, owner, old);
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
                    | Requirement::HandSizeIs(_)
                    | Requirement::OpponentPrizesExactly(_)
                    | Requirement::OwnTeraPokemonInPlay
                    | Requirement::ActiveNamePrefix(_),
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
            if state.immune_under_festival_grounds(target)
                || state.carries_special_condition_immunity(target)
            {
                state.clear_conditions(target);
            }
            if let Some(crate::card::EnergyEffect::DrawCardsOnAttachFromHand(count)) =
                state.def_of(card).as_energy().and_then(|e| e.effect)
            {
                for _ in 0..count {
                    state.draw(player);
                }
            }
            if let Some(crate::card::EnergyEffect::WhenAttachedToTypeSearchesBasicPokemonOfTypeToBench(
                carrier_kind,
                pokemon_kind,
                count,
            )) = state.def_of(card).as_energy().and_then(|e| e.effect)
                && state.pokemon_def(target).kind == carrier_kind
            {
                let any_basic = state.player(player).deck.iter().any(|c| {
                    state.matches_filter(*c, crate::card::CardFilter::BasicPokemonOfType(pokemon_kind))
                });
                if any_basic {
                    state.phase = Phase::SearchingDeckForBasicsOfType {
                        player,
                        kind: pokemon_kind,
                        remaining: count,
                    };
                }
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
            let player = state.current;
            attack(state, index);
            // `Festival Lead`: the first attack this turn from a
            // carrier, with `Festival Grounds` in play, opens the
            // door to a second attack instead of ending the turn —
            // read here, not through `UseAbility`, since nothing
            // about it is a separate action. Read the attacker fresh,
            // after the attack: some attacks (`Dudunsparce`'s own
            // shuffle-self-into-deck shape) leave it no longer in
            // play at all.
            let attacker = state.player(player).active;
            let grants_extra_swing = !state.festival_lead_extra_swing_used
                && attacker.is_some_and(|a| {
                    !state.abilities_disabled_for(a)
                        && state.pokemon_def(a).ability.is_some_and(|ability| {
                            ability.effect == crate::card::AbilityEffect::PassiveFestivalLead
                        })
                })
                && state.stadium_effect()
                    == Some(crate::card::TrainerEffect::EnergizedPokemonImmuneToSpecialConditions);
            if grants_extra_swing {
                state.festival_lead_extra_swing_used = true;
            } else {
                state.pending_end_turn = true;
            }
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

        Action::Promote { pokemon } if matches!(state.phase, Phase::PromotingOwnNamePrefixThenOpponent { .. }) => {
            let player = match state.phase {
                Phase::PromotingOwnNamePrefixThenOpponent { player, .. } => player,
                _ => unreachable!(),
            };
            let side = &mut state.players[player.index()];
            side.bench.retain(|p| *p != pokemon);
            let displaced = side.active.replace(pokemon);
            if let Some(displaced) = displaced {
                side.bench.push(displaced);
            }
            state.promoted_from_bench_this_turn[player.index()] = Some(pokemon);
            let name = state.pokemon_def(pokemon).name;
            state.log.push(format!("{player:?} promotes {name}."));
            if state.player(player.opponent()).bench.is_empty() {
                state.phase = Phase::Main;
                settle(state);
            } else {
                state.phase = Phase::Promoting {
                    of: player.opponent(),
                    chooser: player,
                    then: None,
                };
            }
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
            state.promoted_from_bench_this_turn[of.index()] = Some(pokemon);
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
                Destination::TopOfDeckInOrder => {
                    // The deck was already shuffled when this slot
                    // opened; each card taken lands right back on top,
                    // undisturbed, in the order it was taken.
                    state.zone_mut(chooser, from).retain(|c| *c != card);
                    state.players[chooser.index()].deck.push(card);
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
            let (amount, clear) = match state.phase {
                Phase::HealingChosen {
                    amount,
                    clear_conditions,
                    ..
                } => (amount, clear_conditions),
                Phase::HealingChosenIfRemainingHpAtMost { .. } => (u32::MAX, false),
                _ => return Err(IllegalAction),
            };
            state.pokemon[target.index()].damage =
                state.pokemon(target).damage.saturating_sub(amount);
            if clear {
                state.clear_conditions(target);
            }
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

        Action::TakeFromBottomOfDeck { card } => {
            let player = match state.phase {
                Phase::LookingAtBottomOfDeck { player, .. } => player,
                _ => return Err(IllegalAction),
            };
            state.players[player.index()].deck.retain(|c| *c != card);
            state.players[player.index()].hand.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{player:?} takes {name} from the bottom of the deck."));
            let deck = &mut state.players[player.index()].deck;
            shuffle(state.rng.as_mut(), deck);
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DeclineBottomOfDeck => {
            let player = match state.phase {
                Phase::LookingAtBottomOfDeck { player, .. } => player,
                _ => return Err(IllegalAction),
            };
            let deck = &mut state.players[player.index()].deck;
            shuffle(state.rng.as_mut(), deck);
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
            state.players[player.index()].deck.push(card);
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
                crate::card::Zone::Deck,
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

        Action::DamageBenchedPokemon { target } => {
            let (player, damage) = match state.phase {
                Phase::ChoosingBenchDamageTarget { player, damage } => (player, damage),
                _ => return Err(IllegalAction),
            };
            let name = state.pokemon_def(target).name;
            if state.bench_attack_damage_blocked(player, target) {
                // `Shaymin`'s `Flower Curtain`: this attack was already
                // used, but its damage never lands on a protected target.
                state.log.push(format!("{name} would take {damage}, but Flower Curtain stops it landing."));
            } else if state.bench_attack_effect_blocked(player, target) {
                // `Rabsca`'s `Spherical Shield`: same shape, but with no
                // Rule-Box carve-out.
                state.log.push(format!("{name} would take {damage}, but Spherical Shield stops it landing."));
            } else {
                state.pokemon[target.index()].damage += damage;
                state.log.push(format!("{name} takes {damage}."));
            }
            state.phase = Phase::Main;
            settle(state);
        }

        Action::TakeBasicPokemonForCallForFamily { card } => {
            let (player, remaining) = match state.phase {
                Phase::SearchingDeckForBasics { player, remaining } => (player, remaining),
                _ => return Err(IllegalAction),
            };
            state.players[player.index()].deck.retain(|c| *c != card);
            let pokemon = state.put_into_play(player, card);
            state.players[player.index()].bench.push(pokemon);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} joins the Bench."));
            if remaining <= 1 {
                finish_searching_deck_for_basics(state, player);
            } else {
                state.phase = Phase::SearchingDeckForBasics {
                    player,
                    remaining: remaining - 1,
                };
            }
        }

        Action::FinishCallForFamily => {
            let player = match state.phase {
                Phase::SearchingDeckForBasics { player, .. } => player,
                _ => return Err(IllegalAction),
            };
            finish_searching_deck_for_basics(state, player);
        }

        Action::TakeBasicPokemonOfTypeForEnergyAttach { card } => {
            let (player, kind, remaining) = match state.phase {
                Phase::SearchingDeckForBasicsOfType { player, kind, remaining } => {
                    (player, kind, remaining)
                }
                _ => return Err(IllegalAction),
            };
            state.players[player.index()].deck.retain(|c| *c != card);
            let pokemon = state.put_into_play(player, card);
            state.players[player.index()].bench.push(pokemon);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} joins the Bench."));
            if remaining <= 1 {
                finish_searching_deck_for_basics_of_type(state, player);
            } else {
                state.phase = Phase::SearchingDeckForBasicsOfType {
                    player,
                    kind,
                    remaining: remaining - 1,
                };
            }
        }

        Action::FinishSearchingBasicsOfType => {
            let player = match state.phase {
                Phase::SearchingDeckForBasicsOfType { player, .. } => player,
                _ => return Err(IllegalAction),
            };
            finish_searching_deck_for_basics_of_type(state, player);
        }

        Action::TakeItemFromDeck { card } => {
            let player = match state.phase {
                Phase::SearchingDeckForItem { player } => player,
                _ => return Err(IllegalAction),
            };
            state.players[player.index()].deck.retain(|c| *c != card);
            state.players[player.index()].hand.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} joins the hand."));
            let deck = &mut state.players[player.index()].deck;
            shuffle(state.rng.as_mut(), deck);
            state.phase = Phase::Main;
            settle(state);
        }

        Action::TakeAnyCardFromDeck { card } => {
            let (player, remaining) = match state.phase {
                Phase::SearchingDeckForAnyCards { player, remaining } => (player, remaining),
                _ => return Err(IllegalAction),
            };
            state.players[player.index()].deck.retain(|c| *c != card);
            state.players[player.index()].hand.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} joins the hand."));
            if remaining <= 1 {
                finish_searching_deck_for_any_cards(state, player);
            } else {
                state.phase = Phase::SearchingDeckForAnyCards { player, remaining: remaining - 1 };
            }
        }

        Action::FinishSearchingAnyCards => {
            let player = match state.phase {
                Phase::SearchingDeckForAnyCards { player, .. } => player,
                _ => return Err(IllegalAction),
            };
            finish_searching_deck_for_any_cards(state, player);
        }

        Action::MoveOpponentsActiveEnergyToHand { card } => {
            let (player, remaining) = match state.phase {
                Phase::MovingOpponentsActiveEnergyToHand { player, remaining } => (player, remaining),
                _ => return Err(IllegalAction),
            };
            let opponent = player.opponent();
            let active = state.player(opponent).active.expect("this effect needs an Active to read");
            state.pokemon[active.index()].attached.retain(|c| *c != card);
            state.players[opponent.index()].hand.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} moves to {opponent:?}'s hand."));
            if remaining <= 1 {
                state.phase = Phase::Main;
                settle(state);
            } else {
                state.phase = Phase::MovingOpponentsActiveEnergyToHand {
                    player,
                    remaining: remaining - 1,
                };
            }
        }

        Action::FinishMovingOpponentsActiveEnergyToHand => {
            match state.phase {
                Phase::MovingOpponentsActiveEnergyToHand { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
        }

        Action::TakeTrainerFromDiscard { card } => {
            let player = match state.phase {
                Phase::TakingTrainerFromDiscard { player } => player,
                _ => return Err(IllegalAction),
            };
            state.players[player.index()].discard.retain(|c| *c != card);
            state.players[player.index()].hand.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} returns to hand."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::EvolveWithAscension { card } => {
            let (player, target) = match state.phase {
                Phase::SearchingDeckToEvolveSelf { player, target } => (player, target),
                _ => return Err(IllegalAction),
            };
            state.players[player.index()].deck.retain(|c| *c != card);
            state.pokemon[target.index()].cards.push(card);
            // Rule 22: evolving clears every Special Condition. Damage and
            // attachments are untouched — nothing here moves them.
            state.clear_conditions(target);
            let name = state.pokemon_def(target).name;
            state.log.push(format!("Ascension evolves into {name}."));
            let deck = &mut state.players[player.index()].deck;
            shuffle(state.rng.as_mut(), deck);
            state.phase = Phase::Main;
            settle(state);
        }

        Action::TakePokemonFromDiscard { card } => {
            let player = match state.phase {
                Phase::TakingPokemonFromDiscard { player } => player,
                _ => return Err(IllegalAction),
            };
            state.players[player.index()].discard.retain(|c| *c != card);
            state.players[player.index()].hand.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} returns to hand."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::TakeNamedFromDiscardToBench { card } => {
            let (player, name, remaining) = match state.phase {
                Phase::SearchingDiscardForNamedToBench { player, name, remaining } => {
                    (player, name, remaining)
                }
                _ => return Err(IllegalAction),
            };
            state.players[player.index()].discard.retain(|c| *c != card);
            let pokemon = state.put_into_play(player, card);
            state.players[player.index()].bench.push(pokemon);
            state.log.push(format!("{name} joins the Bench from discard."));
            if remaining <= 1 {
                state.phase = Phase::Main;
                settle(state);
            } else {
                state.phase = Phase::SearchingDiscardForNamedToBench {
                    player,
                    name,
                    remaining: remaining - 1,
                };
            }
        }

        Action::FinishSearchingDiscardForNamedToBench => {
            match state.phase {
                Phase::SearchingDiscardForNamedToBench { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
        }

        Action::AcceptShuffleEnergyForBenchDamage => {
            let (player, attacker, count, damage) = match state.phase {
                Phase::DecidingToShuffleEnergyForBenchDamage { player, attacker, count, damage } => {
                    (player, attacker, count, damage)
                }
                _ => return Err(IllegalAction),
            };
            let energy: Vec<CardId> = state
                .pokemon(attacker)
                .attached
                .iter()
                .copied()
                .filter(|c| state.def_of(*c).is_energy())
                .take(count as usize)
                .collect();
            for card in &energy {
                state.pokemon[attacker.index()].attached.retain(|c| c != card);
            }
            let deck = &mut state.players[player.index()].deck;
            deck.extend(energy);
            shuffle(state.rng.as_mut(), deck);
            let name = state.pokemon_def(attacker).name;
            state.log.push(format!("{name} shuffles Energy back into the deck."));
            let opponent = player.opponent();
            if !state.player(opponent).bench.is_empty() {
                state.phase = Phase::ChoosingBenchDamageTarget { player, damage };
            } else {
                state.phase = Phase::Main;
                settle(state);
            }
        }

        Action::DeclineShuffleEnergyForBenchDamage => {
            match state.phase {
                Phase::DecidingToShuffleEnergyForBenchDamage { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
        }

        Action::MoveOpponentsEnergy { card, target } => {
            let of = match state.phase {
                Phase::MovingOpponentsEnergy { of, .. } => of,
                _ => return Err(IllegalAction),
            };
            for pokemon in state.player(of).in_play() {
                state.pokemon[pokemon.index()].attached.retain(|c| *c != card);
            }
            state.pokemon[target.index()].attached.push(card);
            let energy = state.def_of(card).name();
            let name = state.pokemon_def(target).name;
            state.log.push(format!("{energy} moves to {name}."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::UseAbility { pokemon } => {
            let player = state.current;
            let ability = state
                .pokemon_def(pokemon)
                .ability
                .expect("legal_actions offers UseAbility only for a Pokemon carrying one");
            match ability.effect {
                crate::card::AbilityEffect::OncePerTurnWhileActiveMayDrawCards(count) => {
                    state.spend(Limit::for_ability_use(player, pokemon, ability.name));
                    for _ in 0..count {
                        state.draw(player);
                    }
                    let name = state.pokemon_def(pokemon).name;
                    state.log.push(format!("{player:?} uses {name}'s {}.", ability.name));
                }
                crate::card::AbilityEffect::OncePerTurnIfKnockedOutLastTurnMayDrawCards(count) => {
                    state.spend(Limit::for_ability_use(player, pokemon, ability.name));
                    for _ in 0..count {
                        state.draw(player);
                    }
                    let name = state.pokemon_def(pokemon).name;
                    state.log.push(format!("{player:?} uses {name}'s {}.", ability.name));
                }
                crate::card::AbilityEffect::OncePerTurnMayAttachBasicEnergyOfTypeThenDraw(_) => {
                    // Not spent here: opening the choice is not using it —
                    // only actually attaching (`Action::AttachEnergyForTealDance`) is.
                    state.phase = Phase::DecidingToUseTealDance { player, pokemon };
                }
                crate::card::AbilityEffect::OncePerTurnMayDrawThenShuffleSelfIntoDeck(count) => {
                    state.spend(Limit::for_ability_use(player, pokemon, ability.name));
                    let mut drew_any = false;
                    for _ in 0..count {
                        if state.draw(player) {
                            drew_any = true;
                        }
                    }
                    if drew_any {
                        let was_active = state.player(player).active == Some(pokemon);
                        let has_bench = !state.player(player).bench.is_empty();
                        if !was_active || has_bench {
                            let name = state.pokemon_def(pokemon).name;
                            let cards = std::mem::take(&mut state.pokemon[pokemon.index()].cards);
                            let attached = std::mem::take(&mut state.pokemon[pokemon.index()].attached);
                            let side = &mut state.players[player.index()];
                            side.deck.extend(cards);
                            side.deck.extend(attached);
                            side.bench.retain(|p| *p != pokemon);
                            if was_active {
                                side.active = None;
                            }
                            state.log.push(format!("{name} shuffles itself into the deck."));
                            let deck = &mut state.players[player.index()].deck;
                            shuffle(state.rng.as_mut(), deck);
                            if was_active {
                                state.phase =
                                    Phase::Promoting { of: player, chooser: player, then: None };
                            }
                        }
                    }
                }
                crate::card::AbilityEffect::OncePerTurnWhileActiveMayShuffleSelfIntoDeck => {
                    state.spend(Limit::for_ability_use(player, pokemon, ability.name));
                    let has_bench = !state.player(player).bench.is_empty();
                    if has_bench {
                        let name = state.pokemon_def(pokemon).name;
                        let cards = std::mem::take(&mut state.pokemon[pokemon.index()].cards);
                        let attached = std::mem::take(&mut state.pokemon[pokemon.index()].attached);
                        let side = &mut state.players[player.index()];
                        side.deck.extend(cards);
                        side.deck.extend(attached);
                        side.active = None;
                        state.log.push(format!("{name} shuffles itself into the deck."));
                        let deck = &mut state.players[player.index()].deck;
                        shuffle(state.rng.as_mut(), deck);
                        state.phase = Phase::Promoting { of: player, chooser: player, then: None };
                    }
                }
                crate::card::AbilityEffect::OncePerTurnMayDamageOpponentThenKnockOutSelf(count) => {
                    // Not spent here: opening the choice is not using it —
                    // only actually damaging (`Action::DamageOpponentForCursedBlast`) is.
                    state.phase = Phase::DecidingCursedBlastTarget {
                        player,
                        pokemon,
                        damage: count * 10,
                    };
                }
                crate::card::AbilityEffect::OncePerTurnMaySearchEvolutionPokemonOfType(kind, limit) => {
                    // Not spent here: opening the choice is not using it —
                    // only actually taking a card is.
                    state.phase = Phase::SearchingDeckForEvolutionPokemonOfType {
                        player,
                        pokemon,
                        kind,
                        remaining: limit,
                    };
                }
                crate::card::AbilityEffect::OncePerTurnMayAttachBasicEnergyFromDiscardToChosen => {
                    // Not spent here: opening the choice is not using it —
                    // only actually attaching is.
                    state.phase = Phase::DecidingToUseSeethingSpirit { player, pokemon };
                }
                crate::card::AbilityEffect::OncePerTurnMaySearchAnyCardIfActiveHasNamedAbility(_) => {
                    // Not spent here: opening the choice is not using it —
                    // only actually taking a card is.
                    state.phase = Phase::SearchingDeckForAnyCardAbility { player, pokemon };
                }
                crate::card::AbilityEffect::OncePerTurnMayDiscardFromHandThenDrawCards(draw) => {
                    // Spent here, unlike the choices above: the card's own
                    // text makes the discard the cost of using this Ability
                    // at all, not an optional follow-up once it is open.
                    state.spend(Limit::for_ability_use(player, pokemon, ability.name));
                    state.phase = Phase::DiscardingHandCardThenDrawing { player, pokemon, draw };
                }
                crate::card::AbilityEffect::OncePerTurnMayAttachBasicEnergyOfTypeFromHandToChosenThenHeal(
                    ..,
                ) => {
                    // Not spent here: opening the choice is not using it —
                    // only actually attaching is.
                    state.phase = Phase::DecidingToUseRipeningCharge { player, pokemon };
                }
                crate::card::AbilityEffect::OncePerTurnMaySearchBasicEnergyOfTypeAttachToBenchedThenDamage(
                    kind,
                    damage,
                ) => {
                    // Not spent here: opening the choice is not using it —
                    // only actually attaching is.
                    state.phase = Phase::SearchingForSinisterSurgeTarget {
                        player,
                        pokemon,
                        kind,
                        damage,
                    };
                }
                crate::card::AbilityEffect::OnceDuringFirstTurnMaySearchPokemonOfTypeWithHpAtMost(
                    kind,
                    hp,
                    limit,
                ) => {
                    // Not spent here: opening the choice is not using it.
                    state.phase = Phase::SearchingForFanCall { player, pokemon, kind, hp, remaining: limit };
                }
                crate::card::AbilityEffect::OncePerTurnMaySwitchBenchedOfTypeExcludingNamedThenPoison(
                    kind,
                    excluding,
                ) => {
                    // Not spent here: opening the choice is not using it —
                    // only actually switching is.
                    state.phase = Phase::DecidingToUseSubjugatingChains {
                        player,
                        name: ability.name,
                        kind,
                        excluding,
                    };
                }
                crate::card::AbilityEffect::WhenBenchedFromHandMaySearchSupporter
                | crate::card::AbilityEffect::WhenEvolvedFromHandMayDrawCards(_)
                | crate::card::AbilityEffect::WhenEvolvedFromHandMaySearchTrainersIfOwnTeraInPlay(_)
                | crate::card::AbilityEffect::WhenBenchedFromHandMayDiscardStadium
                | crate::card::AbilityEffect::WhenBenchedFromHandMaySwitchThenMoveAnyEnergy => {
                    unreachable!("legal_actions never offers UseAbility for a play-triggered effect")
                }
                crate::card::AbilityEffect::PassiveOwnBasicPokemonHaveNoRetreatCost
                | crate::card::AbilityEffect::PassiveImmuneToDamageFromOpponentEx
                | crate::card::AbilityEffect::PassiveBlocksDamageCounterMovement
                | crate::card::AbilityEffect::PassiveDisablesSelfKnockOutAbilities
                | crate::card::AbilityEffect::PassiveSetsOpponentTypeWeaknessTo(..)
                | crate::card::AbilityEffect::PassivePreventsAttackDamageToNonRuleBoxBench
                | crate::card::AbilityEffect::PassivePreventsAttackEffectsOnBench
                | crate::card::AbilityEffect::PassiveFutureAttacksDoBonusDamageToActiveExceptNamed(_)
                | crate::card::AbilityEffect::PassiveBonusDamageToActiveIfSelfDamaged(_)
                | crate::card::AbilityEffect::PassiveImmuneToAsleep
                | crate::card::AbilityEffect::PassiveDisablesOpponentActiveAbilityExceptSelf
                | crate::card::AbilityEffect::PassiveNamedAttackCostsLessPerOpponentPrizeTaken(_)
                | crate::card::AbilityEffect::PassiveBlocksOpponentAceSpecPlaysIfSelfHasTool
                | crate::card::AbilityEffect::PassiveNamedAttackCostsJustColorlessIfOpponentDiscardNameContains(
                    ..,
                )
                | crate::card::AbilityEffect::PassiveCoinFlipPreventsAttackKnockOutAtTenHp
                | crate::card::AbilityEffect::PassiveFestivalLead
                | crate::card::AbilityEffect::PassiveDoublesBasicGrassEnergyForCost
                | crate::card::AbilityEffect::PassiveBonusCheckupDamageToOpponentsPoisonedWhileActive(
                    _,
                ) => {
                    unreachable!("legal_actions never offers UseAbility for a standing passive effect")
                }
                crate::card::AbilityEffect::OncePerTurnIfEnergyOfTypeAttachedMayMoveDamageCountersToOpponent(
                    _,
                    limit,
                ) => {
                    // Not spent here: opening the choice is not using it —
                    // only actually moving a counter, or declining, is.
                    state.phase =
                        Phase::MovingDamageCountersFromOwnToOpponent { player, pokemon, limit };
                }
                crate::card::AbilityEffect::OncePerTurnMayLookAtTopCardsTakeOneRestToBottom(count) => {
                    // Spent immediately: unlike a search that may come up
                    // empty, looking at the top of the deck and putting
                    // the rest on the bottom always happens once opened —
                    // there is no "decline" step left to spend it on.
                    let ability = state.pokemon_def(pokemon).ability.expect("named only when carried");
                    state.spend(Limit::for_ability_use(player, pokemon, ability.name));
                    state.phase = Phase::LookingAtTopCardsToTakeOne { player, pokemon, count };
                }
                crate::card::AbilityEffect::OncePerTurnMayLookAtTopCardsAttachFoundBasicEnergyOfType(
                    count,
                    kind,
                ) => {
                    // Spent immediately, the same reasoning as
                    // `OncePerTurnMayLookAtTopCardsTakeOneRestToBottom`.
                    let ability = state.pokemon_def(pokemon).ability.expect("named only when carried");
                    state.spend(Limit::for_ability_use(player, pokemon, ability.name));
                    state.phase =
                        Phase::ResolvingEnergyFoundInTopPeek { player, pokemon, kind, remaining: count };
                }
                crate::card::AbilityEffect::OncePerTurnWhileActiveMayLookAtTopCardsTakeASupporter(count) => {
                    // Not spent here: opening the choice is not using it —
                    // only actually taking a Supporter is. Declining shuffles
                    // the peek back with nothing spent, the same way
                    // `DecidingToUseTealDance` already works.
                    state.phase = Phase::LookingAtTopCardsForSupporter { player, pokemon, count };
                }
            }
        }

        Action::TakeSupporterForLastDitchCatch { card } => {
            let (player, pokemon) = match state.phase {
                Phase::DecidingToUseLastDitchCatch { player, pokemon } => (player, pokemon),
                _ => return Err(IllegalAction),
            };
            let ability = state.pokemon_def(pokemon).ability.expect("named only when carried");
            state.spend(Limit::for_ability_use(player, pokemon, ability.name));
            state.players[player.index()].deck.retain(|c| *c != card);
            state.players[player.index()].hand.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} joins the hand (Last-Ditch Catch)."));
            let deck = &mut state.players[player.index()].deck;
            shuffle(state.rng.as_mut(), deck);
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DeclineLastDitchCatch => {
            match state.phase {
                Phase::DecidingToUseLastDitchCatch { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
        }

        Action::AcceptPsychicDraw => {
            let (player, pokemon, name, count) = match state.phase {
                Phase::DecidingToUsePsychicDraw { player, pokemon, name, count } => {
                    (player, pokemon, name, count)
                }
                _ => return Err(IllegalAction),
            };
            state.spend(Limit::for_ability_use(player, pokemon, name));
            for _ in 0..count {
                state.draw(player);
            }
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DeclinePsychicDraw => {
            match state.phase {
                Phase::DecidingToUsePsychicDraw { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
        }

        Action::AcceptJewelSeeker => {
            let (player, pokemon, name, count) = match state.phase {
                Phase::DecidingToUseJewelSeeker { player, pokemon, name, count } => {
                    (player, pokemon, name, count)
                }
                _ => return Err(IllegalAction),
            };
            state.spend(Limit::for_ability_use(player, pokemon, name));
            state.phase = Phase::SearchingDeckForTrainerCards { player, remaining: count };
        }

        Action::DeclineJewelSeeker => {
            match state.phase {
                Phase::DecidingToUseJewelSeeker { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
        }

        Action::TakeTrainerCardFromDeck { card } => {
            let (player, remaining) = match state.phase {
                Phase::SearchingDeckForTrainerCards { player, remaining } => (player, remaining),
                _ => return Err(IllegalAction),
            };
            state.players[player.index()].deck.retain(|c| *c != card);
            state.players[player.index()].hand.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} joins the hand."));
            if remaining <= 1 {
                finish_searching_deck_for_trainer_cards(state, player);
            } else {
                state.phase = Phase::SearchingDeckForTrainerCards { player, remaining: remaining - 1 };
            }
        }

        Action::FinishSearchingTrainerCards => {
            let player = match state.phase {
                Phase::SearchingDeckForTrainerCards { player, .. } => player,
                _ => return Err(IllegalAction),
            };
            finish_searching_deck_for_trainer_cards(state, player);
        }

        Action::TakePokemonOfTypeOrStadiumFromDeck { card } => {
            let (player, kind, remaining) = match state.phase {
                Phase::SearchingDeckForPokemonOfTypeOrStadium { player, kind, remaining } => {
                    (player, kind, remaining)
                }
                _ => return Err(IllegalAction),
            };
            state.players[player.index()].deck.retain(|c| *c != card);
            state.players[player.index()].hand.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} joins the hand."));
            if remaining <= 1 {
                finish_searching_deck_for_pokemon_of_type_or_stadium(state, player);
            } else {
                state.phase = Phase::SearchingDeckForPokemonOfTypeOrStadium {
                    player,
                    kind,
                    remaining: remaining - 1,
                };
            }
        }

        Action::FinishSearchingPokemonOfTypeOrStadium => {
            let player = match state.phase {
                Phase::SearchingDeckForPokemonOfTypeOrStadium { player, .. } => player,
                _ => return Err(IllegalAction),
            };
            finish_searching_deck_for_pokemon_of_type_or_stadium(state, player);
        }

        Action::DiscardCardFromOpponentsHand { card } => {
            let player = match state.phase {
                Phase::ChoosingCardFromOpponentsHandToDiscard { player } => player,
                _ => return Err(IllegalAction),
            };
            let opponent = player.opponent();
            state.players[opponent.index()].hand.retain(|c| *c != card);
            state.players[opponent.index()].discard.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} is discarded from the opponent's hand."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DamageChosenOpponentPokemon { target } => {
            let damage = match state.phase {
                Phase::ChoosingAnyOpponentPokemonDamageTarget { damage, .. } => damage,
                _ => return Err(IllegalAction),
            };
            state.pokemon[target.index()].damage += damage;
            let name = state.pokemon_def(target).name;
            state.log.push(format!("{name} takes {damage}."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DamageChosenOpponentPokemonWeaknessIfActive { target } => {
            let (attacker, mut damage) = match state.phase {
                Phase::ChoosingAnyOpponentPokemonDamageTargetWeaknessIfActive { attacker, damage, .. } => {
                    (attacker, damage)
                }
                _ => return Err(IllegalAction),
            };
            let owner = state.pokemon(target).owner;
            if state.player(owner).active == Some(target) {
                let attacker_type = state.pokemon_def(attacker).kind;
                if state.effective_weakness(target) == Some(attacker_type) {
                    damage *= 2;
                }
                if state.pokemon_def(target).resistance == Some(attacker_type) {
                    damage = damage.saturating_sub(30);
                }
            }
            state.pokemon[target.index()].damage += damage;
            let name = state.pokemon_def(target).name;
            state.log.push(format!("{name} takes {damage}."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::ChooseOwnBenchedSourceForDamageMove { source } => {
            let player = match state.phase {
                Phase::ChoosingOwnBenchedSourceForDamageMove { player } => player,
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::ChoosingOpponentTargetForDamageMove { player, source };
        }

        Action::MoveDamageToChosenOpponentPokemon { target } => {
            let source = match state.phase {
                Phase::ChoosingOpponentTargetForDamageMove { source, .. } => source,
                _ => return Err(IllegalAction),
            };
            let amount = state.pokemon(source).damage;
            state.pokemon[source.index()].damage = 0;
            state.pokemon[target.index()].damage += amount;
            let source_name = state.pokemon_def(source).name;
            let target_name = state.pokemon_def(target).name;
            state.log.push(format!("{amount} damage moves from {source_name} to {target_name}."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::ChooseBenchedTargetForEnergySearch { target } => {
            let (player, kind, max) = match state.phase {
                Phase::ChoosingBenchedTargetForEnergySearch { player, kind, max } => (player, kind, max),
                _ => return Err(IllegalAction),
            };
            state.phase =
                Phase::SearchingEnergyOfTypeToAttachToChosen { player, kind, target, remaining: max };
        }

        Action::TakeEnergyOfTypeToAttachToChosen { card } => {
            let (player, kind, target, remaining) = match state.phase {
                Phase::SearchingEnergyOfTypeToAttachToChosen { player, kind, target, remaining } => {
                    (player, kind, target, remaining)
                }
                _ => return Err(IllegalAction),
            };
            state.players[player.index()].deck.retain(|c| *c != card);
            state.pokemon[target.index()].attached.push(card);
            let name = state.def_of(card).name();
            let target_name = state.pokemon_def(target).name;
            state.log.push(format!("{name} attaches to {target_name}."));
            if remaining <= 1 {
                finish_searching_energy_of_type_to_attach_to_chosen(state, player);
            } else {
                state.phase = Phase::SearchingEnergyOfTypeToAttachToChosen {
                    player,
                    kind,
                    target,
                    remaining: remaining - 1,
                };
            }
        }

        Action::FinishSearchingEnergyOfTypeToAttachToChosen => {
            let player = match state.phase {
                Phase::SearchingEnergyOfTypeToAttachToChosen { player, .. } => player,
                _ => return Err(IllegalAction),
            };
            finish_searching_energy_of_type_to_attach_to_chosen(state, player);
        }

        Action::DamageOneOfTwoChosenOpponentPokemon { target } => {
            let (player, damage, excluding) = match state.phase {
                Phase::ChoosingTwoOpponentPokemonDamageTargets { player, damage, excluding } => {
                    (player, damage, excluding)
                }
                _ => return Err(IllegalAction),
            };
            state.pokemon[target.index()].damage += damage;
            let name = state.pokemon_def(target).name;
            state.log.push(format!("{name} takes {damage}."));
            if excluding.is_none()
                && state.player(player.opponent()).in_play().iter().any(|p| *p != target)
            {
                // The first pick: reopen the same phase, this pick now
                // excluded, so the second pick names a different one.
                state.phase = Phase::ChoosingTwoOpponentPokemonDamageTargets {
                    player,
                    damage,
                    excluding: Some(target),
                };
            } else {
                state.phase = Phase::Main;
                settle(state);
            }
        }

        Action::DamageOneOfThreeChosenOpponentPokemon { target } => {
            let (player, damage, excluding) = match state.phase {
                Phase::ChoosingThreeOpponentPokemonDamageTargets { player, damage, excluding } => {
                    (player, damage, excluding)
                }
                _ => return Err(IllegalAction),
            };
            state.pokemon[target.index()].damage += damage;
            let name = state.pokemon_def(target).name;
            state.log.push(format!("{name} takes {damage}."));
            let first_open = excluding.iter().position(|slot| slot.is_none());
            let more_targets = state
                .player(player.opponent())
                .in_play()
                .iter()
                .any(|p| *p != target && !excluding.contains(&Some(*p)));
            if let Some(slot) = first_open
                && more_targets
            {
                let mut excluding = excluding;
                excluding[slot] = Some(target);
                state.phase =
                    Phase::ChoosingThreeOpponentPokemonDamageTargets { player, damage, excluding };
            } else {
                state.phase = Phase::Main;
                settle(state);
            }
        }

        Action::CopyBenchedPokemonAttack { pokemon, index } => {
            let player = match state.phase {
                Phase::ChoosingBenchedPokemonAttackToCopy { player, .. } => player,
                _ => return Err(IllegalAction),
            };
            let attacker = state
                .player(player)
                .active
                .expect("Night Joker was used, so its own Pokémon is still Active");
            let Some(defender) = state.player(player.opponent()).active else {
                state.phase = Phase::Main;
                settle(state);
                return Ok(());
            };
            let attack = state.pokemon_def(pokemon).attacks[index].clone();
            state.phase = Phase::Main;
            attack_with(state, attacker, defender, attack);
            state.pending_end_turn = true;
            if state.phase == Phase::Main {
                settle(state);
            }
        }

        Action::DiscardHandCardThenDraw { card } => {
            let (player, pokemon, draw) = match state.phase {
                Phase::DiscardingHandCardThenDrawing { player, pokemon, draw } => {
                    (player, pokemon, draw)
                }
                _ => return Err(IllegalAction),
            };
            state.players[player.index()].hand.retain(|c| *c != card);
            state.players[player.index()].discard.push(card);
            let name = state.pokemon_def(pokemon).name;
            state.log.push(format!("{player:?} uses {name}'s Trade."));
            for _ in 0..draw {
                state.draw(player);
            }
            state.phase = Phase::Main;
            settle(state);
        }

        Action::CopyDiscardedPokemonAttack { index } => {
            let (player, card) = match state.phase {
                Phase::ChoosingDiscardedPokemonAttackToCopy { player, card } => (player, card),
                _ => return Err(IllegalAction),
            };
            let attacker = state
                .player(player)
                .active
                .expect("Seek Inspiration was used, so its own Pokémon is still Active");
            let Some(defender) = state.player(player.opponent()).active else {
                state.phase = Phase::Main;
                settle(state);
                return Ok(());
            };
            let attack = state
                .def_of(card)
                .as_pokemon()
                .expect("only opened for a Pokémon")
                .attacks[index]
                .clone();
            state.phase = Phase::Main;
            attack_with(state, attacker, defender, attack);
            state.pending_end_turn = true;
            if state.phase == Phase::Main {
                settle(state);
            }
        }

        Action::DamageBenchedEx { target } => {
            let (player, damage) = match state.phase {
                Phase::ChoosingBenchedExDamageTarget { player, damage } => (player, damage),
                _ => return Err(IllegalAction),
            };
            let name = state.pokemon_def(target).name;
            if state.bench_attack_effect_blocked(player, target) {
                // `Rabsca`'s `Spherical Shield`: a Benched ex has no
                // Rule Box exemption from Flower Curtain, but Spherical
                // Shield names every Benched Pokemon regardless.
                state.log.push(format!("{name} would take {damage}, but Spherical Shield stops it landing."));
            } else {
                state.pokemon[target.index()].damage += damage;
                state.log.push(format!("{name} takes {damage}."));
            }
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DamageAnyBenched { target } => {
            let (player, damage) = match state.phase {
                Phase::ChoosingAnyBenchedDamageTarget { player, damage } => (player, damage),
                _ => return Err(IllegalAction),
            };
            let name = state.pokemon_def(target).name;
            if state.bench_attack_effect_blocked(player, target) {
                state.log.push(format!("{name} would take {damage}, but Spherical Shield stops it landing."));
            } else {
                state.pokemon[target.index()].damage += damage;
                state.log.push(format!("{name} takes {damage}."));
            }
            state.phase = Phase::Main;
            settle(state);
        }

        Action::AttachSearchedEnergyTo { target } => {
            let player = match state.phase {
                Phase::SearchingForEnergyToAttachToBenchedOfType { player, .. } => player,
                _ => return Err(IllegalAction),
            };
            let energy = *state.players[player.index()]
                .deck
                .iter()
                .find(|c| state.def_of(**c).is_energy())
                .expect("legal_actions offers this only with a qualifying Energy in the deck");
            state.players[player.index()].deck.retain(|c| *c != energy);
            state.pokemon[target.index()].attached.push(energy);
            let name = state.pokemon_def(target).name;
            state.log.push(format!("{name} takes an Energy (Send Flowers)."));
            let deck = &mut state.players[player.index()].deck;
            shuffle(state.rng.as_mut(), deck);
            state.phase = Phase::Main;
            settle(state);
        }

        Action::AttachEnergyForTealDance { card } => {
            let (player, pokemon) = match state.phase {
                Phase::DecidingToUseTealDance { player, pokemon } => (player, pokemon),
                _ => return Err(IllegalAction),
            };
            let ability = state.pokemon_def(pokemon).ability.expect("named only when carried");
            state.spend(Limit::for_ability_use(player, pokemon, ability.name));
            state.remove_from_hand(player, card);
            state.pokemon[pokemon.index()].attached.push(card);
            state.draw(player);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} attaches (Teal Dance), and a card is drawn."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DeclineTealDance => {
            match state.phase {
                Phase::DecidingToUseTealDance { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DamageOpponentForCursedBlast { target } => {
            let (player, pokemon, damage) = match state.phase {
                Phase::DecidingCursedBlastTarget { player, pokemon, damage } => {
                    (player, pokemon, damage)
                }
                _ => return Err(IllegalAction),
            };
            let ability = state.pokemon_def(pokemon).ability.expect("named only when carried");
            state.spend(Limit::for_ability_use(player, pokemon, ability.name));
            let target_name = state.pokemon_def(target).name;
            if state.bench_damage_counters_blocked(player, target) {
                // `Battle Cage`: the Ability is still used — Dusknoir
                // still knocks itself out below — but a Benched target
                // protected by Battle Cage takes no damage counters at
                // all, so nothing lands.
                state.log.push(format!(
                    "{target_name} would take {damage}, but Battle Cage stops it landing."
                ));
            } else {
                state.pokemon[target.index()].damage += damage;
                state.log.push(format!("{target_name} takes {damage}."));
            }
            // "This Pokémon is Knocked Out" outright: raising its own
            // damage to its effective HP, not a separate forced-knockout
            // primitive, so the ordinary sweep still awards the Prize.
            let effective_hp = state.effective_hp(pokemon);
            state.pokemon[pokemon.index()].damage = state.pokemon[pokemon.index()].damage.max(effective_hp);
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DeclineCursedBlast => {
            match state.phase {
                Phase::DecidingCursedBlastTarget { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
        }

        Action::TakeEvolutionPokemonOfType { card } => {
            let (player, pokemon, kind, remaining) = match state.phase {
                Phase::SearchingDeckForEvolutionPokemonOfType { player, pokemon, kind, remaining } => {
                    (player, pokemon, kind, remaining)
                }
                _ => return Err(IllegalAction),
            };
            let ability = state.pokemon_def(pokemon).ability.expect("named only when carried");
            state.spend(Limit::for_ability_use(player, pokemon, ability.name));
            state.players[player.index()].deck.retain(|c| *c != card);
            state.players[player.index()].hand.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} joins the hand."));
            if remaining <= 1 {
                let deck = &mut state.players[player.index()].deck;
                shuffle(state.rng.as_mut(), deck);
                state.phase = Phase::Main;
                settle(state);
            } else {
                state.phase = Phase::SearchingDeckForEvolutionPokemonOfType {
                    player,
                    pokemon,
                    kind,
                    remaining: remaining - 1,
                };
            }
        }

        Action::TakeAnyCardFromDeckForAbility { card } => {
            let (player, pokemon) = match state.phase {
                Phase::SearchingDeckForAnyCardAbility { player, pokemon } => (player, pokemon),
                _ => return Err(IllegalAction),
            };
            let ability = state.pokemon_def(pokemon).ability.expect("named only when carried");
            state.spend(Limit::for_ability_use(player, pokemon, ability.name));
            state.players[player.index()].deck.retain(|c| *c != card);
            state.players[player.index()].hand.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} joins the hand."));
            let deck = &mut state.players[player.index()].deck;
            shuffle(state.rng.as_mut(), deck);
            state.phase = Phase::Main;
            settle(state);
        }

        Action::FinishSearchingDeckForAnyCardAbility => {
            match state.phase {
                Phase::SearchingDeckForAnyCardAbility { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
        }

        Action::FinishSearchingEvolutionPokemonOfType => {
            let player = match state.phase {
                Phase::SearchingDeckForEvolutionPokemonOfType { player, .. } => player,
                _ => return Err(IllegalAction),
            };
            let deck = &mut state.players[player.index()].deck;
            shuffle(state.rng.as_mut(), deck);
            state.phase = Phase::Main;
            settle(state);
        }

        Action::AttachEnergyForSeethingSpirit { card, target } => {
            let (player, pokemon) = match state.phase {
                Phase::DecidingToUseSeethingSpirit { player, pokemon } => (player, pokemon),
                _ => return Err(IllegalAction),
            };
            let ability = state.pokemon_def(pokemon).ability.expect("named only when carried");
            state.spend(Limit::for_ability_use(player, pokemon, ability.name));
            state.players[player.index()].discard.retain(|c| *c != card);
            state.pokemon[target.index()].attached.push(card);
            let name = state.def_of(card).name();
            let target_name = state.pokemon_def(target).name;
            state.log.push(format!("{name} attaches to {target_name} (Seething Spirit)."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DeclineSeethingSpirit => {
            match state.phase {
                Phase::DecidingToUseSeethingSpirit { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
        }

        Action::AttachEnergyForRipeningCharge { card, target } => {
            let (player, pokemon) = match state.phase {
                Phase::DecidingToUseRipeningCharge { player, pokemon } => (player, pokemon),
                _ => return Err(IllegalAction),
            };
            let ability = state.pokemon_def(pokemon).ability.expect("named only when carried");
            let crate::card::AbilityEffect::OncePerTurnMayAttachBasicEnergyOfTypeFromHandToChosenThenHeal(
                _,
                heal,
            ) = ability.effect
            else {
                unreachable!("this phase only ever opens for this effect");
            };
            state.spend(Limit::for_ability_use(player, pokemon, ability.name));
            state.remove_from_hand(player, card);
            state.pokemon[target.index()].attached.push(card);
            state.pokemon[target.index()].damage = state.pokemon(target).damage.saturating_sub(heal);
            let name = state.def_of(card).name();
            let target_name = state.pokemon_def(target).name;
            state.log.push(format!("{name} attaches to {target_name}, healing {heal} (Ripening Charge)."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DeclineRipeningCharge => {
            match state.phase {
                Phase::DecidingToUseRipeningCharge { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
        }

        Action::MoveOwnAttachedEnergyToHand { card } => {
            let (player, attacker) = match state.phase {
                Phase::ChoosingOwnEnergyToHand { player, attacker } => (player, attacker),
                _ => return Err(IllegalAction),
            };
            state.pokemon[attacker.index()].attached.retain(|c| *c != card);
            state.players[player.index()].hand.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} returns to hand."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::MoveEnergyToChosenBenched { card, target } => {
            let (_player, attacker) = match state.phase {
                Phase::ChoosingEnergyAndBenchedTargetToMove { player, attacker } => (player, attacker),
                _ => return Err(IllegalAction),
            };
            state.pokemon[attacker.index()].attached.retain(|c| *c != card);
            state.pokemon[target.index()].attached.push(card);
            let name = state.def_of(card).name();
            let target_name = state.pokemon_def(target).name;
            state.log.push(format!("{name} moves to {target_name}."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::MoveDamageCountersFromOwnToOpponent { source, target, count } => {
            let (player, pokemon) = match state.phase {
                Phase::MovingDamageCountersFromOwnToOpponent { player, pokemon, .. } => {
                    (player, pokemon)
                }
                _ => return Err(IllegalAction),
            };
            let ability = state.pokemon_def(pokemon).ability.expect("named only when carried");
            state.spend(Limit::for_ability_use(player, pokemon, ability.name));
            state.pokemon[source.index()].damage -= count;
            let source_name = state.pokemon_def(source).name;
            let target_name = state.pokemon_def(target).name;
            if state.bench_damage_counters_blocked(player, target) {
                // `Battle Cage`: the counters still leave `source` — the
                // move already started — but they never land, so they
                // vanish rather than piling up somewhere else.
                state.log.push(format!(
                    "{count} damage would move from {source_name} to {target_name}, but Battle Cage stops it landing; the counters vanish."
                ));
            } else {
                state.pokemon[target.index()].damage += count;
                state.log.push(format!("{count} damage moves from {source_name} to {target_name}."));
            }
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DeclineMovingDamageCounters => {
            match state.phase {
                Phase::MovingDamageCountersFromOwnToOpponent { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
        }

        Action::TakeCardFromTopPeek { card } => {
            let (player, count) = match state.phase {
                Phase::LookingAtTopCardsToTakeOne { player, count, .. } => (player, count),
                _ => return Err(IllegalAction),
            };
            let deck = &mut state.players[player.index()].deck;
            let seen = (count as usize).min(deck.len());
            let start = deck.len() - seen;
            let peeked: Vec<CardId> = deck.drain(start..).collect();
            for c in peeked {
                if c == card {
                    state.players[player.index()].hand.push(c);
                } else {
                    state.players[player.index()].deck.insert(0, c);
                }
            }
            let name = state.def_of(card).name();
            state.log.push(format!("{name} taken from the top (Recon Directive)."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::AttachFoundEnergyTo { card, target } => {
            let (player, pokemon, kind, remaining) = match state.phase {
                Phase::ResolvingEnergyFoundInTopPeek { player, pokemon, kind, remaining } => {
                    (player, pokemon, kind, remaining)
                }
                _ => return Err(IllegalAction),
            };
            state.players[player.index()].deck.retain(|c| *c != card);
            state.pokemon[target.index()].attached.push(card);
            let name = state.def_of(card).name();
            let target_name = state.pokemon_def(target).name;
            state.log.push(format!("{name} attaches to {target_name} (Metal Maker)."));
            let left = remaining - 1;
            if left == 0 {
                state.phase = Phase::Main;
                settle(state);
            } else {
                state.phase = Phase::ResolvingEnergyFoundInTopPeek { player, pokemon, kind, remaining: left };
            }
        }

        Action::PutFoundCardOnBottom { card } => {
            let (player, pokemon, kind, remaining) = match state.phase {
                Phase::ResolvingEnergyFoundInTopPeek { player, pokemon, kind, remaining } => {
                    (player, pokemon, kind, remaining)
                }
                _ => return Err(IllegalAction),
            };
            state.players[player.index()].deck.retain(|c| *c != card);
            state.players[player.index()].deck.insert(0, card);
            let left = remaining - 1;
            if left == 0 {
                state.phase = Phase::Main;
                settle(state);
            } else {
                state.phase = Phase::ResolvingEnergyFoundInTopPeek { player, pokemon, kind, remaining: left };
            }
        }

        Action::TakeSupporterFromTopPeek { card } => {
            let (player, pokemon) = match state.phase {
                Phase::LookingAtTopCardsForSupporter { player, pokemon, .. } => (player, pokemon),
                _ => return Err(IllegalAction),
            };
            let ability = state.pokemon_def(pokemon).ability.expect("named only when carried");
            state.spend(Limit::for_ability_use(player, pokemon, ability.name));
            state.players[player.index()].deck.retain(|c| *c != card);
            state.players[player.index()].hand.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} taken from the top (Attract Customers)."));
            let deck = &mut state.players[player.index()].deck;
            shuffle(state.rng.as_mut(), deck);
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DeclineTopPeekSupporter => {
            match state.phase {
                Phase::LookingAtTopCardsForSupporter { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
        }

        Action::AcceptSnowSink => {
            let (player, pokemon) = match state.phase {
                Phase::DecidingToUseSnowSink { player, pokemon } => (player, pokemon),
                _ => return Err(IllegalAction),
            };
            let ability = state.pokemon_def(pokemon).ability.expect("named only when carried");
            state.spend(Limit::for_ability_use(player, pokemon, ability.name));
            let (owner, card) = state.stadium.take().expect("legal_actions offers this only with a Stadium in play");
            state.players[owner.index()].discard.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} is discarded (Snow Sink)."));
            if !open_discard_bench_down_to_if_stadium_left(state, owner, card) {
                state.phase = Phase::Main;
                settle(state);
            }
        }

        Action::DeclineSnowSink => {
            match state.phase {
                Phase::DecidingToUseSnowSink { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
        }

        Action::AcceptRapidVernierSwitch => {
            let (player, pokemon) = match state.phase {
                Phase::DecidingToSwitchInForRapidVernier { player, pokemon } => (player, pokemon),
                _ => return Err(IllegalAction),
            };
            let ability = state.pokemon_def(pokemon).ability.expect("named only when carried");
            state.spend(Limit::for_ability_use(player, pokemon, ability.name));
            let side = &mut state.players[player.index()];
            let old_active = side.active.expect("this trigger only fires with an Active in place");
            side.active = Some(pokemon);
            side.bench.retain(|p| *p != pokemon);
            side.bench.push(old_active);
            state.promoted_from_bench_this_turn[player.index()] = Some(pokemon);
            let name = state.pokemon_def(pokemon).name;
            state.log.push(format!("{name} switches in (Rapid Vernier)."));
            state.phase = Phase::MovingAnyEnergyForRapidVernier { player, pokemon };
        }

        Action::DeclineRapidVernierSwitch => {
            match state.phase {
                Phase::DecidingToSwitchInForRapidVernier { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
        }

        Action::MoveEnergyForRapidVernier { card } => {
            let (player, pokemon) = match state.phase {
                Phase::MovingAnyEnergyForRapidVernier { player, pokemon } => (player, pokemon),
                _ => return Err(IllegalAction),
            };
            for source in state.player(player).in_play() {
                state.pokemon[source.index()].attached.retain(|c| *c != card);
            }
            state.pokemon[pokemon.index()].attached.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} moves (Rapid Vernier)."));
            state.phase = Phase::MovingAnyEnergyForRapidVernier { player, pokemon };
        }

        Action::FinishMovingEnergyForRapidVernier => {
            match state.phase {
                Phase::MovingAnyEnergyForRapidVernier { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
        }

        Action::AttachSinisterSurgeEnergyTo { target } => {
            let (player, pokemon, kind, damage) = match state.phase {
                Phase::SearchingForSinisterSurgeTarget { player, pokemon, kind, damage } => {
                    (player, pokemon, kind, damage)
                }
                _ => return Err(IllegalAction),
            };
            let ability = state.pokemon_def(pokemon).ability.expect("named only when carried");
            state.spend(Limit::for_ability_use(player, pokemon, ability.name));
            let energy = *state.players[player.index()]
                .deck
                .iter()
                .find(|c| state.matches_filter(**c, crate::card::CardFilter::BasicEnergyOfType(kind)))
                .expect("legal_actions offers this only with a qualifying Energy in the deck");
            state.players[player.index()].deck.retain(|c| *c != energy);
            state.pokemon[target.index()].attached.push(energy);
            state.pokemon[target.index()].damage += damage;
            let name = state.pokemon_def(target).name;
            state.log.push(format!("{name} takes an Energy and {damage} (Sinister Surge)."));
            let deck = &mut state.players[player.index()].deck;
            shuffle(state.rng.as_mut(), deck);
            state.phase = Phase::Main;
            settle(state);
        }

        Action::TakeCardForFanCall { card } => {
            let (player, pokemon, kind, hp, remaining) = match state.phase {
                Phase::SearchingForFanCall { player, pokemon, kind, hp, remaining } => {
                    (player, pokemon, kind, hp, remaining)
                }
                _ => return Err(IllegalAction),
            };
            let ability = state.pokemon_def(pokemon).ability.expect("named only when carried");
            state.spend(Limit::for_ability_use(player, pokemon, ability.name));
            state.players[player.index()].deck.retain(|c| *c != card);
            state.players[player.index()].hand.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} joins the hand (Fan Call)."));
            if remaining <= 1 {
                let deck = &mut state.players[player.index()].deck;
                shuffle(state.rng.as_mut(), deck);
                state.phase = Phase::Main;
                settle(state);
            } else {
                state.phase = Phase::SearchingForFanCall {
                    player,
                    pokemon,
                    kind,
                    hp,
                    remaining: remaining - 1,
                };
            }
        }

        Action::FinishFanCall => {
            let player = match state.phase {
                Phase::SearchingForFanCall { player, .. } => player,
                _ => return Err(IllegalAction),
            };
            let deck = &mut state.players[player.index()].deck;
            shuffle(state.rng.as_mut(), deck);
            state.phase = Phase::Main;
            settle(state);
        }

        Action::SwitchForSubjugatingChains { target } => {
            let (player, name) = match state.phase {
                Phase::DecidingToUseSubjugatingChains { player, name, .. } => (player, name),
                _ => return Err(IllegalAction),
            };
            // Subjugating Chains is one of the four name-scoped Abilities.
            state.spend(Limit::AbilityUsedByName(player, name));
            let side = &mut state.players[player.index()];
            let old_active = side.active.expect("this Ability needs an Active to swap with");
            side.active = Some(target);
            side.bench.retain(|p| *p != target);
            side.bench.push(old_active);
            state.promoted_from_bench_this_turn[player.index()] = Some(target);
            state.inflict(target, Condition::Poisoned);
            let target_name = state.pokemon_def(target).name;
            state.log.push(format!("{target_name} switches in and is Poisoned (Subjugating Chains)."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DiscardToolAnywhere { card } => {
            let (player, remaining) = match state.phase {
                Phase::DiscardingToolsAnywhere { player, remaining } => (player, remaining),
                _ => return Err(IllegalAction),
            };
            let owner = [PlayerId::One, PlayerId::Two]
                .into_iter()
                .find(|p| state.player(*p).in_play().iter().any(|pk| state.pokemon(*pk).attached.contains(&card)))
                .expect("legal_actions offers this only for a Tool actually attached somewhere");
            for pokemon in state.player(owner).in_play() {
                state.pokemon[pokemon.index()].attached.retain(|c| *c != card);
            }
            state.players[owner.index()].discard.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} is discarded (Tool Scrapper)."));
            if remaining <= 1 {
                state.phase = Phase::Main;
                settle(state);
            } else {
                state.phase = Phase::DiscardingToolsAnywhere { player, remaining: remaining - 1 };
            }
        }

        Action::FinishDiscardingToolsAnywhere => {
            match state.phase {
                Phase::DiscardingToolsAnywhere { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
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
            state.players[player.index()].deck.retain(|c| *c != card);
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
            if !state.pokemon(target).knocked_out {
                trigger_psychic_draw(state, player, target);
                trigger_jewel_seeker(state, player, target);
            }
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

        Action::DiscardOpponentSpecialEnergy { card } => {
            let of = match state.phase {
                Phase::DiscardingOpponentSpecialEnergy { of, .. } => of,
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

        Action::DiscardDefenderEnergyForAttack { card } => {
            let target = match state.phase {
                Phase::DiscardingDefenderEnergyForAttack { target, .. } => target,
                _ => return Err(IllegalAction),
            };
            let owner = state.pokemon(target).owner;
            state.pokemon[target.index()].attached.retain(|c| *c != card);
            state.players[owner.index()].discard.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} is discarded."));
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DiscardOwnEnergyForAttack { card } => {
            let (player, attacker, remaining) = match state.phase {
                Phase::ChoosingOwnEnergyToDiscardForAttack { player, attacker, remaining } => {
                    (player, attacker, remaining)
                }
                _ => return Err(IllegalAction),
            };
            state.pokemon[attacker.index()].attached.retain(|c| *c != card);
            state.players[player.index()].discard.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} is discarded."));
            if remaining > 1 {
                state.phase = Phase::ChoosingOwnEnergyToDiscardForAttack {
                    player,
                    attacker,
                    remaining: remaining - 1,
                };
            } else {
                state.phase = Phase::Main;
                settle(state);
            }
        }

        Action::AcceptDiscardOwnEnergyForBonusDamage => {
            let (player, attacker, defender, kind, max, bonus) = match state.phase {
                Phase::DecidingToDiscardOwnEnergyForBonusDamage {
                    player,
                    attacker,
                    defender,
                    kind,
                    max,
                    bonus,
                } => (player, attacker, defender, kind, max, bonus),
                _ => return Err(IllegalAction),
            };
            state.pokemon[defender.index()].damage += bonus;
            let name = state.pokemon_def(defender).name;
            state.log.push(format!("{name} takes {bonus} more."));
            state.phase = Phase::ChoosingOwnEnergyToDiscardForBonusDamage {
                player,
                attacker,
                kind,
                remaining: max,
            };
        }

        Action::DeclineDiscardOwnEnergyForBonusDamage => {
            match state.phase {
                Phase::DecidingToDiscardOwnEnergyForBonusDamage { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DiscardOwnEnergyForBonusDamage { card } => {
            let (player, attacker, kind, remaining) = match state.phase {
                Phase::ChoosingOwnEnergyToDiscardForBonusDamage { player, attacker, kind, remaining } => {
                    (player, attacker, kind, remaining)
                }
                _ => return Err(IllegalAction),
            };
            state.pokemon[attacker.index()].attached.retain(|c| *c != card);
            state.players[player.index()].discard.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} is discarded."));
            if remaining > 1 {
                state.phase = Phase::ChoosingOwnEnergyToDiscardForBonusDamage {
                    player,
                    attacker,
                    kind,
                    remaining: remaining - 1,
                };
            } else {
                state.phase = Phase::Main;
                settle(state);
            }
        }

        Action::FinishDiscardingOwnEnergyForBonusDamage => {
            match state.phase {
                Phase::ChoosingOwnEnergyToDiscardForBonusDamage { .. } => {}
                _ => return Err(IllegalAction),
            };
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DiscardBasicEnergyForDamagePerCard { card } => {
            let (player, attacker, defender, per_card, discarded_so_far) = match state.phase {
                Phase::DiscardingAnyBasicEnergyForDamagePerCard {
                    player,
                    attacker,
                    defender,
                    per_card,
                    discarded_so_far,
                } => (player, attacker, defender, per_card, discarded_so_far),
                _ => return Err(IllegalAction),
            };
            let owner = state
                .player(player)
                .in_play()
                .into_iter()
                .find(|p| state.pokemon(*p).attached.contains(&card))
                .expect("legal_actions offers this only for an own attached Basic Energy");
            state.pokemon[owner.index()].attached.retain(|c| *c != card);
            state.players[player.index()].discard.push(card);
            let name = state.def_of(card).name();
            state.log.push(format!("{name} is discarded."));
            state.phase = Phase::DiscardingAnyBasicEnergyForDamagePerCard {
                player,
                attacker,
                defender,
                per_card,
                discarded_so_far: discarded_so_far + 1,
            };
        }

        Action::FinishDiscardingBasicEnergyForDamagePerCard => {
            let (attacker, defender, per_card, discarded_so_far) = match state.phase {
                Phase::DiscardingAnyBasicEnergyForDamagePerCard {
                    attacker, defender, per_card, discarded_so_far, ..
                } => (attacker, defender, per_card, discarded_so_far),
                _ => return Err(IllegalAction),
            };
            if discarded_so_far > 0 {
                let mut bonus = discarded_so_far * per_card;
                let attacker_type = state.pokemon_def(attacker).kind;
                if state.effective_weakness(defender) == Some(attacker_type) {
                    bonus *= 2;
                }
                if state.pokemon_def(defender).resistance == Some(attacker_type) {
                    bonus = bonus.saturating_sub(30);
                }
                state.pokemon[defender.index()].damage += bonus;
                let name = state.pokemon_def(defender).name;
                state.log.push(format!("{name} takes {bonus} more."));
            }
            state.phase = Phase::Main;
            settle(state);
        }

        Action::DiscardBenchedPokemon { pokemon } => {
            let (player, then) = match state.phase {
                Phase::DiscardingBenchDownTo { player, then } => (player, then),
                _ => return Err(IllegalAction),
            };
            discard_benched_pokemon(state, pokemon);
            if !open_discard_bench_down_to(state, player, then) {
                state.phase = Phase::Main;
                settle(state);
            }
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
        let puts_back = slots.iter().any(|s| s.to == Destination::Zone(Zone::Deck));
        // A search that places its cards on top in order already shuffled
        // the rest of the deck when that slot opened, and placed its cards
        // afterward. Shuffling again here would scramble them right back in.
        let already_ordered_on_top = slots
            .iter()
            .any(|s| s.to == Destination::TopOfDeckInOrder);
        // A search that discards its own peeked leftovers does so before
        // the deck below the window shuffles — after, the leftovers are
        // no longer findable at the top to discard.
        if matches!(then, Some(crate::card::Then::DiscardRestOfPeek)) {
            // Each card already taken left the peeked window through the
            // top of the deck, so what is left to discard is the window
            // shrunk by however many that was — not the window's full
            // original size, which would reach past it into cards the
            // peek never showed.
            let peek_size = slots
                .first()
                .and_then(|s| s.peek)
                .unwrap_or(0)
                .saturating_sub(moved) as usize;
            let deck = &mut state.players[chooser.index()].deck;
            let seen = peek_size.min(deck.len());
            let start = deck.len() - seen;
            let rest: Vec<CardId> = deck.drain(start..).collect();
            state.players[chooser.index()].discard.extend(rest);
        }
        if (from == Zone::Deck || puts_back) && !already_ordered_on_top {
            let deck = &mut state.players[chooser.index()].deck;
            shuffle(state.rng.as_mut(), deck);
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
            Some(crate::card::Then::EndTurnIfMoved)
            | Some(crate::card::Then::DiscardRestOfPeek)
            | None => {}
        }
        state.phase = Phase::Main;
        settle(state);
        return;
    };

    // "Shuffle your deck, then put those cards on top of it": the shuffle
    // happens before anything is placed, on whatever the search has not
    // yet taken. Cards taken during this slot are pushed onto the end of
    // the Deck one at a time, undisturbed by any shuffle after this one.
    if slot.to == Destination::TopOfDeckInOrder {
        let deck = &mut state.players[chooser.index()].deck;
        shuffle(state.rng.as_mut(), deck);
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

        TrainerEffect::SwitchOwnNamePrefixThenOpponent(prefix) => {
            state.phase = Phase::PromotingOwnNamePrefixThenOpponent { player, prefix };
        }

        TrainerEffect::Draw(count) => {
            for _ in 0..count {
                state.draw(player);
            }
        }

        TrainerEffect::DrawUpToHandSize(target) => {
            while state.player(player).hand.len() < target as usize {
                if !state.draw(player) {
                    break;
                }
            }
        }

        TrainerEffect::DrawUpToHandSizeOrMoreIfAllOwnNamePrefix { base, bonus, prefix } => {
            let all_match = state
                .player(player)
                .in_play()
                .iter()
                .all(|id| state.pokemon_def(*id).name.starts_with(prefix));
            let target = if all_match { bonus } else { base };
            while state.player(player).hand.len() < target as usize {
                if !state.draw(player) {
                    break;
                }
            }
        }

        TrainerEffect::CoinFlipDraw { heads, tails } => {
            let count = if state.flip_for(player) { heads } else { tails };
            for _ in 0..count {
                state.draw(player);
            }
        }

        TrainerEffect::CoinFlipThen(inner) => {
            if state.flip_for(player) {
                resolve_trainer(state, player, card, *inner);
            }
        }

        TrainerEffect::DrawThenBonusIfOpponentPrizesAtMost {
            base,
            bonus,
            at_most,
        } => {
            for _ in 0..base {
                state.draw(player);
            }
            if state.player(player.opponent()).prizes.len() <= at_most {
                for _ in 0..bonus {
                    state.draw(player);
                }
            }
        }

        TrainerEffect::DrawThenBonusIfHandAtLeast {
            base,
            bonus,
            at_least,
        } => {
            for _ in 0..base {
                state.draw(player);
            }
            if state.player(player).hand.len() >= at_least {
                for _ in 0..bonus {
                    state.draw(player);
                }
            }
        }

        TrainerEffect::DrawPerPokemonInOpponentHand => {
            let count = state
                .player(player.opponent())
                .hand
                .iter()
                .filter(|c| state.def_of(**c).as_pokemon().is_some())
                .count();
            let opponent = player.opponent();
            state
                .log
                .push(format!("{opponent:?} reveals their hand."));
            for _ in 0..count {
                state.draw(player);
            }
        }

        TrainerEffect::DrawPerOwnPokemonWithMarker(marker) => {
            let count = state
                .player(player)
                .in_play()
                .iter()
                .filter(|p| state.pokemon_def(**p).markers.contains(&marker))
                .count();
            for _ in 0..count {
                state.draw(player);
            }
        }

        TrainerEffect::AttachBasicEnergyFromDiscardToEachFuture => {
            let futures: Vec<PokemonId> = state
                .player(player)
                .in_play()
                .into_iter()
                .filter(|p| {
                    state
                        .pokemon_def(*p)
                        .markers
                        .contains(&crate::card::Marker::Future)
                })
                .collect();
            for target in futures {
                let energy = state
                    .player(player)
                    .discard
                    .iter()
                    .copied()
                    .find(|c| {
                        state
                            .def_of(*c)
                            .as_energy()
                            .is_some_and(|e| e.effect.is_none())
                    });
                if let Some(energy) = energy {
                    state.players[player.index()].discard.retain(|c| *c != energy);
                    state.pokemon[target.index()].attached.push(energy);
                }
            }
        }

        TrainerEffect::DrawPerOpponentMegaEx => {
            let count = state
                .player(player.opponent())
                .in_play()
                .iter()
                .filter(|p| {
                    state
                        .pokemon_def(**p)
                        .markers
                        .contains(&crate::card::Marker::Mega)
                })
                .count();
            for _ in 0..count {
                state.draw(player);
            }
        }

        TrainerEffect::DiscardHandThenDraw(count) => {
            let hand = std::mem::take(&mut state.players[player.index()].hand);
            state.players[player.index()].discard.extend(hand);
            for _ in 0..count {
                state.draw(player);
            }
        }

        TrainerEffect::DiscardTopOfDeck(count) => {
            for _ in 0..count {
                let Some(top) = state.players[player.index()].deck.pop() else {
                    break;
                };
                state.players[player.index()].discard.push(top);
            }
        }

        TrainerEffect::GrantSideShieldNextTurn(shield) => {
            state.side_shield_next_turn = Some((player, shield));
        }

        TrainerEffect::InflictOnOpponentActive(first, second) => {
            if let Some(active) = state.player(player.opponent()).active {
                state.inflict(active, first);
                if let Some(second) = second {
                    state.inflict(active, second);
                }
            }
        }

        TrainerEffect::ConfuseBothActivesExceptType(spared) => {
            for side in [player, player.opponent()] {
                if let Some(active) = state.player(side).active
                    && state.pokemon_def(active).kind != spared
                {
                    state.inflict(active, Condition::Confused);
                }
            }
        }

        TrainerEffect::SwitchOutOpponentActive => {
            state.phase = Phase::Promoting {
                of: player.opponent(),
                chooser: player.opponent(),
                then: None,
            };
        }

        TrainerEffect::ShuffleHandThenCoinFlipDraw { heads, tails } => {
            shuffle_hand_into_deck(state, player);
            let count = if state.flip_for(player) { heads } else { tails };
            for _ in 0..count {
                state.draw(player);
            }
        }

        TrainerEffect::BothShuffleHandThenCoinFlipDraw {
            you_heads,
            opponent_heads,
            you_tails,
            opponent_tails,
        } => {
            shuffle_hand_into_deck(state, player);
            shuffle_hand_into_deck(state, player.opponent());
            let heads = state.flip_for(player);
            let (mine, theirs) = if heads {
                (you_heads, opponent_heads)
            } else {
                (you_tails, opponent_tails)
            };
            for _ in 0..mine {
                state.draw(player);
            }
            for _ in 0..theirs {
                state.draw(player.opponent());
            }
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
            shuffle_hand_into_deck(state, player);
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
            let deck = &mut state.players[opponent.index()].deck;
            // A draw takes from the end, so the bottom of the deck is the
            // front of this list.
            for card in hand.into_iter().rev() {
                deck.insert(0, card);
            }
            for _ in 0..count {
                state.draw(opponent);
            }
        }

        TrainerEffect::BothShuffleHandThenDraw { you, opponent } => {
            for (whose, count) in [(player, you), (player.opponent(), opponent)] {
                shuffle_hand_into_deck(state, whose);
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
            state.phase = Phase::HealingChosen {
                player,
                amount,
                clear_conditions: true,
                of_type: None,
            };
        }

        TrainerEffect::HealFullyIfRemainingHpAtMost(at_most) => {
            state.phase = Phase::HealingChosenIfRemainingHpAtMost { player, at_most };
        }

        TrainerEffect::HealActiveAndClearConditions(amount) => {
            if let Some(active) = state.player(player).active {
                state.pokemon[active.index()].damage =
                    state.pokemon(active).damage.saturating_sub(amount);
                state.clear_conditions(active);
            }
        }

        TrainerEffect::HealChosenPlain { amount, of_type } => {
            state.phase = Phase::HealingChosen {
                player,
                amount,
                clear_conditions: false,
                of_type,
            };
        }

        TrainerEffect::HealEachYours { amount, of_type } => {
            for id in state.player(player).in_play() {
                if of_type.is_none_or(|t| state.pokemon_def(id).kind == t) {
                    state.pokemon[id.index()].damage =
                        state.pokemon(id).damage.saturating_sub(amount);
                }
            }
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

        TrainerEffect::LookAtBottomOfDeck { count } => {
            state.phase = Phase::LookingAtBottomOfDeck { player, count };
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
        | TrainerEffect::ReducesDamageFromType { .. }
        | TrainerEffect::IncreasesHpForNamePrefix { .. }
        | TrainerEffect::RaisesBothActiveRetreatWhileCarrierActive(_)
        | TrainerEffect::BonusDamageVsActiveEx(_)
        | TrainerEffect::ReducesDamageFromAbilityHolders(_)
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
        | TrainerEffect::GrassCanEvolveTheTurnItIsPlayed
        | TrainerEffect::TeraAttacksCostMore
        | TrainerEffect::TeraPokemonRaisesBenchLimit
        | TrainerEffect::PreventsDamageCountersOnBench
        | TrainerEffect::StadiumBoostsBasicHp(_)
        | TrainerEffect::StadiumExtraPoisonDamage(_)
        | TrainerEffect::StadiumReducesDamageToType { .. }
        | TrainerEffect::StadiumReducesDamageForNamePrefix { .. }
        | TrainerEffect::AbilitiesDisabled => {}

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

        TrainerEffect::MayDiscardUpToTwoToolsAnywhere => {
            state.phase = Phase::DiscardingToolsAnywhere { player, remaining: 2 };
        }

        TrainerEffect::GrantsBonusPrizeIfOwnTeraAttackerKnocksOutThisTurn => {
            state.bonus_prize_if_own_tera_attacker_knocks_out = Some(player);
        }

        TrainerEffect::CoinFlipDiscardOpponentEnergy => {
            // The flip is the effect, so it happens with no target in
            // play — the card is still legal to play. Heads only opens
            // the discard where there is an Energy to discard.
            if state.flip_for(player) && state.has_energy_in_play(player.opponent()) {
                state.phase = Phase::DiscardingOpponentEnergy {
                    chooser: player,
                    of: player.opponent(),
                };
            }
        }

        TrainerEffect::DiscardOpponentEnergy => {
            if state.has_energy_in_play(player.opponent()) {
                state.phase = Phase::DiscardingOpponentEnergy {
                    chooser: player,
                    of: player.opponent(),
                };
            }
        }

        TrainerEffect::DiscardOpponentSpecialEnergy => {
            let opponent = player.opponent();
            let any_special = state.player(opponent).in_play().iter().any(|p| {
                state
                    .pokemon(*p)
                    .attached
                    .iter()
                    .any(|c| state.def_of(*c).as_energy().is_some_and(|e| e.effect.is_some()))
            });
            if any_special {
                state.phase = Phase::DiscardingOpponentSpecialEnergy { chooser: player, of: opponent };
            }
        }
    }
}

/// Shuffle a player's hand into their Deck. Several Supporters start this
/// way before drawing a fresh hand.
fn shuffle_hand_into_deck(state: &mut GameState, player: PlayerId) {
    let hand = std::mem::take(&mut state.players[player.index()].hand);
    state.players[player.index()].deck.extend(hand);
    let deck = &mut state.players[player.index()].deck;
    shuffle(state.rng.as_mut(), deck);
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
    let deck = &mut state.players[player.index()].deck;
    shuffle(state.rng.as_mut(), deck);
    state.log.push(format!("{player:?} shuffles their deck."));
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
    state.promoted_from_bench_this_turn[player.index()] = Some(to);
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

    // A coin-flipped invulnerability granted last turn: every effect of
    // this attack, not only its damage, is prevented outright.
    if matches!(
        state.opponent_next_turn_restriction,
        Some((target, crate::card::AttackEffect::CoinFlipSelfInvulnerableNextTurn, _))
            if target == defender
    ) {
        let name = state.pokemon_def(defender).name;
        state.log.push(format!("{name} is invulnerable this turn."));
        return;
    }

    // Rule 30: Confusion flips before the attack happens. Rule 52: on tails
    // the attack does not happen and 3 damage counters go on your own Pokémon.
    let attacker_side = state.pokemon(attacker).owner;
    if state.has_condition(attacker, Condition::Confused) && !state.flip_for(attacker_side) {
        state.pokemon[attacker.index()].damage += 30;
        let name = state.pokemon_def(attacker).name;
        state
            .log
            .push(format!("{name} is Confused and hurts itself."));
        return;
    }

    let attack = state.pokemon_def(attacker).attacks[index].clone();
    attack_with(state, attacker, defender, attack);
}

/// The dispatch every attack's own `Attack` value runs through, once
/// it is known — the attacker's own printed attack, ordinarily, or a
/// Benched or discarded Pokémon's attack `Night Joker` or `Seek
/// Inspiration` copied. Every call site shares this, including the
/// actions that resolve those two copies themselves
/// (`Action::CopyBenchedPokemonAttack`, `Action::CopyDiscardedPokemonAttack`),
/// so a copied attack that is itself one of these two copying effects
/// — copying a copy — opens its own choice correctly rather than
/// reaching the dispatch below with nothing to read.
fn attack_with(state: &mut GameState, attacker: PokemonId, defender: PokemonId, attack: crate::card::Attack) {
    let player = state.pokemon(attacker).owner;
    // `Night Joker` names no damage or effect of its own: it borrows
    // a Benched Pokémon's own attack outright, so this opens a choice
    // instead of continuing through the ordinary dispatch below — this
    // function runs again once that choice names a real `Attack` to
    // read `.effect` and `.base_damage` from. `N's Zoroark ex`.
    if let Some(crate::card::AttackEffect::CopiesChosenBenchedPokemonAttackByNamePrefix(prefix)) =
        attack.effect
    {
        let any_candidate = state
            .player(player)
            .bench
            .iter()
            .any(|p| state.pokemon_def(*p).name.starts_with(prefix));
        if any_candidate {
            state.phase = Phase::ChoosingBenchedPokemonAttackToCopy { player, prefix };
        }
        return;
    }
    // `Seek Inspiration` discards the top of the deck outright,
    // then — only if that card turns out to be a Pokémon without a
    // Rule Box — copies one of its own attacks, the same
    // choose-and-run-this-function-again shape `Night Joker` already
    // takes, just found by discarding rather than a player's own
    // choice among the Bench. `Slowking`.
    if matches!(
        attack.effect,
        Some(crate::card::AttackEffect::DiscardsTopOfDeckThenCopiesItsAttackIfNoRuleBox)
    ) {
        let Some(top) = state.players[player.index()].deck.pop() else {
            return;
        };
        state.players[player.index()].discard.push(top);
        let card_name = state.def_of(top).name();
        state.log.push(format!("{player:?} discards {card_name} for Seek Inspiration."));
        let copies = state
            .def_of(top)
            .as_pokemon()
            .is_some_and(|p| p.prizes == 1 && !p.attacks.is_empty());
        if copies {
            state.phase = Phase::ChoosingDiscardedPokemonAttackToCopy { player, card: top };
        }
        return;
    }
    if matches!(attack.effect, Some(crate::card::AttackEffect::FizzlesWithNoStadiumInPlay))
        && state.stadium.is_none()
    {
        let name = state.pokemon_def(attacker).name;
        state.log.push(format!("{name}'s {} does nothing: no Stadium in play.", attack.name));
        return;
    }
    if matches!(attack.effect, Some(crate::card::AttackEffect::DiscardsDefendersTools)) {
        let opponent = state.pokemon(defender).owner;
        let tools: Vec<CardId> = state
            .pokemon(defender)
            .attached
            .iter()
            .copied()
            .filter(|c| state.def_of(*c).as_trainer().is_some_and(|t| t.kind == TrainerKind::Tool))
            .collect();
        for tool in tools {
            state.pokemon[defender.index()].attached.retain(|c| *c != tool);
            state.players[opponent.index()].discard.push(tool);
        }
    }
    let flipper = state.pokemon(attacker).owner;
    let base = match attack.effect {
        Some(crate::card::AttackEffect::DamagePerCount(count, per_unit)) => {
            count_for_attack(state, attacker, defender, count) * per_unit
        }
        Some(crate::card::AttackEffect::DamagePerCoinFlipHeads { flips, per_head }) => {
            let heads = (0..flips).filter(|_| state.flip_for(flipper)).count() as u32;
            heads * per_head
        }
        Some(crate::card::AttackEffect::DamagePerCoinFlipUntilTails(per_head)) => {
            let mut heads = 0;
            while state.flip_for(flipper) {
                heads += 1;
            }
            attack.base_damage + heads * per_head
        }
        Some(crate::card::AttackEffect::CoinFlipBonusDamage(bonus)) => {
            if state.flip_for(flipper) {
                attack.base_damage + bonus
            } else {
                attack.base_damage
            }
        }
        Some(crate::card::AttackEffect::BonusDamageIfOwnDamaged(bonus)) => {
            if state.pokemon(attacker).damage > 0 {
                attack.base_damage + bonus
            } else {
                attack.base_damage
            }
        }
        Some(crate::card::AttackEffect::BonusDamageIfOwnDamageCountersAtLeastIgnoringWeakness(
            threshold,
            bonus,
        )) => {
            if state.pokemon(attacker).damage / 10 >= threshold {
                attack.base_damage + bonus
            } else {
                attack.base_damage
            }
        }
        Some(crate::card::AttackEffect::BonusDamageIfDefenderIsEx(bonus)) => {
            if state.pokemon_def(defender).prizes > 1 {
                attack.base_damage + bonus
            } else {
                attack.base_damage
            }
        }
        Some(crate::card::AttackEffect::BonusDamageIfDefenderIsStage(stage, bonus)) => {
            if state.pokemon_def(defender).stage == stage {
                attack.base_damage + bonus
            } else {
                attack.base_damage
            }
        }
        Some(crate::card::AttackEffect::BonusDamageIfOwnDeckAtMost(threshold, bonus)) => {
            let owner = state.pokemon(attacker).owner;
            if state.player(owner).deck.len() <= threshold {
                attack.base_damage + bonus
            } else {
                attack.base_damage
            }
        }
        Some(crate::card::AttackEffect::BonusDamagePerCount(count, per_unit)) => {
            attack.base_damage + count_for_attack(state, attacker, defender, count) * per_unit
        }
        Some(crate::card::AttackEffect::BonusDamageIfOwnKnockedOutLastTurn(bonus)) => {
            let owner = state.pokemon(attacker).owner;
            if state.knocked_out_last_turn[owner.index()] {
                attack.base_damage + bonus
            } else {
                attack.base_damage
            }
        }
        Some(crate::card::AttackEffect::BonusDamageIfOwnBenchDamaged(bonus)) => {
            let owner = state.pokemon(attacker).owner;
            let any_damaged = state.player(owner).bench.iter().any(|p| state.pokemon(*p).damage > 0);
            if any_damaged {
                attack.base_damage + bonus
            } else {
                attack.base_damage
            }
        }
        Some(crate::card::AttackEffect::BonusDamageIfSharedTypeInPlay(bonus)) => {
            let owner = state.pokemon(attacker).owner;
            let opponent = owner.opponent();
            let own_types: Vec<_> =
                state.player(owner).in_play().iter().map(|p| state.pokemon_def(*p).kind).collect();
            let shared = state
                .player(opponent)
                .in_play()
                .iter()
                .any(|p| own_types.contains(&state.pokemon_def(*p).kind));
            if shared {
                attack.base_damage + bonus
            } else {
                attack.base_damage
            }
        }
        Some(crate::card::AttackEffect::BonusDamageIfStadiumInPlayThenDiscardsIt(bonus)) => {
            if state.stadium.is_some() {
                attack.base_damage + bonus
            } else {
                attack.base_damage
            }
        }
        Some(crate::card::AttackEffect::BonusDamageIfOwnEnergyOfTypeAttached(kind, bonus)) => {
            let count = count_for_attack(
                state,
                attacker,
                defender,
                crate::card::Count::OwnEnergyOfTypeAttachedCount(kind),
            );
            if count > 0 {
                attack.base_damage + bonus
            } else {
                attack.base_damage
            }
        }
        Some(crate::card::AttackEffect::BonusDamageIfSelfPromotedThisTurn(bonus)) => {
            let owner = state.pokemon(attacker).owner;
            if state.promoted_from_bench_this_turn[owner.index()] == Some(attacker) {
                attack.base_damage + bonus
            } else {
                attack.base_damage
            }
        }
        Some(crate::card::AttackEffect::BonusDamageIfExtraEnergyAttached(threshold, bonus)) => {
            let extra = (state.energy_attached(attacker) as usize).saturating_sub(attack.cost.len());
            if extra >= threshold as usize {
                attack.base_damage + bonus
            } else {
                attack.base_damage
            }
        }
        _ => attack.base_damage,
    };
    let ignore_defenders_effects =
        matches!(attack.effect, Some(crate::card::AttackEffect::IgnoresDefendersEffects));
    let ignore_weakness = matches!(
        attack.effect,
        Some(crate::card::AttackEffect::BonusDamageIfOwnDamageCountersAtLeastIgnoringWeakness(..))
    );
    let damage =
        damage_dealt_with(state, attacker, defender, base, ignore_defenders_effects, ignore_weakness);

    // The summary line reports every point the defender took from this
    // attack, its plain damage plus any damage counters the effect places
    // on it — `Powerful Hand` deals 0 plain damage and lands all of it as
    // counters, so "for 0" would read as a whiff.
    let defender_damage_before = state.pokemon[defender.index()].damage;
    state.pokemon[defender.index()].damage += damage;
    // `Lillie's Pearl` tells this knockout apart from one a checkup
    // causes; `settle`'s knockout check consumes this, every call.
    state.attacking_defender = Some(defender);
    if damage > 0 {
        trigger_defenders_tool(state, attacker, defender);
        // `Shellnado Spin`'s own granted counter, "even if Knocked
        // Out" — read before `settle` decides that, the same as
        // `trigger_defenders_tool`.
        if let Some((
            target,
            crate::card::AttackEffect::GrantsSelfCountersAttackerIfDamagedNextTurn(amount),
            _,
        )) = state.opponent_next_turn_restriction
            && target == defender
        {
            state.pokemon[attacker.index()].damage += amount;
            let name = state.pokemon_def(defender).name;
            state.log.push(format!("{name} counters its attacker for {amount}."));
        }
        if let Some(crate::card::EnergyEffect::CountersAttackerOnDamageTakenWhileActive(amount)) =
            state.pokemon(defender).attached.iter().find_map(|c| {
                state.def_of(*c).as_energy().and_then(|e| e.effect)
            })
        {
            state.pokemon[attacker.index()].damage += amount;
            let attacker_name = state.pokemon_def(attacker).name;
            state.log.push(format!("{attacker_name} takes {amount} back."));
        }
    }
    if let Some(condition) = attack.inflicts
        && !state.attack_effects_on_it_prevented(defender)
    {
        state.inflict(defender, condition);
        let name = state.pokemon_def(defender).name;
        state.log.push(format!("{name} is now {condition:?}."));
    }
    // Named before `resolve_attack_effect` runs: `ReturnSelfAndAttachedToHand`
    // empties the attacker's own card stack, and `pokemon_def` reads its
    // last card.
    let attacker_name = state.pokemon_def(attacker).name;
    let defender_name = state.pokemon_def(defender).name;
    if let Some(effect) = attack.effect {
        resolve_attack_effect(state, attacker, defender, effect, attack.name);
    }

    let dealt_to_defender = state.pokemon[defender.index()]
        .damage
        .saturating_sub(defender_damage_before);
    state.log.push(format!(
        "{attacker_name} uses {} on {defender_name} for {dealt_to_defender}.",
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
    attack_name: &'static str,
) {
    let flipper = state.pokemon(attacker).owner;
    match effect {
        crate::card::AttackEffect::Recoil(amount) => {
            state.pokemon[attacker.index()].damage += amount;
            let name = state.pokemon_def(attacker).name;
            state.log.push(format!("{name} also takes {amount} damage."));
        }
        // Already spent, before `damage_dealt` ran — see `attack`'s own
        // `base` computation.
        crate::card::AttackEffect::DamagePerCount(..) => {}
        crate::card::AttackEffect::DamagePerCoinFlipHeads { .. } => {}
        crate::card::AttackEffect::DamagePerCoinFlipUntilTails(_) => {}
        // Already spent, before `damage_dealt_with` ran.
        crate::card::AttackEffect::IgnoresDefendersEffects => {}
        crate::card::AttackEffect::InflictsCondition(condition) => {
            if !state.attack_effects_on_it_prevented(defender) {
                state.inflict(defender, condition);
                let name = state.pokemon_def(defender).name;
                state.log.push(format!("{name} is now {condition:?}."));
            }
        }
        crate::card::AttackEffect::InflictsConditionAndDefenderCannotRetreatNextTurn(condition) => {
            if !state.attack_effects_on_it_prevented(defender) {
                state.inflict(defender, condition);
                let name = state.pokemon_def(defender).name;
                state.log.push(format!("{name} is now {condition:?} and cannot retreat next turn."));
                state.opponent_next_turn_restriction = Some((defender, effect, state.current));
            }
        }
        crate::card::AttackEffect::InflictsConditionOnSelf(condition) => {
            state.inflict(attacker, condition);
            let name = state.pokemon_def(attacker).name;
            state.log.push(format!("{name} is now {condition:?}."));
        }
        crate::card::AttackEffect::KnocksOutBothActivePokemon => {
            let attacker_hp = state.effective_hp(attacker);
            let defender_hp = state.effective_hp(defender);
            state.pokemon[attacker.index()].damage = attacker_hp;
            state.pokemon[defender.index()].damage = defender_hp;
            state.attacking_defender = Some(defender);
        }
        crate::card::AttackEffect::CoinFlipInflicts(condition) => {
            if state.flip_for(flipper) && !state.attack_effects_on_it_prevented(defender) {
                state.inflict(defender, condition);
                let name = state.pokemon_def(defender).name;
                state.log.push(format!("{name} is now {condition:?}."));
            }
        }
        crate::card::AttackEffect::CoinFlipInflictsAndDiscardsDefenderEnergy(condition) => {
            if state.flip_for(flipper) && !state.attack_effects_on_it_prevented(defender) {
                state.inflict(defender, condition);
                let name = state.pokemon_def(defender).name;
                state.log.push(format!("{name} is now {condition:?}."));
                let owner = state.pokemon(attacker).owner;
                if !state.pokemon(defender).attached.is_empty() {
                    state.phase = Phase::DiscardingDefenderEnergyForAttack {
                        chooser: owner,
                        target: defender,
                    };
                }
            }
        }
        crate::card::AttackEffect::CoinFlipDiscardsDefenderEnergy => {
            if state.flip_for(flipper) && !state.pokemon(defender).attached.is_empty() {
                let owner = state.pokemon(attacker).owner;
                state.phase =
                    Phase::DiscardingDefenderEnergyForAttack { chooser: owner, target: defender };
            }
        }
        crate::card::AttackEffect::DiscardsFixedOwnEnergyChosen(count) => {
            let owner = state.pokemon(attacker).owner;
            state.phase = Phase::ChoosingOwnEnergyToDiscardForAttack {
                player: owner,
                attacker,
                remaining: count,
            };
        }
        crate::card::AttackEffect::MayDiscardUpToOwnEnergyOfTypeForFlatBonusDamage(kind, max, bonus) => {
            let owner = state.pokemon(attacker).owner;
            state.phase = Phase::DecidingToDiscardOwnEnergyForBonusDamage {
                player: owner,
                attacker,
                defender,
                kind,
                max,
                bonus,
            };
        }
        crate::card::AttackEffect::MayDiscardAnyOwnBasicEnergyForDamagePerCard(per_card) => {
            let owner = state.pokemon(attacker).owner;
            state.phase = Phase::DiscardingAnyBasicEnergyForDamagePerCard {
                player: owner,
                attacker,
                defender,
                per_card,
                discarded_so_far: 0,
            };
        }
        // Already spent, before `damage_dealt_with` ran — see `attack`'s
        // own `base` computation.
        crate::card::AttackEffect::CoinFlipBonusDamage(_) => {}
        crate::card::AttackEffect::DefenderCannotRetreatNextTurn => {
            if !state.attack_effects_on_it_prevented(defender) {
                state.opponent_next_turn_restriction = Some((defender, effect, state.current));
                let name = state.pokemon_def(defender).name;
                state.log.push(format!("{name} cannot retreat next turn."));
            }
        }
        crate::card::AttackEffect::DefenderCannotRetreatAndTakesMoreDamageNextTurn(amount) => {
            if !state.attack_effects_on_it_prevented(defender) {
                state.opponent_next_turn_restriction = Some((defender, effect, state.current));
                let granting_player = state.pokemon(attacker).owner;
                state.bonus_damage_to_pokemon_on_granting_players_next_turn =
                    Some((defender, amount, granting_player, false));
                let name = state.pokemon_def(defender).name;
                state.log.push(format!("{name} cannot retreat, and takes more damage, next turn."));
            }
        }
        crate::card::AttackEffect::GrantsSelfCountersAttackerIfDamagedNextTurn(_) => {
            state.opponent_next_turn_restriction = Some((attacker, effect, state.current));
            let name = state.pokemon_def(attacker).name;
            state.log.push(format!("{name} counters its attacker next turn if damaged."));
        }
        crate::card::AttackEffect::MayShuffleFixedEnergyThenDamageChosenBenched { count, damage } => {
            let owner = state.pokemon(attacker).owner;
            let energy_count = state
                .pokemon(attacker)
                .attached
                .iter()
                .filter(|c| state.def_of(**c).is_energy())
                .count() as u32;
            if energy_count >= count {
                state.phase = Phase::DecidingToShuffleEnergyForBenchDamage {
                    player: owner,
                    attacker,
                    count,
                    damage,
                };
            }
        }
        crate::card::AttackEffect::MoveOpponentsEnergyBetweenTheirPokemon => {
            let owner = state.pokemon(attacker).owner;
            let opponent = owner.opponent();
            let in_play = state.player(opponent).in_play();
            let any_move = in_play.iter().any(|from| {
                !state.bench_attack_effect_blocked(owner, *from)
                    && state.pokemon(*from).attached.iter().any(|c| state.def_of(*c).is_energy())
                    && in_play.iter().any(|target| {
                        target != from && !state.bench_attack_effect_blocked(owner, *target)
                    })
            });
            if any_move {
                state.phase = Phase::MovingOpponentsEnergy { chooser: owner, of: opponent };
            }
        }
        crate::card::AttackEffect::OpponentCannotPlayItemsNextTurn => {
            state.opponent_next_turn_restriction = Some((defender, effect, state.current));
            let owner = state.pokemon(defender).owner;
            state.log.push(format!("{owner:?} cannot play Item cards next turn."));
        }
        crate::card::AttackEffect::DiscardsOpponentsStadiumThenOpponentCannotPlayStadiumsNextTurn => {
            let opponent = state.pokemon(defender).owner;
            if state.stadium.is_some_and(|(owner, _)| owner == opponent) {
                let (owner, card) = state.stadium.take().unwrap();
                state.players[owner.index()].discard.push(card);
                let name = state.def_of(card).name();
                state.log.push(format!("{name} is discarded."));
                state.opponent_next_turn_restriction = Some((defender, effect, state.current));
                state.log.push(format!("{opponent:?} cannot play Stadium cards next turn."));
            }
        }
        crate::card::AttackEffect::BonusDamageIfStadiumInPlayThenDiscardsIt(_) => {
            if let Some((owner, card)) = state.stadium.take() {
                state.players[owner.index()].discard.push(card);
                let name = state.def_of(card).name();
                state.log.push(format!("{name} is discarded."));
            }
        }
        crate::card::AttackEffect::CoinFlipSelfInvulnerableNextTurn => {
            if state.flip_for(flipper) {
                state.opponent_next_turn_restriction = Some((attacker, effect, state.current));
                let name = state.pokemon_def(attacker).name;
                state.log.push(format!("{name} is invulnerable next turn."));
            }
        }
        crate::card::AttackEffect::DefenderDealsLessDamageNextTurn(amount) => {
            if !state.attack_effects_on_it_prevented(defender) {
                state.opponent_next_turn_restriction = Some((defender, effect, state.current));
                let name = state.pokemon_def(defender).name;
                state
                    .log
                    .push(format!("{name} deals {amount} less damage next turn."));
            }
        }
        crate::card::AttackEffect::AttackerCannotAttackNextTurn => {
            state.own_next_turn_restriction = Some((attacker, effect, false));
            let name = state.pokemon_def(attacker).name;
            state.log.push(format!("{name} cannot attack next turn."));
        }
        crate::card::AttackEffect::CannotUseThisAttackNextTurn => {
            state.locked_attack_next_turn = Some((attacker, attack_name, false));
            let name = state.pokemon_def(attacker).name;
            state.log.push(format!("{name} cannot use {attack_name} next turn."));
        }
        crate::card::AttackEffect::DamageCountersToOpponentBenchAnyWay(count) => {
            let owner = state.pokemon(attacker).owner;
            let any_target = state.player(owner.opponent()).bench.iter().any(|p| {
                !state.bench_damage_counters_blocked(owner, *p)
                    && !state.bench_attack_damage_blocked(owner, *p)
                    && !state.bench_attack_effect_blocked(owner, *p)
            });
            if any_target {
                state.phase = Phase::DistributingDamageCounters {
                    player: owner,
                    remaining: count,
                };
            }
        }
        crate::card::AttackEffect::DiscardsOwnEnergyThenDamagesChosenBenched(damage) => {
            let owner = state.pokemon(attacker).owner;
            let energy: Vec<CardId> = state
                .pokemon(attacker)
                .attached
                .iter()
                .copied()
                .filter(|c| state.def_of(*c).is_energy())
                .collect();
            let mut to_reattach = Vec::new();
            for card in energy {
                state.pokemon[attacker.index()].attached.retain(|c| *c != card);
                state.players[owner.index()].discard.push(card);
                if state.def_of(card).as_energy().and_then(|e| e.effect)
                    == Some(crate::card::EnergyEffect::ReattachesAfterOwnDiscardByAttackEffect)
                {
                    to_reattach.push(card);
                }
            }
            let name = state.pokemon_def(attacker).name;
            state.log.push(format!("{name} discards all its Energy."));
            for card in to_reattach {
                state.players[owner.index()].discard.retain(|c| *c != card);
                state.pokemon[attacker.index()].attached.push(card);
                let energy_name = state.def_of(card).name();
                state.log.push(format!("{energy_name} returns to {name}."));
            }
            if !state.player(owner.opponent()).bench.is_empty() {
                state.phase = Phase::ChoosingBenchDamageTarget {
                    player: owner,
                    damage,
                };
            }
        }
        crate::card::AttackEffect::SwitchOwnActive => {
            let owner = state.pokemon(attacker).owner;
            if !state.player(owner).bench.is_empty() {
                state.phase = Phase::Promoting {
                    of: owner,
                    chooser: owner,
                    then: None,
                };
            }
        }
        crate::card::AttackEffect::SwitchOpponentActive => {
            let owner = state.pokemon(attacker).owner;
            let opponent = owner.opponent();
            if !state.player(opponent).bench.is_empty() {
                state.phase = Phase::Promoting {
                    of: opponent,
                    chooser: owner,
                    then: None,
                };
            }
        }
        // Already spent, before `damage_dealt_with` ran — see `attack`'s
        // own `base` computation.
        crate::card::AttackEffect::BonusDamageIfOwnDamaged(_) => {}
        crate::card::AttackEffect::BonusDamageIfOwnDamageCountersAtLeastIgnoringWeakness(..) => {}
        // Already spent, before `damage_dealt_with` ran — see `attack`'s
        // own `base` computation.
        crate::card::AttackEffect::BonusDamageIfDefenderIsEx(_) => {}
        // Already spent, before `damage_dealt_with` ran — see `attack`'s
        // own `base` computation.
        crate::card::AttackEffect::BonusDamageIfDefenderIsStage(..) => {}
        // Already spent, before `damage_dealt_with` ran — see `attack`'s
        // own `base` computation.
        crate::card::AttackEffect::BonusDamageIfOwnDeckAtMost(..) => {}
        // Already spent, before `damage_dealt_with` ran — see `attack`'s
        // own `base` computation.
        crate::card::AttackEffect::BonusDamagePerCount(..) => {}
        // Already spent, before `damage_dealt_with` ran — see `attack`'s
        // own `base` computation.
        crate::card::AttackEffect::BonusDamageIfOwnKnockedOutLastTurn(_) => {}
        // Already spent, before `damage_dealt_with` ran — see `attack`'s
        // own `base` computation.
        crate::card::AttackEffect::BonusDamageIfOwnBenchDamaged(_) => {}
        // Already spent, before `damage_dealt_with` ran — see `attack`'s
        // own `base` computation.
        crate::card::AttackEffect::BonusDamageIfSharedTypeInPlay(_) => {}
        // Already spent, before `damage_dealt_with` ran — see `attack`'s
        // own `base` computation.
        crate::card::AttackEffect::BonusDamageIfOwnEnergyOfTypeAttached(..) => {}
        // Already spent, before `damage_dealt_with` ran — see `attack`'s
        // own `base` computation.
        crate::card::AttackEffect::BonusDamageIfSelfPromotedThisTurn(_) => {}
        // Already spent, before `damage_dealt_with` ran — see `attack`'s
        // own `base` computation.
        crate::card::AttackEffect::BonusDamageIfExtraEnergyAttached(..) => {}
        crate::card::AttackEffect::DiscardsTopOfOpponentsDeck(count) => {
            let opponent = state.pokemon(attacker).owner.opponent();
            let taken: Vec<_> = state.player(opponent).deck.iter().take(count as usize).copied().collect();
            for card in &taken {
                state.players[opponent.index()].deck.retain(|c| c != card);
                state.players[opponent.index()].discard.push(*card);
            }
            state.log.push(format!("{} card(s) discarded from the opponent's deck.", taken.len()));
        }
        crate::card::AttackEffect::ShufflesOwnEnergyThenDamagesChosenOpponentPokemonWeaknessIfActive(
            damage,
        ) => {
            let owner = state.pokemon(attacker).owner;
            let energy: Vec<_> =
                state.pokemon(attacker).attached.iter().copied().filter(|c| state.def_of(*c).is_energy()).collect();
            for card in &energy {
                state.pokemon[attacker.index()].attached.retain(|c| c != card);
            }
            state.players[owner.index()].deck.extend(energy);
            shuffle(state.rng.as_mut(), &mut state.players[owner.index()].deck);
            let name = state.pokemon_def(attacker).name;
            state.log.push(format!("{name} shuffles its Energy into the deck."));
            state.phase = Phase::ChoosingAnyOpponentPokemonDamageTargetWeaknessIfActive {
                player: owner,
                attacker,
                damage,
            };
        }
        crate::card::AttackEffect::DamagesEveryPokemonWithAnAbility(amount) => {
            for player in [PlayerId::One, PlayerId::Two] {
                for pokemon in state.player(player).in_play() {
                    if state.pokemon_def(pokemon).ability.is_some() {
                        state.pokemon[pokemon.index()].damage += amount;
                    }
                }
            }
            state.log.push(format!("every Pokemon with an Ability takes {amount}."));
        }
        crate::card::AttackEffect::MovesAllDamageFromChosenOwnBenchedToChosenOpponentPokemon => {
            let owner = state.pokemon(attacker).owner;
            if !state.player(owner).bench.is_empty() {
                state.phase = Phase::ChoosingOwnBenchedSourceForDamageMove { player: owner };
            }
        }
        crate::card::AttackEffect::SearchesBasicEnergyOfTypeAttachToChosenBenched(kind, max) => {
            let owner = state.pokemon(attacker).owner;
            let has_energy = state
                .player(owner)
                .deck
                .iter()
                .any(|c| state.matches_filter(*c, crate::card::CardFilter::BasicEnergyOfType(kind)));
            if !state.player(owner).bench.is_empty() && has_energy {
                state.phase = Phase::ChoosingBenchedTargetForEnergySearch { player: owner, kind, max };
            }
        }
        crate::card::AttackEffect::DrawCards(count) => {
            let owner = state.pokemon(attacker).owner;
            for _ in 0..count {
                state.draw(owner);
            }
            let name = state.pokemon_def(attacker).name;
            state.log.push(format!("{name} draws {count}."));
        }
        crate::card::AttackEffect::DiscardsHandThenDrawsCards(count) => {
            let owner = state.pokemon(attacker).owner;
            let hand = std::mem::take(&mut state.players[owner.index()].hand);
            state.players[owner.index()].discard.extend(hand);
            for _ in 0..count {
                state.draw(owner);
            }
            let name = state.pokemon_def(attacker).name;
            state.log.push(format!("{name} discards its hand and draws {count}."));
        }
        crate::card::AttackEffect::RevealOpponentsHand => {
            let opponent = state.pokemon(attacker).owner.opponent();
            let name = state.pokemon_def(attacker).name;
            state.log.push(format!("{name} reveals {opponent:?}'s hand."));
        }
        crate::card::AttackEffect::MayReturnOpponentsActiveEnergyToHand(count) => {
            let owner = state.pokemon(attacker).owner;
            let opponent = owner.opponent();
            let has_energy = state.player(opponent).active.is_some_and(|active| {
                state.pokemon(active).attached.iter().any(|c| state.def_of(*c).is_energy())
            });
            if has_energy {
                state.phase = Phase::MovingOpponentsActiveEnergyToHand {
                    player: owner,
                    remaining: count,
                };
            }
        }
        crate::card::AttackEffect::SearchDeckToEvolveSelf => {
            let owner = state.pokemon(attacker).owner;
            let from = state.pokemon_def(attacker).name;
            let any_evolution = state
                .player(owner)
                .deck
                .iter()
                .any(|c| state.def_of(*c).as_pokemon().is_some_and(|p| p.evolve_from == Some(from)));
            if any_evolution {
                state.phase = Phase::SearchingDeckToEvolveSelf {
                    player: owner,
                    target: attacker,
                };
            }
        }
        crate::card::AttackEffect::SearchDiscardForNamedToBench(name, count) => {
            let owner = state.pokemon(attacker).owner;
            let any_named = state
                .player(owner)
                .discard
                .iter()
                .any(|c| state.matches_filter(*c, crate::card::CardFilter::PokemonNamed(name)));
            if any_named {
                state.phase = Phase::SearchingDiscardForNamedToBench {
                    player: owner,
                    name,
                    remaining: count,
                };
            }
        }
        crate::card::AttackEffect::TakePokemonFromDiscard => {
            let owner = state.pokemon(attacker).owner;
            let any_pokemon = state
                .player(owner)
                .discard
                .iter()
                .any(|c| state.matches_filter(*c, crate::card::CardFilter::AnyPokemon));
            if any_pokemon {
                state.phase = Phase::TakingPokemonFromDiscard { player: owner };
            }
        }
        crate::card::AttackEffect::DamageChosenOpponentPokemon(damage) => {
            let owner = state.pokemon(attacker).owner;
            state.phase = Phase::ChoosingAnyOpponentPokemonDamageTarget { player: owner, damage };
        }
        crate::card::AttackEffect::DamageTwoChosenOpponentPokemon(damage) => {
            let owner = state.pokemon(attacker).owner;
            state.phase = Phase::ChoosingTwoOpponentPokemonDamageTargets {
                player: owner,
                damage,
                excluding: None,
            };
        }
        crate::card::AttackEffect::DamageChosenOpponentBenchedEx(damage) => {
            let owner = state.pokemon(attacker).owner;
            let any_benched_ex = state
                .player(owner.opponent())
                .bench
                .iter()
                .any(|p| state.pokemon_def(*p).prizes > 1);
            if any_benched_ex {
                state.phase = Phase::ChoosingBenchedExDamageTarget { player: owner, damage };
            }
        }
        crate::card::AttackEffect::PlacesDamageCountersOnChosenOpponentBenched(damage) => {
            let owner = state.pokemon(attacker).owner;
            let any_benched = !state.player(owner.opponent()).bench.is_empty();
            if any_benched {
                state.phase = Phase::ChoosingAnyBenchedDamageTarget { player: owner, damage };
            }
        }
        crate::card::AttackEffect::DiscardsOwnEnergyThenDamagesChosenBenchedEx(damage) => {
            let owner = state.pokemon(attacker).owner;
            let energy: Vec<CardId> = state
                .pokemon(attacker)
                .attached
                .iter()
                .copied()
                .filter(|c| state.def_of(*c).is_energy())
                .collect();
            for card in energy {
                state.pokemon[attacker.index()].attached.retain(|c| *c != card);
                state.players[owner.index()].discard.push(card);
            }
            let name = state.pokemon_def(attacker).name;
            state.log.push(format!("{name} discards all its Energy."));
            let any_benched_ex = state
                .player(owner.opponent())
                .bench
                .iter()
                .any(|p| state.pokemon_def(*p).prizes > 1);
            if any_benched_ex {
                state.phase = Phase::ChoosingBenchedExDamageTarget { player: owner, damage };
            }
        }
        crate::card::AttackEffect::DiscardsOwnEnergyThenDamagesThreeChosenOpponentPokemon(damage) => {
            let owner = state.pokemon(attacker).owner;
            let energy: Vec<CardId> = state
                .pokemon(attacker)
                .attached
                .iter()
                .copied()
                .filter(|c| state.def_of(*c).is_energy())
                .collect();
            for card in energy {
                state.pokemon[attacker.index()].attached.retain(|c| *c != card);
                state.players[owner.index()].discard.push(card);
            }
            let name = state.pokemon_def(attacker).name;
            state.log.push(format!("{name} discards all its Energy."));
            if !state.player(owner.opponent()).in_play().is_empty() {
                state.phase = Phase::ChoosingThreeOpponentPokemonDamageTargets {
                    player: owner,
                    damage,
                    excluding: [None, None],
                };
            }
        }
        crate::card::AttackEffect::SearchEnergyAttachToBenchedOfType(kind) => {
            let owner = state.pokemon(attacker).owner;
            let has_energy =
                state.player(owner).deck.iter().any(|c| state.def_of(*c).is_energy());
            let has_target =
                state.player(owner).bench.iter().any(|p| state.pokemon_def(*p).kind == kind);
            if has_energy && has_target {
                state.phase = Phase::SearchingForEnergyToAttachToBenchedOfType { player: owner, kind };
            }
        }
        crate::card::AttackEffect::DamagePerCountToChosenOpponentPokemon(count, per_unit) => {
            let owner = state.pokemon(attacker).owner;
            let damage = count_for_attack(state, attacker, defender, count) * per_unit;
            state.phase = Phase::ChoosingAnyOpponentPokemonDamageTarget { player: owner, damage };
        }
        crate::card::AttackEffect::PlaceDamageCountersOnDefenderPerCount(count, counters) => {
            let amount = count_for_attack(state, attacker, defender, count) * counters * 10;
            state.pokemon[defender.index()].damage += amount;
            let name = state.pokemon_def(defender).name;
            state.log.push(format!("{name} takes {amount}."));
        }
        // Already checked, at the top of `attack` — this arm is only
        // reached when a Stadium is in play, so there is nothing left
        // to do.
        crate::card::AttackEffect::FizzlesWithNoStadiumInPlay => {}
        // Already done, at the top of `attack` — before damage, not
        // after, and unconditional, so there is nothing left to do.
        crate::card::AttackEffect::DiscardsDefendersTools => {}
        // Already handled outright at the top of `attack` — this
        // effect never reaches `attack_with` (never mind
        // `resolve_attack_effect`), since Night Joker deals no damage
        // and reads no `effect` of its own; the copied `Attack`'s own
        // effect is what runs through this dispatch instead.
        crate::card::AttackEffect::CopiesChosenBenchedPokemonAttackByNamePrefix(_) => {
            unreachable!("attack() opens a choice for this effect and never calls attack_with")
        }
        crate::card::AttackEffect::DiscardsTopOfDeckThenCopiesItsAttackIfNoRuleBox => {
            unreachable!("attack() handles this effect outright and never calls attack_with")
        }
        crate::card::AttackEffect::MoveOwnAttachedEnergyToHand => {
            let owner = state.pokemon(attacker).owner;
            let any_energy = state.pokemon(attacker).attached.iter().any(|c| state.def_of(*c).is_energy());
            if any_energy {
                state.phase = Phase::ChoosingOwnEnergyToHand { player: owner, attacker };
            }
        }
        crate::card::AttackEffect::MoveOwnAttachedEnergyToChosenBenched => {
            let owner = state.pokemon(attacker).owner;
            let any_energy = state.pokemon(attacker).attached.iter().any(|c| state.def_of(*c).is_energy());
            let any_bench = !state.player(owner).bench.is_empty();
            if any_energy && any_bench {
                state.phase = Phase::ChoosingEnergyAndBenchedTargetToMove { player: owner, attacker };
            }
        }
        crate::card::AttackEffect::SelfDamageReductionNextTurn(amount) => {
            state.opponent_next_turn_restriction = Some((attacker, effect, state.current));
            let name = state.pokemon_def(attacker).name;
            state.log.push(format!("{name} takes {amount} less damage next turn."));
        }
        crate::card::AttackEffect::ReturnSelfAndAttachedToHand => {
            let owner = state.pokemon(attacker).owner;
            if state.player(owner).bench.is_empty() {
                return;
            }
            let cards = std::mem::take(&mut state.pokemon[attacker.index()].cards);
            let attached = std::mem::take(&mut state.pokemon[attacker.index()].attached);
            let side = &mut state.players[owner.index()];
            side.hand.extend(cards);
            side.hand.extend(attached);
            side.active = None;
            state.log.push("The attacker returns to hand.".to_string());
            state.phase = Phase::Promoting { of: owner, chooser: owner, then: None };
        }
        crate::card::AttackEffect::TakeTrainerFromDiscard => {
            let owner = state.pokemon(attacker).owner;
            let any_trainer = state
                .player(owner)
                .discard
                .iter()
                .any(|c| state.matches_filter(*c, crate::card::CardFilter::AnyTrainer));
            if any_trainer {
                state.phase = Phase::TakingTrainerFromDiscard { player: owner };
            }
        }
        crate::card::AttackEffect::SearchDeckForBasicPokemonToBench(count) => {
            let owner = state.pokemon(attacker).owner;
            let any_basic = state.player(owner).deck.iter().any(|c| {
                state.matches_filter(*c, crate::card::CardFilter::PokemonOfStage(crate::card::Stage::Basic))
            });
            if any_basic {
                state.phase = Phase::SearchingDeckForBasics {
                    player: owner,
                    remaining: count,
                };
            }
        }
        crate::card::AttackEffect::SearchDeckForItemCardToHand => {
            let owner = state.pokemon(attacker).owner;
            let any_item = state.player(owner).deck.iter().any(|c| {
                state.matches_filter(*c, crate::card::CardFilter::TrainerOfKind(TrainerKind::Item))
            });
            if any_item {
                state.phase = Phase::SearchingDeckForItem { player: owner };
            }
        }
        crate::card::AttackEffect::SearchDeckForUpToCardsOfAnyKindToHand(count) => {
            let owner = state.pokemon(attacker).owner;
            if !state.player(owner).deck.is_empty() {
                state.phase = Phase::SearchingDeckForAnyCards { player: owner, remaining: count };
            }
        }
        crate::card::AttackEffect::SearchDeckForUpToPokemonOfTypeOrStadiumToHand(kind, count) => {
            let owner = state.pokemon(attacker).owner;
            if !state.player(owner).deck.is_empty() {
                state.phase = Phase::SearchingDeckForPokemonOfTypeOrStadium {
                    player: owner,
                    kind,
                    remaining: count,
                };
            }
        }
        crate::card::AttackEffect::KnocksOutDefenderIfExactDamageCounters(counters) => {
            if state.pokemon(defender).damage == counters * 10 {
                let name = state.pokemon_def(defender).name;
                state.log.push(format!("{name} is Knocked Out outright (exactly {counters} counters)."));
                let effective_hp = state.effective_hp(defender);
                state.pokemon[defender.index()].damage =
                    state.pokemon[defender.index()].damage.max(effective_hp);
            }
        }
        crate::card::AttackEffect::DiscardsChosenFromOpponentsHand => {
            let owner = state.pokemon(attacker).owner;
            let opponent = owner.opponent();
            if !state.player(opponent).hand.is_empty() {
                state.phase = Phase::ChoosingCardFromOpponentsHandToDiscard { player: owner };
            }
        }
    }
}

/// `Phase::SearchingDeckForBasics` ends either on its own limit or an
/// early decline — both shuffle the deck, the same as any other
/// search that looked through it.
fn finish_searching_deck_for_basics(state: &mut GameState, player: PlayerId) {
    let deck = &mut state.players[player.index()].deck;
    shuffle(state.rng.as_mut(), deck);
    state.phase = Phase::Main;
    settle(state);
}

/// `Phase::SearchingDeckForBasicsOfType` ends either on its own limit or
/// an early decline — both shuffle the deck, the same as
/// `finish_searching_deck_for_basics` does. `Telepathic Psychic Energy`.
fn finish_searching_deck_for_basics_of_type(state: &mut GameState, player: PlayerId) {
    let deck = &mut state.players[player.index()].deck;
    shuffle(state.rng.as_mut(), deck);
    state.phase = Phase::Main;
    settle(state);
}

/// `Phase::SearchingDeckForAnyCards` ends either on its own limit or an
/// early decline — both shuffle the deck, the same as any other search
/// that looked through it. `Noctowl`'s `Talon Hunt`.
fn finish_searching_deck_for_any_cards(state: &mut GameState, player: PlayerId) {
    let deck = &mut state.players[player.index()].deck;
    shuffle(state.rng.as_mut(), deck);
    state.phase = Phase::Main;
    settle(state);
}

fn finish_searching_deck_for_trainer_cards(state: &mut GameState, player: PlayerId) {
    let deck = &mut state.players[player.index()].deck;
    shuffle(state.rng.as_mut(), deck);
    state.phase = Phase::Main;
    settle(state);
}

fn finish_searching_energy_of_type_to_attach_to_chosen(state: &mut GameState, player: PlayerId) {
    let deck = &mut state.players[player.index()].deck;
    shuffle(state.rng.as_mut(), deck);
    state.phase = Phase::Main;
    settle(state);
}

fn finish_searching_deck_for_pokemon_of_type_or_stadium(state: &mut GameState, player: PlayerId) {
    let deck = &mut state.players[player.index()].deck;
    shuffle(state.rng.as_mut(), deck);
    state.phase = Phase::Main;
    settle(state);
}

/// What `AttackEffect::DamagePerCount` reads for this attack, counted
/// fresh from the board.
fn count_for_attack(
    state: &GameState,
    attacker: PokemonId,
    defender: PokemonId,
    count: crate::card::Count,
) -> u32 {
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
        crate::card::Count::OpponentBenchedPokemonCount => {
            state.player(opponent).bench.len() as u32
        }
        crate::card::Count::EnergyOnBothActivesCount => {
            state.energy_attached(attacker) as u32 + state.energy_attached(defender) as u32
        }
        crate::card::Count::OpponentPrizesTakenCount => {
            6 - state.player(opponent).prizes.len() as u32
        }
        crate::card::Count::OwnEnergyOfTypeAttachedCount(kind) => state
            .pokemon(attacker)
            .attached
            .iter()
            .filter(|c| match state.def_of(**c) {
                crate::card::CardDef::Energy(energy) => energy.kind == kind,
                crate::card::CardDef::Pokemon(_) | crate::card::CardDef::Trainer(_) => false,
            })
            .count() as u32,
        crate::card::Count::OwnHandSizeCount => state.player(owner).hand.len() as u32,
        crate::card::Count::BothBenchedPokemonCount => {
            state.player(owner).bench.len() as u32 + state.player(opponent).bench.len() as u32
        }
        crate::card::Count::DefenderEnergyAttachedCount => state.energy_attached(defender) as u32,
        crate::card::Count::DefenderDamageCounters => state.pokemon(defender).damage / 10,
        crate::card::Count::OwnEnergyOfTypeAttachedAcrossSideCount(kind) => state
            .player(owner)
            .in_play()
            .iter()
            .map(|p| {
                state
                    .pokemon(*p)
                    .attached
                    .iter()
                    .filter(|c| match state.def_of(**c) {
                        crate::card::CardDef::Energy(energy) => energy.kind == kind,
                        crate::card::CardDef::Pokemon(_) | crate::card::CardDef::Trainer(_) => false,
                    })
                    .count() as u32
            })
            .sum(),
        crate::card::Count::OwnBenchedPokemonCount => state.player(owner).bench.len() as u32,
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
    damage_dealt_with(state, attacker, defender, base, false, false)
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
    ignore_weakness: bool,
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
        let condition_met = match target {
            crate::card::TurnBonusTarget::OpponentActiveEx => {
                state.pokemon_def(defender).prizes > 1
            }
            crate::card::TurnBonusTarget::OpponentActiveWithoutRuleBox => {
                state.pokemon_def(defender).prizes == 1
            }
            crate::card::TurnBonusTarget::AttackerIsType(kind) => {
                state.pokemon_def(attacker).kind == kind
            }
        };
        let opponent_active = state
            .player(state.pokemon(attacker).owner.opponent())
            .active;
        if condition_met && opponent_active == Some(defender) {
            damage += bonus;
        }
    }

    // A restriction granted on a previous turn against this Pokémon,
    // read only during the granting player's very next turn — the
    // same lifetime `DefenderCannotRetreatNextTurn` already carries.
    if let Some((target, crate::card::AttackEffect::DefenderDealsLessDamageNextTurn(amount), _)) =
        state.opponent_next_turn_restriction
        && target == attacker
    {
        damage = damage.saturating_sub(amount);
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
            crate::card::TrainerEffect::BonusDamageIfPoisonedVsActive(bonus)
                if state.has_condition(attacker, Condition::Poisoned) =>
            {
                damage += bonus;
            }
            crate::card::TrainerEffect::BonusDamageVsActiveEx(bonus)
                if state.pokemon_def(defender).prizes > 1 =>
            {
                damage += bonus;
            }
            _ => {}
        }
    }

    // Step 32c: `Cobalt Command` — read from the attacker's own side,
    // not only the carrier, and gated on the attacker's own `Future`
    // marker and its own name (a second `Iron Crown ex` on the Bench
    // doesn't boost the one attacking).
    let attacker_owner = state.pokemon(attacker).owner;
    let attacker_name = state.pokemon_def(attacker).name;
    if attacker_name != "Iron Crown ex" && state.pokemon_def(attacker).markers.contains(&crate::card::Marker::Future) {
        let cobalt_command_bonus = state.player(attacker_owner).in_play().iter().find_map(|p| {
            match state.pokemon_def(*p).ability.map(|a| a.effect) {
                Some(crate::card::AbilityEffect::PassiveFutureAttacksDoBonusDamageToActiveExceptNamed(bonus)) => {
                    Some(bonus)
                }
                _ => None,
            }
        });
        if let Some(bonus) = cobalt_command_bonus {
            damage += bonus;
        }
    }

    // Step 32d: `Lose Cool` — self-scoped, unlike `Cobalt Command`:
    // only the carrier's own attacks, only the carrier's own damage.
    if state.pokemon(attacker).damage >= 20
        && let Some(crate::card::AbilityEffect::PassiveBonusDamageToActiveIfSelfDamaged(bonus)) =
            state.pokemon_def(attacker).ability.map(|a| a.effect)
    {
        damage += bonus;
    }

    // Step 32e: a Special Energy on the attacker whose printed rider
    // makes the carrier's attacks hit the opponent's Active harder, but
    // only while the carrier's own type matches the type the card
    // provides (its `kind`). `Voltaic Lightning Energy`.
    let attacker_kind = state.pokemon_def(attacker).kind;
    for card in &state.pokemon(attacker).attached {
        if let Some(energy) = state.def_of(*card).as_energy()
            && let Some(crate::card::EnergyEffect::CarrierAttacksHitOpponentActiveHarder(bonus)) =
                energy.effect
            && energy.kind == attacker_kind
        {
            damage += bonus;
        }
    }

    // Step 33: Weakness, then Resistance. Both read the attacker's type.
    // `AttackEffect::IgnoresDefendersEffects` skips this step outright —
    // "isn't affected by any effects on your opponent's Active Pokémon"
    // reads as including Weakness and Resistance, not only a Tool or
    // Stadium bonus.
    if !ignore_defenders_effects {
        let attacker_type = state.pokemon_def(attacker).kind;
        if !ignore_weakness && state.effective_weakness(defender) == Some(attacker_type) {
            damage *= 2;
        }
        if state.pokemon_def(defender).resistance == Some(attacker_type) {
            damage = damage.saturating_sub(30);
        }
    }

    // Step 33b: a Stadium that softens attacks against one type — read
    // for either side, after Weakness and Resistance. `Full Metal Lab`.
    if let Some(crate::card::TrainerEffect::StadiumReducesDamageToType { kind, amount }) =
        state.stadium_effect()
        && state.pokemon_def(defender).kind == kind
    {
        damage = damage.saturating_sub(amount);
    }
    if let Some(crate::card::TrainerEffect::StadiumReducesDamageForNamePrefix { word, amount }) =
        state.stadium_effect()
        && state.pokemon_def(defender).name.contains(word)
    {
        damage = damage.saturating_sub(amount);
    }

    // A side shield a Supporter granted last turn: read for a defender
    // the granting player owns, while it is the opponent's turn (ADR 0098).
    if let Some((granted_by, shield)) = state.side_shield_next_turn
        && state.pokemon(defender).owner == granted_by
        && state.current != granted_by
    {
        match shield {
            crate::card::SideShield::DamageReduction(amount) => {
                damage = damage.saturating_sub(amount);
            }
            crate::card::SideShield::DamageReductionForType(kind, amount)
                if state.pokemon_def(defender).kind == kind =>
            {
                damage = damage.saturating_sub(amount);
            }
            _ => {}
        }
    }

    // Step 33c: a Tool on the defender that softens attacks from one
    // attacker type — the "-Berry" Tools. After Weakness and Resistance,
    // per the printed text. `Jamming Tower` turns Tool reads off.
    if !state.tools_disabled() {
        let attacker_type = state.pokemon_def(attacker).kind;
        let attacker_has_ability = state.pokemon_def(attacker).ability.is_some();
        for tool in &state.pokemon(defender).attached {
            match state.def_of(*tool).as_trainer().map(|t| &t.effect) {
                Some(&crate::card::TrainerEffect::ReducesDamageFromType { kind, amount })
                    if kind == attacker_type =>
                {
                    damage = damage.saturating_sub(amount);
                }
                Some(&crate::card::TrainerEffect::ReducesDamageFromAbilityHolders(amount))
                    if attacker_has_ability =>
                {
                    damage = damage.saturating_sub(amount);
                }
                _ => {}
            }
        }
    }

    // Step 34: effects on the defending Pokémon. A restriction granted
    // on a previous turn against this Pokémon, read only during the
    // granting player's very next turn — the same lifetime
    // `DefenderCannotRetreatNextTurn` already carries, but read after
    // Weakness and Resistance per the printed text.
    if let Some((target, crate::card::AttackEffect::SelfDamageReductionNextTurn(amount), _)) =
        state.opponent_next_turn_restriction
        && target == defender
    {
        damage = damage.saturating_sub(amount);
    }
    // `Pouncing Trap`'s own bonus half — a separate record from its
    // retreat lock, since "during your next turn" names the granting
    // player's own next turn, one turn later than the retreat lock's
    // "during your opponent's next turn." Read after Weakness and
    // Resistance per the printed text, only while `damage` is still
    // positive (a fizzled attack takes no bonus either, matching Step
    // 32's own 0-damage stop), and only once armed — the turn it was
    // granted must not see it.
    if damage > 0
        && let Some((target, amount, granting_player, true)) =
            state.bonus_damage_to_pokemon_on_granting_players_next_turn
        && target == defender
        && state.current == granting_player
    {
        damage += amount;
    }
    if !ignore_defenders_effects
        && !state.abilities_disabled_for(defender)
        && state.pokemon_def(attacker).prizes > 1
        && state
            .pokemon_def(defender)
            .ability
            .is_some_and(|a| a.effect == crate::card::AbilityEffect::PassiveImmuneToDamageFromOpponentEx)
    {
        damage = 0;
    }

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

        // `Area Zero Underdepths`: the moment a player's last Tera
        // Pokémon leaves play, an oversized Bench (raised past
        // `BENCH_LIMIT` while they still had one) must shrink back
        // down. Read every pass through this loop, the same general
        // sweep `knock_out_the_dead` already is, since a Tera
        // Pokémon can leave play by more than one route.
        if state.stadium_effect() == Some(crate::card::TrainerEffect::TeraPokemonRaisesBenchLimit) {
            for player in [PlayerId::One, PlayerId::Two] {
                if state.player(player).bench.len() > BENCH_LIMIT && !state.has_tera_in_play(player) {
                    state.phase = Phase::DiscardingBenchDownTo { player, then: None };
                    return;
                }
            }
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

/// `Meowth ex`'s `Last-Ditch Catch`, and any future Ability sharing its
/// "played from hand onto the Bench" trigger: checked right after
/// `Action::PlayBasic` benches the card, the same site
/// `apply_risky_ruins` already reads from.
fn trigger_last_ditch_catch(state: &mut GameState, player: PlayerId, pokemon: PokemonId) {
    if state.abilities_disabled_for(pokemon) {
        return;
    }
    let Some(ability) = state.pokemon_def(pokemon).ability else {
        return;
    };
    if !matches!(ability.effect, crate::card::AbilityEffect::WhenBenchedFromHandMaySearchSupporter) {
        return;
    }
    if state.is_spent(Limit::for_ability_use(player, pokemon, ability.name)) {
        return;
    }
    let any_supporter = state
        .player(player)
        .deck
        .iter()
        .any(|c| state.matches_filter(*c, crate::card::CardFilter::TrainerOfKind(TrainerKind::Supporter)));
    if any_supporter {
        state.phase = Phase::DecidingToUseLastDitchCatch { player, pokemon };
    }
}

/// `Chien-Pao`'s `Snow Sink`, the same "played from hand onto the
/// Bench" trigger `trigger_last_ditch_catch` reads, but discarding
/// whichever Stadium is in play instead of searching.
fn trigger_snow_sink(state: &mut GameState, player: PlayerId, pokemon: PokemonId) {
    if state.abilities_disabled_for(pokemon) {
        return;
    }
    let Some(ability) = state.pokemon_def(pokemon).ability else {
        return;
    };
    if !matches!(ability.effect, crate::card::AbilityEffect::WhenBenchedFromHandMayDiscardStadium) {
        return;
    }
    if state.is_spent(Limit::for_ability_use(player, pokemon, ability.name)) {
        return;
    }
    if state.stadium.is_some() {
        state.phase = Phase::DecidingToUseSnowSink { player, pokemon };
    }
}

/// `Iron Leaves ex`'s `Rapid Vernier`, the same "played from hand onto
/// the Bench" trigger `trigger_last_ditch_catch` and `trigger_snow_sink`
/// already read, but offering a switch instead.
fn trigger_rapid_vernier(state: &mut GameState, player: PlayerId, pokemon: PokemonId) {
    if state.abilities_disabled_for(pokemon) {
        return;
    }
    let Some(ability) = state.pokemon_def(pokemon).ability else {
        return;
    };
    if !matches!(
        ability.effect,
        crate::card::AbilityEffect::WhenBenchedFromHandMaySwitchThenMoveAnyEnergy
    ) {
        return;
    }
    if state.is_spent(Limit::for_ability_use(player, pokemon, ability.name)) {
        return;
    }
    state.phase = Phase::DecidingToSwitchInForRapidVernier { player, pokemon };
}

/// `Kadabra`'s and `Alakazam`'s `Psychic Draw`, and any future Ability
/// sharing its "evolved from hand" trigger: checked right after
/// `Action::Evolve` and `Action::EvolveSkippingOneStage` (Rare Candy)
/// finish, the same spot `trigger_last_ditch_catch` reads a
/// benched-from-hand trigger from.
fn trigger_psychic_draw(state: &mut GameState, player: PlayerId, target: PokemonId) {
    if state.abilities_disabled_for(target) {
        return;
    }
    let Some(ability) = state.pokemon_def(target).ability else {
        return;
    };
    let crate::card::AbilityEffect::WhenEvolvedFromHandMayDrawCards(count) = ability.effect else {
        return;
    };
    if state.is_spent(Limit::for_ability_use(player, target, ability.name)) {
        return;
    }
    state.phase =
        Phase::DecidingToUsePsychicDraw { player, pokemon: target, name: ability.name, count };
}

/// `Noctowl`'s `Jewel Seeker`, the same "evolved from hand" trigger
/// `trigger_psychic_draw` reads, but gated on the player's own Tera
/// Pokémon in play, and opening a search instead of a draw.
fn trigger_jewel_seeker(state: &mut GameState, player: PlayerId, target: PokemonId) {
    if state.abilities_disabled_for(target) {
        return;
    }
    let Some(ability) = state.pokemon_def(target).ability else {
        return;
    };
    let crate::card::AbilityEffect::WhenEvolvedFromHandMaySearchTrainersIfOwnTeraInPlay(count) =
        ability.effect
    else {
        return;
    };
    if state.is_spent(Limit::for_ability_use(player, target, ability.name)) {
        return;
    }
    if !state.has_tera_in_play(player) {
        return;
    }
    state.phase =
        Phase::DecidingToUseJewelSeeker { player, pokemon: target, name: ability.name, count };
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
    let mut damage = checkup_damage(condition);
    // `Toxic Subjugation`: the opponent's own Active, not any
    // Pokémon in play, and only while Poisoned is the condition
    // actually resolving — read here rather than in `checkup_damage`,
    // since that function knows nothing about who else is in play.
    if condition == Condition::Poisoned
        && let Some(crate::card::TrainerEffect::StadiumExtraPoisonDamage(extra)) =
            state.stadium_effect()
        && state.pokemon_def(pokemon).kind != crate::card::Type::Darkness
    {
        damage += extra;
    }
    if condition == Condition::Poisoned {
        let owner = state.pokemon(pokemon).owner;
        if let Some(opponent_active) = state.player(owner.opponent()).active
            && !state.abilities_disabled_for(opponent_active)
            && let Some(bonus) = state.pokemon_def(opponent_active).ability.and_then(|a| match a.effect {
                crate::card::AbilityEffect::PassiveBonusCheckupDamageToOpponentsPoisonedWhileActive(
                    bonus,
                ) => Some(bonus),
                _ => None,
            })
        {
            damage += bonus;
        }
    }
    if damage > 0 {
        state.pokemon[pokemon.index()].damage += damage;
        let name = state.pokemon_def(pokemon).name;
        state
            .log
            .push(format!("{name} takes {damage} from {condition:?}."));
    }

    // Rules 50 and 53: Asleep and Burned each flip, and heads removes them.
    let checkup_side = state.pokemon(pokemon).owner;
    if matches!(condition, Condition::Burned | Condition::Asleep) && state.flip_for(checkup_side) {
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
            // `Durable Body`: only an attack's own damage — not a
            // checkup's — can be flipped away, and the flip must
            // land before any Prize is taken. `attacking_defender`
            // is the same flag `Lillie's Pearl` already reads to
            // tell an attack-caused Knockout apart from a checkup's.
            if attacking_defender == Some(pokemon)
                && !state.abilities_disabled_for(pokemon)
                && state.pokemon_def(pokemon).ability.is_some_and(|a| {
                    a.effect == crate::card::AbilityEffect::PassiveCoinFlipPreventsAttackKnockOutAtTenHp
                })
                && state.flip_for(player)
            {
                let effective_hp = state.effective_hp(pokemon);
                state.pokemon[pokemon.index()].damage = effective_hp.saturating_sub(10);
                let name = state.pokemon_def(pokemon).name;
                state.log.push(format!("{name} survives Durable Body at 10 HP."));
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
            // `Briar`: this turn only, and only for the player it granted
            // the bonus to — the current player, since only the attacking
            // player's own turn ever reaches this attack-caused branch.
            if attacking_defender == Some(pokemon)
                && state.bonus_prize_if_own_tera_attacker_knocks_out == Some(player.opponent())
                && state
                    .player(player.opponent())
                    .active
                    .is_some_and(|a| state.pokemon_def(a).markers.contains(&crate::card::Marker::Tera))
            {
                count += 1;
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

/// A Benched Pokémon leaving play because the Bench must shrink — not a
/// Knockout: no Prize, no `knocked_out` flag, nothing `Unfair Stamp` or
/// `Lillie's Pearl` should ever read from this. `Area Zero Underdepths`.
fn discard_benched_pokemon(state: &mut GameState, pokemon: PokemonId) {
    let owner = state.pokemon(pokemon).owner;
    let name = state.pokemon_def(pokemon).name;
    let cards = std::mem::take(&mut state.pokemon[pokemon.index()].cards);
    let attached = std::mem::take(&mut state.pokemon[pokemon.index()].attached);
    let side = &mut state.players[owner.index()];
    side.discard.extend(cards);
    side.discard.extend(attached);
    side.bench.retain(|p| *p != pokemon);
    state.log.push(format!("{name} is discarded from the Bench."));
}

/// Opens `Phase::DiscardingBenchDownTo` for `player` if their own Bench
/// still holds more than `BENCH_LIMIT`, chaining to `then` once `player`
/// no longer needs to. Returns whether a phase was opened — `false`
/// means `state.phase` was left untouched, and it is the caller's own
/// job to decide what that means for them (return to `Main` and
/// `settle`, or simply carry on). `Area Zero Underdepths`.
fn open_discard_bench_down_to(state: &mut GameState, player: PlayerId, then: Option<PlayerId>) -> bool {
    if state.player(player).bench.len() > BENCH_LIMIT {
        state.phase = Phase::DiscardingBenchDownTo { player, then };
        true
    } else if let Some(next) = then {
        open_discard_bench_down_to(state, next, None)
    } else {
        false
    }
}

/// `card`, owned by `owner`, just left play — read for `Area Zero
/// Underdepths` alone: if it was the one raising the Bench limit, both
/// players must shrink back down to `BENCH_LIMIT`, `owner` (the player
/// who played this card) first. Returns whether a phase was opened,
/// the same convention `open_discard_bench_down_to` itself carries.
fn open_discard_bench_down_to_if_stadium_left(state: &mut GameState, owner: PlayerId, card: CardId) -> bool {
    state.def_of(card).as_trainer().is_some_and(|t| {
        t.effect == crate::card::TrainerEffect::TeraPokemonRaisesBenchLimit
    }) && open_discard_bench_down_to(state, owner, Some(owner.opponent()))
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
