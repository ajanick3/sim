//! Milestone 4 ticket 03: the Stadium in play.

use sim::action::{Action, legal_actions};
use sim::card::{
    Attack, CardDb, CardDef, Energy, Pokemon, Trainer, TrainerEffect, TrainerKind, Type,
};
use sim::engine::apply;
use sim::ids::{CardDefId, PlayerId};
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

struct Set {
    db: CardDb,
    mon: CardDefId,
    energy: CardDefId,
    stadium: CardDefId,
    other_stadium: CardDefId,
}

fn build() -> Set {
    let mut db = CardDb::new();
    let mon = db.add(CardDef::Pokemon(Pokemon {
        print_id: "test-mon",
        name: "Testmon",
        hp: 60,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        evolve_from: None,
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
        }],
    }));
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
    }));
    let stadium = db.add(CardDef::Trainer(Trainer {
        print_id: "test-stadium",
        name: "Testing Grounds",
        kind: TrainerKind::Stadium,
        effect: TrainerEffect::Nothing,
    }));
    let other_stadium = db.add(CardDef::Trainer(Trainer {
        print_id: "test-stadium-2",
        name: "Other Grounds",
        kind: TrainerKind::Stadium,
        effect: TrainerEffect::Nothing,
    }));
    Set {
        db,
        mon,
        energy,
        stadium,
        other_stadium,
    }
}

fn game(set: &Set, seed: u64) -> GameState {
    let mut decklist = vec![set.mon; 8];
    decklist.push(set.stadium);
    decklist.push(set.other_stadium);
    while decklist.len() < 60 {
        decklist.push(set.energy);
    }
    let mut state = GameState::new(
        set.db.clone(),
        [decklist.clone(), decklist],
        Box::new(SeededRng::new(seed)),
    );
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    state
}

fn ensure_in_hand(state: &mut GameState, player: PlayerId, def: CardDefId) -> sim::ids::CardId {
    if let Some(card) = state
        .player(player)
        .hand
        .iter()
        .find(|c| state.cards[c.index()].def == def)
    {
        return *card;
    }
    let card = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.cards[c.index()].def == def)
        .expect("the deck holds this card");
    state.players[player.index()].library.retain(|c| *c != card);
    state.players[player.index()].hand.push(card);
    card
}

#[test]
fn a_stadium_stays_in_play_rather_than_being_discarded() {
    let set = build();
    let mut state = game(&set, 9);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, set.stadium);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert_eq!(
        state.stadium,
        Some((player, card)),
        "rule 58: a Stadium stays in play"
    );
    assert!(
        !state.player(player).discard.contains(&card),
        "unlike an Item, it is not discarded on play"
    );
    assert!(state.player(player).stadium_played_this_turn);
}

#[test]
fn a_new_stadium_discards_the_one_already_there() {
    let set = build();
    let mut state = game(&set, 9);
    let first_player = state.current;
    let first = ensure_in_hand(&mut state, first_player, set.stadium);
    apply(&mut state, Action::PlayTrainer { card: first }).unwrap();

    // The turn passes, so the other player may play theirs.
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let a = legal_actions(&state)[0];
        apply(&mut state, a).unwrap();
    }
    let second_player = state.current;
    assert_ne!(second_player, first_player);
    let second = ensure_in_hand(&mut state, second_player, set.other_stadium);
    apply(&mut state, Action::PlayTrainer { card: second }).unwrap();

    assert_eq!(state.stadium, Some((second_player, second)));
    assert!(
        state.player(first_player).discard.contains(&first),
        "rule 58: the old one goes to its own owner's discard"
    );
}

#[test]
fn a_stadium_of_the_same_name_cannot_be_played() {
    let set = build();
    let mut state = game(&set, 9);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, set.stadium);
    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let a = legal_actions(&state)[0];
        apply(&mut state, a).unwrap();
    }
    let opponent = state.current;
    let same_name = ensure_in_hand(&mut state, opponent, set.stadium);

    assert!(
        !legal_actions(&state).contains(&Action::PlayTrainer { card: same_name }),
        "rule 59: not one with the same name as the one in play"
    );
}
