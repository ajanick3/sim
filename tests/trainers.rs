//! Ticket 03: the first Trainer effects.

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
    judge: CardDefId,
    lillie: CardDefId,
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
    let judge = db.add(CardDef::Trainer(Trainer {
        print_id: "test-judge",
        name: "Judge",
        kind: TrainerKind::Supporter,
        effect: TrainerEffect::BothShuffleHandThenDraw { count: 4 },
    }));
    let lillie = db.add(CardDef::Trainer(Trainer {
        print_id: "test-lillie",
        name: "Lillie's Determination",
        kind: TrainerKind::Supporter,
        effect: TrainerEffect::ShuffleHandThenDraw {
            normal: 6,
            at_six_prizes: 8,
        },
    }));
    Set {
        db,
        mon,
        energy,
        judge,
        lillie,
    }
}

fn deck(set: &Set, supporter: CardDefId) -> Vec<CardDefId> {
    let mut decklist = vec![set.mon; 6];
    decklist.push(supporter);
    while decklist.len() < 60 {
        decklist.push(set.energy);
    }
    decklist
}

fn game(set: &Set, supporter: CardDefId, seed: u64) -> GameState {
    let decklist = deck(set, supporter);
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
fn a_supporter_is_played_once_per_turn_then_discarded() {
    let set = build();
    let mut state = game(&set, set.judge, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, set.judge);

    assert!(
        legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "a Supporter in hand is playable"
    );
    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert!(
        !state.player(player).hand.contains(&card),
        "played, it leaves the hand"
    );
    assert!(
        state.player(player).discard.contains(&card),
        "a Supporter discards once played"
    );
    assert!(
        state.player(player).supporter_played_this_turn,
        "rule 13: one Supporter a turn"
    );
    assert!(
        !legal_actions(&state)
            .iter()
            .any(|a| matches!(a, Action::PlayTrainer { .. })),
        "a second Supporter cannot be played the same turn"
    );
}

#[test]
fn judge_reshuffles_both_hands_and_draws_four() {
    let set = build();
    let mut state = game(&set, set.judge, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, set.judge);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert_eq!(state.player(PlayerId::One).hand.len(), 4);
    assert_eq!(state.player(PlayerId::Two).hand.len(), 4);
    assert_eq!(state.phase, Phase::Main, "no choice, resolves at once");
}

#[test]
fn lillies_determination_draws_six_normally() {
    // A fresh game starts at exactly 6 Prizes (rule 10) — the bonus case by
    // default. The normal count needs a Prize already taken.
    let set = build();
    let mut state = game(&set, set.lillie, 3);
    let player = state.current;
    let taken = state.players[player.index()].prizes.pop().unwrap();
    state.players[player.index()].hand.push(taken);
    assert_eq!(state.player(player).prizes.len(), 5, "not the bonus case");

    let card = ensure_in_hand(&mut state, player, set.lillie);
    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    assert_eq!(state.player(player).hand.len(), 6);
}

#[test]
fn lillies_determination_draws_eight_at_six_prizes() {
    let set = build();
    let mut state = game(&set, set.lillie, 3);
    let player = state.current;
    state.players[player.index()].prizes = (0..6)
        .map(|_| state.player(player).library[0])
        .collect::<Vec<_>>();
    let card = ensure_in_hand(&mut state, player, set.lillie);
    assert_eq!(state.player(player).prizes.len(), 6);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    assert_eq!(state.player(player).hand.len(), 8);
}
