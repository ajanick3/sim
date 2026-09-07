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
    /// Evolve a Basic in play straight into the named Stage 2 from hand,
    /// skipping the Stage 1 between them.
    EvolveSkippingOneStage { card: CardId, target: PokemonId },
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

    for card in &side.hand {
        let def = state.def_of(*card);
        if def.is_basic_pokemon() && side.bench.len() < BENCH_LIMIT {
            actions.push(Action::PlayBasic { card: *card });
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
                let eligible = state.pokemon_def(target).name == from
                    && state.pokemon(target).played_on_turn < state.turn_number
                    && !state.is_spent(Limit::Evolved(target));
                if eligible {
                    actions.push(Action::Evolve {
                        card: *card,
                        target,
                    });
                }
            }
        }
        // Rule 13: an Item any number of times; a Supporter or a Stadium
        // once a turn. None of the built cards is a Stadium, but the gate is
        // written for the kind, not the card, so one arriving costs nothing.
        if let Some(trainer) = def.as_trainer() {
            let timing = match trainer.kind {
                TrainerKind::Item | TrainerKind::Tool => true,
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
                TrainerEffect::SwitchOwnActive => !side.bench.is_empty(),
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
        let cost = state.pokemon_def(active).retreat_cost;
        if !state.is_spent(Limit::Retreated(player))
            && !held(active)
            && state.energy_attached(active) >= cost
        {
            for pokemon in &side.bench {
                actions.push(Action::Retreat { to: *pokemon });
            }
        }

        // Rule 17: the player going first skips their attack step.
        if !state.is_first_turn_of_game() && !held(active) {
            for (index, attack) in state.pokemon_def(active).attacks.iter().enumerate() {
                if state.pays_cost(active, &attack.cost) {
                    actions.push(Action::Attack { index });
                }
            }
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
            let eligible = state.pokemon_def(target).name == from
                && state.pokemon(target).played_on_turn < state.turn_number
                && !state.is_spent(Limit::Evolved(target));
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
    }
}
