//! Ticket 01: evolution, rules 19 to 22.

use sim::action::{Action, legal_actions};
use sim::card::{Attack, CardDb, CardDef, Energy, Pokemon, Stage, Type};
use sim::engine::apply;
use sim::ids::{CardDefId, PlayerId};
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

struct Set {
    db: CardDb,
    basic: CardDefId,
    stage1: CardDefId,
    energy: CardDefId,
}

fn build() -> Set {
    let mut db = CardDb::new();
    let basic = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-seedling",
        name: "Seedling",
        hp: 60,
        kind: Type::Grass,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Vine",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None, effect: None,
        }],
    }));
    let stage1 = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-bloomling",
        name: "Bloomling",
        hp: 90,
        kind: Type::Grass,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Stage1,
        evolve_from: Some("Seedling"),
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Petal Slash",
            cost: vec![Type::Grass, Type::Colorless],
            base_damage: 40,
            inflicts: None, effect: None,
        }],
    }));
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-grass-energy",
        name: "Grass Energy",
        kind: Type::Grass,
        effect: None,
    }));
    Set {
        db,
        basic,
        stage1,
        energy,
    }
}

/// A deck of the Basic, its evolution, and Energy — enough to test the
/// evolution rules without deck construction mattering.
fn deck(set: &Set) -> Vec<CardDefId> {
    let mut decklist = vec![set.basic; 6];
    decklist.extend(vec![set.stage1; 6]);
    decklist.extend(vec![set.energy; 48]);
    decklist
}

/// Deal a game and drive setup, then end both players' first turns —
/// rule 18 forbids evolving on either, so a test evolves only once the
/// game is past them, back on the first player's second turn.
fn game(set: &Set, seed: u64) -> GameState {
    let decklist = deck(set);
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
        apply(&mut state, Action::EndTurn).unwrap();
    }
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    state
}

fn find_in_hand(state: &GameState, player: PlayerId, def: CardDefId) -> sim::ids::CardId {
    *state
        .player(player)
        .hand
        .iter()
        .chain(state.player(player).library.iter())
        .find(|c| state.cards[c.index()].def == def)
        .expect("the deck holds this card")
}

/// Take a card from wherever it is (hand or library) into the acting
/// player's hand, without spending a real draw. Setup already dealt a hand;
/// this just guarantees the specific card under test is in it.
fn ensure_in_hand(state: &mut GameState, player: PlayerId, card: sim::ids::CardId) {
    if state.player(player).hand.contains(&card) {
        return;
    }
    state.players[player.index()].library.retain(|c| *c != card);
    state.players[player.index()].hand.push(card);
}

#[test]
fn a_basic_evolves_into_the_card_that_names_it() {
    let set = build();
    let mut state = game(&set, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    assert_eq!(state.pokemon_def(active).name, "Seedling");

    let evolution = find_in_hand(&state, player, set.stage1);
    ensure_in_hand(&mut state, player, evolution);

    let action = Action::Evolve {
        card: evolution,
        target: active,
    };
    assert!(
        legal_actions(&state).contains(&action),
        "an evolution in hand may evolve the Basic it names"
    );
    apply(&mut state, action).unwrap();

    assert_eq!(state.pokemon_def(active).name, "Bloomling");
    assert!(!state.player(player).hand.contains(&evolution));
}

#[test]
fn evolution_keeps_damage_and_attachments_and_clears_conditions() {
    let set = build();
    let mut state = game(&set, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();

    state.pokemon[active.index()].damage = 20;
    let energy = find_in_hand(&state, player, set.energy);
    ensure_in_hand(&mut state, player, energy);
    state.remove_from_hand(player, energy);
    state.pokemon[active.index()].attached.push(energy);
    state.inflict(active, sim::card::Condition::Poisoned);

    let evolution = find_in_hand(&state, player, set.stage1);
    ensure_in_hand(&mut state, player, evolution);
    apply(
        &mut state,
        Action::Evolve {
            card: evolution,
            target: active,
        },
    )
    .unwrap();

    assert_eq!(
        state.pokemon(active).damage,
        20,
        "rule 22: damage carries over"
    );
    assert_eq!(
        state.pokemon(active).attached,
        vec![energy],
        "rule 22: attachments carry over"
    );
    assert!(
        state.pokemon(active).conditions.is_empty(),
        "rule 22: Special Conditions are cleared"
    );
}

#[test]
fn a_pokemon_cannot_evolve_the_turn_it_was_played() {
    let set = build();
    let mut state = game(&set, 3);
    let player = state.current;

    // Play a fresh Basic to the Bench this turn.
    let fresh_basic = find_in_hand(&state, player, set.basic);
    ensure_in_hand(&mut state, player, fresh_basic);
    let play = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::PlayBasic { card } if *card == fresh_basic))
        .expect("a Basic in hand can be benched");
    apply(&mut state, play).unwrap();
    let benched = *state.player(player).bench.last().unwrap();

    let evolution = find_in_hand(&state, player, set.stage1);
    ensure_in_hand(&mut state, player, evolution);
    assert!(
        !legal_actions(&state).contains(&Action::Evolve {
            card: evolution,
            target: benched
        }),
        "rule 20: not the turn it was played"
    );
}

#[test]
fn a_pokemon_cannot_evolve_twice_in_one_turn() {
    let set = build();
    let mut state = game(&set, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();

    let evolution = find_in_hand(&state, player, set.stage1);
    ensure_in_hand(&mut state, player, evolution);
    apply(
        &mut state,
        Action::Evolve {
            card: evolution,
            target: active,
        },
    )
    .unwrap();

    // A second copy of the same evolution, if one were in hand, still could
    // not apply — the Pokémon itself is spent for the turn.
    assert!(
        !legal_actions(&state)
            .iter()
            .any(|a| matches!(a, Action::Evolve { target, .. } if *target == active)),
        "rule 20: cannot evolve the same Pokémon twice in one turn"
    );
}

#[test]
fn neither_player_evolves_on_the_first_turn_of_the_game() {
    let set = build();
    let decklist = deck(&set);
    let mut state = GameState::new(
        set.db.clone(),
        [decklist.clone(), decklist],
        Box::new(SeededRng::new(3)),
    );
    fn drive_to_main(state: &mut GameState) {
        while state.phase != Phase::Main && !state.is_over() {
            let first = legal_actions(state)[0];
            apply(state, first).unwrap();
        }
    }
    // Both Actives are `Seedling`, the only Basic in the deck. Give the
    // acting player its `Bloomling` so nothing but rule 18 can stop the
    // evolution, then read whether it is offered.
    let stage1 = set.stage1;
    let evolve_offered = |state: &mut GameState| -> bool {
        let player = state.current;
        let card = find_in_hand(state, player, stage1);
        ensure_in_hand(state, player, card);
        legal_actions(state).iter().any(|a| matches!(a, Action::Evolve { .. }))
    };

    drive_to_main(&mut state);
    assert!(!evolve_offered(&mut state), "rule 18: not on the first player's first turn");

    apply(&mut state, Action::EndTurn).unwrap();
    drive_to_main(&mut state);
    assert!(!evolve_offered(&mut state), "rule 18: not on the second player's first turn either");

    apply(&mut state, Action::EndTurn).unwrap();
    drive_to_main(&mut state);
    assert!(evolve_offered(&mut state), "the first player's second turn: evolution is legal");
}

#[test]
fn a_knockout_discards_every_stage_the_pokemon_evolved_through() {
    let set = build();
    let mut state = game(&set, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();

    let evolution = find_in_hand(&state, player, set.stage1);
    ensure_in_hand(&mut state, player, evolution);
    apply(
        &mut state,
        Action::Evolve {
            card: evolution,
            target: active,
        },
    )
    .unwrap();

    let hp = state.pokemon_def(active).hp;
    state.pokemon[active.index()].damage = hp;
    let discard_before = state.player(player).discard.len();

    // Any action that runs settle() will notice the fainted Pokémon.
    apply(&mut state, Action::EndTurn).unwrap();

    assert_eq!(
        state.player(player).discard.len(),
        discard_before + 2,
        "both Seedling and Bloomling go to the discard together"
    );
}

#[test]
fn evolution_works_on_a_benched_pokemon() {
    let set = build();
    let mut state = game(&set, 3);
    let player = state.current;

    let benched = state
        .player(player)
        .bench
        .first()
        .copied()
        .expect("setup filled the Bench");
    assert_eq!(state.pokemon_def(benched).name, "Seedling");

    let evolution = find_in_hand(&state, player, set.stage1);
    ensure_in_hand(&mut state, player, evolution);
    apply(
        &mut state,
        Action::Evolve {
            card: evolution,
            target: benched,
        },
    )
    .unwrap();

    assert_eq!(state.pokemon_def(benched).name, "Bloomling");
}
