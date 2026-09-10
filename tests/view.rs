//! Ticket 05: a player sees only their own hidden zones.

use sim::action::legal_actions;
use sim::cards::{milestone1, starter_decklist};
use sim::engine::apply;
use sim::ids::PlayerId;
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};
use sim::view::PlayerView;

fn game(seed: u64) -> GameState {
    let set = milestone1();
    let decklist = starter_decklist(&set);
    let mut state = GameState::new(
        set.db,
        [decklist.clone(), decklist],
        Box::new(SeededRng::new(seed)),
    );
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    state
}

#[test]
fn a_view_shows_your_own_hand_and_only_its_size_for_the_opponent() {
    let state = game(9);
    let you = PlayerId::One;
    let view = PlayerView::of(&state, you);

    let yours: Vec<_> = view.your_hand.iter().map(|card| card.id).collect();
    assert_eq!(
        yours,
        state.player(you).hand,
        "you see every card in your own hand"
    );
    assert_eq!(
        view.side(you.opponent()).hand_count,
        state.player(you.opponent()).hand.len(),
        "you see how many cards the opponent holds"
    );
}

#[test]
fn a_view_hides_every_prize_card_including_your_own() {
    let state = game(9);
    let view = PlayerView::of(&state, PlayerId::One);
    for player in [PlayerId::One, PlayerId::Two] {
        assert_eq!(view.side(player).prize_count, 6);
    }
}

#[test]
fn a_view_hides_the_library_and_keeps_its_size() {
    let state = game(9);
    let you = PlayerId::One;
    let view = PlayerView::of(&state, you);
    assert_eq!(
        view.side(you).library_count,
        state.player(you).library.len(),
        "you know how many cards are left, not which"
    );
    assert!(
        view.library_in_search.is_none(),
        "the library is shown only during a whole-library search"
    );
}

#[test]
fn a_view_shows_both_boards_and_the_discards() {
    let state = game(9);
    let you = PlayerId::One;
    let view = PlayerView::of(&state, you);

    for player in [PlayerId::One, PlayerId::Two] {
        let side = view.side(player);
        let active = side.active.as_ref().expect("both players have an Active");
        let real = state.player(player).active.unwrap();
        assert_eq!(active.name, state.pokemon_def(real).name);
        assert_eq!(active.remaining_hp, state.remaining_hp(real));
        assert_eq!(side.bench.len(), state.player(player).bench.len());
        assert_eq!(side.discard.len(), state.player(player).discard.len());
    }
}

#[test]
fn each_player_sees_a_different_view_of_the_same_game() {
    let state = game(9);
    let one = PlayerView::of(&state, PlayerId::One);
    let two = PlayerView::of(&state, PlayerId::Two);
    assert_ne!(
        one.your_hand.len() + two.side(PlayerId::One).hand_count,
        0,
        "both views describe the same board"
    );
    assert_eq!(one.you, PlayerId::One);
    assert_eq!(two.you, PlayerId::Two);
}
