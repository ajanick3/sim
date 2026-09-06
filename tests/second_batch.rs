//! The second batch of Trainers: the nine most-played cards the engine
//! refused after milestone 3, and the primitives each one needs.

use sim::action::{Action, legal_actions};
use sim::card::{
    Attack, CardDb, CardDef, CardFilter, Destination, Energy, Pokemon, Requirement, Trainer,
    TrainerEffect, TrainerKind, Type, Zone,
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
        requirement: None,
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
        requirement: None,
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
        requirement: None,
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
        state.players[player.index()]
            .library
            .retain(|c| *c != basic);
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
        state.players[player.index()]
            .library
            .retain(|c| *c != basic);
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
        requirement: None,
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

// --- Ticket 03: a requirement paid to play a card ---

/// `Ultra Ball` and `Special Red Card` as printed.
fn with_requirements(set: Set) -> (Set, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let ultra_ball = db.add(CardDef::Trainer(Trainer {
        print_id: "test-ultra-ball",
        name: "Ultra Ball",
        kind: TrainerKind::Item,
        requirement: Some(Requirement::DiscardOtherCardsFromHand(2)),
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            to: Destination::Zone(Zone::Hand),
            filter: CardFilter::AnyPokemon,
            limit: 1,
            then: None,
        },
    }));
    let red_card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-special-red-card",
        name: "Special Red Card",
        kind: TrainerKind::Item,
        requirement: Some(Requirement::OpponentPrizesAtMost(3)),
        effect: TrainerEffect::OpponentHandToBottomThenDraw { count: 3 },
    }));
    (Set { db, ..set }, ultra_ball, red_card)
}

#[test]
fn ultra_ball_cannot_be_played_holding_nothing_else() {
    let (set, ultra_ball, _) = with_requirements(build());
    let mut state = game(&set, ultra_ball, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, ultra_ball);

    // Hold the card and one other: one short of the cost.
    let keep: Vec<CardId> = state
        .player(player)
        .hand
        .iter()
        .filter(|c| **c != card)
        .take(1)
        .copied()
        .collect();
    state.players[player.index()].hand = keep.clone();
    state.players[player.index()].hand.push(card);
    assert!(
        !legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "one other card does not pay for two"
    );

    // A second other card, and it may be played.
    let extra = state.players[player.index()].library.pop().unwrap();
    state.players[player.index()].hand.push(extra);
    assert!(
        legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "two other cards pay for it"
    );
}

#[test]
fn a_cost_is_paid_as_a_phase_before_the_effect_runs() {
    let (set, ultra_ball, _) = with_requirements(build());
    let mut state = game(&set, ultra_ball, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, ultra_ball);
    let hand_before = state.player(player).hand.len();
    let discard_before = state.player(player).discard.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    assert!(
        matches!(state.phase, Phase::Paying { .. }),
        "the cost comes first: {:?}",
        state.phase
    );
    // Every card still in hand may pay, and nothing else is offered.
    let paying = legal_actions(&state);
    assert!(!paying.is_empty());
    for action in &paying {
        assert!(
            matches!(action, Action::PayWithCard { .. }),
            "the cost is the only choice: {action:?}"
        );
    }

    for _ in 0..2 {
        let pay = match legal_actions(&state)[0] {
            Action::PayWithCard { card } => card,
            other => panic!("expected a payment: {other:?}"),
        };
        apply(&mut state, Action::PayWithCard { card: pay }).unwrap();
    }

    assert!(
        matches!(state.phase, Phase::Deciding { .. }),
        "the cost paid, the effect runs: {:?}",
        state.phase
    );
    // The Ultra Ball itself and the two cards that paid for it.
    assert_eq!(state.player(player).discard.len(), discard_before + 3);
    assert_eq!(state.player(player).hand.len(), hand_before - 3);
}

#[test]
fn special_red_card_waits_for_the_opponents_prizes() {
    let (set, _, red_card) = with_requirements(build());
    let mut state = game(&set, red_card, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, red_card);

    assert_eq!(state.player(player.opponent()).prizes.len(), 6);
    assert!(
        !legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "six Prizes is more than three"
    );

    state.players[player.opponent().index()].prizes.truncate(3);
    assert!(
        legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "three Prizes is not more than three"
    );
}

#[test]
fn special_red_card_puts_the_hand_under_the_deck_and_deals_three() {
    let (set, _, red_card) = with_requirements(build());
    let mut state = game(&set, red_card, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, red_card);
    state.players[player.opponent().index()].prizes.truncate(3);

    let opponent = player.opponent();
    let their_hand = state.player(opponent).hand.clone();
    let held = their_hand.len();
    assert!(held > 0, "the opponent holds a hand to lose");
    let library_before = state.player(opponent).library.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert_eq!(state.phase, Phase::Main, "no choice is left in the card");
    assert_eq!(
        state.player(opponent).hand.len(),
        3,
        "they draw three for the hand they gave up"
    );
    assert_eq!(
        state.player(opponent).library.len(),
        library_before + held - 3
    );
    // The bottom of the deck is where a draw reaches last.
    let bottom = &state.player(opponent).library[..held];
    for card in &their_hand {
        assert!(
            bottom.contains(card),
            "every card of the old hand went under the deck"
        );
    }
}

#[test]
fn a_card_with_no_requirement_still_plays() {
    let set = build();
    let mut state = game(&set, set.cyrano, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, set.cyrano);
    assert!(legal_actions(&state).contains(&Action::PlayTrainer { card }));
    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    assert!(
        matches!(state.phase, Phase::Deciding { .. }),
        "no cost stands between the card and its effect"
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
