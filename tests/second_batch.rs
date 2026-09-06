//! The second batch of Trainers: the nine most-played cards the engine
//! refused after milestone 3, and the primitives each one needs.

use sim::action::{Action, legal_actions};
use sim::card::{
    Attack, CardDb, CardDef, CardFilter, Energy, Pokemon, Trainer, TrainerEffect, TrainerKind,
    Type, Zone,
};
use sim::engine::apply;
use sim::ids::{CardDefId, CardId, PlayerId};
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

/// A pool wide enough to tell the new filters apart: an ordinary Basic, a
/// Basic worth two Prizes, a small Basic, and a Stage 1.
struct Set {
    db: CardDb,
    mon: CardDefId,
    mon_ex: CardDefId,
    small: CardDefId,
    stage1: CardDefId,
    energy: CardDefId,
    cyrano: CardDefId,
}

fn basic(
    db: &mut CardDb,
    print_id: &'static str,
    name: &'static str,
    hp: u32,
    prizes: u32,
    evolve_from: Option<&'static str>,
) -> CardDefId {
    db.add(CardDef::Pokemon(Pokemon {
        print_id,
        name,
        hp,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes,
        evolve_from,
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
        }],
    }))
}

fn build() -> Set {
    let mut db = CardDb::new();
    let mon = basic(&mut db, "test-mon", "Testmon", 100, 1, None);
    let mon_ex = basic(&mut db, "test-mon-ex", "Testmon ex", 200, 2, None);
    let small = basic(&mut db, "test-small", "Smallmon", 70, 1, None);
    let stage1 = basic(&mut db, "test-stage1", "Bigmon", 120, 1, Some("Smallmon"));
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
    }));
    let cyrano = db.add(CardDef::Trainer(Trainer {
        print_id: "test-cyrano",
        name: "Cyrano",
        kind: TrainerKind::Supporter,
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            to: Zone::Hand,
            filter: CardFilter::PokemonEx,
            limit: 3,
            then: None,
        },
    }));
    Set {
        db,
        mon,
        mon_ex,
        small,
        stage1,
        energy,
        cyrano,
    }
}

/// A deck holding a few of each card, then Energy to sixty.
fn deck(set: &Set, extra: CardDefId) -> Vec<CardDefId> {
    let mut decklist = vec![set.mon; 6];
    decklist.extend([set.mon_ex; 4]);
    decklist.extend([set.small; 4]);
    decklist.extend([set.stage1; 4]);
    decklist.push(extra);
    while decklist.len() < 60 {
        decklist.push(set.energy);
    }
    decklist
}

/// Play to the second player's first Main phase, so no turn-one rule is in
/// the way of playing a card.
fn game(set: &Set, extra: CardDefId, seed: u64) -> GameState {
    let decklist = deck(set, extra);
    let mut state = GameState::new(
        set.db.clone(),
        [decklist.clone(), decklist],
        Box::new(SeededRng::new(seed)),
    );
    for _ in 0..2 {
        while state.phase != Phase::Main && !state.is_over() {
            let first = legal_actions(&state)[0];
            apply(&mut state, first).unwrap();
        }
        if state.turn_number > 1 {
            break;
        }
        apply(&mut state, Action::EndTurn).unwrap();
    }
    state
}

/// Put a copy of `def` in this player's hand, taking it from the deck.
fn ensure_in_hand(state: &mut GameState, player: PlayerId, def: CardDefId) -> CardId {
    if let Some(card) = state
        .player(player)
        .hand
        .iter()
        .find(|c| state.cards[c.index()].def == def)
    {
        return *card;
    }
    // A single copy can be dealt into the Prizes, so every face-down zone is
    // searched, not only the deck.
    let side = state.player(player);
    let card = *side
        .library
        .iter()
        .chain(side.prizes.iter())
        .chain(side.discard.iter())
        .find(|c| state.cards[c.index()].def == def)
        .expect("the deal put this card somewhere face down");
    let side = &mut state.players[player.index()];
    side.library.retain(|c| *c != card);
    side.prizes.retain(|c| *c != card);
    side.discard.retain(|c| *c != card);
    side.hand.push(card);
    card
}

/// Every card the current `Deciding` phase offers.
fn offered(state: &GameState) -> Vec<CardId> {
    legal_actions(state)
        .into_iter()
        .filter_map(|a| match a {
            Action::TakeCard { card } => Some(card),
            _ => None,
        })
        .collect()
}

// --- Ticket 02: a filter on stage and HP ---

#[test]
fn the_ex_filter_offers_only_a_pokemon_worth_more_than_one_prize() {
    let set = build();
    let mut state = game(&set, set.cyrano, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, set.cyrano);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let choices = offered(&state);
    assert!(!choices.is_empty(), "the deck holds four Testmon ex");
    for card in &choices {
        let def = state.def_of(*card);
        let pokemon = def.as_pokemon().expect("only a Pokémon is offered");
        assert!(
            pokemon.prizes > 1,
            "{} is not a Pokémon ex and must not be offered",
            pokemon.name
        );
    }
}

#[test]
fn cyrano_takes_up_to_three_and_may_stop_early() {
    let set = build();
    let mut state = game(&set, set.cyrano, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, set.cyrano);
    let before = state.player(player).hand.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    for _ in 0..3 {
        let take = offered(&state)[0];
        apply(&mut state, Action::TakeCard { card: take }).unwrap();
    }
    assert!(
        offered(&state).is_empty(),
        "three is the limit the card prints"
    );
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.phase, Phase::Main);
    // Three taken, and the Supporter itself left the hand.
    assert_eq!(state.player(player).hand.len(), before + 2);
}

#[test]
fn the_small_basic_filter_reads_both_the_stage_and_the_hp() {
    let set = build();
    let mut db = set.db.clone();
    let poffin = db.add(CardDef::Trainer(Trainer {
        print_id: "test-poffin",
        name: "Poffin",
        kind: TrainerKind::Item,
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            to: Zone::Hand,
            filter: CardFilter::BasicPokemonWithHpAtMost(70),
            limit: 2,
            then: None,
        },
    }));
    let set = Set { db, ..set };
    let mut state = game(&set, poffin, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, poffin);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let choices = offered(&state);
    assert!(!choices.is_empty(), "the deck holds four Smallmon");
    for card in &choices {
        let pokemon = state
            .def_of(*card)
            .as_pokemon()
            .expect("only a Pokémon is offered")
            .clone();
        assert!(pokemon.hp <= 70, "{} has too much HP", pokemon.name);
        assert!(
            pokemon.evolve_from.is_none(),
            "{} is not a Basic",
            pokemon.name
        );
    }
}

// --- The card data ---

#[test]
fn cyrano_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let cyrano = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Cyrano")
        .expect("Cyrano plays");
    assert_eq!(
        cyrano.effect,
        TrainerEffect::Decide {
            from: Zone::Library,
            to: Zone::Hand,
            filter: CardFilter::PokemonEx,
            limit: 3,
            then: None,
        }
    );
}
