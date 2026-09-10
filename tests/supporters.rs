//! Milestone 7: every Supporter in the field. One fixture, shared across
//! this milestone's tickets, the same way `second_batch.rs` served
//! milestones 5 and 6.

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

    // Give the opponent's Active an Energy to lose. Without one in play
    // the card would do nothing, so it is not playable until now.
    let their_active = state.player(opponent).active.unwrap();
    let energy = *state
        .player(opponent)
        .library
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    state.players[opponent.index()].library.retain(|c| *c != energy);
    state.pokemon[their_active.index()].attached.push(energy);

    assert!(legal_actions(&state).contains(&Action::PlayTrainer { card }));
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

// --- Ticket 04: Pokémon Center Lady ---

fn with_pokemon_center_lady(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let lady = db.add(CardDef::Trainer(Trainer {
        print_id: "test-pokemon-center-lady",
        name: "Pokémon Center Lady",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::HealChosen(60),
    }));
    (Set { db, ..set }, lady)
}

#[test]
fn pokemon_center_lady_can_heal_a_benched_pokemon_not_only_the_active() {
    let (set, lady) = with_pokemon_center_lady(build());
    let mut state = game(&set, lady, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, lady);
    let bench = state.player(player).bench[0];
    state.pokemon[bench.index()].damage = 100;
    state.pokemon[bench.index()]
        .conditions
        .push(sim::card::Condition::Poisoned);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let targets: Vec<PokemonId> = legal_actions(&state)
        .into_iter()
        .filter_map(|a| match a {
            Action::HealTarget { target } => Some(target),
            _ => None,
        })
        .collect();
    assert!(
        targets.contains(&bench),
        "a Benched Pokémon may be healed, not only the Active"
    );

    apply(&mut state, Action::HealTarget { target: bench }).unwrap();
    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(bench).damage, 40, "60 healed off 100 leaves 40");
    assert!(
        state.pokemon(bench).conditions.is_empty(),
        "every Special Condition clears"
    );
}

#[test]
fn pokemon_center_lady_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let lady = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Pokémon Center Lady")
        .expect("Pokémon Center Lady plays");
    assert_eq!(lady.effect, TrainerEffect::HealChosen(60));
}

// --- Ticket 05: Rosa's Encouragement ---

fn with_rosas_encouragement(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let rosa = db.add(CardDef::Trainer(Trainer {
        print_id: "test-rosas-encouragement",
        name: "Rosa's Encouragement",
        kind: TrainerKind::Supporter,
        requirement: Some(Requirement::MorePrizesThanOpponent),
        effect: TrainerEffect::Decide {
            from: Zone::Discard,
            slots: vec![Slot {
                filter: CardFilter::BasicEnergy,
                to: Destination::Attach(TargetFilter::OfStage(Stage::Stage2)),
                limit: 2,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    }));
    (Set { db, ..set }, rosa)
}

#[test]
fn rosas_encouragement_cannot_be_played_without_more_prizes() {
    let (set, rosa) = with_rosas_encouragement(build());
    let mut state = game(&set, rosa, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, rosa);

    // A fresh game starts even, six Prizes each.
    assert!(!legal_actions(&state).contains(&Action::PlayTrainer { card }));

    // The opponent has taken a Prize; the player has not. 6 remaining
    // against the opponent's 5 is more.
    state.players[player.opponent().index()].prizes.truncate(5);
    assert!(
        legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "6 remaining against the opponent's 5 is more"
    );
}

#[test]
fn rosas_encouragement_targets_only_a_stage_2_active_or_benched() {
    let (set, rosa) = with_rosas_encouragement(build());
    let mut state = game(&set, rosa, 3);
    let player = state.current;
    state.players[player.opponent().index()].prizes.truncate(5);
    let card = ensure_in_hand(&mut state, player, rosa);

    // Put the Stage 2 into play as the Active, an ordinary Basic on the
    // Bench, so the filter is proven against both a wrong stage and a
    // wrong Bench assumption at once.
    let active = state.player(player).active.unwrap();
    let stage2_card = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.cards[c.index()].def == set.stage2)
        .unwrap();
    state.players[player.index()].library.retain(|c| *c != stage2_card);
    state.pokemon[active.index()].cards.push(stage2_card);

    let energy = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    state.players[player.index()].library.retain(|c| *c != energy);
    state.players[player.index()].discard.push(energy);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let targets: Vec<PokemonId> = legal_actions(&state)
        .into_iter()
        .filter_map(|a| match a {
            Action::TakeCardOnto { target, .. } => Some(target),
            _ => None,
        })
        .collect();
    assert!(
        targets.contains(&active),
        "the Stage 2 Active is offered, not only a Benched one"
    );
    for bench in &state.player(player).bench {
        assert!(
            !targets.contains(bench),
            "an ordinary Benched Pokémon is not a Stage 2"
        );
    }
}

#[test]
fn rosas_encouragement_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let rosa = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Rosa's Encouragement")
        .expect("Rosa's Encouragement plays");
    assert_eq!(rosa.requirement, Some(Requirement::MorePrizesThanOpponent));
    assert_eq!(
        rosa.effect,
        TrainerEffect::Decide {
            from: Zone::Discard,
            slots: vec![Slot {
                filter: CardFilter::BasicEnergy,
                to: Destination::Attach(TargetFilter::OfStage(Stage::Stage2)),
                limit: 2,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        }
    );
}

// --- Ticket 06: AZ's Tranquility and Surfer ---

fn with_az_tranquility(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let az = db.add(CardDef::Trainer(Trainer {
        print_id: "test-az-tranquility",
        name: "AZ's Tranquility",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::SwitchOwnActiveWithFollowUp(PromoteFollowUp::HealDisplacedIfEx(80)),
    }));
    (Set { db, ..set }, az)
}

fn with_surfer(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let surfer = db.add(CardDef::Trainer(Trainer {
        print_id: "test-surfer",
        name: "Surfer",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::SwitchOwnActiveWithFollowUp(PromoteFollowUp::DrawUpTo(5)),
    }));
    (Set { db, ..set }, surfer)
}

#[test]
fn az_tranquility_heals_an_ex_moved_to_the_bench_but_not_an_ordinary_mon() {
    let (set, az) = with_az_tranquility(build());
    let mut state = game(&set, az, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, az);

    // The Active is an ordinary Pokémon here; the follow-up must not fire.
    let active = state.player(player).active.unwrap();
    state.pokemon[active.index()].damage = 50;
    let bench = state.player(player).bench[0];

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::Promote { pokemon: bench }).unwrap();
    assert_eq!(
        state.pokemon(active).damage,
        50,
        "an ordinary Pokémon moved to the Bench is not healed"
    );
}

#[test]
fn az_tranquility_heals_an_ex_that_gets_displaced() {
    let (set, az) = with_az_tranquility(build());
    let mut state = game(&set, az, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, az);

    // Swap the Active for a Pokémon ex directly, bypassing the once-a-turn
    // Evolve/Play limits this fixture does not need.
    let active = state.player(player).active.unwrap();
    let ex_card = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.cards[c.index()].def == set.mon_ex)
        .unwrap();
    state.players[player.index()].library.retain(|c| *c != ex_card);
    state.pokemon[active.index()].cards = vec![ex_card];
    state.pokemon[active.index()].damage = 50;
    let bench = state.player(player).bench[0];

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::Promote { pokemon: bench }).unwrap();
    assert_eq!(
        state.pokemon(active).damage,
        0,
        "the displaced Pokémon ex heals 80, floored at zero from 50"
    );
}

#[test]
fn surfer_draws_up_to_five_once_the_switch_happens() {
    let (set, surfer) = with_surfer(build());
    let mut state = game(&set, surfer, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, surfer);
    // Empty the hand down to nothing but the card about to be played.
    state.players[player.index()].hand = vec![card];
    let bench = state.player(player).bench[0];

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::Promote { pokemon: bench }).unwrap();
    assert_eq!(state.player(player).hand.len(), 5);
}

#[test]
fn az_tranquility_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let az = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "AZ's Tranquility")
        .expect("AZ's Tranquility plays");
    assert_eq!(
        az.effect,
        TrainerEffect::SwitchOwnActiveWithFollowUp(PromoteFollowUp::HealDisplacedIfEx(80))
    );
}

#[test]
fn surfer_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let surfer = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Surfer")
        .expect("Surfer plays");
    assert_eq!(
        surfer.effect,
        TrainerEffect::SwitchOwnActiveWithFollowUp(PromoteFollowUp::DrawUpTo(5))
    );
}

// --- Ticket 07: Black Belt's Training and Gladion's Final Battle ---

fn with_black_belts_training(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-black-belts-training",
        name: "Black Belt's Training",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::BonusDamageThisTurn(40, TurnBonusTarget::OpponentActiveEx),
    }));
    (Set { db, ..set }, card)
}

fn with_gladions_final_battle(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-gladions-final-battle",
        name: "Gladion's Final Battle",
        kind: TrainerKind::Supporter,
        requirement: Some(Requirement::HandSizeIs(1)),
        effect: TrainerEffect::BonusDamageThisTurn(80, TurnBonusTarget::OpponentActiveWithoutRuleBox),
    }));
    (Set { db, ..set }, card)
}

#[test]
fn black_belts_training_adds_forty_only_against_an_ex() {
    let (set, card) = with_black_belts_training(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let opponent = player.opponent();
    let attacker = state.player(player).active.unwrap();
    let ex_defender = state.player(opponent).active.unwrap();
    let stage1 = state.player(opponent).bench[0]; // an ordinary, non-ex Pokémon

    // Force the opponent's Active to actually be an ex — setup chose
    // whatever Basic came up first, not necessarily this one.
    let ex_card = *state
        .player(opponent)
        .library
        .iter()
        .find(|c| state.cards[c.index()].def == set.mon_ex)
        .unwrap();
    state.players[opponent.index()].library.retain(|c| *c != ex_card);
    state.pokemon[ex_defender.index()].cards = vec![ex_card];

    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();

    assert_eq!(
        sim::engine::damage_dealt(&state, attacker, ex_defender, 100),
        140,
        "the ex on the Active takes the bonus"
    );
    assert_eq!(
        sim::engine::damage_dealt(&state, attacker, stage1, 100),
        100,
        "an ordinary Pokémon does not"
    );
}

#[test]
fn the_bonus_expires_once_the_turn_ends() {
    let (set, card) = with_black_belts_training(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let opponent = player.opponent();
    let attacker = state.player(player).active.unwrap();
    let defender = state.player(opponent).active.unwrap();
    let ex_card = *state
        .player(opponent)
        .library
        .iter()
        .find(|c| state.cards[c.index()].def == set.mon_ex)
        .unwrap();
    state.players[opponent.index()].library.retain(|c| *c != ex_card);
    state.pokemon[defender.index()].cards = vec![ex_card];

    let played = ensure_in_hand(&mut state, player, card);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    assert_eq!(sim::engine::damage_dealt(&state, attacker, defender, 100), 140);

    end_turn_and_advance(&mut state);
    end_turn_and_advance(&mut state);
    assert_eq!(state.current, player, "back to the same player's turn");
    assert_eq!(
        sim::engine::damage_dealt(&state, attacker, defender, 100),
        100,
        "the bonus does not survive past the turn it was played"
    );
}

#[test]
fn gladions_final_battle_cannot_be_played_holding_any_other_card() {
    let (set, card) = with_gladions_final_battle(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    assert!(
        !legal_actions(&state).contains(&Action::PlayTrainer { card: played }),
        "the hand holds more than this card"
    );

    state.players[player.index()].hand = vec![played];
    assert!(legal_actions(&state).contains(&Action::PlayTrainer { card: played }));
}

#[test]
fn gladions_final_battle_adds_eighty_only_without_a_rule_box() {
    let (set, card) = with_gladions_final_battle(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let opponent = player.opponent();
    let attacker = state.player(player).active.unwrap();
    let ordinary_defender = state.player(opponent).active.unwrap();
    // Give the opponent's Active a Rule Box (an ex) directly.
    let ex_card = *state
        .player(opponent)
        .library
        .iter()
        .find(|c| state.cards[c.index()].def == set.mon_ex)
        .unwrap();
    state.players[opponent.index()].library.retain(|c| *c != ex_card);
    state.pokemon[ordinary_defender.index()].cards = vec![ex_card];

    let played = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.cards[c.index()].def == card)
        .unwrap();
    state.players[player.index()].library.retain(|c| *c != played);
    state.players[player.index()].hand = vec![played];

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    assert_eq!(
        sim::engine::damage_dealt(&state, attacker, ordinary_defender, 100),
        100,
        "the Active now carries a Rule Box, so no bonus"
    );
}

#[test]
fn black_belts_training_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let card = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Black Belt's Training")
        .expect("Black Belt's Training plays");
    assert_eq!(
        card.effect,
        TrainerEffect::BonusDamageThisTurn(40, TurnBonusTarget::OpponentActiveEx)
    );
}

#[test]
fn gladions_final_battle_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let card = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Gladion's Final Battle")
        .expect("Gladion's Final Battle plays");
    assert_eq!(card.requirement, Some(Requirement::HandSizeIs(1)));
    assert_eq!(
        card.effect,
        TrainerEffect::BonusDamageThisTurn(80, TurnBonusTarget::OpponentActiveWithoutRuleBox)
    );
}

// --- Ticket 08: Kieran ---

fn with_kieran(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let kieran = db.add(CardDef::Trainer(Trainer {
        print_id: "test-kieran",
        name: "Kieran",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::ChooseOneOf(
            Box::new(TrainerEffect::SwitchOwnActive),
            Box::new(TrainerEffect::BonusDamageThisTurn(30, TurnBonusTarget::OpponentActiveEx)),
        ),
    }));
    (Set { db, ..set }, kieran)
}

#[test]
fn kieran_offers_a_choice_and_resolves_only_the_one_picked() {
    let (set, kieran) = with_kieran(build());
    let mut state = game(&set, kieran, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, kieran);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let options = legal_actions(&state);
    assert!(options.contains(&Action::ChooseOption { first: true }));
    assert!(options.contains(&Action::ChooseOption { first: false }));

    apply(&mut state, Action::ChooseOption { first: false }).unwrap();
    // The damage-bonus branch was chosen: no Promoting phase opens.
    assert_eq!(state.phase, Phase::Main);
    assert!(state.turn_bonus.is_some(), "the unchosen switch never ran");
}

#[test]
fn kieran_can_choose_the_switch_instead() {
    let (set, kieran) = with_kieran(build());
    let mut state = game(&set, kieran, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, kieran);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::ChooseOption { first: true }).unwrap();
    assert!(
        matches!(state.phase, Phase::Promoting { .. }),
        "the switch branch opens Promoting"
    );
    assert!(state.turn_bonus.is_none(), "the unchosen bonus never ran");
}

#[test]
fn kieran_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let kieran = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Kieran")
        .expect("Kieran plays");
    assert_eq!(
        kieran.effect,
        TrainerEffect::ChooseOneOf(
            Box::new(TrainerEffect::SwitchOwnActive),
            Box::new(TrainerEffect::BonusDamageThisTurn(30, TurnBonusTarget::OpponentActiveEx)),
        )
    );
}

// --- Ticket 09: Morty's Conviction ---

fn with_mortys_conviction(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-mortys-conviction",
        name: "Morty's Conviction",
        kind: TrainerKind::Supporter,
        requirement: Some(Requirement::DiscardOtherCardsFromHand(1)),
        effect: TrainerEffect::DrawPerOpponentBenched,
    }));
    (Set { db, ..set }, card)
}

#[test]
fn mortys_conviction_cannot_be_played_holding_nothing_else_to_discard() {
    let (set, card) = with_mortys_conviction(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    state.players[player.index()].hand = vec![played];
    assert!(!legal_actions(&state).contains(&Action::PlayTrainer { card: played }));
}

#[test]
fn mortys_conviction_draws_one_per_opponent_benched_pokemon() {
    let (set, card) = with_mortys_conviction(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let opponent = player.opponent();
    let benched_count = state.player(opponent).bench.len();
    assert!(benched_count > 0, "setup filled the opponent's Bench");

    let played = ensure_in_hand(&mut state, player, card);
    let discard = *state
        .player(player)
        .hand
        .iter()
        .find(|c| **c != played)
        .expect("something else in hand to discard");
    let hand_before = state.player(player).hand.len();
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    assert!(matches!(state.phase, Phase::Paying { .. }));
    apply(&mut state, Action::PayWithCard { card: discard }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    // The card played and the card discarded both left the hand; drawing
    // one per Benched Pokémon is what comes back.
    assert_eq!(
        state.player(player).hand.len(),
        hand_before - 2 + benched_count
    );
}

#[test]
fn mortys_conviction_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let card = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Morty's Conviction")
        .expect("Morty's Conviction plays");
    assert_eq!(
        card.requirement,
        Some(Requirement::DiscardOtherCardsFromHand(1))
    );
    assert_eq!(card.effect, TrainerEffect::DrawPerOpponentBenched);
}

// --- Ticket 10: Xerosic's Machinations and Eri ---

fn with_xerosics_machinations(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-xerosics-machinations",
        name: "Xerosic's Machinations",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::OpponentDiscardsDownTo(3),
    }));
    (Set { db, ..set }, card)
}

fn with_eri(set: Set) -> (Set, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let item = db.add(CardDef::Trainer(Trainer {
        print_id: "test-some-item",
        name: "Test Item",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::Nothing,
    }));
    let eri = db.add(CardDef::Trainer(Trainer {
        print_id: "test-eri",
        name: "Eri",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::DiscardFromOpponentsHand {
            filter: CardFilter::TrainerOfKind(TrainerKind::Item),
            limit: 2,
        },
    }));
    (Set { db, ..set }, eri, item)
}

#[test]
fn xerosics_machinations_discards_the_opponents_own_hand_to_their_own_choice() {
    let (set, card) = with_xerosics_machinations(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let opponent = player.opponent();
    let played = ensure_in_hand(&mut state, player, card);
    let opponent_hand_before = state.player(opponent).hand.len();
    assert!(opponent_hand_before > 3, "the fixture deals a full hand");

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    assert!(
        matches!(state.phase, Phase::DiscardingFromHand { chooser, of, .. }
            if chooser == opponent && of == opponent),
        "the opponent discards from their own hand: {:?}",
        state.phase
    );

    while state.player(opponent).hand.len() > 3 {
        let discard = match legal_actions(&state).into_iter().find(|a| {
            matches!(a, Action::DiscardFromHand { .. })
        }) {
            Some(Action::DiscardFromHand { card }) => card,
            _ => panic!("expected something left to discard"),
        };
        apply(&mut state, Action::DiscardFromHand { card: discard }).unwrap();
    }
    apply(&mut state, Action::FinishDiscardingFromHand).unwrap();
    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.player(opponent).hand.len(), 3);
}

#[test]
fn eri_lets_the_player_discard_only_the_opponents_items() {
    let (set, eri, item_def) = with_eri(build());
    let mut state = game(&set, eri, 3);
    let player = state.current;
    let opponent = player.opponent();
    let played = ensure_in_hand(&mut state, player, eri);

    // Deal the opponent an Item and note a non-Item already in their hand.
    let item_card = deal_new_card(&mut state, opponent, item_def);
    state.players[opponent.index()].hand.push(item_card);
    let non_item = *state
        .player(opponent)
        .hand
        .iter()
        .find(|c| **c != item_card)
        .unwrap();

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    let offered: Vec<CardId> = legal_actions(&state)
        .into_iter()
        .filter_map(|a| match a {
            Action::DiscardFromHand { card } => Some(card),
            _ => None,
        })
        .collect();
    assert!(offered.contains(&item_card), "the opponent's Item is offered");
    assert!(
        !offered.contains(&non_item),
        "a non-Item in the opponent's hand is not"
    );

    apply(&mut state, Action::DiscardFromHand { card: item_card }).unwrap();
    assert!(
        legal_actions(&state).contains(&Action::FinishDiscardingFromHand),
        "up to 2 — stopping early is legal"
    );
    apply(&mut state, Action::FinishDiscardingFromHand).unwrap();
    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(opponent).discard.contains(&item_card));
    assert!(!state.player(opponent).hand.contains(&item_card));
}

#[test]
fn xerosics_machinations_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let card = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Xerosic's Machinations")
        .expect("Xerosic's Machinations plays");
    assert_eq!(card.effect, TrainerEffect::OpponentDiscardsDownTo(3));
}

#[test]
fn eri_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let card = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Eri")
        .expect("Eri plays");
    assert_eq!(
        card.effect,
        TrainerEffect::DiscardFromOpponentsHand {
            filter: CardFilter::TrainerOfKind(TrainerKind::Item),
            limit: 2,
        }
    );
}

// --- Ticket 11: Brock's Scouting ---

fn with_brocks_scouting(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-brocks-scouting",
        name: "Brock's Scouting",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![
                Slot {
                    filter: CardFilter::PokemonOfStage(Stage::Basic),
                    to: Destination::Zone(Zone::Hand),
                    limit: 2,
                    excludes_type_of_previous: false,
                    peek: None,
                },
                Slot {
                    filter: CardFilter::EvolutionPokemon,
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                },
            ],
            then: None,
        },
    }));
    (Set { db, ..set }, card)
}

#[test]
fn brocks_scouting_takes_from_both_categories_in_one_search() {
    // The ticket that planned this milestone worried a shared sequence of
    // slots (`Dawn`'s shape) could not express "two counts, each
    // independent of the other" — but taking from one slot never reduces
    // what a later slot allows, so the two-slot search already is that,
    // with nothing new to build. This test is the check that finding
    // rests on: both categories are actually reachable from one card.
    let (set, card) = with_brocks_scouting(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);
    let hand_before = state.player(player).hand.len();

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    // Slot 0: up to 2 Basic.
    let first_basic = offered(&state)[0];
    apply(&mut state, Action::TakeCard { card: first_basic }).unwrap();
    let second_basic = offered(&state)[0];
    apply(&mut state, Action::TakeCard { card: second_basic }).unwrap();
    assert!(offered(&state).is_empty(), "two Basics is that slot's limit");
    apply(&mut state, Action::FinishDeciding).unwrap();

    // Slot 1: up to 1 Evolution — untouched by what slot 0 already took.
    let evolution = offered(&state)[0];
    assert!(state.def_of(evolution).as_pokemon().unwrap().stage != Stage::Basic);
    apply(&mut state, Action::TakeCard { card: evolution }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.phase, Phase::Main);
    // The Supporter itself, plus 2 Basics and 1 Evolution.
    assert_eq!(state.player(player).hand.len(), hand_before + 3 - 1);
}

#[test]
fn brocks_scouting_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let card = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Brock's Scouting")
        .expect("Brock's Scouting plays");
    assert_eq!(
        card.effect,
        TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![
                Slot {
                    filter: CardFilter::PokemonOfStage(Stage::Basic),
                    to: Destination::Zone(Zone::Hand),
                    limit: 2,
                    excludes_type_of_previous: false,
                    peek: None,
                },
                Slot {
                    filter: CardFilter::EvolutionPokemon,
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                },
            ],
            then: None,
        }
    );
}

// --- Ticket 12: Wally's Compassion ---

fn with_wallys_compassion(set: Set) -> (Set, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let mega_ex = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-mega-ex",
        name: "Mega Testmon ex",
        hp: 300,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 2,
        prizes: 3,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Smash",
            cost: vec![Type::Colorless, Type::Colorless],
            base_damage: 100,
            inflicts: None, effect: None,
        }],
    }));
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-wallys-compassion",
        name: "Wally's Compassion",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::HealMegaExAndTakeEnergyIfHealed,
    }));
    (Set { db, ..set }, card, mega_ex)
}

#[test]
fn wallys_compassion_heals_to_full_and_moves_energy_only_if_it_healed() {
    let (set, card, mega_ex) = with_wallys_compassion(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);

    // Put a Mega ex into play directly, with damage and Energy attached.
    let mega_card = deal_new_card(&mut state, player, mega_ex);
    let mega = state.put_into_play(player, mega_card);
    state.players[player.index()].bench.push(mega);
    let energy = deal_new_card(&mut state, player, set.energy);
    state.pokemon[mega.index()].attached.push(energy);
    state.pokemon[mega.index()].damage = 120;

    // An ordinary Pokémon must not be offered as a target.
    let ordinary = state.player(player).active.unwrap();

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    let targets: Vec<PokemonId> = legal_actions(&state)
        .into_iter()
        .filter_map(|a| match a {
            Action::HealMegaEx { target } => Some(target),
            _ => None,
        })
        .collect();
    assert!(targets.contains(&mega), "the Mega ex is offered");
    assert!(!targets.contains(&ordinary), "an ordinary Pokémon is not");

    apply(&mut state, Action::HealMegaEx { target: mega }).unwrap();
    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(mega).damage, 0, "healed to full");
    assert!(
        state.pokemon(mega).attached.is_empty(),
        "the Energy moved off since damage was healed"
    );
    assert!(state.player(player).hand.contains(&energy));
}

#[test]
fn wallys_compassion_moves_nothing_when_there_was_no_damage_to_heal() {
    let (set, card, mega_ex) = with_wallys_compassion(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);

    let mega_card = deal_new_card(&mut state, player, mega_ex);
    let mega = state.put_into_play(player, mega_card);
    state.players[player.index()].bench.push(mega);
    let energy = deal_new_card(&mut state, player, set.energy);
    state.pokemon[mega.index()].attached.push(energy);
    // No damage this time.

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    apply(&mut state, Action::HealMegaEx { target: mega }).unwrap();
    assert!(
        state.pokemon(mega).attached.contains(&energy),
        "nothing was healed, so the Energy stays"
    );
    assert!(!state.player(player).hand.contains(&energy));
}

#[test]
fn wallys_compassion_leaves_an_attached_tool_in_place() {
    // A regression test: "all Energy attached to it" moves Energy only,
    // not a Tool sharing the same `attached` list. Nothing built a Tool
    // yet when Wally's Compassion first shipped, so this went unchecked
    // until the Tools milestone's own audit found it.
    let (set, card, mega_ex) = with_wallys_compassion(build());
    let mut db = set.db.clone();
    let tool = db.add(CardDef::Trainer(Trainer {
        print_id: "test-a-tool",
        name: "A Tool",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::Nothing,
    }));
    let set = Set { db, ..set };
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);

    let mega_card = deal_new_card(&mut state, player, mega_ex);
    let mega = state.put_into_play(player, mega_card);
    state.players[player.index()].bench.push(mega);
    let energy = deal_new_card(&mut state, player, set.energy);
    let tool_card = deal_new_card(&mut state, player, tool);
    state.pokemon[mega.index()].attached.push(energy);
    state.pokemon[mega.index()].attached.push(tool_card);
    state.pokemon[mega.index()].damage = 120;

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    apply(&mut state, Action::HealMegaEx { target: mega }).unwrap();

    assert!(state.player(player).hand.contains(&energy), "the Energy moved");
    assert!(
        state.pokemon(mega).attached.contains(&tool_card),
        "the Tool stays attached"
    );
    assert!(!state.player(player).hand.contains(&tool_card));
}

#[test]
fn wallys_compassion_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let card = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Wally's Compassion")
        .expect("Wally's Compassion plays");
    assert_eq!(card.effect, TrainerEffect::HealMegaExAndTakeEnergyIfHealed);
}

// --- Ticket 13: Janine's Secret Art ---

fn with_janines_secret_art(set: Set) -> (Set, CardDefId, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let dark_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-dark-mon",
        name: "Darkmon",
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
        attacks: Vec::new(),
    }));
    let dark_energy = db.add(CardDef::Energy(Energy {
        print_id: "test-dark-energy",
        name: "Darkness Energy",
        kind: Type::Darkness,
        effect: None,
    }));
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-janines-secret-art",
        name: "Janine's Secret Art",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::JaninesSecretArt,
    }));
    (Set { db, ..set }, card, dark_mon, dark_energy)
}

#[test]
fn janines_secret_art_attaches_to_each_chosen_target_and_poisons_the_active() {
    let (set, card, dark_mon, dark_energy) = with_janines_secret_art(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);

    // Replace the Active with a Darkness Pokémon, and give a Benched one
    // too — both Darkness, so both are legal choices.
    let active = state.player(player).active.unwrap();
    let active_dark_card = deal_new_card(&mut state, player, dark_mon);
    state.pokemon[active.index()].cards = vec![active_dark_card];
    let bench_dark = state.player(player).bench[0];
    let bench_dark_card = deal_new_card(&mut state, player, dark_mon);
    state.pokemon[bench_dark.index()].cards = vec![bench_dark_card];

    // Two Darkness Energy sit in the deck for the two searches.
    let e1 = deal_new_card(&mut state, player, dark_energy);
    let e2 = deal_new_card(&mut state, player, dark_energy);
    state.players[player.index()].library.push(e1);
    state.players[player.index()].library.push(e2);

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    let targets: Vec<PokemonId> = legal_actions(&state)
        .into_iter()
        .filter_map(|a| match a {
            Action::ChooseJaninesTarget { target } => Some(target),
            _ => None,
        })
        .collect();
    assert!(targets.contains(&active) && targets.contains(&bench_dark));

    apply(&mut state, Action::ChooseJaninesTarget { target: active }).unwrap();
    apply(&mut state, Action::ChooseJaninesTarget { target: bench_dark }).unwrap();
    apply(&mut state, Action::FinishChoosingJaninesTargets).unwrap();

    // The first target's search.
    let take = match legal_actions(&state).into_iter().find(|a| {
        matches!(a, Action::TakeEnergyForJanine { .. })
    }) {
        Some(Action::TakeEnergyForJanine { card }) => card,
        other => panic!("expected an Energy to take: {other:?}"),
    };
    apply(&mut state, Action::TakeEnergyForJanine { card: take }).unwrap();
    // The second target's search.
    let take = match legal_actions(&state).into_iter().find(|a| {
        matches!(a, Action::TakeEnergyForJanine { .. })
    }) {
        Some(Action::TakeEnergyForJanine { card }) => card,
        other => panic!("expected an Energy to take: {other:?}"),
    };
    apply(&mut state, Action::TakeEnergyForJanine { card: take }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(active).attached.len(), 1);
    assert_eq!(state.pokemon(bench_dark).attached.len(), 1);
    assert!(
        state.has_condition(active, sim::card::Condition::Poisoned),
        "the Active got an Energy this way, so it is Poisoned"
    );
    assert!(
        !state.has_condition(bench_dark, sim::card::Condition::Poisoned),
        "a Benched Pokémon is never Poisoned by this"
    );
}

#[test]
fn janines_secret_art_may_choose_fewer_than_two_or_none() {
    let (set, card, ..) = with_janines_secret_art(build());
    let mut state = game(&set, card, 3);
    let player = state.current;
    let played = ensure_in_hand(&mut state, player, card);

    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    assert!(legal_actions(&state).contains(&Action::FinishChoosingJaninesTargets));
    apply(&mut state, Action::FinishChoosingJaninesTargets).unwrap();
    assert_eq!(
        state.phase,
        Phase::Main,
        "choosing none ends the card with nothing to search"
    );
}

#[test]
fn janines_secret_art_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let card = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Janine's Secret Art")
        .expect("Janine's Secret Art plays");
    assert_eq!(card.effect, TrainerEffect::JaninesSecretArt);
}

// --- Beyond the map: a bonus Prize when a Tera attacker knocks out ---

fn with_briar(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let briar = db.add(CardDef::Trainer(Trainer {
        print_id: "test-briar",
        name: "Briar",
        kind: TrainerKind::Supporter,
        requirement: Some(Requirement::OpponentPrizesExactly(2)),
        effect: TrainerEffect::GrantsBonusPrizeIfOwnTeraAttackerKnocksOutThisTurn,
    }));
    (Set { db, ..set }, briar)
}

#[test]
fn briar_grants_a_bonus_prize_when_the_own_tera_attacker_knocks_out() {
    let (set, briar) = with_briar(build());
    let mut db = set.db.clone();
    let tera_attacker = db.add(CardDef::Pokemon(Pokemon {
        markers: vec![sim::card::Marker::Ex, sim::card::Marker::Tera],
        print_id: "test-tera-attacker",
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
            name: "Big Tackle",
            cost: vec![Type::Colorless],
            base_damage: 100,
            inflicts: None,
            effect: None,
        }],
    }));
    let set = Set { db, ..set };

    let mut state = game(&set, briar, 3);
    let player = state.current;
    let opponent = player.opponent();
    state.players[opponent.index()].prizes.truncate(2);

    // Swap the attacking player's Active for the Tera attacker.
    let attacker_card = deal_new_card(&mut state, player, tera_attacker);
    let attacker = state.put_into_play(player, attacker_card);
    state.players[player.index()].active = Some(attacker);
    let energy_card = deal_new_card(&mut state, player, set.energy);
    state.pokemon[attacker.index()].attached.push(energy_card);

    let played = ensure_in_hand(&mut state, player, briar);
    apply(&mut state, Action::PlayTrainer { card: played }).unwrap();
    assert_eq!(state.phase, Phase::Main);

    let prizes_before = state.player(player).prizes.len();
    let attack = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("the Tera attacker is paid for");
    apply(&mut state, attack).unwrap();

    assert_eq!(
        state.player(player).prizes.len(),
        prizes_before - 2,
        "1 Prize for the Knockout, 1 more from Briar"
    );
}

#[test]
fn no_bonus_prize_without_briar_played_this_turn() {
    let (set, _briar) = with_briar(build());
    let mut db = set.db.clone();
    let tera_attacker = db.add(CardDef::Pokemon(Pokemon {
        markers: vec![sim::card::Marker::Ex, sim::card::Marker::Tera],
        print_id: "test-tera-attacker-no-briar",
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
            name: "Big Tackle",
            cost: vec![Type::Colorless],
            base_damage: 100,
            inflicts: None,
            effect: None,
        }],
    }));
    let set = Set { db, ..set };

    let mut state = game(&set, set.mon, 3);
    let player = state.current;
    let opponent = player.opponent();
    state.players[opponent.index()].prizes.truncate(2);

    let attacker_card = deal_new_card(&mut state, player, tera_attacker);
    let attacker = state.put_into_play(player, attacker_card);
    state.players[player.index()].active = Some(attacker);
    let energy_card = deal_new_card(&mut state, player, set.energy);
    state.pokemon[attacker.index()].attached.push(energy_card);

    let prizes_before = state.player(player).prizes.len();
    let attack = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("the Tera attacker is paid for");
    apply(&mut state, attack).unwrap();

    assert_eq!(state.player(player).prizes.len(), prizes_before - 1, "no Briar played this turn");
}

#[test]
fn briar_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let card = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Briar")
        .expect("Briar plays");
    assert_eq!(card.effect, TrainerEffect::GrantsBonusPrizeIfOwnTeraAttackerKnocksOutThisTurn);
}

// --- Beyond the field: plain-draw Supporters ---

fn with_plain_draw(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let cheren = db.add(CardDef::Trainer(Trainer {
        print_id: "test-cheren",
        name: "Cheren",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::Draw(3),
    }));
    (Set { db, ..set }, cheren)
}

#[test]
fn cheren_draws_three() {
    let (set, cheren) = with_plain_draw(build());
    let mut state = game(&set, cheren, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, cheren);
    let before = state.player(player).hand.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    // Cheren left the hand for the discard, then drew three.
    assert_eq!(state.player(player).hand.len(), before - 1 + 3);
}

#[test]
fn the_plain_draw_supporters_are_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    for name in ["Cheren", "Friends in Paldea", "Urbain"] {
        assert!(
            import.cards.iter().any(|c| c.name == name && c.playable.is_some()),
            "{name} should play",
        );
    }
}

// --- Beyond the field: heal-each and heal-Active Supporters ---

fn with_heals(set: Set) -> (Set, CardDefId, CardDefId, CardDefId, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let cook = db.add(CardDef::Trainer(Trainer {
        print_id: "test-cook",
        name: "Cook",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::HealActive(70),
    }));
    let fennel = db.add(CardDef::Trainer(Trainer {
        print_id: "test-fennel",
        name: "Fennel",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::HealEachYours { amount: 40, of_type: None },
    }));
    let clemont = db.add(CardDef::Trainer(Trainer {
        print_id: "test-clemont",
        name: "Clemont's Quick Wit",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::HealEachYours {
            amount: 60,
            of_type: Some(Type::Lightning),
        },
    }));
    let lightning_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-lightning-mon",
        name: "Sparkmon",
        hp: 120,
        kind: Type::Lightning,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Zap",
            cost: vec![Type::Lightning],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    (Set { db, ..set }, cook, fennel, clemont, lightning_mon, lightning_mon)
}

#[test]
fn cook_heals_seventy_from_the_active() {
    let (set, cook, ..) = with_heals(build());
    let mut state = game(&set, cook, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    state.pokemon[active.index()].damage = 90;
    let card = ensure_in_hand(&mut state, player, cook);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert_eq!(state.pokemon(active).damage, 20);
}

#[test]
fn fennel_heals_forty_from_every_pokemon() {
    let (set, _cook, fennel, ..) = with_heals(build());
    let mut state = game(&set, fennel, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let benched_card = deal_new_card(&mut state, player, set.mon);
    let benched = state.put_into_play(player, benched_card);
    state.players[player.index()].bench.push(benched);
    state.pokemon[active.index()].damage = 100;
    state.pokemon[benched.index()].damage = 30;
    let card = ensure_in_hand(&mut state, player, fennel);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert_eq!(state.pokemon(active).damage, 60);
    assert_eq!(state.pokemon(benched).damage, 0, "40 heal floors the 30");
}

#[test]
fn clemonts_quick_wit_heals_only_lightning_pokemon() {
    let (set, _c, _f, clemont, lightning_mon, _) = with_heals(build());
    let mut state = game(&set, clemont, 3);
    let player = state.current;
    let colorless_active = state.player(player).active.unwrap();
    let spark_card = deal_new_card(&mut state, player, lightning_mon);
    let spark = state.put_into_play(player, spark_card);
    state.players[player.index()].bench.push(spark);
    state.pokemon[colorless_active.index()].damage = 100;
    state.pokemon[spark.index()].damage = 100;
    let card = ensure_in_hand(&mut state, player, clemont);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert_eq!(state.pokemon(colorless_active).damage, 100, "not a {{L}} Pokemon");
    assert_eq!(state.pokemon(spark).damage, 40, "60 healed from the {{L}} Pokemon");
}

#[test]
fn the_heal_supporters_are_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    for name in ["Cook", "Fennel", "Clemont's Quick Wit"] {
        assert!(
            import.cards.iter().any(|c| c.name == name && c.playable.is_some()),
            "{name} should play",
        );
    }
}

// --- Beyond the field: draw variants ---

fn with_draw_variants(set: Set) -> (Set, CardDefId, CardDefId, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let picnicker = db.add(CardDef::Trainer(Trainer {
        print_id: "test-picnicker",
        name: "Picnicker",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::CoinFlipDraw { heads: 4, tails: 2 },
    }));
    let jett = db.add(CardDef::Trainer(Trainer {
        print_id: "test-jett",
        name: "Jett",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::DrawPerOpponentMegaEx,
    }));
    let iris = db.add(CardDef::Trainer(Trainer {
        print_id: "test-iris",
        name: "Iris's Fighting Spirit",
        kind: TrainerKind::Supporter,
        requirement: Some(Requirement::DiscardOtherCardsFromHand(1)),
        effect: TrainerEffect::DrawUpToHandSize(6),
    }));
    let mega = db.add(CardDef::Pokemon(Pokemon {
        markers: vec![sim::card::Marker::Mega, sim::card::Marker::Ex],
        print_id: "test-mega-ex",
        name: "Megamon ex",
        hp: 330,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 3,
        prizes: 3,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Stomp",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    (Set { db, ..set }, picnicker, jett, iris, mega)
}

#[test]
fn picnicker_draws_four_on_heads_and_two_on_tails() {
    use sim::rng::ScriptedRng;
    for (flip, expected) in [(vec![1u32], 4), (vec![0u32], 2)] {
        let (set, picnicker, ..) = with_draw_variants(build());
        let decklist = deck(&set, picnicker);
        let mut state = GameState::new(
            set.db.clone(),
            [decklist.clone(), decklist],
            Box::new(ScriptedRng::new(flip)),
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
        let card = ensure_in_hand(&mut state, player, picnicker);
        let before = state.player(player).hand.len();
        apply(&mut state, Action::PlayTrainer { card }).unwrap();
        assert_eq!(state.player(player).hand.len(), before - 1 + expected);
    }
}

#[test]
fn jett_draws_one_per_opponent_mega_ex() {
    let (set, _p, jett, _i, mega) = with_draw_variants(build());
    let mut state = game(&set, jett, 3);
    let player = state.current;
    let opp = player.opponent();
    for _ in 0..2 {
        let c = deal_new_card(&mut state, opp, mega);
        let m = state.put_into_play(opp, c);
        state.players[opp.index()].bench.push(m);
    }
    let card = ensure_in_hand(&mut state, player, jett);
    let before = state.player(player).hand.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert_eq!(state.player(player).hand.len(), before - 1 + 2);
}

#[test]
fn iris_draws_up_to_six_after_the_discard_cost() {
    let (set, _p, _j, iris, _m) = with_draw_variants(build());
    let mut state = game(&set, iris, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, iris);
    // Trim the hand so drawing up to six actually draws.
    while state.player(player).hand.len() > 3 {
        let spare = *state
            .player(player)
            .hand
            .iter()
            .find(|c| **c != card)
            .unwrap();
        let side = &mut state.players[player.index()];
        side.hand.retain(|c| *c != spare);
        side.library.push(spare);
    }
    // Hand is now [iris, one spare, one spare] — enough to pay the cost.
    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    // Pay the one-card discard cost, then the draw resolves.
    let pay = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::PayWithCard { .. }))
        .expect("a card to discard for the cost");
    apply(&mut state, pay).unwrap();

    assert_eq!(state.player(player).hand.len(), 6);
}

#[test]
fn the_draw_variant_supporters_are_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    for name in ["Picnicker", "Jett", "Iris's Fighting Spirit"] {
        assert!(
            import.cards.iter().any(|c| c.name == name && c.playable.is_some()),
            "{name} should play",
        );
    }
}

// --- Beyond the field: hand-refresh and last-card Supporters ---

fn with_hand_refresh(set: Set) -> (Set, CardDefId, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let carmine = db.add(CardDef::Trainer(Trainer {
        print_id: "test-carmine",
        name: "Carmine",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::DiscardHandThenDraw(5),
    }));
    let archer = db.add(CardDef::Trainer(Trainer {
        print_id: "test-tr-archer",
        name: "Team Rocket's Archer",
        kind: TrainerKind::Supporter,
        requirement: Some(Requirement::KnockedOutDuringOpponentsLastTurn),
        effect: TrainerEffect::BothShuffleHandThenDraw { you: 5, opponent: 3 },
    }));
    let cassiopeia = db.add(CardDef::Trainer(Trainer {
        print_id: "test-cassiopeia",
        name: "Cassiopeia",
        kind: TrainerKind::Supporter,
        requirement: Some(Requirement::HandSizeIs(1)),
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![Slot {
                filter: CardFilter::AnyCard,
                to: Destination::Zone(Zone::Hand),
                limit: 2,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    }));
    (Set { db, ..set }, carmine, archer, cassiopeia)
}

#[test]
fn carmine_discards_the_hand_and_draws_five() {
    let (set, carmine, ..) = with_hand_refresh(build());
    let mut state = game(&set, carmine, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, carmine);
    let discard_before = state.player(player).discard.len();
    let hand_before = state.player(player).hand.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert_eq!(state.player(player).hand.len(), 5, "drew a fresh five");
    // The old hand minus Carmine, plus Carmine, all in the discard.
    assert_eq!(
        state.player(player).discard.len(),
        discard_before + hand_before,
    );
}

#[test]
fn team_rockets_archer_refreshes_both_hands() {
    let (set, _c, archer, _cass) = with_hand_refresh(build());
    let mut state = game(&set, archer, 3);
    let player = state.current;
    state.knocked_out_last_turn[player.index()] = true;
    let card = ensure_in_hand(&mut state, player, archer);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert_eq!(state.player(player).hand.len(), 5);
    assert_eq!(state.player(player.opponent()).hand.len(), 3);
}

#[test]
fn cassiopeia_searches_two_cards_when_it_is_the_last_in_hand() {
    let (set, _c, _a, cassiopeia) = with_hand_refresh(build());
    let mut state = game(&set, cassiopeia, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, cassiopeia);
    // Make Cassiopeia the only card in hand.
    let others: Vec<_> = state
        .player(player)
        .hand
        .iter()
        .copied()
        .filter(|c| *c != card)
        .collect();
    let side = &mut state.players[player.index()];
    side.hand.retain(|c| *c == card);
    side.library.extend(others);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    for _ in 0..2 {
        let take = offered(&state)[0];
        apply(&mut state, Action::TakeCard { card: take }).unwrap();
    }
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.player(player).hand.len(), 2);
}

#[test]
fn the_hand_refresh_supporters_are_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    for name in ["Carmine", "Team Rocket's Archer", "Cassiopeia"] {
        assert!(
            import.cards.iter().any(|c| c.name == name && c.playable.is_some()),
            "{name} should play",
        );
    }
}
