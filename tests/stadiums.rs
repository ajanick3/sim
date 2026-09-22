//! Milestone 10: every Stadium in the field. One fixture, shared across
//! this milestone's tickets, the same way `tools.rs` served Milestone 9.

use sim::action::{Action, legal_actions};
use sim::card::{
    Ability, AbilityEffect, Attack, CardDb, CardDef, Energy, Pokemon, Stage, Trainer, TrainerEffect,
    TrainerKind, Type,
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
    let deck_before = state.player(player).deck.len();
    assert!(legal_actions(&state).contains(&Action::PutOnTopOfDeckForAcademyAtNight {
        card: to_put,
    }));
    apply(
        &mut state,
        Action::PutOnTopOfDeckForAcademyAtNight { card: to_put },
    )
    .unwrap();

    assert!(!state.player(player).hand.contains(&to_put));
    assert_eq!(*state.player(player).deck.last().unwrap(), to_put);
    assert_eq!(state.player(player).deck.len(), deck_before + 1);

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

// --- Ticket 06: Jamming Tower ---

fn with_jamming_tower(set: Set) -> (Set, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let cape = db.add(CardDef::Trainer(Trainer {
        print_id: "test-heros-cape-jt",
        name: "Hero's Cape",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::IncreasesHp(100),
    }));
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-jamming-tower",
        name: "Jamming Tower",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::ToolsHaveNoEffect,
    }));
    (Set { db, ..set }, card, cape)
}

#[test]
fn jamming_tower_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Jamming Tower")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Jamming Tower should play");
}

#[test]
fn jamming_tower_turns_off_an_attached_tools_effect() {
    let (set, card, cape) = with_jamming_tower(build());
    let mut state = game(&set, cape, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let cape_card = ensure_in_hand(&mut state, player, cape);
    apply(&mut state, Action::PlayTool { card: cape_card, target: active }).unwrap();
    assert_eq!(state.effective_hp(active), state.pokemon_def(active).hp + 100);

    let played = deal_new_card(&mut state, player, card);
    state.players[player.index()].hand.push(played);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    assert_eq!(
        state.effective_hp(active),
        state.pokemon_def(active).hp,
        "the Tool is still attached, but does nothing"
    );
    assert!(state.pokemon(active).attached.contains(&cape_card));
}

// --- Ticket 07: Risky Ruins ---

fn with_risky_ruins(set: Set) -> (Set, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let darkness_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-darkness-mon-rr",
        name: "Duskmon",
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
        attacks: vec![],
    }));
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-risky-ruins",
        name: "Risky Ruins",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::DamagesNonDarknessBasicBenched(20),
    }));
    (Set { db, ..set }, card, darkness_mon)
}

#[test]
fn risky_ruins_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Risky Ruins")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Risky Ruins should play");
}

#[test]
fn risky_ruins_damages_a_non_darkness_basic_benched_from_hand() {
    let (set, card, _) = with_risky_ruins(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    let basic_card = ensure_in_hand(&mut state, player, set.mon);
    apply(&mut state, Action::PlayBasic { card: basic_card }).unwrap();
    let benched = *state.player(player).bench.last().unwrap();

    assert_eq!(state.pokemon(benched).damage, 20);
}

#[test]
fn risky_ruins_spares_a_darkness_basic() {
    let (set, card, darkness_mon) = with_risky_ruins(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    let dark_card = deal_new_card(&mut state, player, darkness_mon);
    state.players[player.index()].hand.push(dark_card);
    apply(&mut state, Action::PlayBasic { card: dark_card }).unwrap();
    let benched = *state.player(player).bench.last().unwrap();

    assert_eq!(state.pokemon(benched).damage, 0, "Darkness is exempt");
}

#[test]
fn risky_ruins_applies_on_the_opponents_side_too() {
    let (set, card, _) = with_risky_ruins(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let opponent = player.opponent();
    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    // Now the opponent's own turn, benching their own Basic.
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    assert_eq!(state.current, opponent);
    let basic_card = ensure_in_hand(&mut state, opponent, set.mon);
    apply(&mut state, Action::PlayBasic { card: basic_card }).unwrap();
    let benched = *state.player(opponent).bench.last().unwrap();

    assert_eq!(state.pokemon(benched).damage, 20, "both sides alike");
}

// --- Ticket 08: Forest of Vitality ---

fn with_forest_of_vitality(set: Set) -> (Set, CardDefId, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let grass_basic = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-grass-basic-fov",
        name: "Leafmon",
        hp: 90,
        kind: Type::Grass,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![],
    }));
    let grass_stage1 = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-grass-stage1-fov",
        name: "Vinemon",
        hp: 120,
        kind: Type::Grass,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Stage1,
        evolve_from: Some("Leafmon"),
        evolves_from_basic: None,
        ability: None,
        attacks: vec![],
    }));
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-forest-of-vitality",
        name: "Forest of Vitality",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::GrassCanEvolveTheTurnItIsPlayed,
    }));
    (Set { db, ..set }, card, grass_basic, grass_stage1)
}

#[test]
fn forest_of_vitality_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Forest of Vitality")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Forest of Vitality should play");
}

#[test]
fn forest_of_vitality_lets_a_grass_evolution_play_the_same_turn() {
    let (set, card, grass_basic, grass_stage1) = with_forest_of_vitality(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    let basic_card = deal_new_card(&mut state, player, grass_basic);
    state.players[player.index()].hand.push(basic_card);
    apply(&mut state, Action::PlayBasic { card: basic_card }).unwrap();
    let target = *state.player(player).bench.last().unwrap();

    let evo_card = deal_new_card(&mut state, player, grass_stage1);
    state.players[player.index()].hand.push(evo_card);
    assert!(
        legal_actions(&state).contains(&Action::Evolve { card: evo_card, target }),
        "Grass into Grass, same turn, bypasses the usual timing rule"
    );
}

#[test]
fn without_forest_of_vitality_the_usual_timing_rule_applies() {
    let (set, _, grass_basic, grass_stage1) = with_forest_of_vitality(build());
    let mut state = game(&set, set.mon, 3);
    let player = state.current;

    let basic_card = deal_new_card(&mut state, player, grass_basic);
    state.players[player.index()].hand.push(basic_card);
    apply(&mut state, Action::PlayBasic { card: basic_card }).unwrap();
    let target = *state.player(player).bench.last().unwrap();

    let evo_card = deal_new_card(&mut state, player, grass_stage1);
    state.players[player.index()].hand.push(evo_card);
    assert!(
        !legal_actions(&state).contains(&Action::Evolve { card: evo_card, target }),
        "no Stadium to bypass the timing rule"
    );
}

// --- Ticket 09: Festival Grounds ---

fn with_festival_grounds(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-festival-grounds",
        name: "Festival Grounds",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::EnergizedPokemonImmuneToSpecialConditions,
    }));
    (Set { db, ..set }, card)
}

#[test]
fn festival_grounds_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Festival Grounds")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Festival Grounds should play");
}

#[test]
fn festival_grounds_blocks_a_new_condition_on_an_energized_pokemon() {
    let (set, card) = with_festival_grounds(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let energy = deal_new_card(&mut state, player, set.energy);
    state.pokemon[active.index()].attached.push(energy);

    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    state.inflict(active, sim::card::Condition::Poisoned);
    assert!(
        !state.has_condition(active, sim::card::Condition::Poisoned),
        "carries Energy, so it cannot be Poisoned"
    );
}

#[test]
fn festival_grounds_immediately_clears_an_existing_condition() {
    let (set, card) = with_festival_grounds(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let energy = deal_new_card(&mut state, player, set.energy);
    state.pokemon[active.index()].attached.push(energy);
    state.pokemon[active.index()]
        .conditions
        .push(sim::card::Condition::Poisoned);

    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    assert!(
        !state.has_condition(active, sim::card::Condition::Poisoned),
        "recovers the moment the Stadium enters play"
    );
}

#[test]
fn festival_grounds_does_not_protect_a_bare_pokemon() {
    let (set, card) = with_festival_grounds(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    state.inflict(active, sim::card::Condition::Poisoned);
    assert!(
        state.has_condition(active, sim::card::Condition::Poisoned),
        "no Energy attached, so no protection"
    );
}

// --- Beyond the map: a Stadium that surcharges Tera attacks on both sides ---

fn with_nighttime_mine(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-nighttime-mine",
        name: "Nighttime Mine",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::TeraAttacksCostMore,
    }));
    (Set { db, ..set }, card)
}

#[test]
fn nighttime_mine_surcharges_a_tera_attacker_own_or_opponents() {
    let (set, card) = with_nighttime_mine(build());
    let mut db = set.db.clone();
    let tera_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: vec![sim::card::Marker::Ex, sim::card::Marker::Tera],
        print_id: "test-tera-attacker-mine",
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
    let set = Set { db, ..set };
    let mut state = game(&set, card, 3);
    let player = state.current;
    let opponent = player.opponent();

    // Swap both players' Actives for the Tera attacker.
    for who in [player, opponent] {
        let attacker_card = deal_new_card(&mut state, who, tera_mon);
        let attacker = state.put_into_play(who, attacker_card);
        state.players[who.index()].active = Some(attacker);
    }
    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    let active = state.player(player).active.unwrap();
    assert!(
        !legal_actions(&state).contains(&Action::Attack { index: 0 }),
        "1 Colorless is not enough once Nighttime Mine surcharges a Tera attacker"
    );

    for _ in 0..2 {
        let energy = deal_new_card(&mut state, player, set.energy);
        state.pokemon[active.index()].attached.push(energy);
    }
    assert!(
        legal_actions(&state).contains(&Action::Attack { index: 0 }),
        "2 Colorless pays the surcharged cost"
    );
}

#[test]
fn nighttime_mine_does_not_surcharge_a_plain_attacker() {
    let (set, card) = with_nighttime_mine(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let energy = deal_new_card(&mut state, player, set.energy);
    state.pokemon[active.index()].attached.push(energy);

    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    assert!(
        legal_actions(&state).contains(&Action::Attack { index: 0 }),
        "a plain (non-Tera) attacker pays only its printed cost"
    );
}

#[test]
fn nighttime_mine_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Nighttime Mine" && c.playable.is_some()),
        "Nighttime Mine should play"
    );
}

// --- Beyond the map: a Stadium that raises the Bench limit, and cleans up after itself ---

fn with_area_zero(set: Set) -> (Set, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let area_zero = db.add(CardDef::Trainer(Trainer {
        print_id: "test-area-zero",
        name: "Area Zero Underdepths",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::TeraPokemonRaisesBenchLimit,
    }));
    let tera_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: vec![sim::card::Marker::Ex, sim::card::Marker::Tera],
        print_id: "test-tera-area-zero",
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
    (Set { db, ..set }, area_zero, tera_mon)
}

fn fill_bench(state: &mut GameState, player: PlayerId, def: CardDefId, count: usize) {
    for _ in 0..count {
        let card = deal_new_card(state, player, def);
        let pokemon = state.put_into_play(player, card);
        state.players[player.index()].bench.push(pokemon);
    }
}

#[test]
fn area_zero_raises_the_bench_limit_only_with_an_own_tera_pokemon() {
    let (set, area_zero, tera_mon) = with_area_zero(build());
    let mut state = game(&set, area_zero, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, area_zero);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    // Top off to exactly 5, whatever setup itself already benched.
    let to_fill = 5 - state.player(player).bench.len();
    fill_bench(&mut state, player, set.mon, to_fill);
    assert_eq!(state.player(player).bench.len(), 5);
    assert!(
        !legal_actions(&state).iter().any(|a| matches!(a, Action::PlayBasic { .. })),
        "no Tera Pokemon yet: still capped at 5"
    );

    let tera_card = deal_new_card(&mut state, player, tera_mon);
    let tera_pokemon = state.put_into_play(player, tera_card);
    state.players[player.index()].bench.push(tera_pokemon);
    assert_eq!(state.player(player).bench.len(), 6);

    fill_bench(&mut state, player, set.mon, 1);
    assert_eq!(state.player(player).bench.len(), 7);
    let basic_in_hand = deal_new_card(&mut state, player, set.mon);
    state.players[player.index()].hand.push(basic_in_hand);
    assert!(
        legal_actions(&state).iter().any(|a| matches!(a, Action::PlayBasic { .. })),
        "a Tera Pokemon in play raises the limit to 8"
    );

    apply(&mut state, Action::PlayBasic { card: basic_in_hand }).unwrap();
    assert_eq!(state.player(player).bench.len(), 8);
    let basic_in_hand = deal_new_card(&mut state, player, set.mon);
    state.players[player.index()].hand.push(basic_in_hand);
    assert!(
        !legal_actions(&state).iter().any(|a| matches!(a, Action::PlayBasic { .. })),
        "8 is still a limit"
    );
}

#[test]
fn losing_the_last_tera_pokemon_forces_a_discard_down_to_five() {
    let (set, area_zero, tera_mon) = with_area_zero(build());
    let mut state = game(&set, area_zero, 3);
    let player = state.current;
    let opponent = player.opponent();
    let played = ensure_in_hand(&mut state, player, area_zero);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    // The Tera Pokemon is the Active itself — a low-HP stand-in, so a
    // single attack Knocks it out — and it alone is what raises the
    // Bench limit for the 6 filler Pokemon behind it.
    let tera_card = deal_new_card(&mut state, player, tera_mon);
    let low_hp_tera = state.put_into_play(player, tera_card);
    state.pokemon[low_hp_tera.index()].damage = 190; // 10 HP left, of 200
    state.players[player.index()].active = Some(low_hp_tera);
    fill_bench(&mut state, player, set.mon, 6);
    assert!(state.player(player).bench.len() > 5, "the raised limit allows this");

    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    assert_eq!(state.current, opponent, "the opponent's own turn, to attack");

    // A known Active, so its printed attack (Tackle, 10 damage) is
    // guaranteed to match the low HP left on `low_hp_tera` exactly.
    let opp_active_card = deal_new_card(&mut state, opponent, set.mon);
    let opp_active = state.put_into_play(opponent, opp_active_card);
    state.players[opponent.index()].active = Some(opp_active);
    let energy = deal_new_card(&mut state, opponent, set.energy);
    state.pokemon[opp_active.index()].attached.push(energy);
    let attack = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("the opponent's Active is paid for");
    apply(&mut state, attack).unwrap();

    assert_eq!(
        state.phase,
        Phase::DiscardingBenchDownTo { player, then: None },
        "the last Tera Pokemon just left play, over the raised limit"
    );

    while state.player(player).bench.len() > 5 {
        let pokemon = state.player(player).bench[0];
        apply(&mut state, Action::DiscardBenchedPokemon { pokemon }).unwrap();
    }
    assert_eq!(state.player(player).bench.len(), 5);
    // The Knocked Out Active itself still needs replacing — bench
    // cleanup runs first (`settle`'s own ordering), promotion after.
    assert!(
        matches!(state.phase, Phase::Promoting { of, .. } if of == player),
        "the Active that was Knocked Out still needs promoting: {:?}",
        state.phase
    );
}

#[test]
fn area_zero_leaving_play_discards_both_players_down_to_five_owner_first() {
    let (set, area_zero, tera_mon) = with_area_zero(build());
    let mut db = set.db.clone();
    let replacement = db.add(CardDef::Trainer(Trainer {
        print_id: "test-replacement-stadium",
        name: "Some Other Stadium",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::Nothing,
    }));
    let set = Set { db, ..set };
    let mut state = game(&set, area_zero, 3);
    let player = state.current;
    let opponent = player.opponent();

    let played = ensure_in_hand(&mut state, player, area_zero);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    for who in [player, opponent] {
        let tera_card = deal_new_card(&mut state, who, tera_mon);
        let tera_pokemon = state.put_into_play(who, tera_card);
        state.players[who.index()].bench.push(tera_pokemon);
        fill_bench(&mut state, who, set.mon, 6);
        assert!(state.player(who).bench.len() > 5, "well over the un-raised limit");
    }

    // Rule 13: one Stadium a turn — already spent this turn on Area
    // Zero itself, so play the replacement on `player`'s next turn.
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    assert_eq!(state.current, player, "back to Area Zero's own owner");

    // Never part of the decklist, so dealt directly rather than found
    // through `ensure_in_hand`.
    let replacement_card = deal_new_card(&mut state, player, replacement);
    state.players[player.index()].hand.push(replacement_card);
    apply(&mut state, Action::PlayTrainer { card: replacement_card }).unwrap();

    assert_eq!(
        state.phase,
        Phase::DiscardingBenchDownTo { player, then: Some(opponent) },
        "Area Zero's own owner discards first"
    );

    while state.player(player).bench.len() > 5 {
        let pokemon = state.player(player).bench[0];
        apply(&mut state, Action::DiscardBenchedPokemon { pokemon }).unwrap();
    }
    assert_eq!(
        state.phase,
        Phase::DiscardingBenchDownTo { player: opponent, then: None },
        "then the opponent"
    );

    while state.player(opponent).bench.len() > 5 {
        let pokemon = state.player(opponent).bench[0];
        apply(&mut state, Action::DiscardBenchedPokemon { pokemon }).unwrap();
    }
    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.player(player).bench.len(), 5);
    assert_eq!(state.player(opponent).bench.len(), 5);
}

#[test]
fn area_zero_underdepths_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Area Zero Underdepths" && c.playable.is_some()),
        "Area Zero Underdepths should play"
    );
}

// --- Beyond the map: a Stadium that blocks damage counters placed on the Bench ---

fn with_battle_cage(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-battle-cage",
        name: "Battle Cage",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::PreventsDamageCountersOnBench,
    }));
    (Set { db, ..set }, card)
}

#[test]
fn battle_cage_blocks_a_damage_counter_spread_attack_onto_the_bench() {
    let (set, card) = with_battle_cage(build());
    let mut db = set.db.clone();
    let spreader = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-spreader",
        name: "Spreadmon",
        hp: 200,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Spread Blast",
            cost: vec![Type::Colorless],
            base_damage: 0,
            inflicts: None,
            effect: Some(sim::card::AttackEffect::DamageCountersToOpponentBenchAnyWay(3)),
        }],
    }));
    let set = Set { db, ..set };
    let mut state = game(&set, card, 3);
    let player = state.current;
    let opponent = player.opponent();
    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    let attacker_card = deal_new_card(&mut state, player, spreader);
    let attacker = state.put_into_play(player, attacker_card);
    state.players[player.index()].active = Some(attacker);
    let energy = deal_new_card(&mut state, player, set.energy);
    state.pokemon[attacker.index()].attached.push(energy);

    let bench_card = deal_new_card(&mut state, opponent, set.mon);
    let bench_mon = state.put_into_play(opponent, bench_card);
    state.players[opponent.index()].bench.push(bench_mon);

    let attack = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("the spreader is paid for");
    apply(&mut state, attack).unwrap();

    assert_eq!(state.phase, Phase::Main, "no unblocked Bench target, so the effect fizzles");
    assert_eq!(state.pokemon(bench_mon).damage, 0);
}

#[test]
fn without_battle_cage_the_same_attack_still_spreads_normally() {
    let mut db = build().db;
    let spreader = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-spreader-no-cage",
        name: "Spreadmon",
        hp: 200,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Spread Blast",
            cost: vec![Type::Colorless],
            base_damage: 0,
            inflicts: None,
            effect: Some(sim::card::AttackEffect::DamageCountersToOpponentBenchAnyWay(3)),
        }],
    }));
    let set = Set { db, ..build() };
    let mut state = game(&set, spreader, 3);
    let player = state.current;
    let opponent = player.opponent();

    let attacker_card = deal_new_card(&mut state, player, spreader);
    let attacker = state.put_into_play(player, attacker_card);
    state.players[player.index()].active = Some(attacker);
    let energy = deal_new_card(&mut state, player, set.energy);
    state.pokemon[attacker.index()].attached.push(energy);

    let bench_card = deal_new_card(&mut state, opponent, set.mon);
    let bench_mon = state.put_into_play(opponent, bench_card);
    state.players[opponent.index()].bench.push(bench_mon);

    let attack = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("the spreader is paid for");
    apply(&mut state, attack).unwrap();

    assert!(matches!(state.phase, Phase::DistributingDamageCounters { .. }), "no Battle Cage in play");
    apply(&mut state, Action::PlaceDamageCounter { target: bench_mon }).unwrap();
    assert_eq!(state.pokemon(bench_mon).damage, 10);
}

#[test]
fn battle_cage_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Battle Cage" && c.playable.is_some()),
        "Battle Cage should play"
    );
}

// --- No Pokemon has an Ability while this Stadium stands ---

fn with_watchtower(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-watchtower",
        name: "Team Rocket's Watchtower",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::AbilitiesDisabled,
    }));
    (Set { db, ..set }, card)
}

fn abled_mon(db: &mut CardDb, print_id: &'static str) -> CardDefId {
    db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id,
        name: "Drawmon",
        hp: 200,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: Some(Ability {
            name: "Draw Power",
            effect: AbilityEffect::OncePerTurnWhileActiveMayDrawCards(1),
        }),
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }))
}

#[test]
fn watchtower_offers_no_ability_from_either_side() {
    let (set, card) = with_watchtower(build());
    let mut db = set.db.clone();
    let mine = abled_mon(&mut db, "test-drawmon-mine");
    let theirs = abled_mon(&mut db, "test-drawmon-theirs");
    let set = Set { db, ..set };
    let mut state = game(&set, card, 3);
    let player = state.current;
    let opponent = player.opponent();
    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    let mine_card = deal_new_card(&mut state, player, mine);
    let mine_active = state.put_into_play(player, mine_card);
    state.players[player.index()].active = Some(mine_active);

    let their_card = deal_new_card(&mut state, opponent, theirs);
    let their_active = state.put_into_play(opponent, their_card);
    state.players[opponent.index()].active = Some(their_active);

    assert!(
        !legal_actions(&state).iter().any(|a| matches!(a, Action::UseAbility { .. })),
        "Team Rocket's Watchtower leaves no Ability to use, either side's"
    );
}

#[test]
fn without_watchtower_the_same_ability_is_offered() {
    let mut db = build().db;
    let mine = abled_mon(&mut db, "test-drawmon-no-tower");
    let set = Set { db, ..build() };
    let mut state = game(&set, mine, 3);
    let player = state.current;

    let mine_card = deal_new_card(&mut state, player, mine);
    let mine_active = state.put_into_play(player, mine_card);
    state.players[player.index()].active = Some(mine_active);

    assert!(
        legal_actions(&state).iter().any(|a| matches!(a, Action::UseAbility { pokemon } if *pokemon == mine_active)),
        "no Watchtower in play"
    );
}

#[test]
fn watchtower_leaves_a_non_colorless_pokemons_ability_alone() {
    let (set, card) = with_watchtower(build());
    let mut db = set.db.clone();
    let grass_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-drawmon-grass",
        name: "Drawmon",
        hp: 200,
        kind: Type::Grass,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: Some(Ability {
            name: "Draw Power",
            effect: AbilityEffect::OncePerTurnWhileActiveMayDrawCards(1),
        }),
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let set = Set { db, ..set };
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    let mine_card = deal_new_card(&mut state, player, grass_mon);
    let mine_active = state.put_into_play(player, mine_card);
    state.players[player.index()].active = Some(mine_active);

    assert!(
        legal_actions(&state)
            .iter()
            .any(|a| matches!(a, Action::UseAbility { pokemon } if *pokemon == mine_active)),
        "Team Rocket's Watchtower names only {{C}} Pokémon; a Grass Pokémon keeps its Ability"
    );
}

#[test]
fn team_rockets_watchtower_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Team Rocket's Watchtower" && c.playable.is_some()),
        "Team Rocket's Watchtower should play"
    );
}

// --- Beyond the field: static damage / HP Stadiums ---

fn plain_stadium(db: &mut CardDb, print_id: &'static str, name: &'static str, effect: TrainerEffect) -> CardDefId {
    db.add(CardDef::Trainer(Trainer {
        print_id,
        name,
        kind: TrainerKind::Stadium,
        requirement: None,
        effect,
    }))
}

#[test]
fn lively_stadium_gives_every_basic_thirty_more_hp() {
    let mut set = build();
    let lively = plain_stadium(&mut set.db, "test-lively", "Lively Stadium", TrainerEffect::StadiumBoostsBasicHp(30));
    let mut state = game(&set, lively, 3);
    let player = state.current;
    let opponent = player.opponent();
    let played = ensure_in_hand(&mut state, player, lively);

    let my_basic = state.player(player).active.unwrap();
    let their_basic = state.player(opponent).active.unwrap();
    let evo = deal_new_card(&mut state, player, set.stage2);
    let evo_mon = state.put_into_play(player, evo);
    state.players[player.index()].bench.push(evo_mon);

    let my_before = state.pokemon_def(my_basic).hp;
    let evo_before = state.effective_hp(evo_mon);

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    assert_eq!(state.effective_hp(my_basic), my_before + 30);
    assert_eq!(state.effective_hp(their_basic), state.pokemon_def(their_basic).hp + 30);
    assert_eq!(state.effective_hp(evo_mon), evo_before, "not a Basic");
}

#[test]
fn full_metal_lab_softens_attacks_against_metal_pokemon_both_sides() {
    let mut set = build();
    let lab = plain_stadium(
        &mut set.db,
        "test-full-metal-lab",
        "Full Metal Lab",
        TrainerEffect::StadiumReducesDamageToType { kind: Type::Metal, amount: 30 },
    );
    let metal_mon = set.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-fml-metal",
        name: "Ironmon",
        hp: 150,
        kind: Type::Metal,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack { name: "Tap", cost: vec![Type::Colorless], base_damage: 10, inflicts: None, effect: None }],
    }));
    let mut state = game(&set, lab, 3);
    let attacker_player = state.current;
    let defender_player = attacker_player.opponent();
    let played = ensure_in_hand(&mut state, attacker_player, lab);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    // Give the defender a Metal Active, then attack it.
    let d_card = deal_new_card(&mut state, defender_player, metal_mon);
    let d = state.put_into_play(defender_player, d_card);
    state.players[defender_player.index()].active = Some(d);

    let attacker = state.player(attacker_player).active.unwrap();
    let energy = state.player(attacker_player).deck.iter().find(|c| state.def_of(**c).is_energy()).copied().unwrap();
    state.players[attacker_player.index()].deck.retain(|c| *c != energy);
    state.pokemon[attacker.index()].attached.push(energy);
    let attack = legal_actions(&state).into_iter().find(|a| matches!(a, Action::Attack { .. })).unwrap();
    apply(&mut state, attack).unwrap();

    // Fixture attacker's Tackle is 10; Full Metal Lab floors it to 0.
    assert_eq!(state.pokemon(d).damage, 0);
}

#[test]
fn the_static_stadiums_are_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    for name in ["Lively Stadium", "Full Metal Lab"] {
        assert!(
            import.cards.iter().any(|c| c.name == name && c.playable.is_some()),
            "{name} should play",
        );
    }
}

// --- Beyond the field: two more standing Stadium actions ---

#[test]
fn prism_tower_trades_two_discards_for_one_draw() {
    let mut db = build().db;
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-prism-tower",
        name: "Prism Tower",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::StadiumMayDiscardTwoToDrawOne,
    }));
    let set = Set { db, ..build() };
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    assert!(legal_actions(&state).contains(&Action::UsePrismTower));
    apply(&mut state, Action::UsePrismTower).unwrap();
    let hand_before_draw = state.player(player).hand.len();
    let discards: Vec<_> = state.player(player).hand[..2].to_vec();
    for c in discards {
        apply(&mut state, Action::DiscardFromHand { card: c }).unwrap();
    }
    apply(&mut state, Action::FinishDiscardingFromHand).unwrap();

    assert_eq!(state.player(player).hand.len(), hand_before_draw - 2 + 1);
    assert_eq!(state.phase, Phase::Main);
    assert!(!legal_actions(&state).contains(&Action::UsePrismTower), "once a turn");
}

#[test]
fn prism_tower_declining_draws_nothing() {
    let mut db = build().db;
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-prism-tower-2",
        name: "Prism Tower",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::StadiumMayDiscardTwoToDrawOne,
    }));
    let set = Set { db, ..build() };
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    apply(&mut state, Action::UsePrismTower).unwrap();
    let hand_before = state.player(player).hand.len();
    let one = state.player(player).hand[0];
    apply(&mut state, Action::DiscardFromHand { card: one }).unwrap();
    apply(&mut state, Action::FinishDiscardingFromHand).unwrap();

    assert_eq!(
        state.player(player).hand.len(),
        hand_before - 1,
        "only one discard happened, so no draw"
    );
}

#[test]
fn community_center_heals_everyone_only_after_a_supporter() {
    let mut db = build().db;
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-community-center",
        name: "Community Center",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::StadiumMayHealAllIfPlayedSupporter(10),
    }));
    let supporter = db.add(CardDef::Trainer(Trainer {
        print_id: "test-a-plain-supporter",
        name: "A Plain Supporter",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::Draw(1),
    }));
    let set = Set { db, ..build() };
    let mut state = game(&set, card, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let bench = state.player(player).bench[0];
    state.pokemon[active.index()].damage = 30;
    state.pokemon[bench.index()].damage = 30;

    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    assert!(
        !legal_actions(&state).contains(&Action::UseCommunityCenter),
        "no Supporter played yet this turn"
    );

    let supporter_card = deal_new_card(&mut state, player, supporter);
    state.players[player.index()].hand.push(supporter_card);
    apply(&mut state, Action::PlayTrainer { card: supporter_card }).unwrap();

    assert!(legal_actions(&state).contains(&Action::UseCommunityCenter));
    apply(&mut state, Action::UseCommunityCenter).unwrap();

    assert_eq!(state.pokemon(active).damage, 20);
    assert_eq!(state.pokemon(bench).damage, 20);
}

#[test]
fn the_two_more_standing_stadiums_are_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    for name in ["Prism Tower", "Community Center"] {
        assert!(
            import.cards.iter().any(|c| c.name == name && c.playable.is_some()),
            "{name} should play",
        );
    }
}

// --- Beyond the field: a name-prefix damage boost, and an exact-name retreat cut ---

#[test]
fn postwick_boosts_damage_for_hops_pokemon_either_side() {
    let mut db = build().db;
    let hop_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-hops-mon",
        name: "Hop's Wooloo",
        hp: 90,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack { name: "Tackle", cost: vec![Type::Colorless], base_damage: 10, inflicts: None, effect: None }],
    }));
    let postwick = db.add(CardDef::Trainer(Trainer {
        print_id: "test-postwick",
        name: "Postwick",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::StadiumBoostsDamageForNamePrefix { word: "Hop's", amount: 30 },
    }));
    let set = Set { db, ..build() };
    let mut state = game(&set, postwick, 3);
    let attacker_player = state.current;
    let defender_player = attacker_player.opponent();
    let played = ensure_in_hand(&mut state, attacker_player, postwick);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    let hop_card = deal_new_card(&mut state, attacker_player, hop_mon);
    let hop_attacker = state.put_into_play(attacker_player, hop_card);
    state.players[attacker_player.index()].active = Some(hop_attacker);
    let energy = deal_new_card(&mut state, attacker_player, set.energy);
    state.pokemon[hop_attacker.index()].attached.push(energy);
    let defender = state.player(defender_player).active.unwrap();

    let attack = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("the Hop's Pokemon is paid for");
    apply(&mut state, attack).unwrap();

    assert_eq!(state.pokemon(defender).damage, 40, "10 base plus Postwick's 30");
}

#[test]
fn postwick_does_nothing_for_an_ordinary_attacker() {
    let mut db = build().db;
    let postwick = db.add(CardDef::Trainer(Trainer {
        print_id: "test-postwick-ordinary",
        name: "Postwick",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::StadiumBoostsDamageForNamePrefix { word: "Hop's", amount: 30 },
    }));
    let set = Set { db, ..build() };
    let mut state = game(&set, postwick, 3);
    let attacker_player = state.current;
    let defender_player = attacker_player.opponent();
    let played = ensure_in_hand(&mut state, attacker_player, postwick);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    let defender = state.player(defender_player).active.unwrap();

    let energy = deal_new_card(&mut state, attacker_player, set.energy);
    let attacker = state.player(attacker_player).active.unwrap();
    state.pokemon[attacker.index()].attached.push(energy);
    let attack = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("the ordinary Active is paid for");
    apply(&mut state, attack).unwrap();

    assert_eq!(state.pokemon(defender).damage, 10, "no Postwick bonus for a non-Hop's attacker");
}

#[test]
fn postwick_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Postwick" && c.playable.is_some()),
        "Postwick should play"
    );
}

#[test]
fn paradise_resort_reduces_retreat_cost_for_psyduck_by_name_either_side() {
    let mut db = build().db;
    let psyduck = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-psyduck",
        name: "Psyduck",
        hp: 60,
        kind: Type::Water,
        weakness: None,
        resistance: None,
        retreat_cost: 2,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![],
    }));
    let resort = db.add(CardDef::Trainer(Trainer {
        print_id: "test-paradise-resort",
        name: "Paradise Resort",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::StadiumReducesRetreatCostForName("Psyduck", 1),
    }));
    let set = Set { db, ..build() };
    let mut state = game(&set, resort, 3);
    let player = state.current;
    let opponent = player.opponent();
    let played = ensure_in_hand(&mut state, player, resort);

    let mine = deal_new_card(&mut state, player, psyduck);
    let mine_mon = state.put_into_play(player, mine);
    state.players[player.index()].bench.push(mine_mon);
    let theirs = deal_new_card(&mut state, opponent, psyduck);
    let their_mon = state.put_into_play(opponent, theirs);
    state.players[opponent.index()].bench.push(their_mon);
    let ordinary = state.player(player).active.unwrap();

    assert_eq!(state.effective_retreat_cost(mine_mon), 2);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    assert_eq!(state.effective_retreat_cost(mine_mon), 1, "my own Psyduck");
    assert_eq!(state.effective_retreat_cost(their_mon), 1, "the opponent's too");
    assert_eq!(
        state.effective_retreat_cost(ordinary),
        1,
        "an ordinary Pokémon keeps its printed cost"
    );
}

#[test]
fn paradise_resort_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Paradise Resort" && c.playable.is_some()),
        "Paradise Resort should play"
    );
}
