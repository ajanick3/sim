//! Milestone 8: every Item in the field. One fixture, shared across this
//! milestone's tickets, the same way `supporters.rs` served Milestone 7.

use sim::action::{Action, legal_actions};
use sim::card::{
    Attack, CardDb, CardDef, CardFilter, Destination, Energy, Pokemon, Requirement, Slot, Stage,
    Marker, TargetFilter, Then, Trainer, TrainerEffect, TrainerKind, TurnBonusTarget, Type, Zone,
};
use sim::engine::apply;
use sim::ids::{CardDefId, CardId, PlayerId};
use sim::rng::SeededRng;
use sim::state::GameState;
use sim::state::Phase;

/// A pool wide enough for this milestone's filters: an ordinary Basic, a
/// Basic worth two Prizes, a Stage 1, a Stage 2, and a card with a Rule Box
/// but no evolution line (an ex Basic already covers that).
struct Set {
    db: CardDb,
    mon: CardDefId,
    mon_ex: CardDefId,
    stage1: CardDefId,
    stage2: CardDefId,
    energy: CardDefId,
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
        markers: Vec::new(),
        print_id,
        name,
        hp,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes,
        stage: match evolve_from {
            None => Stage::Basic,
            Some(_) => Stage::Stage1,
        },
        evolve_from,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None, effect: None,
        }],
    }))
}

fn build() -> Set {
    let mut db = CardDb::new();
    let mon = basic(&mut db, "test-mon", "Testmon", 100, 1, None);
    let mon_ex = basic(&mut db, "test-mon-ex", "Testmon ex", 200, 2, None);
    let stage1 = basic(&mut db, "test-stage1", "Bigmon", 120, 1, Some("Smallmon"));
    let stage2 = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-stage2",
        name: "Hugemon",
        hp: 180,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 2,
        prizes: 1,
        stage: Stage::Stage2,
        evolve_from: Some("Bigmon"),
        evolves_from_basic: Some("Smallmon"),
        ability: None,
        attacks: vec![Attack {
            name: "Slam",
            cost: vec![Type::Colorless, Type::Colorless],
            base_damage: 60,
            inflicts: None, effect: None,
        }],
    }));
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    Set {
        db,
        mon,
        mon_ex,
        stage1,
        stage2,
        energy,
    }
}

/// A deck holding a few of each card, then Energy to sixty.
fn deck(set: &Set, extra: CardDefId) -> Vec<CardDefId> {
    let mut decklist = vec![set.mon; 6];
    decklist.extend([set.mon_ex; 4]);
    decklist.extend([set.stage1; 4]);
    decklist.extend([set.stage2; 2]);
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

/// Put a copy of `def` in this player's hand, taking it from wherever the
/// deal put it.
fn ensure_in_hand(state: &mut GameState, player: PlayerId, def: CardDefId) -> CardId {
    if let Some(card) = state
        .player(player)
        .hand
        .iter()
        .find(|c| state.cards[c.index()].def == def)
    {
        return *card;
    }
    let side = state.player(player);
    let card = *side
        .deck
        .iter()
        .chain(side.prizes.iter())
        .chain(side.discard.iter())
        .find(|c| state.cards[c.index()].def == def)
        .expect("the deal put this card somewhere face down");
    let side = &mut state.players[player.index()];
    side.deck.retain(|c| *c != card);
    side.prizes.retain(|c| *c != card);
    side.discard.retain(|c| *c != card);
    side.hand.push(card);
    card
}

/// A physical card of `def`, for a definition that was never part of the
/// sixty-card decklist. Placed nowhere; the caller pushes it to whatever
/// zone the test needs.
fn deal_new_card(state: &mut GameState, player: PlayerId, def: CardDefId) -> CardId {
    let card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def, owner: player });
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

/// Put a copy of `def` into the discard pile, taking it from the deck.
fn deal_to_discard(state: &mut GameState, player: PlayerId, def: CardDefId) -> CardId {
    let card = *state
        .player(player)
        .deck
        .iter()
        .find(|c| state.cards[c.index()].def == def)
        .expect("the deck holds this card");
    state.players[player.index()].deck.retain(|c| *c != card);
    state.players[player.index()].discard.push(card);
    card
}

// --- Ticket 01: Energy Search & Energy Retrieval ---

fn with_energy_search(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-energy-search",
        name: "Energy Search",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Deck,
            slots: vec![Slot {
                filter: CardFilter::BasicEnergy,
                to: Destination::Zone(Zone::Hand),
                limit: 1,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    }));
    (Set { db, ..set }, card)
}

fn with_energy_retrieval(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-energy-retrieval",
        name: "Energy Retrieval",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Discard,
            slots: vec![Slot {
                filter: CardFilter::BasicEnergy,
                to: Destination::Zone(Zone::Hand),
                limit: 2,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    }));
    (Set { db, ..set }, card)
}

#[test]
fn energy_search_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Energy Search")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Energy Search should play");
}

#[test]
fn energy_search_finds_one_basic_energy_from_the_deck() {
    let (set, card) = with_energy_search(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    let choices = offered(&state);
    assert!(!choices.is_empty(), "the deck holds Basic Energy to find");
    let take = choices[0];
    apply(&mut state, Action::TakeCard { card: take }).unwrap();
    assert!(
        offered(&state).is_empty(),
        "one is the limit the card prints"
    );
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).hand.contains(&take));
}

#[test]
fn energy_retrieval_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Energy Retrieval")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Energy Retrieval should play");
}

#[test]
fn energy_retrieval_takes_up_to_two_from_the_discard() {
    let (set, card) = with_energy_retrieval(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    let e1 = deal_to_discard(&mut state, player, set.energy);
    let e2 = deal_to_discard(&mut state, player, set.energy);

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    apply(&mut state, Action::TakeCard { card: e1 }).unwrap();
    apply(&mut state, Action::TakeCard { card: e2 }).unwrap();
    assert!(
        offered(&state).is_empty(),
        "two is the limit the card prints"
    );
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).hand.contains(&e1));
    assert!(state.player(player).hand.contains(&e2));
}

// --- Ticket 02: Energy Recycler ---

fn with_energy_recycler(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-energy-recycler",
        name: "Energy Recycler",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Discard,
            slots: vec![Slot {
                filter: CardFilter::BasicEnergy,
                to: Destination::Zone(Zone::Deck),
                limit: 5,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    }));
    (Set { db, ..set }, card)
}

#[test]
fn energy_recycler_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Energy Recycler")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Energy Recycler should play");
}

#[test]
fn energy_recycler_shuffles_up_to_five_from_discard_into_the_deck() {
    let (set, card) = with_energy_recycler(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    let e1 = deal_to_discard(&mut state, player, set.energy);
    let e2 = deal_to_discard(&mut state, player, set.energy);
    let deck_before = state.player(player).deck.len();

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    apply(&mut state, Action::TakeCard { card: e1 }).unwrap();
    apply(&mut state, Action::TakeCard { card: e2 }).unwrap();
    assert!(
        !state.player(player).discard.contains(&e1) && !state.player(player).discard.contains(&e2),
        "both energy left the discard"
    );
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.player(player).deck.len(), deck_before + 2);
    assert!(state.player(player).deck.contains(&e1));
    assert!(state.player(player).deck.contains(&e2));
}

// --- Ticket 03: Team Rocket's Transceiver ---

fn with_team_rockets_transceiver(set: Set) -> (Set, CardDefId, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let petrel = db.add(CardDef::Trainer(Trainer {
        print_id: "test-tr-petrel",
        name: "Team Rocket's Petrel",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::Nothing,
    }));
    let ordinary_supporter = db.add(CardDef::Trainer(Trainer {
        print_id: "test-ordinary-supporter",
        name: "Ordinary Supporter",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::Nothing,
    }));
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-tr-transceiver",
        name: "Team Rocket's Transceiver",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Deck,
            slots: vec![Slot {
                filter: CardFilter::SupporterNameContains("Team Rocket"),
                to: Destination::Zone(Zone::Hand),
                limit: 1,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    }));
    (Set { db, ..set }, card, petrel, ordinary_supporter)
}

#[test]
fn team_rockets_transceiver_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Team Rocket's Transceiver")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Team Rocket's Transceiver should play");
}

#[test]
fn team_rockets_transceiver_finds_only_a_team_rocket_supporter() {
    let (set, card, petrel, ordinary) = with_team_rockets_transceiver(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    let petrel_card = deal_new_card(&mut state, player, petrel);
    state.players[player.index()].deck.push(petrel_card);
    let ordinary_card = deal_new_card(&mut state, player, ordinary);
    state.players[player.index()].deck.push(ordinary_card);

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    let choices = offered(&state);
    assert!(choices.contains(&petrel_card), "the Team Rocket name matches");
    assert!(
        !choices.contains(&ordinary_card),
        "a Supporter without the name is not offered"
    );
}

// --- Ticket 05: Hand Trimmer ---

fn with_hand_trimmer(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-hand-trimmer",
        name: "Hand Trimmer",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::BothDiscardDownTo(5),
    }));
    (Set { db, ..set }, card)
}

#[test]
fn hand_trimmer_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Hand Trimmer")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Hand Trimmer should play");
}

#[test]
fn hand_trimmer_trims_the_opponent_first_then_the_player() {
    let (set, card) = with_hand_trimmer(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let opponent = player.opponent();
    let played = ensure_in_hand(&mut state, player, card);

    // Both hands well past 5.
    for _ in 0..8 {
        ensure_in_hand(&mut state, player, set.mon);
        ensure_in_hand(&mut state, opponent, set.mon);
    }
    let opponent_hand_before = state.player(opponent).hand.len();
    let player_hand_before = state.player(player).hand.len();
    assert!(opponent_hand_before > 5 && player_hand_before > 5);

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    assert_eq!(
        sim::action::player_to_act(&state),
        Some(opponent),
        "the opponent discards first"
    );
    while let Phase::DiscardingFromHand { chooser, .. } = state.phase {
        if chooser != opponent {
            break;
        }
        let discard = legal_actions(&state)
            .into_iter()
            .find_map(|a| match a {
                Action::DiscardFromHand { card } => Some(card),
                _ => None,
            });
        match discard {
            Some(card) => apply(&mut state, Action::DiscardFromHand { card }).unwrap(),
            None => apply(&mut state, Action::FinishDiscardingFromHand).unwrap(),
        }
    }
    assert_eq!(state.player(opponent).hand.len(), 5);
    assert_eq!(
        sim::action::player_to_act(&state),
        Some(player),
        "the player trims their own hand next"
    );
    while state.phase != Phase::Main {
        let discard = legal_actions(&state)
            .into_iter()
            .find_map(|a| match a {
                Action::DiscardFromHand { card } => Some(card),
                _ => None,
            });
        match discard {
            Some(card) => apply(&mut state, Action::DiscardFromHand { card }).unwrap(),
            None => apply(&mut state, Action::FinishDiscardingFromHand).unwrap(),
        }
    }
    assert_eq!(state.player(player).hand.len(), 5);
}

// --- Ticket 06: Secret Box ---

fn with_secret_box(set: Set) -> (Set, CardDefId, CardDefId, CardDefId, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let an_item = db.add(CardDef::Trainer(Trainer {
        print_id: "test-an-item",
        name: "An Item",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::Nothing,
    }));
    let a_tool = db.add(CardDef::Trainer(Trainer {
        print_id: "test-a-tool",
        name: "A Tool",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::Nothing,
    }));
    let a_supporter = db.add(CardDef::Trainer(Trainer {
        print_id: "test-a-supporter",
        name: "A Supporter",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::Nothing,
    }));
    let a_stadium = db.add(CardDef::Trainer(Trainer {
        print_id: "test-a-stadium",
        name: "A Stadium",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::Nothing,
    }));
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-secret-box",
        name: "Secret Box",
        kind: TrainerKind::Item,
        requirement: Some(Requirement::DiscardOtherCardsFromHand(3)),
        effect: TrainerEffect::Decide {
            from: Zone::Deck,
            slots: vec![
                Slot {
                    filter: CardFilter::TrainerOfKind(TrainerKind::Item),
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                },
                Slot {
                    filter: CardFilter::TrainerOfKind(TrainerKind::Tool),
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                },
                Slot {
                    filter: CardFilter::TrainerOfKind(TrainerKind::Supporter),
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                },
                Slot {
                    filter: CardFilter::TrainerOfKind(TrainerKind::Stadium),
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                },
            ],
            then: None,
        },
    }));
    (Set { db, ..set }, card, an_item, a_tool, a_supporter, a_stadium)
}

#[test]
fn secret_box_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Secret Box")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Secret Box should play");
}

#[test]
fn secret_box_cannot_be_played_without_three_other_cards_to_discard() {
    let (set, card, ..) = with_secret_box(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    state.players[player.index()].hand.retain(|c| *c == played);

    assert!(
        !legal_actions(&state).contains(&Action::PlayTrainer { card: played }),
        "no other cards in hand to pay the cost"
    );
}

#[test]
fn secret_box_pays_its_cost_then_finds_one_of_each_kind() {
    let (set, card, an_item, a_tool, a_supporter, a_stadium) = with_secret_box(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    for _ in 0..3 {
        ensure_in_hand(&mut state, player, set.mon);
    }
    for def in [an_item, a_tool, a_supporter, a_stadium] {
        let placed = deal_new_card(&mut state, player, def);
        state.players[player.index()].deck.push(placed);
    }

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    assert!(matches!(state.phase, Phase::Paying { .. }));
    for _ in 0..3 {
        let pay = match legal_actions(&state)[0] {
            Action::PayWithCard { card } => card,
            other => panic!("expected a payment: {other:?}"),
        };
        apply(&mut state, Action::PayWithCard { card: pay }).unwrap();
    }
    assert!(
        matches!(state.phase, Phase::Deciding { .. }),
        "the cost paid, the search runs: {:?}",
        state.phase
    );

    for expect in [an_item, a_tool, a_supporter, a_stadium] {
        let found = offered(&state)[0];
        assert_eq!(
            state.cards[found.index()].def, expect,
            "each slot offers only its own kind"
        );
        apply(&mut state, Action::TakeCard { card: found }).unwrap();
        apply(&mut state, Action::FinishDeciding).unwrap();
    }

    assert_eq!(state.phase, Phase::Main);
    for def in [an_item, a_tool, a_supporter, a_stadium] {
        assert!(
            state
                .player(player)
                .hand
                .iter()
                .any(|c| state.cards[c.index()].def == def),
            "one of each kind was found"
        );
    }
}

#[test]
fn a_whole_deck_search_shows_the_searcher_every_card_in_it() {
    use sim::view::PlayerView;

    let (set, card, an_item, a_tool, a_supporter, a_stadium) = with_secret_box(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    for _ in 0..3 {
        ensure_in_hand(&mut state, player, set.mon);
    }
    for def in [an_item, a_tool, a_supporter, a_stadium] {
        let placed = deal_new_card(&mut state, player, def);
        state.players[player.index()].deck.push(placed);
    }

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    for _ in 0..3 {
        let pay = match legal_actions(&state)[0] {
            Action::PayWithCard { card } => card,
            other => panic!("expected a payment: {other:?}"),
        };
        apply(&mut state, Action::PayWithCard { card: pay }).unwrap();
    }
    assert!(matches!(state.phase, Phase::Deciding { .. }), "the search runs");

    let mine = PlayerView::of(&state, player);
    let shown = mine
        .deck_in_search
        .expect("the searcher sees the whole deck");
    let shown_ids: std::collections::HashSet<_> = shown.iter().map(|c| c.id).collect();
    let deck_ids: std::collections::HashSet<_> =
        state.player(player).deck.iter().copied().collect();
    assert_eq!(shown_ids, deck_ids, "every deck card is shown, no more");

    assert!(
        PlayerView::of(&state, player.opponent())
            .deck_in_search
            .is_none(),
        "the opponent never sees the searcher's deck"
    );
}

// --- Ticket 08: Dusk Ball ---

fn with_dusk_ball(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-dusk-ball",
        name: "Dusk Ball",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::LookAtBottomOfDeck { count: 7 },
    }));
    (Set { db, ..set }, card)
}

#[test]
fn dusk_ball_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Dusk Ball")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Dusk Ball should play");
}

#[test]
fn dusk_ball_offers_only_a_pokemon_from_the_bottom_seven() {
    let (set, card) = with_dusk_ball(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    // Put a known Pokémon at the very bottom (index 0) of the deck.
    let bottom_mon = deal_new_card(&mut state, player, set.mon);
    state.players[player.index()].deck.insert(0, bottom_mon);
    // And a known Pokémon far from the bottom, outside the 7-card window.
    let deep_mon = deal_new_card(&mut state, player, set.mon);
    let deep_index = state.player(player).deck.len() / 2;
    state.players[player.index()].deck.insert(deep_index, deep_mon);

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    let choices: Vec<CardId> = legal_actions(&state)
        .into_iter()
        .filter_map(|a| match a {
            Action::TakeFromBottomOfDeck { card } => Some(card),
            _ => None,
        })
        .collect();
    assert!(choices.contains(&bottom_mon), "the bottom card is in range");
    assert!(
        !choices.contains(&deep_mon),
        "a card outside the bottom 7 is not offered"
    );
    assert!(
        legal_actions(&state).contains(&Action::DeclineBottomOfDeck),
        "declining is always offered"
    );
}

#[test]
fn dusk_ball_takes_the_pokemon_and_shuffles_the_rest_back() {
    let (set, card) = with_dusk_ball(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    let bottom_mon = deal_new_card(&mut state, player, set.mon);
    state.players[player.index()].deck.insert(0, bottom_mon);
    let deck_before = state.player(player).deck.len();

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    apply(&mut state, Action::TakeFromBottomOfDeck { card: bottom_mon }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).hand.contains(&bottom_mon));
    assert_eq!(state.player(player).deck.len(), deck_before - 1);
}

// --- Ticket 09: Prime Catcher ---

fn with_prime_catcher(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-prime-catcher",
        name: "Prime Catcher",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::SwitchOpponentActiveThenOwn,
    }));
    (Set { db, ..set }, card)
}

#[test]
fn prime_catcher_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Prime Catcher")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Prime Catcher should play");
}

#[test]
fn prime_catcher_switches_both_sides_and_displaces_neither() {
    let (set, card) = with_prime_catcher(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let opponent = player.opponent();
    let played = ensure_in_hand(&mut state, player, card);
    let opponent_active_before = state.player(opponent).active.unwrap();
    let opponent_bench_target = state.player(opponent).bench[0];
    let player_active_before = state.player(player).active.unwrap();
    let player_bench_target = state.player(player).bench[0];

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    assert_eq!(
        sim::action::player_to_act(&state),
        Some(player),
        "the player chooses the opponent's new Active"
    );
    apply(&mut state, Action::Promote { pokemon: opponent_bench_target }).unwrap();
    assert_eq!(state.player(opponent).active, Some(opponent_bench_target));
    assert!(
        state.player(opponent).bench.contains(&opponent_active_before),
        "the opponent's old Active is displaced, not lost"
    );

    // The follow-up: the player's own switch.
    assert!(matches!(state.phase, Phase::Promoting { .. }));
    apply(&mut state, Action::Promote { pokemon: player_bench_target }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.player(player).active, Some(player_bench_target));
    assert!(
        state.player(player).bench.contains(&player_active_before),
        "the player's own old Active is displaced, not lost"
    );
}

#[test]
fn prime_catcher_skips_its_own_switch_with_an_empty_bench() {
    let (set, card) = with_prime_catcher(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let opponent = player.opponent();
    let played = ensure_in_hand(&mut state, player, card);
    let opponent_bench_target = state.player(opponent).bench[0];
    state.players[player.index()].bench.clear();

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    apply(&mut state, Action::Promote { pokemon: opponent_bench_target }).unwrap();

    assert_eq!(
        state.phase,
        Phase::Main,
        "no Bench to switch to, so the follow-up does nothing"
    );
}

// --- Ticket 10: Strange Timepiece ---

fn with_strange_timepiece(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-strange-timepiece",
        name: "Strange Timepiece",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::DevolveChosen,
    }));
    (Set { db, ..set }, card)
}

#[test]
fn strange_timepiece_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Strange Timepiece")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Strange Timepiece should play");
}

#[test]
fn strange_timepiece_cannot_be_played_with_nothing_evolved() {
    let (set, card) = with_strange_timepiece(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    // Nothing in play is evolved in this fixture by default.

    assert!(
        !legal_actions(&state).contains(&Action::PlayTrainer { card: played }),
        "no evolved Pokémon to devolve"
    );
}

#[test]
fn strange_timepiece_devolves_one_layer_and_blocks_evolving_this_turn() {
    let (set, card) = with_strange_timepiece(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    let stage1 = state.player(player).bench[0];
    let evolution_card = deal_new_card(&mut state, player, set.stage1);
    let basic_underneath = state.pokemon(stage1).cards[0];
    state.pokemon[stage1.index()].cards = vec![basic_underneath, evolution_card];

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    apply(&mut state, Action::ChooseDevolveTarget { target: stage1 }).unwrap();
    assert!(
        legal_actions(&state).contains(&Action::RemoveOneEvolutionCard),
        "one evolution card sits on top"
    );
    apply(&mut state, Action::RemoveOneEvolutionCard).unwrap();
    assert!(
        !legal_actions(&state).contains(&Action::RemoveOneEvolutionCard),
        "back to the Basic, nothing left to remove"
    );
    apply(&mut state, Action::FinishDevolving).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(stage1).cards, vec![basic_underneath]);
    assert!(state.player(player).hand.contains(&evolution_card));
    assert!(state.pokemon(stage1).cannot_evolve_this_turn);
}

// --- Ticket 11: Transformation Tome ---

fn with_transformation_tome(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-transformation-tome",
        name: "Transformation Tome",
        kind: TrainerKind::Item,
        requirement: Some(Requirement::SecondCopyOfThisInHand),
        effect: TrainerEffect::SwapBasicWithDiscard,
    }));
    (Set { db, ..set }, card)
}

#[test]
fn transformation_tome_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Transformation Tome")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Transformation Tome should play");
}

#[test]
fn transformation_tome_cannot_be_played_with_only_one_copy() {
    let (set, card) = with_transformation_tome(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    // Remove any second copy the deal might have dealt into hand.
    state.players[player.index()].hand.retain(|c| *c == played);

    assert!(
        !legal_actions(&state).contains(&Action::PlayTrainer { card: played }),
        "one copy cannot pay for two"
    );
}

#[test]
fn transformation_tome_consumes_its_second_copy_and_swaps_in_the_discard() {
    let (set, card) = with_transformation_tome(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    let second_copy = deal_new_card(&mut state, player, card);
    state.players[player.index()].hand.push(second_copy);

    let target = state.player(player).active.unwrap();
    // Give the target some damage, an attached Energy, and a played-on
    // turn to check they survive the swap.
    state.pokemon[target.index()].damage = 30;
    let attached = deal_new_card(&mut state, player, set.energy);
    state.pokemon[target.index()].attached.push(attached);
    let old_card = state.pokemon(target).cards[0];

    let replacement = deal_new_card(&mut state, player, set.mon);
    state.players[player.index()].discard.push(replacement);

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    assert!(
        !state.player(player).hand.contains(&second_copy),
        "the second copy is consumed as the cost"
    );
    assert!(state.player(player).discard.contains(&second_copy));

    apply(&mut state, Action::ChooseIdentitySwapTarget { target }).unwrap();
    apply(&mut state, Action::SwapIdentityWithDiscarded { card: replacement }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(target).cards, vec![replacement]);
    assert!(state.player(player).discard.contains(&old_card));
    assert!(!state.player(player).discard.contains(&replacement));
    // Damage and attachments stayed on the same Pokémon in play.
    assert_eq!(state.pokemon(target).damage, 30);
    assert!(state.pokemon(target).attached.contains(&attached));
}

// --- Beyond the milestone's own ticket order: Tool Scrapper ---

#[test]
fn discards_up_to_two_tools_anywhere_in_play() {
    let set = build();
    let mut state = game(&set, set.mon, 3);
    let player = state.current;
    let opponent = player.opponent();

    let tool_def = state.db.add(CardDef::Trainer(Trainer {
        print_id: "test-tool",
        name: "Test Tool",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::MoveAttachedEnergy,
    }));
    let own_active = state.player(player).active.unwrap();
    let own_tool = deal_new_card(&mut state, player, tool_def);
    state.pokemon[own_active.index()].attached.push(own_tool);

    let opp_active = state.player(opponent).active.unwrap();
    let opp_tool = deal_new_card(&mut state, opponent, tool_def);
    state.pokemon[opp_active.index()].attached.push(opp_tool);

    let scrapper_def = state.db.add(CardDef::Trainer(Trainer {
        print_id: "test-scrapper",
        name: "Tool Scrapper",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::MayDiscardUpToTwoToolsAnywhere,
    }));
    let scrapper = deal_new_card(&mut state, player, scrapper_def);
    state.players[player.index()].hand.push(scrapper);

    apply(&mut state, Action::PlayTrainer { card: scrapper }).unwrap();

    assert!(matches!(state.phase, Phase::DiscardingToolsAnywhere { .. }));
    apply(&mut state, Action::DiscardToolAnywhere { card: own_tool }).unwrap();
    apply(&mut state, Action::DiscardToolAnywhere { card: opp_tool }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).discard.contains(&own_tool));
    assert!(state.player(opponent).discard.contains(&opp_tool));
    assert!(!state.pokemon(own_active).attached.contains(&own_tool));
    assert!(!state.pokemon(opp_active).attached.contains(&opp_tool));
}

#[test]
fn tool_scrapper_can_be_declined_early() {
    let set = build();
    let mut state = game(&set, set.mon, 3);
    let player = state.current;

    let tool_def = state.db.add(CardDef::Trainer(Trainer {
        print_id: "test-tool",
        name: "Test Tool",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::MoveAttachedEnergy,
    }));
    let own_active = state.player(player).active.unwrap();
    let own_tool = deal_new_card(&mut state, player, tool_def);
    state.pokemon[own_active.index()].attached.push(own_tool);

    let scrapper_def = state.db.add(CardDef::Trainer(Trainer {
        print_id: "test-scrapper",
        name: "Tool Scrapper",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::MayDiscardUpToTwoToolsAnywhere,
    }));
    let scrapper = deal_new_card(&mut state, player, scrapper_def);
    state.players[player.index()].hand.push(scrapper);

    apply(&mut state, Action::PlayTrainer { card: scrapper }).unwrap();
    apply(&mut state, Action::FinishDiscardingToolsAnywhere).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.pokemon(own_active).attached.contains(&own_tool), "declined, nothing discarded");
}

#[test]
fn tool_scrapper_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Tool Scrapper" && c.playable.is_some()),
        "Tool Scrapper should play"
    );
}

// --- Beyond the map: search the deck for a Tera Pokemon ---

#[test]
fn tera_orb_finds_a_tera_pokemon_but_not_a_plain_one() {
    let set = build();
    let mut db = set.db.clone();
    let tera_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: vec![sim::card::Marker::Ex, sim::card::Marker::Tera],
        print_id: "test-tera-mon",
        name: "Testmon ex Tera",
        hp: 200,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 2,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let tera_orb_def = db.add(CardDef::Trainer(Trainer {
        print_id: "test-tera-orb",
        name: "Tera Orb",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Deck,
            slots: vec![Slot {
                filter: CardFilter::TeraPokemon,
                to: Destination::Zone(Zone::Hand),
                limit: 1,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    }));
    let set = Set { db, ..set };

    let mut state = game(&set, tera_orb_def, 3);
    let player = state.current;
    let tera_card = deal_new_card(&mut state, player, tera_mon);
    state.players[player.index()].deck.push(tera_card);
    let card = ensure_in_hand(&mut state, player, tera_orb_def);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    assert!(matches!(state.phase, Phase::Deciding { .. }));

    let plain_card = *state
        .player(player)
        .deck
        .iter()
        .find(|c| state.cards[c.index()].def == set.mon)
        .expect("the deck holds a plain Testmon");
    let actions = legal_actions(&state);
    assert!(actions.contains(&Action::TakeCard { card: tera_card }));
    assert!(
        !actions.contains(&Action::TakeCard { card: plain_card }),
        "a plain (non-Tera) Pokemon in the deck is not offered"
    );

    apply(&mut state, Action::TakeCard { card: tera_card }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).hand.contains(&tera_card));
}

#[test]
fn tera_orb_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Tera Orb" && c.playable.is_some()),
        "Tera Orb should play"
    );
}

// --- Beyond the field: heal Items ---

fn with_heal_items(set: Set) -> (Set, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let potion = db.add(CardDef::Trainer(Trainer {
        print_id: "test-potion",
        name: "Potion",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::HealChosenPlain { amount: 30, of_type: None },
    }));
    let vital = db.add(CardDef::Trainer(Trainer {
        print_id: "test-poke-vital-a",
        name: "Poké Vital A",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::HealChosenPlain { amount: 150, of_type: None },
    }));
    (Set { db, ..set }, potion, vital)
}

#[test]
fn potion_heals_thirty_from_a_chosen_pokemon_and_leaves_conditions() {
    let (set, potion, _v) = with_heal_items(build());
    let mut state = game(&set, potion, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    state.pokemon[active.index()].damage = 50;
    state.inflict(active, sim::card::Condition::Poisoned);
    let card = ensure_in_hand(&mut state, player, potion);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::HealTarget { target: active }).unwrap();

    assert_eq!(state.pokemon(active).damage, 20);
    assert!(
        state.pokemon(active).conditions.contains(&sim::card::Condition::Poisoned),
        "Potion does not clear conditions",
    );
    assert_eq!(state.phase, Phase::Main);
}

#[test]
fn poke_vital_a_heals_one_hundred_fifty() {
    let (set, _p, vital) = with_heal_items(build());
    let mut state = game(&set, vital, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    state.pokemon[active.index()].damage = 90;
    let card = ensure_in_hand(&mut state, player, vital);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::HealTarget { target: active }).unwrap();

    assert_eq!(state.pokemon(active).damage, 0);
}

#[test]
fn the_heal_items_are_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    for name in ["Potion", "Poké Vital A", "Jacinthe"] {
        assert!(
            import.cards.iter().any(|c| c.name == name && c.playable.is_some()),
            "{name} should play",
        );
    }
}

// --- Beyond the field: deck-search and discard-retrieval Items ---

fn search_item(
    db: &mut CardDb,
    print_id: &'static str,
    name: &'static str,
    from: Zone,
    filter: CardFilter,
    limit: u32,
    then: Option<Then>,
) -> CardDefId {
    db.add(CardDef::Trainer(Trainer {
        print_id,
        name,
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::Decide {
            from,
            slots: vec![Slot {
                filter,
                to: Destination::Zone(Zone::Hand),
                limit,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then,
        },
    }))
}

#[test]
fn master_ball_pulls_a_pokemon_from_the_deck() {
    let mut set = build();
    let master_ball = search_item(
        &mut set.db,
        "test-master-ball",
        "Master Ball",
        Zone::Deck,
        CardFilter::AnyPokemon,
        1,
        None,
    );
    let mut state = game(&set, master_ball, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, master_ball);
    let before = state.player(player).hand.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let take = legal_actions(&state)
        .into_iter()
        .find_map(|a| match a {
            Action::TakeCard { card } => Some(card),
            _ => None,
        })
        .expect("a Pokemon is in the deck");
    apply(&mut state, Action::TakeCard { card: take }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.player(player).hand.len(), before - 1 + 1);
    assert!(state.def_of(take).as_pokemon().is_some());
}

#[test]
fn boxed_order_ends_the_turn_after_the_search() {
    let mut set = build();
    let boxed_order = search_item(
        &mut set.db,
        "test-boxed-order",
        "Boxed Order",
        Zone::Deck,
        CardFilter::TrainerOfKind(TrainerKind::Item),
        2,
        Some(Then::EndTurnIfMoved),
    );
    // Put a couple of Items in the deck to find.
    let mut state = game(&set, boxed_order, 3);
    let player = state.current;
    for _ in 0..2 {
        let c = deal_new_card(&mut state, player, boxed_order);
        state.players[player.index()].deck.push(c);
    }
    let card = ensure_in_hand(&mut state, player, boxed_order);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    while let Some(t) = legal_actions(&state).into_iter().find_map(|a| match a {
        Action::TakeCard { card } => Some(card),
        _ => None,
    }) {
        apply(&mut state, Action::TakeCard { card: t }).unwrap();
    }
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_ne!(state.current, player, "the turn ended");
}

#[test]
fn the_search_items_are_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    for name in [
        "Master Ball",
        "Hyper Aroma",
        "Treasure Tracker",
        "Boxed Order",
        "Max Rod",
        "Miracle Headset",
    ] {
        assert!(
            import.cards.iter().any(|c| c.name == name && c.playable.is_some()),
            "{name} should play",
        );
    }
}

#[test]
fn precious_trolley_puts_basics_from_the_deck_onto_the_bench() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Precious Trolley" && c.playable.is_some()),
        "Precious Trolley should play"
    );

    let mut set = build();
    let trolley = set.db.add(CardDef::Trainer(Trainer {
        print_id: "test-precious-trolley",
        name: "Precious Trolley",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Deck,
            slots: vec![Slot {
                filter: CardFilter::PokemonOfStage(Stage::Basic),
                to: Destination::Bench,
                limit: 5,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    }));
    let mut state = game(&set, trolley, 3);
    let player = state.current;
    let bench_before = state.player(player).bench.len();
    let card = ensure_in_hand(&mut state, player, trolley);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    for _ in 0..2 {
        let take = offered(&state)[0];
        apply(&mut state, Action::TakeCard { card: take }).unwrap();
    }
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.player(player).bench.len(), bench_before + 2, "two Basics joined the Bench");
}

#[test]
fn energy_search_pro_takes_one_basic_energy_per_type() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Energy Search Pro" && c.playable.is_some()),
        "Energy Search Pro should play"
    );

    let mut set = build();
    let pro = set.db.add(CardDef::Trainer(Trainer {
        print_id: "test-energy-search-pro",
        name: "Energy Search Pro",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Deck,
            slots: vec![Slot {
                filter: CardFilter::BasicEnergy,
                to: Destination::Zone(Zone::Hand),
                limit: 9,
                excludes_type_of_previous: true,
                peek: None,
            }],
            then: None,
        },
    }));
    let mut state = game(&set, pro, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, pro);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let first = offered(&state)[0];
    apply(&mut state, Action::TakeCard { card: first }).unwrap();
    assert!(
        offered(&state).is_empty(),
        "the deck only holds one Basic Energy type, so no second pick is offered"
    );
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert!(state.player(player).hand.contains(&first));
}

#[test]
fn mega_signal_pulls_a_mega_pokemon_from_the_deck() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Mega Signal" && c.playable.is_some()),
        "Mega Signal should play"
    );

    let mut set = build();
    let mega = set.db.add(CardDef::Pokemon(Pokemon {
        markers: vec![Marker::Mega, Marker::Ex],
        print_id: "test-mega-mon-ex",
        name: "Megamon ex",
        hp: 330,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 2,
        prizes: 3,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![],
    }));
    let signal = search_item(
        &mut set.db,
        "test-mega-signal",
        "Mega Signal",
        Zone::Deck,
        CardFilter::MegaPokemon,
        1,
        None,
    );
    let mut state = game(&set, signal, 3);
    let player = state.current;
    let in_deck = deal_new_card(&mut state, player, mega);
    state.players[player.index()].deck.push(in_deck);
    let card = ensure_in_hand(&mut state, player, signal);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let offered_now = offered(&state);
    assert_eq!(offered_now, vec![in_deck], "only the Mega Pokemon is on offer");
    apply(&mut state, Action::TakeCard { card: in_deck }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert!(state.player(player).hand.contains(&in_deck));
}

#[test]
fn tm_machine_pulls_technical_machine_tools_from_the_deck() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "TM Machine" && c.playable.is_some()),
        "TM Machine should play"
    );

    let mut set = build();
    let tm_tool = set.db.add(CardDef::Trainer(Trainer {
        print_id: "test-tm-fluorite",
        name: "Technical Machine: Fluorite",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::IncreasesHp(0),
    }));
    let plain_tool = set.db.add(CardDef::Trainer(Trainer {
        print_id: "test-plain-tool",
        name: "Rescue Board",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::IncreasesHp(0),
    }));
    let machine = search_item(
        &mut set.db,
        "test-tm-machine",
        "TM Machine",
        Zone::Deck,
        CardFilter::ToolNameContains("Technical Machine"),
        3,
        None,
    );
    let mut state = game(&set, machine, 3);
    let player = state.current;
    for _ in 0..2 {
        let c = deal_new_card(&mut state, player, tm_tool);
        state.players[player.index()].deck.push(c);
    }
    let decoy = deal_new_card(&mut state, player, plain_tool);
    state.players[player.index()].deck.push(decoy);
    let card = ensure_in_hand(&mut state, player, machine);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let offered_now = offered(&state);
    assert_eq!(offered_now.len(), 2, "only the two Technical Machine Tools, not the decoy");
    assert!(!offered_now.contains(&decoy));
    for c in offered_now {
        apply(&mut state, Action::TakeCard { card: c }).unwrap();
    }
    apply(&mut state, Action::FinishDeciding).unwrap();
}

// --- Beyond the field: deck manipulation and a gust ---

fn plain_item(db: &mut CardDb, print_id: &'static str, name: &'static str, effect: TrainerEffect) -> CardDefId {
    db.add(CardDef::Trainer(Trainer {
        print_id,
        name,
        kind: TrainerKind::Item,
        requirement: None,
        effect,
    }))
}

#[test]
fn hole_digging_shovel_discards_the_top_two_of_the_deck() {
    let mut set = build();
    let shovel = plain_item(&mut set.db, "test-shovel", "Hole-Digging Shovel", TrainerEffect::DiscardTopOfDeck(2));
    let mut state = game(&set, shovel, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, shovel);
    let deck_before = state.player(player).deck.len();
    let discard_before = state.player(player).discard.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert_eq!(state.player(player).deck.len(), deck_before - 2);
    // Two from the deck, plus the Shovel itself.
    assert_eq!(state.player(player).discard.len(), discard_before + 3);
}

#[test]
fn repel_makes_the_opponent_promote_a_new_active() {
    let mut set = build();
    let repel = plain_item(&mut set.db, "test-repel", "Repel", TrainerEffect::SwitchOutOpponentActive);
    let mut state = game(&set, repel, 3);
    let player = state.current;
    let opp = player.opponent();
    let benched_card = deal_new_card(&mut state, opp, set.mon);
    let benched = state.put_into_play(opp, benched_card);
    state.players[opp.index()].bench.push(benched);
    let old_active = state.player(opp).active.unwrap();
    let card = ensure_in_hand(&mut state, player, repel);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    // The opponent now chooses the replacement.
    let promote = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Promote { .. }))
        .expect("the opponent promotes");
    apply(&mut state, promote).unwrap();

    assert_ne!(state.player(opp).active, Some(old_active), "the Active changed");
    assert!(state.player(opp).bench.contains(&old_active), "and went to the Bench");
}

#[test]
fn the_deck_manip_items_are_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    for name in ["Hole-Digging Shovel", "Repel", "Brilliant Blender"] {
        assert!(
            import.cards.iter().any(|c| c.name == name && c.playable.is_some()),
            "{name} should play",
        );
    }
}

// --- Beyond the field: condition-inflicting Items ---

#[test]
fn dangerous_laser_burns_and_confuses_the_opponents_active() {
    let mut set = build();
    let laser = plain_item(
        &mut set.db,
        "test-dangerous-laser",
        "Dangerous Laser",
        TrainerEffect::InflictOnOpponentActive(
            sim::card::Condition::Burned,
            Some(sim::card::Condition::Confused),
        ),
    );
    let mut state = game(&set, laser, 3);
    let player = state.current;
    let opp_active = state.player(player.opponent()).active.unwrap();
    let card = ensure_in_hand(&mut state, player, laser);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    let conditions = &state.pokemon(opp_active).conditions;
    assert!(conditions.contains(&sim::card::Condition::Burned));
    assert!(conditions.contains(&sim::card::Condition::Confused));
}

#[test]
fn dark_bell_confuses_both_actives_but_spares_a_darkness_one() {
    let mut set = build();
    let dark_mon = set.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-dark-mon",
        name: "Shademon",
        hp: 90,
        kind: Type::Darkness,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Bite",
            cost: vec![Type::Darkness],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let bell = plain_item(
        &mut set.db,
        "test-dark-bell",
        "Dark Bell",
        TrainerEffect::ConfuseBothActivesExceptType(Type::Darkness),
    );
    let mut state = game(&set, bell, 3);
    let player = state.current;
    let opp = player.opponent();
    // Swap the opponent's Active for a Darkness one.
    let dark_card = deal_new_card(&mut state, opp, dark_mon);
    let dark = state.put_into_play(opp, dark_card);
    state.players[opp.index()].active = Some(dark);
    let my_active = state.player(player).active.unwrap();
    let card = ensure_in_hand(&mut state, player, bell);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert!(
        state.pokemon(my_active).conditions.contains(&sim::card::Condition::Confused),
        "the non-Darkness Active is Confused",
    );
    assert!(
        state.pokemon(dark).conditions.is_empty(),
        "the Darkness Active is spared",
    );
}

#[test]
fn the_condition_items_are_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    for name in ["Dangerous Laser", "Dark Bell"] {
        assert!(
            import.cards.iter().any(|c| c.name == name && c.playable.is_some()),
            "{name} should play",
        );
    }
}

// --- Beyond the field: name-prefix searches and modifiers ---

#[test]
fn team_rockets_proton_pulls_named_basics_to_hand() {
    let mut set = build();
    let tr_mon = set.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-tr-grunt-mon",
        name: "Team Rocket's Grunt Rattata",
        hp: 60,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack { name: "Nip", cost: vec![Type::Colorless], base_damage: 10, inflicts: None, effect: None }],
    }));
    let proton = plain_item(
        &mut set.db,
        "test-tr-proton",
        "Team Rocket's Proton",
        TrainerEffect::Decide {
            from: Zone::Deck,
            slots: vec![Slot {
                filter: CardFilter::BasicPokemonNameContains("Team Rocket's"),
                to: Destination::Zone(Zone::Hand),
                limit: 3,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    );
    let mut state = game(&set, proton, 3);
    let player = state.current;
    for _ in 0..2 {
        let c = deal_new_card(&mut state, player, tr_mon);
        state.players[player.index()].deck.push(c);
    }
    // A non-matching Basic must not be offered.
    let plain = deal_new_card(&mut state, player, set.mon);
    state.players[player.index()].deck.push(plain);
    let card = ensure_in_hand(&mut state, player, proton);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let mut took = 0;
    while let Some(t) = legal_actions(&state).into_iter().find_map(|a| match a {
        Action::TakeCard { card } => Some(card),
        _ => None,
    }) {
        assert!(state.def_of(t).as_pokemon().unwrap().name.contains("Team Rocket's"));
        apply(&mut state, Action::TakeCard { card: t }).unwrap();
        took += 1;
    }
    apply(&mut state, Action::FinishDeciding).unwrap();
    assert_eq!(took, 2);
}

#[test]
fn cynthias_power_weight_adds_hp_only_to_a_cynthias_pokemon() {
    let mut set = build();
    let cynthia_mon = set.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-cynthias-mon",
        name: "Cynthia's Garchomp",
        hp: 150,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 2,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack { name: "Slash", cost: vec![Type::Colorless], base_damage: 30, inflicts: None, effect: None }],
    }));
    let weight = set.db.add(CardDef::Trainer(Trainer {
        print_id: "test-cynthias-power-weight",
        name: "Cynthia's Power Weight",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::IncreasesHpForNamePrefix { word: "Cynthia's", amount: 70 },
    }));
    let mut state = game(&set, weight, 3);
    let player = state.current;
    let cyn_card = deal_new_card(&mut state, player, cynthia_mon);
    let cyn = state.put_into_play(player, cyn_card);
    state.players[player.index()].bench.push(cyn);
    let plain = state.player(player).active.unwrap();

    let tool_a = deal_new_card(&mut state, player, weight);
    state.pokemon[cyn.index()].attached.push(tool_a);
    let tool_b = deal_new_card(&mut state, player, weight);
    state.pokemon[plain.index()].attached.push(tool_b);

    assert_eq!(state.effective_hp(cyn), 150 + 70);
    assert_eq!(state.effective_hp(plain), state.pokemon_def(plain).hp, "not a Cynthia's Pokemon");
}

#[test]
fn the_name_prefix_cards_are_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    for name in ["Team Rocket's Proton", "Hop's Bag", "Cynthia's Power Weight", "Granite Cave"] {
        assert!(
            import.cards.iter().any(|c| c.name == name && c.playable.is_some()),
            "{name} should play",
        );
    }
}

// --- Beyond the field: a two-branch coin-flipped search ---

fn team_rockets_great_ball_fixture(set: &Set) -> (CardDb, CardDefId, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let basic_tr_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-tr-great-ball-basic",
        name: "Team Rocket's Grunt Rattata",
        hp: 60,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack { name: "Nip", cost: vec![Type::Colorless], base_damage: 10, inflicts: None, effect: None }],
    }));
    let evolution_tr_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-tr-great-ball-evolution",
        name: "Team Rocket's Grunt Raticate",
        hp: 120,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Stage1,
        evolve_from: Some("Team Rocket's Grunt Rattata"),
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack { name: "Bite", cost: vec![Type::Colorless], base_damage: 40, inflicts: None, effect: None }],
    }));
    let great_ball = db.add(CardDef::Trainer(Trainer {
        print_id: "test-tr-great-ball",
        name: "Team Rocket's Great Ball",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::CoinFlipEitherThen(
            Box::new(TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::EvolutionPokemonNameContains("Team Rocket's"),
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            }),
            Box::new(TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::BasicPokemonNameContains("Team Rocket's"),
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            }),
        ),
    }));
    (db, great_ball, basic_tr_mon, evolution_tr_mon)
}

#[test]
fn team_rockets_great_ball_finds_an_evolution_on_heads() {
    use sim::rng::ScriptedRng;
    let set = build();
    let (db, great_ball, basic_tr_mon, evolution_tr_mon) = team_rockets_great_ball_fixture(&set);
    let set = Set { db, ..set };
    let decklist = deck(&set, great_ball);
    let mut state = GameState::new(
        set.db.clone(),
        [decklist.clone(), decklist],
        Box::new(ScriptedRng::new(vec![1u32])), // heads
    );
    for _ in 0..2 {
        while state.phase != Phase::Main && !state.is_over() {
            let a = legal_actions(&state)[0];
            apply(&mut state, a).unwrap();
        }
        if state.turn_number > 1 {
            break;
        }
        apply(&mut state, Action::EndTurn).unwrap();
    }
    let player = state.current;
    let evolution_card = deal_new_card(&mut state, player, evolution_tr_mon);
    state.players[player.index()].deck.push(evolution_card);
    let basic_card = deal_new_card(&mut state, player, basic_tr_mon);
    state.players[player.index()].deck.push(basic_card);
    let card = ensure_in_hand(&mut state, player, great_ball);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let take = legal_actions(&state)
        .into_iter()
        .find_map(|a| match a {
            Action::TakeCard { card } => Some(card),
            _ => None,
        })
        .expect("heads opens a search");
    assert_eq!(take, evolution_card, "heads finds the Evolution, not the Basic");
    apply(&mut state, Action::TakeCard { card: take }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert!(state.player(player).hand.contains(&evolution_card));
}

#[test]
fn team_rockets_great_ball_finds_a_basic_on_tails() {
    use sim::rng::ScriptedRng;
    let set = build();
    let (db, great_ball, basic_tr_mon, evolution_tr_mon) = team_rockets_great_ball_fixture(&set);
    let set = Set { db, ..set };
    let decklist = deck(&set, great_ball);
    let mut state = GameState::new(
        set.db.clone(),
        [decklist.clone(), decklist],
        Box::new(ScriptedRng::new(vec![0u32])), // tails
    );
    for _ in 0..2 {
        while state.phase != Phase::Main && !state.is_over() {
            let a = legal_actions(&state)[0];
            apply(&mut state, a).unwrap();
        }
        if state.turn_number > 1 {
            break;
        }
        apply(&mut state, Action::EndTurn).unwrap();
    }
    let player = state.current;
    let evolution_card = deal_new_card(&mut state, player, evolution_tr_mon);
    state.players[player.index()].deck.push(evolution_card);
    let basic_card = deal_new_card(&mut state, player, basic_tr_mon);
    state.players[player.index()].deck.push(basic_card);
    let card = ensure_in_hand(&mut state, player, great_ball);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let take = legal_actions(&state)
        .into_iter()
        .find_map(|a| match a {
            Action::TakeCard { card } => Some(card),
            _ => None,
        })
        .expect("tails opens a search");
    assert_eq!(take, basic_card, "tails finds the Basic, not the Evolution");
    apply(&mut state, Action::TakeCard { card: take }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert!(state.player(player).hand.contains(&basic_card));
}

#[test]
fn team_rockets_great_ball_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Team Rocket's Great Ball" && c.playable.is_some()),
        "Team Rocket's Great Ball should play"
    );
}

// --- Beyond the field: a coin-flipped search and retreat modifiers ---

#[test]
fn poke_ball_searches_for_a_pokemon_on_heads() {
    use sim::rng::ScriptedRng;
    let mut set = build();
    let poke_ball = plain_item(
        &mut set.db,
        "test-poke-ball",
        "Poké Ball",
        TrainerEffect::CoinFlipThen(Box::new(TrainerEffect::Decide {
            from: Zone::Deck,
            slots: vec![Slot {
                filter: CardFilter::AnyPokemon,
                to: Destination::Zone(Zone::Hand),
                limit: 1,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        })),
    );
    let decklist = deck(&set, poke_ball);
    let mut state = GameState::new(
        set.db.clone(),
        [decklist.clone(), decklist],
        Box::new(ScriptedRng::new(vec![1u32])), // heads
    );
    for _ in 0..2 {
        while state.phase != Phase::Main && !state.is_over() {
            let a = legal_actions(&state)[0];
            apply(&mut state, a).unwrap();
        }
        if state.turn_number > 1 {
            break;
        }
        apply(&mut state, Action::EndTurn).unwrap();
    }
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, poke_ball);
    let before = state.player(player).hand.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let take = legal_actions(&state)
        .into_iter()
        .find_map(|a| match a {
            Action::TakeCard { card } => Some(card),
            _ => None,
        })
        .expect("heads opens the search");
    apply(&mut state, Action::TakeCard { card: take }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.player(player).hand.len(), before - 1 + 1);
}

#[test]
fn energy_coin_attaches_a_basic_energy_on_two_heads() {
    use sim::rng::ScriptedRng;
    let mut set = build();
    let energy_coin = plain_item(
        &mut set.db,
        "test-energy-coin",
        "Energy Coin",
        TrainerEffect::CoinFlipAllThen(
            2,
            Box::new(TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::BasicEnergy,
                    to: Destination::Attach(TargetFilter::AnyInPlay),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            }),
        ),
    );
    let decklist = deck(&set, energy_coin);
    let mut state = GameState::new(
        set.db.clone(),
        [decklist.clone(), decklist],
        Box::new(ScriptedRng::new(vec![1u32, 1u32])), // heads, heads
    );
    for _ in 0..2 {
        while state.phase != Phase::Main && !state.is_over() {
            let a = legal_actions(&state)[0];
            apply(&mut state, a).unwrap();
        }
        if state.turn_number > 1 {
            break;
        }
        apply(&mut state, Action::EndTurn).unwrap();
    }
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let energy_card = deal_new_card(&mut state, player, set.energy);
    state.players[player.index()].deck.push(energy_card);
    let card = ensure_in_hand(&mut state, player, energy_coin);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let take = legal_actions(&state)
        .into_iter()
        .find_map(|a| match a {
            Action::TakeCardOnto { card, target } => Some((card, target)),
            _ => None,
        })
        .expect("two heads opens the search");
    apply(&mut state, Action::TakeCardOnto { card: take.0, target: take.1 }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert!(state.pokemon(active).attached.contains(&take.0));
}

#[test]
fn energy_coin_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Energy Coin" && c.playable.is_some()),
        "Energy Coin should play"
    );
}

#[test]
fn pokemon_catcher_switches_the_opponents_active_on_heads() {
    use sim::rng::ScriptedRng;
    let mut set = build();
    let catcher = plain_item(
        &mut set.db,
        "test-pokemon-catcher",
        "Pokémon Catcher",
        TrainerEffect::CoinFlipThen(Box::new(TrainerEffect::SwitchOpponentActive)),
    );
    let decklist = deck(&set, catcher);
    let mut state = GameState::new(
        set.db.clone(),
        [decklist.clone(), decklist],
        Box::new(ScriptedRng::new(vec![1u32])), // heads
    );
    for _ in 0..2 {
        while state.phase != Phase::Main && !state.is_over() {
            let a = legal_actions(&state)[0];
            apply(&mut state, a).unwrap();
        }
        if state.turn_number > 1 {
            break;
        }
        apply(&mut state, Action::EndTurn).unwrap();
    }
    let player = state.current;
    let opponent = player.opponent();
    let benched_card = deal_new_card(&mut state, opponent, set.mon);
    let benched = state.put_into_play(opponent, benched_card);
    state.players[opponent.index()].bench.push(benched);
    let old_active = state.player(opponent).active.unwrap();
    let card = ensure_in_hand(&mut state, player, catcher);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let promote = legal_actions(&state)
        .into_iter()
        .find_map(|a| match a {
            Action::Promote { pokemon } => Some(pokemon),
            _ => None,
        })
        .expect("heads opens the opponent's promotion");
    apply(&mut state, Action::Promote { pokemon: promote }).unwrap();

    assert_eq!(state.player(opponent).active, Some(benched));
    assert_ne!(state.player(opponent).active, Some(old_active));
}

#[test]
fn pokemon_catcher_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Pokémon Catcher")
        .expect("Pokémon Catcher should play");
    assert_eq!(
        card.effect,
        TrainerEffect::CoinFlipThen(Box::new(TrainerEffect::SwitchOpponentActive))
    );
}

#[test]
fn gravity_gemstone_raises_both_actives_retreat_cost() {
    let mut set = build();
    let gemstone = set.db.add(CardDef::Trainer(Trainer {
        print_id: "test-gravity-gemstone",
        name: "Gravity Gemstone",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::RaisesBothActiveRetreatWhileCarrierActive(1),
    }));
    let mut state = game(&set, gemstone, 3);
    let player = state.current;
    let opponent = player.opponent();
    let my_active = state.player(player).active.unwrap();
    let their_active = state.player(opponent).active.unwrap();
    let my_before = state.effective_retreat_cost(my_active);
    let their_before = state.effective_retreat_cost(their_active);

    let tool = deal_new_card(&mut state, player, gemstone);
    state.pokemon[my_active.index()].attached.push(tool);

    assert_eq!(state.effective_retreat_cost(my_active), my_before + 1);
    assert_eq!(state.effective_retreat_cost(their_active), their_before + 1);
}

#[test]
fn the_coin_and_retreat_cards_are_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    for name in ["Poké Ball", "Rescue Board", "Gravity Gemstone"] {
        assert!(
            import.cards.iter().any(|c| c.name == name && c.playable.is_some()),
            "{name} should play",
        );
    }
}

// --- Beyond the field: marker-count Items and a Checkup Stadium ---

fn marked_basic(db: &mut CardDb, print_id: &'static str, name: &'static str, marker: Marker) -> CardDefId {
    db.add(CardDef::Pokemon(Pokemon {
        markers: vec![marker],
        print_id,
        name,
        hp: 120,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack { name: "Hit", cost: vec![Type::Colorless], base_damage: 10, inflicts: None, effect: None }],
    }))
}

#[test]
fn awakening_drum_draws_one_per_ancient_pokemon() {
    let mut set = build();
    let ancient = marked_basic(&mut set.db, "test-ancient-mon", "Ancientmon", Marker::Ancient);
    let drum = plain_item(&mut set.db, "test-awakening-drum", "Awakening Drum", TrainerEffect::DrawPerOwnPokemonWithMarker(Marker::Ancient));
    let mut state = game(&set, drum, 3);
    let player = state.current;
    for _ in 0..3 {
        let c = deal_new_card(&mut state, player, ancient);
        let m = state.put_into_play(player, c);
        state.players[player.index()].bench.push(m);
    }
    let card = ensure_in_hand(&mut state, player, drum);
    let before = state.player(player).hand.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert_eq!(state.player(player).hand.len(), before - 1 + 3);
}

#[test]
fn reboot_pod_attaches_a_basic_energy_to_each_future_pokemon() {
    let mut set = build();
    let future = marked_basic(&mut set.db, "test-future-mon", "Futuremon", Marker::Future);
    let pod = plain_item(&mut set.db, "test-reboot-pod", "Reboot Pod", TrainerEffect::AttachBasicEnergyFromDiscardToEachFuture);
    let mut state = game(&set, pod, 3);
    let player = state.current;
    let mut futures = vec![];
    for _ in 0..2 {
        let c = deal_new_card(&mut state, player, future);
        let m = state.put_into_play(player, c);
        state.players[player.index()].bench.push(m);
        futures.push(m);
    }
    for _ in 0..3 {
        let e = deal_new_card(&mut state, player, set.energy);
        state.players[player.index()].discard.push(e);
    }
    let card = ensure_in_hand(&mut state, player, pod);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    for f in futures {
        assert_eq!(state.pokemon(f).attached.len(), 1, "each Future Pokemon got one");
    }
}

#[test]
fn the_marker_cards_are_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    for name in ["Awakening Drum", "Reboot Pod", "Perilous Jungle"] {
        assert!(
            import.cards.iter().any(|c| c.name == name && c.playable.is_some()),
            "{name} should play",
        );
    }
}

// --- Beyond the field: more heal Items ---

#[test]
fn lumiose_galette_heals_the_active_and_clears_its_condition() {
    let mut set = build();
    let galette = plain_item(&mut set.db, "test-lumiose-galette", "Lumiose Galette", TrainerEffect::HealActiveAndClearConditions(20));
    let mut state = game(&set, galette, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    state.pokemon[active.index()].damage = 50;
    state.inflict(active, sim::card::Condition::Asleep);
    let card = ensure_in_hand(&mut state, player, galette);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert_eq!(state.pokemon(active).damage, 30);
    assert!(state.pokemon(active).conditions.is_empty());
}

#[test]
fn the_more_heal_items_are_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    for name in ["Lumiose Galette", "Dragon Elixir"] {
        assert!(
            import.cards.iter().any(|c| c.name == name && c.playable.is_some()),
            "{name} should play",
        );
    }
}

// --- Beyond the field: a bonus restricted to the attacker's own Type ---

#[test]
fn premium_power_pro_adds_thirty_only_from_a_fighting_type_attacker() {
    let mut set = build();
    let fighter = set.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-fighter",
        name: "Testfighter",
        hp: 100,
        kind: Type::Fighting,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let card = plain_item(
        &mut set.db,
        "test-premium-power-pro",
        "Premium Power Pro",
        TrainerEffect::BonusDamageThisTurn(30, TurnBonusTarget::AttackerIsType(Type::Fighting)),
    );
    let mut state = game(&set, card, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    // Give the current player's Active the Fighting type directly, rather
    // than reshaping the whole deck for one attacker.
    let colorless_attacker = state.player(player).active.unwrap();
    let fighting_card = deal_new_card(&mut state, player, fighter);
    state.pokemon[colorless_attacker.index()].cards = vec![fighting_card];

    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    assert_eq!(
        sim::engine::damage_dealt(&state, colorless_attacker, defender, 100),
        130,
        "a Fighting-type attacker gets the bonus"
    );
}

#[test]
fn premium_power_pro_adds_nothing_from_any_other_type() {
    let mut set = build();
    let card = plain_item(
        &mut set.db,
        "test-premium-power-pro-2",
        "Premium Power Pro",
        TrainerEffect::BonusDamageThisTurn(30, TurnBonusTarget::AttackerIsType(Type::Fighting)),
    );
    let mut state = game(&set, card, 3);
    let player = state.current;
    let opponent = player.opponent();
    let attacker = state.player(player).active.unwrap();
    let defender = state.player(opponent).active.unwrap();

    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    assert_eq!(
        sim::engine::damage_dealt(&state, attacker, defender, 100),
        100,
        "the Colorless attacker built by default gets no bonus"
    );
}

#[test]
fn premium_power_pro_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Premium Power Pro")
        .expect("Premium Power Pro should play");
    assert_eq!(
        card.effect,
        TrainerEffect::BonusDamageThisTurn(30, TurnBonusTarget::AttackerIsType(Type::Fighting))
    );
}


// --- Beyond the field: a type-restricted Basic-or-Energy search ---

#[test]
fn fighting_gong_finds_a_basic_fighting_pokemon_or_energy() {
    let mut set = build();
    let fighter = set.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-fighting-gong-mon",
        name: "Testfighter",
        hp: 100,
        kind: Type::Fighting,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let fighting_energy = set.db.add(CardDef::Energy(Energy {
        print_id: "test-fighting-energy",
        name: "Fighting Energy",
        kind: Type::Fighting,
        effect: None,
    }));
    let gong = plain_item(
        &mut set.db,
        "test-fighting-gong",
        "Fighting Gong",
        TrainerEffect::Decide {
            from: Zone::Deck,
            slots: vec![Slot {
                filter: CardFilter::BasicPokemonOfTypeOrBasicEnergyOfType(Type::Fighting),
                to: Destination::Zone(Zone::Hand),
                limit: 1,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    );
    let mut state = game(&set, gong, 3);
    let player = state.current;
    // Neither card is in the default deck; deal both into it directly.
    let mon_card = deal_new_card(&mut state, player, fighter);
    state.players[player.index()].deck.push(mon_card);
    let energy_card = deal_new_card(&mut state, player, fighting_energy);
    state.players[player.index()].deck.push(energy_card);
    let card = ensure_in_hand(&mut state, player, gong);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let offered: Vec<_> = legal_actions(&state)
        .into_iter()
        .filter_map(|a| match a {
            Action::TakeCard { card } => Some(card),
            _ => None,
        })
        .collect();
    assert!(offered.contains(&mon_card), "{offered:?}");
    assert!(offered.contains(&energy_card), "{offered:?}");
    apply(&mut state, Action::TakeCard { card: mon_card }).unwrap();

    assert!(state.player(player).hand.contains(&mon_card));
}

#[test]
fn fighting_gong_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import
            .cards
            .iter()
            .any(|c| c.name == "Fighting Gong" && c.playable.is_some()),
        "Fighting Gong should play",
    );
}

// --- Beyond the field: return a Pokemon and its attachments to hand ---

#[test]
fn scoop_up_cyclone_returns_the_active_and_promotes_a_new_one() {
    let mut set = build();
    let cyclone = plain_item(&mut set.db, "test-scoop-up-cyclone", "Scoop Up Cyclone", TrainerEffect::ReturnChosenToHand);
    let mut state = game(&set, cyclone, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let energy_card = deal_new_card(&mut state, player, set.energy);
    state.pokemon[active.index()].attached.push(energy_card);
    let stack: Vec<_> = state.pokemon(active).cards.clone();

    let card = ensure_in_hand(&mut state, player, cyclone);
    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    let offered: Vec<_> = legal_actions(&state)
        .into_iter()
        .filter_map(|a| match a {
            Action::ReturnToHand { target } => Some(target),
            _ => None,
        })
        .collect();
    assert!(offered.contains(&active));
    apply(&mut state, Action::ReturnToHand { target: active }).unwrap();

    assert!(state.player(player).hand.contains(&energy_card), "the attached Energy lands in hand");
    for c in &stack {
        assert!(
            state.player(player).hand.contains(c),
            "the Pokemon's whole stack lands in hand: {c:?}"
        );
    }

    // The Active is gone, so a new one is promoted from the Bench.
    let promote = legal_actions(&state)
        .into_iter()
        .find_map(|a| match a {
            Action::Promote { pokemon } => Some(pokemon),
            _ => None,
        })
        .expect("a new Active must be promoted");
    apply(&mut state, Action::Promote { pokemon: promote }).unwrap();
    assert_eq!(state.player(player).active, Some(promote));
    assert_eq!(state.phase, Phase::Main);
}

#[test]
fn scoop_up_cyclone_can_return_a_benched_pokemon_without_promoting() {
    let mut set = build();
    let cyclone = plain_item(&mut set.db, "test-scoop-up-cyclone-2", "Scoop Up Cyclone", TrainerEffect::ReturnChosenToHand);
    let mut state = game(&set, cyclone, 3);
    let player = state.current;
    let benched = state.player(player).bench[0];

    let card = ensure_in_hand(&mut state, player, cyclone);
    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::ReturnToHand { target: benched }).unwrap();

    assert_eq!(state.phase, Phase::Main, "no Active was lost, so nothing to promote");
    assert!(!state.player(player).bench.contains(&benched));
}

#[test]
fn scoop_up_cyclone_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import
            .cards
            .iter()
            .any(|c| c.name == "Scoop Up Cyclone" && c.playable.is_some()),
        "Scoop Up Cyclone should play",
    );
}

// --- Beyond the field: a coin flip choosing which side takes counters ---

#[test]
fn team_rockets_venture_bomb_targets_a_chosen_opponent_on_heads() {
    use sim::rng::ScriptedRng;
    let mut set = build();
    let bomb = plain_item(
        &mut set.db,
        "test-venture-bomb",
        "Team Rocket's Venture Bomb",
        TrainerEffect::CoinFlipDamageCountersOnChosenOpponentElseOwnActive(20),
    );
    let decklist = deck(&set, bomb);
    let mut state = GameState::new(
        set.db.clone(),
        [decklist.clone(), decklist],
        Box::new(ScriptedRng::new(vec![1u32])), // heads
    );
    for _ in 0..2 {
        while state.phase != Phase::Main && !state.is_over() {
            let a = legal_actions(&state)[0];
            apply(&mut state, a).unwrap();
        }
        if state.turn_number > 1 {
            break;
        }
        apply(&mut state, Action::EndTurn).unwrap();
    }
    let player = state.current;
    let opponent = player.opponent();
    let opp_active = state.player(opponent).active.unwrap();
    let my_active = state.player(player).active.unwrap();
    let card = ensure_in_hand(&mut state, player, bomb);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let target = legal_actions(&state)
        .into_iter()
        .find_map(|a| match a {
            Action::PlaceDamageCountersOn { target } => Some(target),
            _ => None,
        })
        .expect("heads opens a choice among the opponent's Pokemon");
    apply(&mut state, Action::PlaceDamageCountersOn { target }).unwrap();

    assert_eq!(state.pokemon(opp_active).damage, 20);
    assert_eq!(state.pokemon(my_active).damage, 0, "tails alone would hit my own Active");
}

#[test]
fn team_rockets_venture_bomb_hits_the_players_own_active_on_tails() {
    use sim::rng::ScriptedRng;
    let mut set = build();
    let bomb = plain_item(
        &mut set.db,
        "test-venture-bomb-2",
        "Team Rocket's Venture Bomb",
        TrainerEffect::CoinFlipDamageCountersOnChosenOpponentElseOwnActive(20),
    );
    let decklist = deck(&set, bomb);
    let mut state = GameState::new(
        set.db.clone(),
        [decklist.clone(), decklist],
        Box::new(ScriptedRng::new(vec![0u32])), // tails
    );
    for _ in 0..2 {
        while state.phase != Phase::Main && !state.is_over() {
            let a = legal_actions(&state)[0];
            apply(&mut state, a).unwrap();
        }
        if state.turn_number > 1 {
            break;
        }
        apply(&mut state, Action::EndTurn).unwrap();
    }
    let player = state.current;
    let my_active = state.player(player).active.unwrap();
    let card = ensure_in_hand(&mut state, player, bomb);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert_eq!(state.pokemon(my_active).damage, 20);
    assert_eq!(state.phase, Phase::Main, "tails resolves at once, no choice to make");
}

#[test]
fn team_rockets_venture_bomb_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import
            .cards
            .iter()
            .any(|c| c.name == "Team Rocket's Venture Bomb" && c.playable.is_some()),
        "Team Rocket's Venture Bomb should play",
    );
}
