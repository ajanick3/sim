//! Ticket 04: a knockout takes what the card is worth.

use sim::action::{Action, legal_actions};
use sim::card::{Attack, CardDb, CardDef, Energy, Pokemon, Stage, Type};
use sim::engine::apply;
use sim::import::load;
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

/// A game where the defender is worth `prizes` and dies to one attack.
fn game(prizes: u32) -> (GameState, u32) {
    let mut db = CardDb::new();
    let victim = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-target",
        name: "Target",
        hp: 30,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        prizes,
        attacks: vec![Attack {
            name: "Nudge",
            cost: vec![Type::Colorless],
            base_damage: 30,
            inflicts: None, effect: None,
        }],
    }));
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-colorless-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));

    let mut decklist = vec![victim; 12];
    while decklist.len() < 60 {
        decklist.push(energy);
    }
    let mut state = GameState::new(
        db,
        [decklist.clone(), decklist],
        Box::new(SeededRng::new(9)),
    );
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }

    // Pay for the attack.
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let side = state.player(player);
    let card = side
        .hand
        .iter()
        .chain(side.library.iter())
        .find(|c| state.def_of(**c).is_energy())
        .copied()
        .expect("the deck is mostly Energy");
    state.remove_from_hand(player, card);
    state.players[player.index()].library.retain(|c| *c != card);
    state.pokemon[active.index()].attached.push(card);

    let before = state.player(player).prizes.len() as u32;
    let attack = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("a paid-for Active can attack");
    apply(&mut state, attack).unwrap();
    let after = state.player(player).prizes.len() as u32;
    (state, before - after)
}

#[test]
fn a_knockout_takes_what_the_card_is_worth() {
    assert_eq!(game(1).1, 1, "an ordinary Pokémon is worth 1 Prize");
    assert_eq!(game(2).1, 2, "a Pokémon ex is worth 2");
    assert_eq!(game(3).1, 3, "a Mega Evolution ex is worth 3");
}

#[test]
fn the_import_reads_the_prize_value_from_the_name() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = load(&json).unwrap();

    // Ticket 03 admitted eight real Trainers too, so not every admitted card
    // is a Pokémon any more.
    let worth = |name: &str| {
        import
            .admitted
            .iter()
            .filter_map(|id| import.db.get(*id).as_pokemon())
            .find(|p| p.name == name)
            .map(|p| p.prizes)
    };

    assert_eq!(worth("Pikachu ex"), Some(2), "a card named ex is worth 2");
    assert!(
        import
            .admitted
            .iter()
            .filter_map(|id| import.db.get(*id).as_pokemon())
            .any(|p| !p.name.ends_with(" ex") && p.prizes == 1),
        "an ordinary card is worth 1"
    );
}

#[test]
fn taking_more_prizes_than_remain_still_wins() {
    let (mut state, _) = game(3);
    // Drive to the end from a board where a 3-Prize knockout can outrun the
    // Prizes left.
    state.players[state.current.index()].prizes.truncate(1);
    let player = state.current;
    let before = state.player(player).prizes.len();
    assert!(before <= 1);
    assert!(!state.is_over() || state.outcome.unwrap().winner == player);
}

#[test]
fn the_naming_convention_still_holds_in_the_data() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = load(&json).unwrap();

    let ex = import
        .cards
        .iter()
        .filter(|c| c.name.to_lowercase().ends_with(" ex"))
        .count();
    let mega = import
        .cards
        .iter()
        .filter(|c| c.name.starts_with("Mega ") && c.name.to_lowercase().ends_with(" ex"))
        .count();

    // ADR 0010 reads a prize value from the name. If a set breaks the naming
    // convention, these numbers move and the rule needs looking at again.
    assert_eq!(
        ex, 559,
        "cards named ex, Trainers excluded by the ' ex' ending"
    );
    assert_eq!(mega, 130, "Mega ex cards, worth 3 Prizes");
}
