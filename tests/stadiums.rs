//! Milestone 10: every Stadium in the field. One fixture, shared across
//! this milestone's tickets, the same way `tools.rs` served Milestone 9.

use sim::action::{Action, legal_actions};
use sim::card::{
    Attack, CardDb, CardDef, CardFilter, Destination, Energy, Pokemon, PromoteFollowUp,
    Requirement, Slot, Stage, TargetFilter, Then, Trainer, TrainerEffect, TrainerKind,
    TurnBonusTarget, Type, Zone,
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
        attacks: vec![Attack {
            name: "Slam",
            cost: vec![Type::Colorless, Type::Colorless],
            base_damage: 60,
            inflicts: None,
        }],
    }));
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
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


// --- Ticket 01: Gravity Mountain & N's Castle ---

fn with_gravity_mountain(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-gravity-mountain",
        name: "Gravity Mountain",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::ReducesHpForStage(Stage::Stage2, 30),
    }));
    (Set { db, ..set }, card)
}

fn with_ns_castle(set: Set) -> (Set, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let ns_mon = basic(&mut db, "test-ns-mon", "N's Testmon", 90, 1, None);
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-ns-castle",
        name: "N's Castle",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::RemovesRetreatCostForNamePrefix("N's"),
    }));
    (Set { db, ..set }, card, ns_mon)
}

#[test]
fn gravity_mountain_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Gravity Mountain")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Gravity Mountain should play");
}

#[test]
fn gravity_mountain_lowers_hp_for_every_stage_2_both_sides() {
    let (set, card) = with_gravity_mountain(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let opponent = player.opponent();
    let played = ensure_in_hand(&mut state, player, card);

    let mine = deal_new_card(&mut state, player, set.stage2);
    let mine_mon = state.put_into_play(player, mine);
    state.players[player.index()].bench.push(mine_mon);
    let theirs = deal_new_card(&mut state, opponent, set.stage2);
    let their_mon = state.put_into_play(opponent, theirs);
    state.players[opponent.index()].bench.push(their_mon);
    let ordinary = state.player(player).active.unwrap();

    let before = state.effective_hp(mine_mon);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    assert_eq!(state.effective_hp(mine_mon), before - 30, "my own Stage 2");
    assert_eq!(state.effective_hp(their_mon), before - 30, "the opponent's too");
    assert_eq!(
        state.effective_hp(ordinary),
        state.pokemon_def(ordinary).hp,
        "an ordinary Basic is untouched"
    );
}

#[test]
fn ns_castle_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "N's Castle")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "N's Castle should play");
}

#[test]
fn ns_castle_removes_retreat_cost_for_ns_pokemon_both_sides() {
    let (set, card, ns_mon) = with_ns_castle(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let opponent = player.opponent();
    let played = ensure_in_hand(&mut state, player, card);

    let mine = deal_new_card(&mut state, player, ns_mon);
    let mine_active = state.put_into_play(player, mine);
    let ordinary = state.player(player).active.unwrap();
    state.players[player.index()].bench.push(mine_active);
    let theirs = deal_new_card(&mut state, opponent, ns_mon);
    let their_active = state.put_into_play(opponent, theirs);
    state.players[opponent.index()].bench.push(their_active);

    assert_eq!(state.effective_retreat_cost(mine_active), 1);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    assert_eq!(state.effective_retreat_cost(mine_active), 0, "my own N's Pokémon");
    assert_eq!(state.effective_retreat_cost(their_active), 0, "the opponent's too");
    assert_eq!(
        state.effective_retreat_cost(ordinary),
        1,
        "an ordinary Pokémon keeps its printed cost"
    );
}

// --- Ticket 02: Academy at Night ---

fn with_academy_at_night(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-academy-at-night",
        name: "Academy at Night",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::MayPutHandCardOnTopOfDeck,
    }));
    (Set { db, ..set }, card)
}

#[test]
fn academy_at_night_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Academy at Night")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Academy at Night should play");
}

#[test]
fn academy_at_night_puts_a_hand_card_on_top_once_a_turn() {
    let (set, card) = with_academy_at_night(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    let to_put = *state.player(player).hand.first().unwrap();
    let library_before = state.player(player).library.len();
    assert!(legal_actions(&state).contains(&Action::PutOnTopOfDeckForAcademyAtNight {
        card: to_put,
    }));
    apply(
        &mut state,
        Action::PutOnTopOfDeckForAcademyAtNight { card: to_put },
    )
    .unwrap();

    assert!(!state.player(player).hand.contains(&to_put));
    assert_eq!(*state.player(player).library.last().unwrap(), to_put);
    assert_eq!(state.player(player).library.len(), library_before + 1);

    // Spent for the turn: no longer offered, even with cards left in hand.
    assert!(
        !legal_actions(&state)
            .into_iter()
            .any(|a| matches!(a, Action::PutOnTopOfDeckForAcademyAtNight { .. })),
        "once a turn, not once a card"
    );
}

#[test]
fn academy_at_night_is_not_offered_without_the_stadium_in_play() {
    let (set, _) = with_academy_at_night(build());
    let state = game(&set, set.mon, 3);
    assert!(
        !legal_actions(&state)
            .into_iter()
            .any(|a| matches!(a, Action::PutOnTopOfDeckForAcademyAtNight { .. }))
    );
}

// --- Ticket 03: Team Rocket's Factory ---

fn with_team_rockets_factory(set: Set) -> (Set, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let petrel = db.add(CardDef::Trainer(Trainer {
        print_id: "test-tr-petrel-2",
        name: "Team Rocket's Petrel",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::Nothing,
    }));
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-tr-factory",
        name: "Team Rocket's Factory",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::MayDrawTwoIfPlayedTeamRocketSupporter,
    }));
    (Set { db, ..set }, card, petrel)
}

#[test]
fn team_rockets_factory_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Team Rocket's Factory")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Team Rocket's Factory should play");
}

#[test]
fn team_rockets_factory_offers_a_draw_only_after_a_team_rocket_supporter() {
    let (set, card, petrel) = with_team_rockets_factory(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    assert!(
        !legal_actions(&state).contains(&Action::DrawTwoForTeamRocketsFactory),
        "no Team Rocket Supporter played yet this turn"
    );

    let petrel_card = deal_new_card(&mut state, player, petrel);
    state.players[player.index()].hand.push(petrel_card);
    apply(&mut state, Action::PlayTrainer { card: petrel_card }).unwrap();

    assert!(legal_actions(&state).contains(&Action::DrawTwoForTeamRocketsFactory));
    let hand_before = state.player(player).hand.len();
    apply(&mut state, Action::DrawTwoForTeamRocketsFactory).unwrap();
    assert_eq!(state.player(player).hand.len(), hand_before + 2);

    assert!(
        !legal_actions(&state).contains(&Action::DrawTwoForTeamRocketsFactory),
        "once a turn"
    );
}

#[test]
fn team_rockets_factory_forgets_by_the_next_turn() {
    let (set, card, petrel) = with_team_rockets_factory(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    let petrel_card = deal_new_card(&mut state, player, petrel);
    state.players[player.index()].hand.push(petrel_card);
    apply(&mut state, Action::PlayTrainer { card: petrel_card }).unwrap();

    end_turn_and_advance(&mut state);
    end_turn_and_advance(&mut state);
    assert_eq!(state.current, player, "back to the same player's turn");

    assert!(
        !legal_actions(&state).contains(&Action::DrawTwoForTeamRocketsFactory),
        "the fact does not survive past the turn it happened"
    );
}

/// Apply `EndTurn`, then every legal action until Main or the game ends.
fn end_turn_and_advance(state: &mut GameState) {
    apply(state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(state)[0];
        apply(state, first).unwrap();
    }
}

// --- Ticket 04: Lumiose City ---

fn with_lumiose_city(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-lumiose-city",
        name: "Lumiose City",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::MaySearchBasicToBenchThenMaybeEndTurn,
    }));
    (Set { db, ..set }, card)
}

#[test]
fn lumiose_city_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Lumiose City")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Lumiose City should play");
}

#[test]
fn lumiose_city_search_ends_the_turn_when_a_basic_is_taken() {
    let (set, card) = with_lumiose_city(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    assert!(legal_actions(&state).contains(&Action::UseLumioseCity));
    apply(&mut state, Action::UseLumioseCity).unwrap();
    assert!(matches!(state.phase, Phase::Deciding { .. }));

    let bench_before = state.player(player).bench.len();
    let choice = offered(&state)[0];
    apply(&mut state, Action::TakeCard { card: choice }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.player(player).bench.len(), bench_before + 1);
    assert_ne!(state.current, player, "the turn ended because a Basic was taken");
}

#[test]
fn lumiose_city_declining_does_not_end_the_turn() {
    let (set, card) = with_lumiose_city(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    apply(&mut state, Action::UseLumioseCity).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.current, player, "declining does not end the turn");
    assert!(
        !legal_actions(&state).contains(&Action::UseLumioseCity),
        "once a turn either way"
    );
}
