//! The browser boundary for the engine.
//!
//! `Game` owns a live [`GameState`]. JavaScript holds the handle for the life
//! of the game and calls methods on it; no game state crosses the boundary as
//! data. Every method returns a `String` holding JSON, so the seam also runs
//! under an ordinary `cargo test` on the host.

use sim::GameState;
use sim::action::{describe, legal_actions, player_to_act};
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

/// One game, held open across calls.
#[wasm_bindgen]
pub struct Game {
    state: GameState,
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
        Game { state }
    }

    /// A game of the Standard set. Each decklist is the text of a `.txt`
    /// deck file. Both must be fully playable.
    pub fn standard(
        data: &mut CardData,
        deck_a: &str,
        deck_b: &str,
        seed: u64,
    ) -> Result<Game, String> {
        let a = data.build_deck(deck_a)?;
        let b = data.build_deck(deck_b)?;
        let state = GameState::new(
            data.import.db.clone(),
            [a, b],
            Box::new(SeededRng::new(seed)),
        );
        Ok(Game { state })
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

    /// Apply the action at `index` in the current `legal_actions` list.
    pub fn apply(&mut self, index: usize) -> Result<(), String> {
        let actions = legal_actions(&self.state);
        let action = actions
            .get(index)
            .ok_or_else(|| format!("no legal action at index {index}"))?;
        apply(&mut self.state, *action).map_err(|error| format!("{error:?}"))
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
        let wire = wire_view(&PlayerView::of(&self.state, you));
        serde_json::to_string(&wire).expect("the wire view serializes")
    }
}

// The wire structs mirror `PlayerView`. They exist so the engine's own view
// types carry no `serde` derive: those types reach into `card.rs`, `state.rs`,
// and `ids.rs`, and a browser contract has no business there. See ADR 0096.

use serde::Serialize;

#[derive(Serialize)]
struct WireCard {
    id: usize,
    name: String,
    def: usize,
}

#[derive(Serialize)]
struct WirePokemon {
    name: String,
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
    sides: [WireSide; 2],
}

fn wire_card(card: &sim::view::CardView) -> WireCard {
    WireCard {
        id: card.id.index(),
        name: card.name.to_string(),
        def: card.def.index(),
    }
}

fn wire_pokemon(pokemon: &sim::view::PokemonView) -> WirePokemon {
    WirePokemon {
        name: pokemon.name.to_string(),
        hp: pokemon.hp,
        damage: pokemon.damage,
        remaining_hp: pokemon.remaining_hp,
        conditions: pokemon
            .conditions
            .iter()
            .map(|condition| format!("{condition:?}"))
            .collect(),
        attached: pokemon.attached.iter().map(wire_card).collect(),
    }
}

fn wire_side(side: &sim::view::SideView) -> WireSide {
    WireSide {
        player: side.player.index() as u8,
        hand_count: side.hand_count,
        library_count: side.library_count,
        prize_count: side.prize_count,
        discard: side.discard.iter().map(wire_card).collect(),
        active: side.active.as_ref().map(wire_pokemon),
        bench: side.bench.iter().map(wire_pokemon).collect(),
    }
}

fn wire_view(view: &PlayerView) -> WireView {
    WireView {
        you: view.you.index() as u8,
        current: view.current.index() as u8,
        turn_number: view.turn_number,
        phase: phase_tag(view),
        your_hand: view.your_hand.iter().map(wire_card).collect(),
        sides: [
            wire_side(view.side(PlayerId::One)),
            wire_side(view.side(PlayerId::Two)),
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
