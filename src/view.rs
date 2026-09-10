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
//!
//! One scoped exception: while a player searches their whole own library,
//! [`PlayerView::library_in_search`] shows that player every card in it,
//! sorted so the order the search saw is gone. This lets the board show the
//! cards the search cannot reach next to the cards it can. See ADR 0101.

use crate::card::Condition;
use crate::ids::{CardDefId, CardId, PlayerId, PokemonId};
use crate::state::{GameState, Phase};

/// A card the viewer may see, with the name they read off it, and its own
/// definition — a card's own identity, and what it says as printed, are
/// never hidden information: a player already knows their own hand, and
/// `CardDb` is the shared rules text of every card, true of every copy.
/// What a view withholds is zone membership, not this.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CardView {
    pub id: CardId,
    pub name: &'static str,
    pub def: CardDefId,
    /// The TCGdex print id, e.g. `"sv6-160"` — the key card art is filed under.
    pub print_id: &'static str,
}

/// A Pokémon in play. Everything about it is public.
#[derive(Debug, Clone)]
pub struct PokemonView {
    pub id: PokemonId,
    pub name: &'static str,
    /// The TCGdex print id of this Pokémon's top card.
    pub print_id: &'static str,
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
    /// The Stadium in play, or none.
    pub stadium: Option<CardView>,
    /// Every card in your own library, sorted by kind then name, while you
    /// search the whole of it — otherwise none. The order the search saw is
    /// dropped by the sort, so this leaks nothing the mask holds back.
    pub library_in_search: Option<Vec<CardView>>,
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
            stadium: state.stadium.map(|(_, card)| card_view(state, card)),
            library_in_search: state.whole_library_search().filter(|owner| *owner == you).map(
                |_| {
                    let mut cards = state.player(you).library.clone();
                    // A Pokémon's place inside its own bucket follows its
                    // evolution line, not its name: the bigger the line
                    // is in this library, the sooner it shows, and within
                    // a line a Basic leads its Stage 1 and Stage 2.
                    let mut line_counts: std::collections::HashMap<&str, usize> =
                        std::collections::HashMap::new();
                    for card in &cards {
                        if let crate::card::CardDef::Pokemon(_) = state.def_of(*card) {
                            *line_counts.entry(root_basic_name(state.def_of(*card))).or_insert(0) +=
                                1;
                        }
                    }
                    cards.sort_by(|a, b| {
                        let (da, db) = (state.def_of(*a), state.def_of(*b));
                        category_rank(da).cmp(&category_rank(db)).then_with(|| {
                            match (da, db) {
                                (
                                    crate::card::CardDef::Pokemon(pa),
                                    crate::card::CardDef::Pokemon(pb),
                                ) => {
                                    let (ra, rb) = (root_basic_name(da), root_basic_name(db));
                                    let (ca, cb) = (
                                        line_counts.get(ra).copied().unwrap_or(0),
                                        line_counts.get(rb).copied().unwrap_or(0),
                                    );
                                    cb.cmp(&ca) // the bigger line first
                                        .then_with(|| ra.cmp(rb))
                                        .then_with(|| stage_rank(pa.stage).cmp(&stage_rank(pb.stage)))
                                        .then_with(|| da.name().cmp(db.name()))
                                }
                                _ => da.name().cmp(db.name()),
                            }
                        })
                    });
                    cards.iter().map(|card| card_view(state, *card)).collect()
                },
            ),
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

/// A coarse sort bucket for a library listing: Pokémon, then Trainers by
/// kind, then Energy. It only has to be stable, not meaningful.
fn category_rank(def: &crate::card::CardDef) -> u8 {
    use crate::card::{CardDef, TrainerKind};
    match def {
        CardDef::Pokemon(_) => 0,
        CardDef::Trainer(t) => match t.kind {
            TrainerKind::Supporter => 1,
            TrainerKind::Item => 2,
            TrainerKind::Tool => 3,
            TrainerKind::Stadium => 4,
        },
        CardDef::Energy(_) => 5,
    }
}

/// The Basic at the root of a Pokémon's evolution line — itself, on a
/// Basic. A Stage 2 reads `evolves_from_basic` where that resolved at
/// import; a chain it did not resolve falls back to `evolve_from`, one
/// link up, rather than losing the card from its line entirely.
fn root_basic_name(def: &crate::card::CardDef) -> &'static str {
    use crate::card::{CardDef, Stage};
    match def {
        CardDef::Pokemon(p) => match p.stage {
            Stage::Basic => p.name,
            Stage::Stage1 => p.evolve_from.unwrap_or(p.name),
            Stage::Stage2 => p.evolves_from_basic.or(p.evolve_from).unwrap_or(p.name),
        },
        _ => "",
    }
}

/// A Pokémon's depth in its own line: Basic leads, then Stage 1, then
/// Stage 2.
fn stage_rank(stage: crate::card::Stage) -> u8 {
    use crate::card::Stage;
    match stage {
        Stage::Basic => 0,
        Stage::Stage1 => 1,
        Stage::Stage2 => 2,
    }
}

fn card_view(state: &GameState, card: CardId) -> CardView {
    CardView {
        id: card,
        name: state.def_of(card).name(),
        def: state.cards[card.index()].def,
        print_id: state.def_of(card).print_id(),
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
        print_id: def.print_id,
        hp: state.effective_hp(id),
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
