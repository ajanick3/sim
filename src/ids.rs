//! Typed indices into the game's arenas.
//!
//! The state is a graph: a Pokémon points at the cards attached to it, a zone
//! points at the cards it holds. Rust owns that graph badly through references.
//! So every object lives in one `Vec` for the life of the game and everything
//! else names it by index. An index is `Copy`, so passing one around never
//! borrows the arena that holds the object.

/// Which seat. A seat is not a turn order: the opening coin flip decides who
/// goes first, so either seat may start. `PlayerId` doubles as an index into
/// [`crate::state::GameState::players`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerId {
    /// The first seat.
    One,
    /// The second seat.
    Two,
}

impl PlayerId {
    pub fn opponent(self) -> PlayerId {
        match self {
            PlayerId::One => PlayerId::Two,
            PlayerId::Two => PlayerId::One,
        }
    }

    pub fn index(self) -> usize {
        match self {
            PlayerId::One => 0,
            PlayerId::Two => 1,
        }
    }
}

/// One physical card, created at setup and never destroyed. Zones move the id,
/// not the card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CardId(pub u32);

/// One printed card, shared by every copy of it. Milestone 1 writes these as
/// literals; a later milestone reads them from the JSON the `pkmn` repo exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardDefId(pub u32);

/// One Pokémon in play. A Pokémon leaves play when it is knocked out, so this
/// arena grows over a game and holds knocked-out entries as history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PokemonId(pub u32);

impl CardId {
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

impl CardDefId {
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

impl PokemonId {
    pub fn index(self) -> usize {
        self.0 as usize
    }
}
