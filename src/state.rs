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
    /// Every card this Pokémon has been, oldest first: the Basic it was
    /// placed as, then each evolution played on top of it. A knockout
    /// discards the whole stack together (rule 38); the current name, HP, and
    /// attacks read from the last one.
    pub cards: Vec<CardId>,
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

impl PokemonInPlay {
    /// The card that says what this Pokémon currently is.
    pub fn top_card(&self) -> CardId {
        *self
            .cards
            .last()
            .expect("a Pokémon in play is at least one card")
    }
}

/// Something a player or a Pokémon may do only once in a turn. Rule 13 sets
/// most of them; rule 20 sets the last, which belongs to a Pokémon rather
/// than to a player, and that is why a limit names its own owner.
///
/// A once-per-game limit would be the same shape with a list that is never
/// cleared. Nothing needs one: `Legacy Energy` is the only card in the pool
/// that reads "once per game", and it is a special Energy the engine refuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Limit {
    EnergyAttached(PlayerId),
    Retreated(PlayerId),
    SupporterPlayed(PlayerId),
    StadiumPlayed(PlayerId),
    Evolved(PokemonId),
}

/// One player's zones.
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
    /// A Pokémon owned by `of` must be chosen from `of`'s Bench to become
    /// their new Active, and `chooser` picks it. It interrupts whoever's turn
    /// it is. After a knockout, `of == chooser`: you promote your own. A card
    /// that switches an opponent's Active makes them differ.
    Promoting { of: PlayerId, chooser: PlayerId },
    /// A Trainer effect is moving up to `remaining` cards matching `filter`
    /// from one zone to another, `chooser`'s choice each time. Most of the
    /// primitives in `docs/architecture/effects.md` share this one phase — a
    /// deck search, a discard-pile recovery, a hand thinned for a bigger
    /// draw are all an instance of it with different zones.
    Deciding {
        chooser: PlayerId,
        from: crate::card::Zone,
        to: crate::card::Destination,
        filter: crate::card::CardFilter,
        remaining: u32,
        /// How many have been taken so far. `then` reads it when the choice
        /// ends; nothing else needs it.
        moved: u32,
        /// What to do, beyond the move itself, once the choice ends.
        then: Option<crate::card::Then>,
    },
    /// `chooser` picks one Energy attached to a Pokémon `of` controls, in
    /// play, to discard. Crushing Hammer's heads case is the only card that
    /// needs this; a Pokémon's attachments are not a `Zone`, so `Deciding`
    /// cannot express it.
    DiscardingOpponentEnergy { chooser: PlayerId, of: PlayerId },
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
    /// The Stadium in play and who played it. Rule 58: one at a time, and a
    /// new one discards the old to its own owner's pile — which is why the
    /// player is kept beside the card.
    pub stadium: Option<(PlayerId, CardId)>,
    /// The once-per-turn limits spent so far. Cleared when a turn begins.
    pub spent: Vec<Limit>,
    pub rng: Box<dyn Rng>,
    /// What happened, in order, as prose for a reader. It is not the record
    /// a replay reads — that is [`GameState::history`].
    pub log: Vec<String>,
    /// Every action applied to this game, in order. A refused action is not
    /// here: it changed nothing, so there is nothing to replay. With the
    /// seed and the decklists, this is the whole game.
    pub history: Vec<crate::action::Action>,
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
            stadium: None,
            spent: Vec::new(),
            rng,
            log: Vec::new(),
            history: Vec::new(),
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
            cards: vec![card],
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
        self.def_of(self.pokemon[pokemon.index()].top_card())
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

    /// The cards in one of a player's zones. A Trainer effect moves between
    /// these; a Pokémon's attachments are not one of them.
    pub fn zone(&self, player: PlayerId, zone: crate::card::Zone) -> &Vec<CardId> {
        let side = self.player(player);
        match zone {
            crate::card::Zone::Hand => &side.hand,
            crate::card::Zone::Discard => &side.discard,
            crate::card::Zone::Library => &side.library,
        }
    }

    pub fn zone_mut(&mut self, player: PlayerId, zone: crate::card::Zone) -> &mut Vec<CardId> {
        let side = &mut self.players[player.index()];
        match zone {
            crate::card::Zone::Hand => &mut side.hand,
            crate::card::Zone::Discard => &mut side.discard,
            crate::card::Zone::Library => &mut side.library,
        }
    }

    /// Move one card from one zone to another, for the same player. Neither
    /// zone is a Pokémon's attachments; that is a different move entirely.
    pub fn move_card(
        &mut self,
        player: PlayerId,
        card: CardId,
        from: crate::card::Zone,
        to: crate::card::Zone,
    ) {
        let source = self.zone_mut(player, from);
        if let Some(at) = source.iter().position(|c| *c == card) {
            source.remove(at);
        }
        self.zone_mut(player, to).push(card);
    }

    /// Whether a card meets a Trainer effect's filter.
    pub fn matches_filter(&self, card: CardId, filter: crate::card::CardFilter) -> bool {
        use crate::card::CardFilter;
        match filter {
            CardFilter::AnyPokemon => self.def_of(card).as_pokemon().is_some(),
            CardFilter::PokemonWithoutRuleBox => self
                .def_of(card)
                .as_pokemon()
                .is_some_and(|p| p.prizes == 1),
            CardFilter::PokemonOrBasicEnergy => {
                let def = self.def_of(card);
                def.as_pokemon().is_some() || def.is_energy()
            }
            CardFilter::PokemonEx => self.def_of(card).as_pokemon().is_some_and(|p| p.prizes > 1),
            CardFilter::BasicPokemonWithHpAtMost(hp) => self
                .def_of(card)
                .as_pokemon()
                .is_some_and(|p| p.evolve_from.is_none() && p.hp <= hp),
        }
    }

    /// The type each attached Energy provides, in the order attached.
    pub fn attached_energy_types(&self, id: PokemonId) -> Vec<Type> {
        self.pokemon(id)
            .attached
            .iter()
            .filter_map(|c| match self.def_of(*c) {
                CardDef::Energy(energy) => Some(energy.kind),
                CardDef::Pokemon(_) | CardDef::Trainer(_) => None,
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
        // Every turn limit is cleared, not only the current player's. The
        // opponent's were unreadable during this turn anyway — every gate
        // asks about whoever is acting — so clearing all of them is the
        // same game and one line.
        self.spent.clear();
    }

    /// Whether a once-per-turn limit has been spent.
    pub fn is_spent(&self, limit: Limit) -> bool {
        self.spent.contains(&limit)
    }

    /// Spend a once-per-turn limit. Spending one twice is not an error; the
    /// gates that read it ask only whether it is spent.
    pub fn spend(&mut self, limit: Limit) {
        if !self.is_spent(limit) {
            self.spent.push(limit);
        }
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
