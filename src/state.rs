//! The whole game, as one value.

use crate::card::{CardDb, CardDef, Pokemon};
use crate::ids::{CardDefId, CardId, PlayerId, PokemonId};
use crate::rng::{Rng, shuffle};

/// The bench holds 5.
pub const BENCH_LIMIT: usize = 5;
/// Each player sets 6 Prize cards aside.
pub const PRIZE_COUNT: usize = 6;
/// The opening hand.
pub const OPENING_HAND: usize = 7;

/// One physical card. `owner` never changes: a knocked-out Pokémon and its
/// attachments go to *its owner's* discard, not the opponent's.
#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub def: CardDefId,
    pub owner: PlayerId,
}

/// One Pokémon in play, with what has happened to it.
#[derive(Debug, Clone)]
pub struct PokemonInPlay {
    pub card: CardId,
    pub owner: PlayerId,
    /// Damage in points, not counters. A counter is 10 points.
    pub damage: u32,
    pub attached: Vec<CardId>,
    /// The turn this Pokémon came into play. Evolution reads it; Milestone 1
    /// records it so the rule has somewhere to land.
    pub played_on_turn: u32,
    pub knocked_out: bool,
}

/// One player's zones and their once-per-turn flags.
#[derive(Debug, Clone)]
pub struct PlayerState {
    /// The draw pile. `Deck` is the tournament decklist, so the pile in play
    /// is the Library.
    pub library: Vec<CardId>,
    pub hand: Vec<CardId>,
    pub discard: Vec<CardId>,
    pub prizes: Vec<CardId>,
    pub active: Option<PokemonId>,
    pub bench: Vec<PokemonId>,
    pub energy_attached_this_turn: bool,
    pub retreated_this_turn: bool,
}

impl PlayerState {
    fn empty() -> PlayerState {
        PlayerState {
            library: Vec::new(),
            hand: Vec::new(),
            discard: Vec::new(),
            prizes: Vec::new(),
            active: None,
            bench: Vec::new(),
            energy_attached_this_turn: false,
            retreated_this_turn: false,
        }
    }

    /// Every Pokémon in play, Active first.
    pub fn in_play(&self) -> Vec<PokemonId> {
        let mut all = Vec::new();
        all.extend(self.active);
        all.extend(self.bench.iter().copied());
        all
    }
}

/// What the engine is waiting for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// The current player is taking their turn.
    Main,
    /// This player lost their Active and must promote one from the Bench.
    /// It interrupts whoever's turn it is.
    Promoting(PlayerId),
    /// The game is decided.
    Over,
}

/// Why the game ended. Rule 44: taking the last Prize wins even when another
/// condition lands at the same moment, so the reasons are checked in order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WinReason {
    /// The winner took their last Prize card.
    AllPrizesTaken,
    /// The loser had no Pokémon to promote.
    NoPokemonInPlay,
    /// The loser could not draw at the start of their turn.
    CouldNotDraw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Outcome {
    pub winner: PlayerId,
    pub reason: WinReason,
}

pub struct GameState {
    pub db: CardDb,
    pub cards: Vec<Card>,
    pub pokemon: Vec<PokemonInPlay>,
    pub players: [PlayerState; 2],
    pub turn_number: u32,
    pub current: PlayerId,
    pub phase: Phase,
    /// An attack ends the turn, but a knockout it caused is settled first.
    /// The flag remembers that the turn still owes its ending.
    pub pending_end_turn: bool,
    pub outcome: Option<Outcome>,
    pub rng: Box<dyn Rng>,
    /// What happened, in order, for the text interface and for tests.
    pub log: Vec<String>,
}

impl GameState {
    /// Deal a new game: shuffle, mulligan, place Pokémon, set Prizes.
    ///
    /// Milestone 1 shortcut: the coin flip for who goes first is skipped —
    /// [`PlayerId::First`] starts — and setup places the first Basic in hand as
    /// Active and the rest on the Bench rather than asking. Both become
    /// ordinary choices once the Setup phase grows its own legal actions.
    pub fn new(db: CardDb, decklists: [Vec<CardDefId>; 2], rng: Box<dyn Rng>) -> GameState {
        let mut cards = Vec::new();
        let mut players = [PlayerState::empty(), PlayerState::empty()];

        for (slot, decklist) in decklists.iter().enumerate() {
            let owner = if slot == 0 {
                PlayerId::First
            } else {
                PlayerId::Second
            };
            for def in decklist {
                let id = CardId(cards.len() as u32);
                cards.push(Card { def: *def, owner });
                players[slot].library.push(id);
            }
        }

        let mut state = GameState {
            db,
            cards,
            pokemon: Vec::new(),
            players,
            turn_number: 0,
            current: PlayerId::First,
            phase: Phase::Main,
            pending_end_turn: false,
            outcome: None,
            rng,
            log: Vec::new(),
        };

        let mulligans = [
            state.deal_opening_hand(PlayerId::First),
            state.deal_opening_hand(PlayerId::Second),
        ];

        // Rule 8: for each mulligan the opponent took, you may draw 1 extra.
        // Milestone 1 always takes them.
        for player in [PlayerId::First, PlayerId::Second] {
            let extra = mulligans[player.opponent().index()];
            for _ in 0..extra {
                state.draw(player);
            }
        }

        for player in [PlayerId::First, PlayerId::Second] {
            state.place_opening_pokemon(player);
            state.set_prizes(player);
        }

        state
            .log
            .push(format!("Turn {} begins.", state.turn_number + 1));
        state.begin_turn();
        state
    }

    fn deal_opening_hand(&mut self, player: PlayerId) -> usize {
        let mut mulligans = 0;
        loop {
            let slot = player.index();
            shuffle(self.rng.as_mut(), &mut self.players[slot].library);
            for _ in 0..OPENING_HAND {
                self.draw(player);
            }
            if self.hand_has_basic(player) {
                self.log
                    .push(format!("{player:?} draws an opening hand of 7."));
                return mulligans;
            }
            // Rule 7: no Basic, so reveal, shuffle back, and draw again.
            let hand = std::mem::take(&mut self.players[slot].hand);
            self.players[slot].library.extend(hand);
            mulligans += 1;
            self.log.push(format!("{player:?} mulligans."));
        }
    }

    fn hand_has_basic(&self, player: PlayerId) -> bool {
        self.players[player.index()]
            .hand
            .iter()
            .any(|c| self.def_of(*c).is_basic_pokemon())
    }

    fn place_opening_pokemon(&mut self, player: PlayerId) {
        let slot = player.index();
        let basics: Vec<CardId> = self.players[slot]
            .hand
            .iter()
            .copied()
            .filter(|c| self.def_of(*c).is_basic_pokemon())
            .take(1 + BENCH_LIMIT)
            .collect();
        for (position, card) in basics.into_iter().enumerate() {
            self.remove_from_hand(player, card);
            let pokemon = self.put_into_play(player, card);
            if position == 0 {
                self.players[slot].active = Some(pokemon);
            } else {
                self.players[slot].bench.push(pokemon);
            }
        }
    }

    fn set_prizes(&mut self, player: PlayerId) {
        let slot = player.index();
        for _ in 0..PRIZE_COUNT {
            match self.players[slot].library.pop() {
                Some(card) => self.players[slot].prizes.push(card),
                None => break,
            }
        }
    }

    pub fn put_into_play(&mut self, player: PlayerId, card: CardId) -> PokemonId {
        let id = PokemonId(self.pokemon.len() as u32);
        self.pokemon.push(PokemonInPlay {
            card,
            owner: player,
            damage: 0,
            attached: Vec::new(),
            played_on_turn: self.turn_number,
            knocked_out: false,
        });
        id
    }

    pub fn remove_from_hand(&mut self, player: PlayerId, card: CardId) {
        let hand = &mut self.players[player.index()].hand;
        if let Some(at) = hand.iter().position(|c| *c == card) {
            hand.remove(at);
        }
    }

    /// Take the top card. `false` means the library was empty, which loses the
    /// game at the start of a turn (rule 12) and nowhere else.
    pub fn draw(&mut self, player: PlayerId) -> bool {
        let slot = player.index();
        match self.players[slot].library.pop() {
            Some(card) => {
                self.players[slot].hand.push(card);
                true
            }
            None => false,
        }
    }

    pub fn def_of(&self, card: CardId) -> &CardDef {
        self.db.get(self.cards[card.index()].def)
    }

    pub fn pokemon_def(&self, pokemon: PokemonId) -> &Pokemon {
        self.def_of(self.pokemon[pokemon.index()].card)
            .as_pokemon()
            .expect("a Pokémon in play is a Pokémon card")
    }

    pub fn pokemon(&self, id: PokemonId) -> &PokemonInPlay {
        &self.pokemon[id.index()]
    }

    pub fn player(&self, id: PlayerId) -> &PlayerState {
        &self.players[id.index()]
    }

    pub fn remaining_hp(&self, id: PokemonId) -> u32 {
        self.pokemon_def(id)
            .hp
            .saturating_sub(self.pokemon(id).damage)
    }

    /// How many Energy cards are attached. Milestone 1 pays a cost by count.
    pub fn energy_attached(&self, id: PokemonId) -> u8 {
        self.pokemon(id)
            .attached
            .iter()
            .filter(|c| self.def_of(**c).is_energy())
            .count() as u8
    }

    /// Clear the once-per-turn flags for whoever is about to play.
    pub fn begin_turn(&mut self) {
        let slot = self.current.index();
        self.players[slot].energy_attached_this_turn = false;
        self.players[slot].retreated_this_turn = false;
    }

    pub fn is_over(&self) -> bool {
        self.outcome.is_some()
    }

    /// Rule 15-18: the player going first draws, cannot play a Supporter, and
    /// skips their attack step.
    pub fn is_first_turn_of_game(&self) -> bool {
        self.turn_number == 0
    }
}
