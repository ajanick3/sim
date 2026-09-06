//! The second batch of Trainers: the nine most-played cards the engine
//! refused after milestone 3, and the primitives each one needs.

use sim::action::{Action, legal_actions};
use sim::card::{
    Attack, CardDb, CardDef, CardFilter, Destination, Energy, Pokemon, Trainer, TrainerEffect,
    TrainerKind, Type, Zone,
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
            to: Destination::Zone(Zone::Hand),
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
            to: Destination::Zone(Zone::Hand),
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

// --- Ticket 01: a search that puts a Pokémon into play ---

/// `Buddy-Buddy Poffin` as printed: up to 2 small Basics, deck to Bench.
fn with_poffin(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let poffin = db.add(CardDef::Trainer(Trainer {
        print_id: "test-poffin",
        name: "Buddy-Buddy Poffin",
        kind: TrainerKind::Item,
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            to: Destination::Bench,
            filter: CardFilter::BasicPokemonWithHpAtMost(70),
            limit: 2,
            then: None,
        },
    }));
    (Set { db, ..set }, poffin)
}

#[test]
fn a_search_can_put_a_pokemon_into_play() {
    let (set, poffin) = with_poffin(build());
    let mut state = game(&set, poffin, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, poffin);
    let bench_before = state.player(player).bench.len();
    let hand_before = state.player(player).hand.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let take = offered(&state)[0];
    apply(&mut state, Action::TakeCard { card: take }).unwrap();

    assert_eq!(
        state.player(player).bench.len(),
        bench_before + 1,
        "the searched card is a Pokémon in play, not a card in a zone"
    );
    let benched = *state.player(player).bench.last().unwrap();
    assert_eq!(
        state.pokemon(benched).cards,
        vec![take],
        "the Pokémon in play is the card that was taken"
    );
    assert_eq!(
        state.pokemon(benched).played_on_turn,
        state.turn_number,
        "it came into play this turn, the same as a Basic from hand"
    );
    assert!(
        !state.player(player).hand.contains(&take),
        "and it never passed through the hand"
    );
    // The Item itself left the hand; nothing else joined it.
    assert_eq!(state.player(player).hand.len(), hand_before - 1);
    assert!(
        !state.player(player).library.contains(&take),
        "the card left the deck"
    );
}

#[test]
fn a_full_bench_offers_nothing_to_take() {
    let (set, poffin) = with_poffin(build());
    let mut state = game(&set, poffin, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, poffin);

    // Fill the Bench from the deck, the way an ordinary turn would.
    while state.player(player).bench.len() < sim::state::BENCH_LIMIT {
        let basic = *state
            .player(player)
            .library
            .iter()
            .find(|c| state.def_of(**c).is_basic_pokemon())
            .expect("the deck holds Basics");
        state.players[player.index()].library.retain(|c| *c != basic);
        let pokemon = state.put_into_play(player, basic);
        state.players[player.index()].bench.push(pokemon);
    }

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    assert!(
        offered(&state).is_empty(),
        "there is nowhere to put a Pokémon"
    );
    assert!(
        legal_actions(&state).contains(&Action::FinishDeciding),
        "and the choice can still be ended"
    );
    apply(&mut state, Action::FinishDeciding).unwrap();
    assert_eq!(state.phase, Phase::Main);
}

#[test]
fn a_bench_that_fills_part_way_through_ends_the_choice() {
    let (set, poffin) = with_poffin(build());
    let mut state = game(&set, poffin, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, poffin);

    // One space left, and the card offers two.
    while state.player(player).bench.len() < sim::state::BENCH_LIMIT - 1 {
        let basic = *state
            .player(player)
            .library
            .iter()
            .find(|c| state.def_of(**c).is_basic_pokemon())
            .expect("the deck holds Basics");
        state.players[player.index()].library.retain(|c| *c != basic);
        let pokemon = state.put_into_play(player, basic);
        state.players[player.index()].bench.push(pokemon);
    }

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let take = offered(&state)[0];
    apply(&mut state, Action::TakeCard { card: take }).unwrap();
    assert_eq!(state.player(player).bench.len(), sim::state::BENCH_LIMIT);
    assert!(
        offered(&state).is_empty(),
        "the second card has nowhere to go"
    );
}

#[test]
fn a_search_that_ends_in_the_library_still_shuffles() {
    // The destination is a value now. The rule that a card put back into the
    // deck shuffles it must still read the value and not a `Zone` field that
    // no longer exists.
    let set = build();
    let mut db = set.db.clone();
    let ash = db.add(CardDef::Trainer(Trainer {
        print_id: "test-ash",
        name: "Sacred Ash",
        kind: TrainerKind::Item,
        effect: TrainerEffect::Decide {
            from: Zone::Discard,
            to: Destination::Zone(Zone::Library),
            filter: CardFilter::AnyPokemon,
            limit: 5,
            then: None,
        },
    }));
    let set = Set { db, ..set };
    let mut state = game(&set, ash, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, ash);

    // Put a Pokémon in the discard for it to find.
    let mon = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.def_of(**c).as_pokemon().is_some())
        .unwrap();
    state.players[player.index()].library.retain(|c| *c != mon);
    state.players[player.index()].discard.push(mon);
    let order_before = state.player(player).library.clone();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::TakeCard { card: mon }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert!(state.player(player).library.contains(&mon));
    assert_ne!(
        state.player(player).library,
        order_before,
        "the deck is shuffled once the choice ends"
    );
}

// --- The card data ---

#[test]
fn buddy_buddy_poffin_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let poffin = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Buddy-Buddy Poffin")
        .expect("Buddy-Buddy Poffin plays");
    assert_eq!(
        poffin.effect,
        TrainerEffect::Decide {
            from: Zone::Library,
            to: Destination::Bench,
            filter: CardFilter::BasicPokemonWithHpAtMost(70),
            limit: 2,
            then: None,
        }
    );
}

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
            to: Destination::Zone(Zone::Hand),
            filter: CardFilter::PokemonEx,
            limit: 3,
            then: None,
        }
    );
}
