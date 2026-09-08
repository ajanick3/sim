//! Milestone 8: every Item in the field. One fixture, shared across this
//! milestone's tickets, the same way `supporters.rs` served Milestone 7.

use sim::action::{Action, legal_actions};
use sim::card::{
    Attack, CardDb, CardDef, CardFilter, Destination, Energy, Pokemon, PromoteFollowUp,
    Requirement, Slot, Stage, TargetFilter, Trainer, TrainerEffect, TrainerKind, TurnBonusTarget,
    Type, Zone,
};
use sim::engine::apply;
use sim::ids::{CardDefId, CardId, PlayerId, PokemonId};
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

/// Put a copy of `def` into the discard pile, taking it from the library.
fn deal_to_discard(state: &mut GameState, player: PlayerId, def: CardDefId) -> CardId {
    let card = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.cards[c.index()].def == def)
        .expect("the deck holds this card");
    state.players[player.index()].library.retain(|c| *c != card);
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
            from: Zone::Library,
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
fn energy_search_finds_one_basic_energy_from_the_library() {
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
                to: Destination::Zone(Zone::Library),
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
fn energy_recycler_shuffles_up_to_five_from_discard_into_the_library() {
    let (set, card) = with_energy_recycler(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    let e1 = deal_to_discard(&mut state, player, set.energy);
    let e2 = deal_to_discard(&mut state, player, set.energy);
    let library_before = state.player(player).library.len();

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    apply(&mut state, Action::TakeCard { card: e1 }).unwrap();
    apply(&mut state, Action::TakeCard { card: e2 }).unwrap();
    assert!(
        !state.player(player).discard.contains(&e1) && !state.player(player).discard.contains(&e2),
        "both energy left the discard"
    );
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.player(player).library.len(), library_before + 2);
    assert!(state.player(player).library.contains(&e1));
    assert!(state.player(player).library.contains(&e2));
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
            from: Zone::Library,
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
    state.players[player.index()].library.push(petrel_card);
    let ordinary_card = deal_new_card(&mut state, player, ordinary);
    state.players[player.index()].library.push(ordinary_card);

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
            from: Zone::Library,
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
        state.players[player.index()].library.push(placed);
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

// --- Ticket 08: Dusk Ball ---

fn with_dusk_ball(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-dusk-ball",
        name: "Dusk Ball",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::LookAtBottomOfLibrary { count: 7 },
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
    // Put a known Pokémon at the very bottom (index 0) of the library.
    let bottom_mon = deal_new_card(&mut state, player, set.mon);
    state.players[player.index()].library.insert(0, bottom_mon);
    // And a known Pokémon far from the bottom, outside the 7-card window.
    let deep_mon = deal_new_card(&mut state, player, set.mon);
    let deep_index = state.player(player).library.len() / 2;
    state.players[player.index()].library.insert(deep_index, deep_mon);

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    let choices: Vec<CardId> = legal_actions(&state)
        .into_iter()
        .filter_map(|a| match a {
            Action::TakeFromBottomOfLibrary { card } => Some(card),
            _ => None,
        })
        .collect();
    assert!(choices.contains(&bottom_mon), "the bottom card is in range");
    assert!(
        !choices.contains(&deep_mon),
        "a card outside the bottom 7 is not offered"
    );
    assert!(
        legal_actions(&state).contains(&Action::DeclineBottomOfLibrary),
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
    state.players[player.index()].library.insert(0, bottom_mon);
    let library_before = state.player(player).library.len();

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    apply(&mut state, Action::TakeFromBottomOfLibrary { card: bottom_mon }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).hand.contains(&bottom_mon));
    assert_eq!(state.player(player).library.len(), library_before - 1);
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
