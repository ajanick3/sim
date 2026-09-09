//! A Pokémon TCG rules engine.
//!
//! The engine is pure: it holds no I/O, no async, and no clock. A game is a
//! value. Feed it a seed and a list of actions and it replays exactly.

pub mod action;
pub mod card;
pub mod decklist;
pub mod engine;
pub mod ids;
pub mod import;
pub mod rng;
pub mod state;
pub mod strategy;
pub mod view;

pub mod cards;

pub use action::{Action, legal_actions};
pub use engine::apply;
pub use state::{GameState, Outcome};
