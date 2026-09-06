//! The whole game, as one value.

use crate::card::{CardDb, CardDef, Condition, Pokemon, Type};
use crate::ids::{CardDefId, CardId, PlayerId, PokemonId};
use crate::rng::{Rng, shuffle};

/// Whether a condition rotates the card, which is what makes Asleep,
/// Confused, and Paralyzed exclude each other.
fn rotates(condition: Condition) -> bool {
    matches!(
        condition,
        Condition::Asleep | Condition::Confused | Condition::Paralyzed
    )
}

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
    /// The Special Conditions on this Pokémon. Only the Active carries any.
    pub conditions: Vec<Condition>,
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
    /// Rule 13: one Supporter and one Stadium a turn. Nothing plays either
    /// yet; the turn records them so the card that does has its limit.
    pub supporter_played_this_turn: bool,
    pub stadium_played_this_turn: bool,
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
            supporter_played_this_turn: false,
            stadium_played_this_turn: false,
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
///
/// Setup is four phases, not a function that runs to completion. Every choice
/// the rulebook gives a player at setup is an ordinary legal action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// The coin flip landed and this player chooses who takes the first turn.
    ChoosingWhoGoesFirst { winner: PlayerId },
    /// This player may draw one card for each mulligan the opponent took.
    TakingBonusDraws { player: PlayerId, remaining: usize },
    /// This player places a Basic from hand face down as their Active.
    PlacingActive { player: PlayerId },
    /// This player fills the Bench, and stops when they choose to.
    PlacingBench { player: PlayerId },
    /// This player resolves their own between-turn effects, in the order they
    /// choose (rule 47).
    Checkup { player: PlayerId },
    /// This player is retreating and chooses which Energy pays the cost.
    DiscardingForRetreat {
        player: PlayerId,
        /// Where the Active goes once the cost is paid.
        to: PokemonId,
        /// How many Energy the player still owes.
        remaining: u8,
    },
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
    /// The between-turn effects still to resolve, oldest first.
    pub checkup_pending: Vec<(PlayerId, PokemonId, Condition)>,
    /// The checkup has finished and the next turn is owed.
    pub pending_turn_start: bool,
    /// Setup bookkeeping: the bonus draws each player has not yet taken or
    /// declined, and whether they have finished their Bench.
    pub bonus_draws: [usize; 2],
    pub bench_placed: [bool; 2],
    pub outcome: Option<Outcome>,
    pub rng: Box<dyn Rng>,
    /// What happened, in order, for the text interface and for tests.
    pub log: Vec<String>,
}

impl GameState {
    /// Deal a new game and stop at the first choice.
    ///
    /// Shuffling, drawing, and mulliganing are not choices, so they happen
    /// here. Everything the rulebook lets a player decide — who goes first,
    /// the bonus draws, where each Pokémon goes — is a setup phase, and the
    /// caller drives it through [`crate::action::legal_actions`].
    pub fn new(db: CardDb, decklists: [Vec<CardDefId>; 2], rng: Box<dyn Rng>) -> GameState {
        let mut cards = Vec::new();
        let mut players = [PlayerState::empty(), PlayerState::empty()];

        for (slot, decklist) in decklists.iter().enumerate() {
            let owner = if slot == 0 {
                PlayerId::One
            } else {
                PlayerId::Two
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
            current: PlayerId::One,
            phase: Phase::Main,
            pending_end_turn: false,
            checkup_pending: Vec::new(),
            pending_turn_start: false,
            bonus_draws: [0, 0],
            bench_placed: [false, false],
            outcome: None,
            rng,
            log: Vec::new(),
        };

        let mulligans = [
            state.deal_opening_hand(PlayerId::One),
            state.deal_opening_hand(PlayerId::Two),
        ];

        // Rule 8: each mulligan the opponent took is worth one extra card, and
        // the player may take it or leave it.
        for player in [PlayerId::One, PlayerId::Two] {
            state.bonus_draws[player.index()] = mulligans[player.opponent().index()];
        }

        // Rule 5: the coin flip decides who chooses, not who starts.
        let winner = if state.rng.flip() {
            PlayerId::One
        } else {
            PlayerId::Two
        };
        state.log.push(format!("{winner:?} wins the coin flip."));
        state.phase = Phase::ChoosingWhoGoesFirst { winner };
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

    pub fn set_prizes(&mut self, player: PlayerId) {
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
            conditions: Vec::new(),
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

    /// How many Energy cards are attached. Retreat pays by count, because a
    /// Retreat Cost is always Colorless.
    pub fn energy_attached(&self, id: PokemonId) -> u8 {
        self.attached_energy_types(id).len() as u8
    }

    /// The type each attached Energy provides, in the order attached.
    pub fn attached_energy_types(&self, id: PokemonId) -> Vec<Type> {
        self.pokemon(id)
            .attached
            .iter()
            .filter_map(|c| match self.def_of(*c) {
                CardDef::Energy(energy) => Some(energy.kind),
                CardDef::Pokemon(_) => None,
            })
            .collect()
    }

    /// Whether the Energy attached to this Pokémon pays a cost.
    ///
    /// Every named type is matched first, because a Colorless entry takes any
    /// Energy and would otherwise eat the one Energy a named entry needed.
    pub fn pays_cost(&self, id: PokemonId, cost: &[Type]) -> bool {
        let mut available = self.attached_energy_types(id);
        for required in cost.iter().filter(|t| **t != Type::Colorless) {
            match available.iter().position(|kind| kind == required) {
                Some(at) => {
                    available.remove(at);
                }
                None => return false,
            }
        }
        let colorless = cost.iter().filter(|t| **t == Type::Colorless).count();
        available.len() >= colorless
    }

    /// Clear the once-per-turn flags for whoever is about to play.
    pub fn begin_turn(&mut self) {
        let slot = self.current.index();
        self.players[slot].energy_attached_this_turn = false;
        self.players[slot].retreated_this_turn = false;
        self.players[slot].supporter_played_this_turn = false;
        self.players[slot].stadium_played_this_turn = false;
    }

    pub fn has_condition(&self, id: PokemonId, condition: Condition) -> bool {
        self.pokemon(id).conditions.contains(&condition)
    }

    /// Put a Special Condition on a Pokémon.
    ///
    /// Rule 55: Asleep, Confused, and Paralyzed all rotate the card, so the
    /// newest replaces whichever of them was there. Rule 56: Burned and
    /// Poisoned are independent of that and of each other. Rule 57: a second
    /// Burn or Poison replaces the first rather than stacking.
    pub fn inflict(&mut self, id: PokemonId, condition: Condition) {
        if rotates(condition) {
            self.pokemon[id.index()].conditions.retain(|c| !rotates(*c));
        } else {
            self.remove_condition(id, condition);
        }
        self.pokemon[id.index()].conditions.push(condition);
    }

    /// Rule 27: a Pokémon that leaves the Active spot loses every condition.
    pub fn clear_conditions(&mut self, id: PokemonId) {
        self.pokemon[id.index()].conditions.clear();
    }

    pub fn remove_condition(&mut self, id: PokemonId, condition: Condition) {
        self.pokemon[id.index()]
            .conditions
            .retain(|c| *c != condition);
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
