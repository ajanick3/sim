//! Milestone 7: every Supporter in the field. One fixture, shared across
//! this milestone's tickets, the same way `second_batch.rs` served
//! milestones 5 and 6.

use sim::action::{Action, legal_actions};
use sim::card::{
    Attack, CardDb, CardDef, CardFilter, Destination, Energy, Pokemon, Requirement, Slot, Stage,
    Trainer, TrainerEffect, TrainerKind, Type, Zone,
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

// --- Ticket 01: Lana's Aid ---

fn with_lanas_aid(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let lanas_aid = db.add(CardDef::Trainer(Trainer {
        print_id: "test-lanas-aid",
        name: "Lana's Aid",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Discard,
            slots: vec![Slot {
                filter: CardFilter::PokemonWithoutRuleBoxOrBasicEnergy,
                to: Destination::Zone(Zone::Hand),
                limit: 3,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    }));
    (Set { db, ..set }, lanas_aid)
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

#[test]
fn lanas_aid_offers_a_rule_box_free_pokemon_and_basic_energy_but_not_an_ex() {
    let (set, lanas_aid) = with_lanas_aid(build());
    let mut state = game(&set, lanas_aid, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, lanas_aid);

    let ordinary = deal_to_discard(&mut state, player, set.mon);
    let with_rule_box = deal_to_discard(&mut state, player, set.mon_ex);
    let energy = deal_to_discard(&mut state, player, set.energy);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let choices = offered(&state);
    assert!(choices.contains(&ordinary), "a plain Pokémon is offered");
    assert!(choices.contains(&energy), "a Basic Energy is offered");
    assert!(
        !choices.contains(&with_rule_box),
        "a Pokémon ex carries a Rule Box and must not be offered"
    );
}

#[test]
fn lanas_aid_takes_up_to_three_in_any_combination() {
    let (set, lanas_aid) = with_lanas_aid(build());
    let mut state = game(&set, lanas_aid, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, lanas_aid);
    deal_to_discard(&mut state, player, set.mon);
    deal_to_discard(&mut state, player, set.energy);
    deal_to_discard(&mut state, player, set.energy);
    let hand_before = state.player(player).hand.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    for _ in 0..3 {
        let take = offered(&state)[0];
        apply(&mut state, Action::TakeCard { card: take }).unwrap();
    }
    assert!(offered(&state).is_empty(), "three is the limit the card prints");
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.phase, Phase::Main);
    // Three taken, and the Supporter itself left the hand.
    assert_eq!(state.player(player).hand.len(), hand_before + 2);
}

#[test]
fn lanas_aid_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let lanas_aid = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Lana's Aid")
        .expect("Lana's Aid plays");
    assert_eq!(
        lanas_aid.effect,
        TrainerEffect::Decide {
            from: Zone::Discard,
            slots: vec![Slot {
                filter: CardFilter::PokemonWithoutRuleBoxOrBasicEnergy,
                to: Destination::Zone(Zone::Hand),
                limit: 3,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        }
    );
}

// --- Ticket 02: Rust Syndicate Grunt ---

fn with_rust_syndicate_grunt(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let grunt = db.add(CardDef::Trainer(Trainer {
        print_id: "test-rust-syndicate-grunt",
        name: "Rust Syndicate Grunt",
        kind: TrainerKind::Supporter,
        requirement: Some(Requirement::KnockedOutDuringOpponentsLastTurn),
        effect: TrainerEffect::DiscardOpponentEnergy,
    }));
    (Set { db, ..set }, grunt)
}

fn end_turn_and_advance(state: &mut GameState) {
    apply(state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(state)[0];
        apply(state, first).unwrap();
    }
}

fn make_lethally_damaged(state: &mut GameState, pokemon: PokemonId) {
    let hp = state.pokemon_def(pokemon).hp;
    state.pokemon[pokemon.index()].damage = hp;
}

#[test]
fn rust_syndicate_grunt_cannot_be_played_with_no_knockout_to_point_to() {
    let (set, grunt) = with_rust_syndicate_grunt(build());
    let mut state = game(&set, grunt, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, grunt);
    assert!(!legal_actions(&state).contains(&Action::PlayTrainer { card }));
}

#[test]
fn rust_syndicate_grunt_discards_an_energy_with_no_coin_flip() {
    let (set, grunt) = with_rust_syndicate_grunt(build());
    let mut state = game(&set, grunt, 3);
    let player = state.current;
    let opponent = player.opponent();

    end_turn_and_advance(&mut state); // into the opponent's turn
    let victim = state.player(player).active.unwrap();
    make_lethally_damaged(&mut state, victim);
    end_turn_and_advance(&mut state); // Knockout, promote, into the player's turn
    assert_eq!(state.current, player);

    let card = ensure_in_hand(&mut state, player, grunt);
    assert!(legal_actions(&state).contains(&Action::PlayTrainer { card }));

    // Give the opponent's Active an Energy to lose.
    let their_active = state.player(opponent).active.unwrap();
    let energy = *state
        .player(opponent)
        .library
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    state.players[opponent.index()].library.retain(|c| *c != energy);
    state.pokemon[their_active.index()].attached.push(energy);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    assert!(
        matches!(state.phase, Phase::DiscardingOpponentEnergy { .. }),
        "no coin flip: the phase opens outright"
    );
    apply(&mut state, Action::DiscardOpponentEnergy { card: energy }).unwrap();
    assert_eq!(state.phase, Phase::Main);
    assert!(!state.pokemon(their_active).attached.contains(&energy));
}

#[test]
fn rust_syndicate_grunt_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let grunt = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Rust Syndicate Grunt")
        .expect("Rust Syndicate Grunt plays");
    assert_eq!(
        grunt.requirement,
        Some(Requirement::KnockedOutDuringOpponentsLastTurn)
    );
    assert_eq!(grunt.effect, TrainerEffect::DiscardOpponentEnergy);
}

// --- Ticket 03: N's Plan ---

fn with_ns_plan(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let ns_plan = db.add(CardDef::Trainer(Trainer {
        print_id: "test-ns-plan",
        name: "N's Plan",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::MoveEnergyFromBenchToActive { limit: 2 },
    }));
    (Set { db, ..set }, ns_plan)
}

fn attach_energy_to(state: &mut GameState, player: PlayerId, pokemon: PokemonId) -> CardId {
    let energy = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .expect("the deck is mostly Energy");
    state.players[player.index()].library.retain(|c| *c != energy);
    state.pokemon[pokemon.index()].attached.push(energy);
    energy
}

#[test]
fn ns_plan_offers_only_bench_to_active_never_the_reverse() {
    let (set, ns_plan) = with_ns_plan(build());
    let mut state = game(&set, ns_plan, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, ns_plan);

    let active = state.player(player).active.unwrap();
    let bench = state.player(player).bench[0];
    let benched_energy = attach_energy_to(&mut state, player, bench);
    let active_energy = attach_energy_to(&mut state, player, active);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let offered_cards: Vec<CardId> = legal_actions(&state)
        .into_iter()
        .filter_map(|a| match a {
            Action::MoveEnergyToActive { card } => Some(card),
            _ => None,
        })
        .collect();
    assert!(
        offered_cards.contains(&benched_energy),
        "a Benched Pokémon's Energy may move to the Active"
    );
    assert!(
        !offered_cards.contains(&active_energy),
        "the Active's own Energy is not offered; there is nowhere for it to go"
    );
}

#[test]
fn ns_plan_moves_up_to_two_and_may_stop_early() {
    let (set, ns_plan) = with_ns_plan(build());
    let mut state = game(&set, ns_plan, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, ns_plan);
    let active = state.player(player).active.unwrap();
    let bench = state.player(player).bench[0];
    let e1 = attach_energy_to(&mut state, player, bench);
    let e2 = attach_energy_to(&mut state, player, bench);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::MoveEnergyToActive { card: e1 }).unwrap();
    apply(&mut state, Action::MoveEnergyToActive { card: e2 }).unwrap();
    assert!(
        legal_actions(&state)
            .into_iter()
            .all(|a| !matches!(a, Action::MoveEnergyToActive { .. })),
        "two is the limit the card prints"
    );
    apply(&mut state, Action::FinishMovingEnergyToActive).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.pokemon(active).attached.contains(&e1));
    assert!(state.pokemon(active).attached.contains(&e2));
    assert!(!state.pokemon(bench).attached.contains(&e1));
}

#[test]
fn ns_plan_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let ns_plan = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "N's Plan")
        .expect("N's Plan plays");
    assert_eq!(
        ns_plan.effect,
        TrainerEffect::MoveEnergyFromBenchToActive { limit: 2 }
    );
}
