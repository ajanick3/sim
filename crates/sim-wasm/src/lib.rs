//! The browser boundary for the engine.
//!
//! `Game` owns a live [`GameState`]. JavaScript holds the handle for the life
//! of the game and calls methods on it; no game state crosses the boundary as
//! data. Every method returns a `String` holding JSON, so the seam also runs
//! under an ordinary `cargo test` on the host.

use sim::GameState;
use sim::action::{Action, describe, legal_actions, player_to_act};
use sim::cards::{milestone1, starter_decklist};
use sim::decklist;
use sim::engine::apply;
use sim::ids::{CardDefId, PlayerId};
use sim::import::{Import, load};
use sim::rng::SeededRng;
use sim::view::PlayerView;
use wasm_bindgen::prelude::*;

/// The card artifact, read once. Parsing and interning the 2 MB of card data
/// is done here, so a page that starts many games pays it a single time.
#[wasm_bindgen]
pub struct CardData {
    import: Import,
}

#[wasm_bindgen]
impl CardData {
    /// Read `data/cards.json`, passed in as a string.
    pub fn new(cards_json: &str) -> Result<CardData, String> {
        load(cards_json).map(|import| CardData { import })
    }
}

impl CardData {
    fn build_deck(&mut self, text: &str) -> Result<Vec<CardDefId>, String> {
        let list = decklist::parse(text);
        let report = decklist::check(&list, &self.import);
        if !report.is_playable() {
            return Err("decklist has a card the engine cannot play".to_string());
        }
        Ok(decklist::to_deck(&list, &report, &mut self.import))
    }
}

/// One game, held open across calls. `applied` records the index of every
/// action taken, in order — the tail of a recipe that can rebuild this game
/// from its start (ticket `web-followups/01`).
#[wasm_bindgen]
pub struct Game {
    state: GameState,
    applied: Vec<u32>,
}

impl Game {
    fn standard_state(
        data: &mut CardData,
        deck_a: &str,
        deck_b: &str,
        seed: u64,
    ) -> Result<GameState, String> {
        let a = data.build_deck(deck_a)?;
        let b = data.build_deck(deck_b)?;
        Ok(GameState::new(
            data.import.db.clone(),
            [a, b],
            Box::new(SeededRng::new(seed)),
        ))
    }

    fn apply_index(&mut self, index: u32) -> Result<(), String> {
        let actions = legal_actions(&self.state);
        let action = actions
            .get(index as usize)
            .ok_or_else(|| format!("no legal action at index {index}"))?;
        apply(&mut self.state, *action).map_err(|error| format!("{error:?}"))?;
        self.applied.push(index);
        Ok(())
    }
}

#[wasm_bindgen]
impl Game {
    /// A game of the synthetic set, both seats dealt the starter decklist.
    /// This needs no card artifact, so it is the path a test and a demo take.
    pub fn synthetic(seed: u64) -> Game {
        let set = milestone1();
        let deck = starter_decklist(&set);
        let state = GameState::new(
            set.db,
            [deck.clone(), deck],
            Box::new(SeededRng::new(seed)),
        );
        Game { state, applied: Vec::new() }
    }

    /// A game of the Standard set. Each decklist is the text of a `.txt`
    /// deck file. Both must be fully playable.
    pub fn standard(
        data: &mut CardData,
        deck_a: &str,
        deck_b: &str,
        seed: u64,
    ) -> Result<Game, String> {
        Ok(Game {
            state: Game::standard_state(data, deck_a, deck_b, seed)?,
            applied: Vec::new(),
        })
    }

    /// Rebuild a Standard game and replay `indices` from its start, each an
    /// index into the legal-action list at that point. The end state is the
    /// game those moves produced live.
    pub fn replay_standard(
        data: &mut CardData,
        deck_a: &str,
        deck_b: &str,
        seed: u64,
        indices: Vec<u32>,
    ) -> Result<Game, String> {
        let mut game = Game {
            state: Game::standard_state(data, deck_a, deck_b, seed)?,
            applied: Vec::with_capacity(indices.len()),
        };
        for (seq, index) in indices.into_iter().enumerate() {
            game.apply_index(index)
                .map_err(|error| format!("move {seq}: {error}"))?;
        }
        Ok(game)
    }

    /// The legal actions right now, each as the text `describe` prints.
    /// The index into this list is the argument `apply` takes.
    pub fn legal_actions(&self) -> String {
        let labels: Vec<String> = legal_actions(&self.state)
            .iter()
            .map(|action| describe(&self.state, *action))
            .collect();
        serde_json::to_string(&labels).expect("a list of strings serializes")
    }

    /// What each legal action touches, index-aligned with `legal_actions`.
    /// Each entry is `{ kind, card, target }` — the `Action` variant's
    /// name, the `CardId` it names (or null), the `PokemonId` it names
    /// (or null). See ADR 0099. Actions this does not recognise still
    /// appear, with `card` and `target` both null.
    pub fn action_meta(&self) -> String {
        let meta: Vec<ActionMeta> = legal_actions(&self.state)
            .iter()
            .map(|action| {
                let (card, target) = action_handles(*action);
                ActionMeta {
                    kind: action_kind(*action),
                    card,
                    target,
                    card_face: card.map(|i| {
                        let id = sim::ids::CardId(i as u32);
                        let def = self.state.def_of(id);
                        WireCardFace {
                            print_id: def.print_id().to_string(),
                            name: def.name().to_string(),
                            energy_type: def.as_energy().map(|e| format!("{:?}", e.kind)),
                            category: card_category(def),
                        }
                    }),
                }
            })
            .collect();
        serde_json::to_string(&meta).expect("the action meta serializes")
    }

    /// Apply the action at `index` in the current `legal_actions` list.
    pub fn apply(&mut self, index: usize) -> Result<(), String> {
        self.apply_index(index as u32)
    }

    /// Every action index applied so far, oldest first. JSON `number[]`.
    pub fn history(&self) -> String {
        serde_json::to_string(&self.applied).expect("a list of numbers serializes")
    }

    /// The whole log, oldest line first.
    pub fn log(&self) -> String {
        serde_json::to_string(&self.state.log).expect("the log serializes")
    }

    /// Which seat must act next, `0` or `1`, or nothing once the game is over.
    pub fn player_to_act(&self) -> Option<u8> {
        player_to_act(&self.state).map(|player| player.index() as u8)
    }

    /// Whether the game has finished.
    pub fn is_over(&self) -> bool {
        self.state.is_over()
    }

    /// The board as the seat to act sees it, masked. JSON of [`WireView`].
    pub fn view(&self) -> String {
        let you = player_to_act(&self.state).unwrap_or(PlayerId::One);
        let wire = wire_view(&self.state.db, &PlayerView::of(&self.state, you));
        serde_json::to_string(&wire).expect("the wire view serializes")
    }
}

// The wire structs mirror `PlayerView`. They exist so the engine's own view
// types carry no `serde` derive: those types reach into `card.rs`, `state.rs`,
// and `ids.rs`, and a browser contract has no business there. See ADR 0096.

use serde::Serialize;
use sim::card::CardDb;

#[derive(Serialize)]
struct WireCard {
    id: usize,
    name: String,
    def: usize,
    /// TCGdex print id — the key card art is filed under.
    print_id: String,
    /// The Energy type this card provides, e.g. `"Fire"`, or `null` when
    /// the card is not an Energy — a Tool, say.
    energy_type: Option<String>,
    /// A coarse bucket for sorting a hand: `"pokemon"`, `"supporter"`,
    /// `"item"`, `"tool"`, `"stadium"`, `"special-energy"`, `"energy"`.
    category: String,
}

#[derive(Serialize)]
struct ActionMeta {
    kind: String,
    card: Option<usize>,
    target: Option<usize>,
    /// Enough to draw the face of the card this action names — set for
    /// any zone, so a deck-search prompt can show real art.
    card_face: Option<WireCardFace>,
}

#[derive(Serialize)]
struct WireCardFace {
    print_id: String,
    name: String,
    energy_type: Option<String>,
    category: String,
}

/// The `Action` variant's name, from its `Debug` form — the same trick
/// `phase_tag` uses. Exhaustive without a match to maintain.
fn action_kind(action: Action) -> String {
    let debug = format!("{action:?}");
    debug
        .split(|c: char| c.is_whitespace() || c == '{' || c == '(')
        .next()
        .unwrap_or("")
        .to_string()
}

/// The `CardId` and `PokemonId` an action names, as numbers. Wired for
/// the moves a player makes over the board on an ordinary turn; every
/// other variant returns `(None, None)` and stays label-only (ADR 0099).
fn action_handles(action: Action) -> (Option<usize>, Option<usize>) {
    let card = |c: sim::ids::CardId| Some(c.index());
    let mon = |p: sim::ids::PokemonId| Some(p.index());
    match action {
        Action::PlaceActive { card: c }
        | Action::PlaceOnBench { card: c }
        | Action::PlayBasic { card: c }
        | Action::PlayTrainer { card: c }
        | Action::TakeCard { card: c }
        | Action::DiscardEnergy { card: c }
        | Action::PayWithCard { card: c }
        | Action::MoveEnergyToActive { card: c }
        | Action::TakeBasicPokemonForCallForFamily { card: c }
        | Action::TakeBasicPokemonOfTypeForEnergyAttach { card: c }
        | Action::TakeItemFromLibrary { card: c }
        | Action::TakeAnyCardFromLibrary { card: c }
        | Action::TakeTrainerCardFromLibrary { card: c }
        | Action::TakePokemonOfTypeOrStadiumFromLibrary { card: c }
        | Action::TakeEvolutionPokemonOfType { card: c }
        | Action::TakeCardForFanCall { card: c }
        | Action::TakeFromBottomOfLibrary { card: c }
        | Action::TakeTrainerFromDiscard { card: c }
        | Action::TakePokemonFromDiscard { card: c }
        | Action::TakeNamedFromDiscardToBench { card: c }
        | Action::TakeCardFromTopPeek { card: c }
        | Action::TakeSupporterFromTopPeek { card: c }
        | Action::TakeSupporterForLastDitchCatch { card: c } => (card(c), None),
        Action::Evolve { card: c, target: t }
        | Action::AttachEnergy { card: c, target: t }
        | Action::PlayTool { card: c, target: t }
        | Action::TakeCardOnto { card: c, target: t }
        | Action::MoveEnergy { card: c, target: t } => (card(c), mon(t)),
        Action::Retreat { to } => (None, mon(to)),
        Action::Promote { pokemon } | Action::UseAbility { pokemon } => (None, mon(pokemon)),
        Action::HealTarget { target } | Action::HealMegaEx { target } => (None, mon(target)),
        _ => (None, None),
    }
}

#[derive(Serialize)]
struct WirePokemon {
    id: usize,
    name: String,
    print_id: String,
    hp: u32,
    damage: u32,
    remaining_hp: u32,
    conditions: Vec<String>,
    attached: Vec<WireCard>,
}

#[derive(Serialize)]
struct WireSide {
    player: u8,
    hand_count: usize,
    library_count: usize,
    prize_count: usize,
    discard: Vec<WireCard>,
    active: Option<WirePokemon>,
    bench: Vec<WirePokemon>,
}

#[derive(Serialize)]
struct WireView {
    you: u8,
    current: u8,
    turn_number: u32,
    phase: String,
    your_hand: Vec<WireCard>,
    /// The Stadium card in play, or null.
    stadium: Option<WireCard>,
    /// Every card in your own library, sorted, while you search the whole
    /// of it — null otherwise. The board dims the cards this search cannot
    /// take. See ADR 0101.
    library_in_search: Option<Vec<WireCard>>,
    sides: [WireSide; 2],
}

fn wire_card(db: &CardDb, card: &sim::view::CardView) -> WireCard {
    let def = db.get(card.def);
    WireCard {
        id: card.id.index(),
        name: card.name.to_string(),
        def: card.def.index(),
        print_id: card.print_id.to_string(),
        energy_type: def.as_energy().map(|energy| format!("{:?}", energy.kind)),
        category: card_category(def),
    }
}

/// The hand-sorting bucket for a card.
fn card_category(def: &sim::card::CardDef) -> String {
    use sim::card::{CardDef, TrainerKind};
    match def {
        CardDef::Pokemon(_) => "pokemon",
        CardDef::Energy(e) if e.effect.is_some() => "special-energy",
        CardDef::Energy(_) => "energy",
        CardDef::Trainer(t) => match t.kind {
            TrainerKind::Supporter => "supporter",
            TrainerKind::Item => "item",
            TrainerKind::Tool => "tool",
            TrainerKind::Stadium => "stadium",
        },
    }
    .to_string()
}

fn wire_pokemon(db: &CardDb, pokemon: &sim::view::PokemonView) -> WirePokemon {
    WirePokemon {
        id: pokemon.id.index(),
        name: pokemon.name.to_string(),
        print_id: pokemon.print_id.to_string(),
        hp: pokemon.hp,
        damage: pokemon.damage,
        remaining_hp: pokemon.remaining_hp,
        conditions: pokemon
            .conditions
            .iter()
            .map(|condition| format!("{condition:?}"))
            .collect(),
        attached: pokemon.attached.iter().map(|c| wire_card(db, c)).collect(),
    }
}

fn wire_side(db: &CardDb, side: &sim::view::SideView) -> WireSide {
    WireSide {
        player: side.player.index() as u8,
        hand_count: side.hand_count,
        library_count: side.library_count,
        prize_count: side.prize_count,
        discard: side.discard.iter().map(|c| wire_card(db, c)).collect(),
        active: side.active.as_ref().map(|p| wire_pokemon(db, p)),
        bench: side.bench.iter().map(|p| wire_pokemon(db, p)).collect(),
    }
}

fn wire_view(db: &CardDb, view: &PlayerView) -> WireView {
    WireView {
        you: view.you.index() as u8,
        current: view.current.index() as u8,
        turn_number: view.turn_number,
        phase: phase_tag(view),
        your_hand: view.your_hand.iter().map(|c| wire_card(db, c)).collect(),
        stadium: view.stadium.as_ref().map(|c| wire_card(db, c)),
        library_in_search: view
            .library_in_search
            .as_ref()
            .map(|cards| cards.iter().map(|c| wire_card(db, c)).collect()),
        sides: [
            wire_side(db, view.side(PlayerId::One)),
            wire_side(db, view.side(PlayerId::Two)),
        ],
    }
}

/// The phase's name without its fields — enough for the UI to say whose
/// decision it is and of what kind.
fn phase_tag(view: &PlayerView) -> String {
    let debug = format!("{:?}", view.phase);
    debug
        .split(|c: char| c.is_whitespace() || c == '(')
        .next()
        .unwrap_or("")
        .to_string()
}
