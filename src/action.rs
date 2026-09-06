//! What a player may do, and what the engine will accept.
//!
//! [`legal_actions`] is the engine's primary interface. A human interface
//! numbers the list and reads a choice; a bot is
//! `(state, legal_actions) -> Action`. Neither can reach a state the other
//! cannot, and neither can cheat by acting outside the list.

use crate::card::Condition;
use crate::ids::{CardId, PlayerId, PokemonId};
use crate::state::{BENCH_LIMIT, GameState, Phase};

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
}

/// Whose choice the engine is waiting for. It is not always the player whose
/// turn it is: a knockout hands the choice to the player who lost the Pokémon.
pub fn player_to_act(state: &GameState) -> Option<PlayerId> {
    match state.phase {
        Phase::Main => Some(state.current),
        Phase::Promoting(player) => Some(player),
        Phase::ChoosingWhoGoesFirst { winner } => Some(winner),
        Phase::TakingBonusDraws { player, .. } => Some(player),
        Phase::PlacingActive { player } => Some(player),
        Phase::PlacingBench { player } => Some(player),
        Phase::DiscardingForRetreat { player, .. } => Some(player),
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
        _ => {}
    }

    if let Phase::Promoting(_) = state.phase {
        for pokemon in &side.bench {
            actions.push(Action::Promote { pokemon: *pokemon });
        }
        return actions;
    }

    for card in &side.hand {
        let def = state.def_of(*card);
        if def.is_basic_pokemon() && side.bench.len() < BENCH_LIMIT {
            actions.push(Action::PlayBasic { card: *card });
        }
        if def.is_energy() && !side.energy_attached_this_turn {
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
                    && !state.pokemon(target).evolved_this_turn;
                if eligible {
                    actions.push(Action::Evolve {
                        card: *card,
                        target,
                    });
                }
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
        if !side.retreated_this_turn && !held(active) && state.energy_attached(active) >= cost {
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
