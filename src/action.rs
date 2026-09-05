//! What a player may do, and what the engine will accept.
//!
//! [`legal_actions`] is the engine's primary interface. A human interface
//! numbers the list and reads a choice; a bot is
//! `(state, legal_actions) -> Action`. Neither can reach a state the other
//! cannot, and neither can cheat by acting outside the list.

use crate::ids::{CardId, PlayerId, PokemonId};
use crate::state::{BENCH_LIMIT, GameState, Phase};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Put a Basic Pokémon from hand onto the Bench.
    PlayBasic { card: CardId },
    /// Attach an Energy from hand. Once per turn.
    AttachEnergy { card: CardId, target: PokemonId },
    /// Retreat the Active, promoting a Benched Pokémon. Once per turn.
    Retreat { to: PokemonId },
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
        Phase::Over => None,
    }
}

pub fn legal_actions(state: &GameState) -> Vec<Action> {
    let mut actions = Vec::new();
    let Some(player) = player_to_act(state) else {
        return actions;
    };
    let side = state.player(player);

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
    }

    // Rules 23-24: once per turn, pay the Retreat Cost in Energy, and only
    // with somewhere to retreat to.
    if let Some(active) = side.active {
        let cost = state.pokemon_def(active).retreat_cost;
        if !side.retreated_this_turn && state.energy_attached(active) >= cost {
            for pokemon in &side.bench {
                actions.push(Action::Retreat { to: *pokemon });
            }
        }

        // Rule 17: the player going first skips their attack step.
        if !state.is_first_turn_of_game() {
            for (index, attack) in state.pokemon_def(active).attacks.iter().enumerate() {
                if state.energy_attached(active) >= attack.cost {
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
    }
}
