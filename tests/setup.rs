//! Ticket 01: setup is four phases, and every choice in it is the player's.

use sim::action::{Action, legal_actions, player_to_act};
use sim::cards::milestone1;
use sim::engine::apply;
use sim::ids::CardDefId;
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

/// These decklists ignore deck construction — the engine does not check it
/// yet — so that a test can force a hand that always has a Basic, or one that
/// almost never does.
fn game(seed: u64, decklists: [Vec<CardDefId>; 2]) -> GameState {
    let set = milestone1();
    GameState::new(set.db, decklists, Box::new(SeededRng::new(seed)))
}

fn all_basics() -> Vec<CardDefId> {
    let set = milestone1();
    vec![set.sparkmouse; 60]
}

fn one_basic() -> Vec<CardDefId> {
    let set = milestone1();
    let mut decklist = vec![set.lightning_energy; 59];
    decklist.push(set.sparkmouse);
    decklist
}

fn drive_to_main(state: &mut GameState) {
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(state)[0];
        apply(state, first).unwrap();
    }
}

#[test]
fn the_coin_flip_winner_may_send_the_opponent_first() {
    let mut state = game(4, [all_basics(), all_basics()]);
    let Phase::ChoosingWhoGoesFirst { winner } = state.phase else {
        panic!("a dealt game waits on the coin flip");
    };
    assert_eq!(player_to_act(&state), Some(winner));

    let actions = legal_actions(&state);
    assert!(actions.contains(&Action::ChooseWhoGoesFirst { first: winner }));
    assert!(actions.contains(&Action::ChooseWhoGoesFirst {
        first: winner.opponent()
    }));

    apply(
        &mut state,
        Action::ChooseWhoGoesFirst {
            first: winner.opponent(),
        },
    )
    .unwrap();
    assert_eq!(
        state.current,
        winner.opponent(),
        "rule 5: the winner chooses, and may choose the opponent"
    );
}

#[test]
fn a_player_places_their_own_active_and_bench() {
    let mut state = game(9, [all_basics(), all_basics()]);
    let first = legal_actions(&state)[0];
    apply(&mut state, first).unwrap();

    let Phase::PlacingActive { player } = state.phase else {
        panic!("with no mulligans, placement is the next choice");
    };
    let chosen = match legal_actions(&state)[2] {
        Action::PlaceActive { card } => card,
        other => panic!("expected a placement, got {other:?}"),
    };
    apply(&mut state, Action::PlaceActive { card: chosen }).unwrap();

    let active = state.player(player).active.expect("the Active is placed");
    assert_eq!(
        state.pokemon(active).top_card(),
        chosen,
        "the Pokémon placed is the one the player chose"
    );

    assert!(matches!(state.phase, Phase::PlacingBench { .. }));
    apply(&mut state, Action::FinishPlacing).unwrap();
    assert!(
        state.player(player).bench.is_empty(),
        "a player may keep the Bench empty"
    );
}

#[test]
fn the_bench_stops_at_five() {
    let mut state = game(9, [all_basics(), all_basics()]);
    drive_to_main(&mut state);
    for player in [sim::ids::PlayerId::One, sim::ids::PlayerId::Two] {
        assert_eq!(
            state.player(player).bench.len(),
            5,
            "taking every placement fills the Bench and stops"
        );
    }
}

#[test]
fn the_bonus_draws_can_be_declined() {
    // One Basic in 60 forces mulligans, so the opponent is owed bonus cards.
    let mut state = game(1, [one_basic(), one_basic()]);
    let first = legal_actions(&state)[0];
    apply(&mut state, first).unwrap();

    let Phase::TakingBonusDraws { player, remaining } = state.phase else {
        panic!("a mulligan owes the opponent a bonus card");
    };
    assert!(remaining > 0);

    let before = state.player(player).hand.len();
    apply(&mut state, Action::DeclineBonusDraws).unwrap();
    assert_eq!(
        state.player(player).hand.len(),
        before,
        "rule 8: the extra card is a may, not a must"
    );
    assert!(
        !matches!(state.phase, Phase::TakingBonusDraws { player: p, .. } if p == player),
        "declining ends that player's bonus draws"
    );
}

#[test]
fn a_bonus_draw_adds_one_card() {
    let mut state = game(1, [one_basic(), one_basic()]);
    let first = legal_actions(&state)[0];
    apply(&mut state, first).unwrap();

    let Phase::TakingBonusDraws { player, remaining } = state.phase else {
        panic!("a mulligan owes the opponent a bonus card");
    };
    let before = state.player(player).hand.len();
    apply(&mut state, Action::TakeBonusDraw).unwrap();
    assert_eq!(state.player(player).hand.len(), before + 1);

    if remaining > 1 {
        assert!(matches!(
            state.phase,
            Phase::TakingBonusDraws { player: p, remaining: r } if p == player && r == remaining - 1
        ));
    }
}

#[test]
fn the_prizes_come_off_after_the_pokemon_go_down() {
    let mut state = game(9, [all_basics(), all_basics()]);
    let first = legal_actions(&state)[0];
    apply(&mut state, first).unwrap();
    assert!(matches!(state.phase, Phase::PlacingActive { .. }));
    for player in [sim::ids::PlayerId::One, sim::ids::PlayerId::Two] {
        assert!(
            state.player(player).prizes.is_empty(),
            "rule 10: the Prizes come off the top after the Pokémon are down"
        );
    }

    drive_to_main(&mut state);
    for player in [sim::ids::PlayerId::One, sim::ids::PlayerId::Two] {
        assert_eq!(state.player(player).prizes.len(), 6);
    }
}

#[test]
fn the_starting_player_draws_on_the_first_turn() {
    // No mulligans and no bonus draws: 7 drawn, 6 placed, 1 left in hand.
    let mut state = game(9, [all_basics(), all_basics()]);
    drive_to_main(&mut state);
    assert_eq!(
        state.player(state.current).hand.len(),
        2,
        "rule 15: the player going first draws; they only skip the attack"
    );
    assert_eq!(state.player(state.current.opponent()).hand.len(), 1);
}
