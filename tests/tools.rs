//! Milestone 9: every Tool in the field. One fixture, shared across this
//! milestone's tickets, the same way `items.rs` served Milestone 8.

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


// --- Ticket 01: the attach mechanism ---

fn with_a_tool(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-a-tool",
        name: "A Tool",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::Nothing,
    }));
    (Set { db, ..set }, card)
}

#[test]
fn a_tool_attaches_immediately_with_a_target_named_at_play_time() {
    let (set, tool) = with_a_tool(build());
    let mut state = game(&set, tool, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, tool);
    let target = state.player(player).active.unwrap();

    assert!(legal_actions(&state).contains(&Action::PlayTool { card, target }));
    apply(&mut state, Action::PlayTool { card, target }).unwrap();

    assert!(state.pokemon(target).attached.contains(&card));
    assert!(!state.player(player).hand.contains(&card));
    assert_eq!(state.phase, Phase::Main, "a Tool needs no phase, like Energy");
}

#[test]
fn a_pokemon_carries_at_most_one_tool() {
    let (set, tool) = with_a_tool(build());
    let mut state = game(&set, tool, 3);
    let player = state.current;
    let first = ensure_in_hand(&mut state, player, tool);
    let target = state.player(player).active.unwrap();
    apply(&mut state, Action::PlayTool { card: first, target }).unwrap();

    let second = deal_new_card(&mut state, player, tool);
    state.players[player.index()].hand.push(second);
    assert!(
        !legal_actions(&state).contains(&Action::PlayTool { card: second, target }),
        "the target already carries a Tool"
    );
    // A different, bare Pokémon may still take one.
    let bare = state.player(player).bench[0];
    assert!(legal_actions(&state).contains(&Action::PlayTool { card: second, target: bare }));
}

#[test]
fn a_tool_is_unlimited_per_turn_unlike_energy() {
    let (set, tool) = with_a_tool(build());
    let mut state = game(&set, tool, 3);
    let player = state.current;
    let first = ensure_in_hand(&mut state, player, tool);
    let first_target = state.player(player).active.unwrap();
    apply(&mut state, Action::PlayTool { card: first, target: first_target }).unwrap();

    let second = deal_new_card(&mut state, player, tool);
    state.players[player.index()].hand.push(second);
    let second_target = state.player(player).bench[0];
    assert!(
        legal_actions(&state).contains(&Action::PlayTool { card: second, target: second_target }),
        "no once-per-turn limit on playing a Tool"
    );
}

// --- Ticket 02: Air Balloon ---

fn with_air_balloon(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-air-balloon",
        name: "Air Balloon",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::ReducesRetreatCost(2),
    }));
    (Set { db, ..set }, card)
}

#[test]
fn air_balloon_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Air Balloon")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Air Balloon should play");
}

#[test]
fn air_balloon_reduces_the_retreat_cost_by_two() {
    let (set, balloon) = with_air_balloon(build());
    let mut state = game(&set, balloon, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    // The fixture's ordinary Basic prints a Retreat Cost of 1.
    assert_eq!(state.effective_retreat_cost(active), 1);

    let card = ensure_in_hand(&mut state, player, balloon);
    apply(&mut state, Action::PlayTool { card, target: active }).unwrap();

    assert_eq!(
        state.effective_retreat_cost(active),
        0,
        "floored at zero, not negative"
    );
    assert!(
        legal_actions(&state).contains(&Action::Retreat { to: state.player(player).bench[0] }),
        "retreating for free is now legal with no Energy attached"
    );
}

#[test]
fn air_balloon_does_nothing_for_a_pokemon_it_is_not_attached_to() {
    let (set, balloon) = with_air_balloon(build());
    let mut state = game(&set, balloon, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let bench_mon = state.player(player).bench[0];
    let card = ensure_in_hand(&mut state, player, balloon);
    apply(&mut state, Action::PlayTool { card, target: bench_mon }).unwrap();

    assert_eq!(
        state.effective_retreat_cost(active),
        1,
        "the Active's own cost is untouched"
    );
}
