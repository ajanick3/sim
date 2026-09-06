//! What one player is allowed to see.
//!
//! A bot handed the whole [`GameState`] can read the opponent's hand and the
//! Prize cards, and cheat without meaning to. A view carries only what its
//! player may see, so the mask is a type, not a rule someone remembers.
//!
//! What a view hides: the cards in the opponent's hand, the cards in either
//! library and their order, and the cards in either Prize pile — a player
//! cannot see their own Prizes either. Each of those keeps its count, because
//! a count is public.

use crate::card::Condition;
use crate::ids::{CardId, PlayerId, PokemonId};
use crate::state::{GameState, Phase};

/// A card the viewer may see, with the name they read off it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CardView {
    pub id: CardId,
    pub name: &'static str,
}

/// A Pokémon in play. Everything about it is public.
#[derive(Debug, Clone)]
pub struct PokemonView {
    pub id: PokemonId,
    pub name: &'static str,
    pub hp: u32,
    pub damage: u32,
    pub remaining_hp: u32,
    pub conditions: Vec<Condition>,
    pub attached: Vec<CardView>,
}

/// One player's side, as the viewer sees it.
#[derive(Debug, Clone)]
pub struct SideView {
    pub player: PlayerId,
    pub hand_count: usize,
    pub library_count: usize,
    pub prize_count: usize,
    pub discard: Vec<CardView>,
    /// Public, like the discard pile: everyone sees what left the game.
    pub lost_zone: Vec<CardView>,
    pub active: Option<PokemonView>,
    pub bench: Vec<PokemonView>,
}

/// The game as one player sees it.
#[derive(Debug, Clone)]
pub struct PlayerView {
    pub you: PlayerId,
    pub current: PlayerId,
    pub turn_number: u32,
    pub phase: Phase,
    /// Your own hand. The opponent's is a count on their side.
    pub your_hand: Vec<CardView>,
    sides: [SideView; 2],
}

impl PlayerView {
    pub fn of(state: &GameState, you: PlayerId) -> PlayerView {
        PlayerView {
            you,
            current: state.current,
            turn_number: state.turn_number,
            phase: state.phase,
            your_hand: state
                .player(you)
                .hand
                .iter()
                .map(|card| card_view(state, *card))
                .collect(),
            sides: [
                side_view(state, PlayerId::One),
                side_view(state, PlayerId::Two),
            ],
        }
    }

    pub fn side(&self, player: PlayerId) -> &SideView {
        &self.sides[player.index()]
    }
}

fn card_view(state: &GameState, card: CardId) -> CardView {
    CardView {
        id: card,
        name: state.def_of(card).name(),
    }
}

fn side_view(state: &GameState, player: PlayerId) -> SideView {
    let side = state.player(player);
    SideView {
        player,
        hand_count: side.hand.len(),
        library_count: side.library.len(),
        prize_count: side.prizes.len(),
        discard: side
            .discard
            .iter()
            .map(|card| card_view(state, *card))
            .collect(),
        lost_zone: side
            .lost_zone
            .iter()
            .map(|card| card_view(state, *card))
            .collect(),
        active: side.active.map(|p| pokemon_view(state, p)),
        bench: side.bench.iter().map(|p| pokemon_view(state, *p)).collect(),
    }
}

fn pokemon_view(state: &GameState, id: PokemonId) -> PokemonView {
    let in_play = state.pokemon(id);
    let def = state.pokemon_def(id);
    PokemonView {
        id,
        name: def.name,
        hp: def.hp,
        damage: in_play.damage,
        remaining_hp: state.remaining_hp(id),
        conditions: in_play.conditions.clone(),
        attached: in_play
            .attached
            .iter()
            .map(|card| card_view(state, *card))
            .collect(),
    }
}
