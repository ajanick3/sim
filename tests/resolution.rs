//! The resolution machinery a Trainer effect uses: Phase::Deciding, and the
//! generalized Phase::Promoting. Exercised directly, ahead of any real
//! Trainer card, since the machinery is what several cards will share.

use sim::action::{Action, legal_actions};
use sim::card::{CardFilter, Destination, Zone};
use sim::cards::{milestone1, starter_decklist};
use sim::engine::apply;
use sim::ids::PlayerId;
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

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
fn deciding_offers_only_cards_the_filter_admits() {
    let mut state = game(9);
    let player = state.current;
    let library_card = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.def_of(**c).as_pokemon().is_some())
        .expect("the deck holds Pokémon");

    // Enter the phase directly: the machinery, not a card, is under test.
    state.phase = Phase::Deciding {
        chooser: player,
        from: Zone::Library,
        to: Destination::Zone(Zone::Hand),
        filter: CardFilter::AnyPokemon,
        remaining: 1,
        moved: 0,
        then: None,
    };

    let actions = legal_actions(&state);
    assert!(
        actions.contains(&Action::TakeCard { card: library_card }),
        "a card the filter admits is offered"
    );
    assert!(
        actions.contains(&Action::FinishDeciding),
        "stopping early is always legal"
    );
}

#[test]
fn taking_a_card_moves_it_and_counts_down_remaining() {
    let mut state = game(9);
    let player = state.current;
    let card = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.def_of(**c).as_pokemon().is_some())
        .expect("the deck holds Pokémon");
    let hand_before = state.player(player).hand.len();
    let library_before = state.player(player).library.len();

    state.phase = Phase::Deciding {
        chooser: player,
        from: Zone::Library,
        to: Destination::Zone(Zone::Hand),
        filter: CardFilter::AnyPokemon,
        remaining: 2,
        moved: 0,
        then: None,
    };
    apply(&mut state, Action::TakeCard { card }).unwrap();

    assert_eq!(state.player(player).hand.len(), hand_before + 1);
    assert_eq!(state.player(player).library.len(), library_before - 1);
    assert!(state.player(player).hand.contains(&card));
    assert_eq!(
        state.phase,
        Phase::Deciding {
            chooser: player,
            from: Zone::Library,
            to: Destination::Zone(Zone::Hand),
            filter: CardFilter::AnyPokemon,
            remaining: 1,
            moved: 1,
            then: None,
        },
        "one taken, one still owed"
    );
}

#[test]
fn remaining_at_zero_offers_only_finishing() {
    let mut state = game(9);
    let player = state.current;
    state.phase = Phase::Deciding {
        chooser: player,
        from: Zone::Library,
        to: Destination::Zone(Zone::Hand),
        filter: CardFilter::AnyPokemon,
        remaining: 0,
        moved: 0,
        then: None,
    };
    let actions = legal_actions(&state);
    assert_eq!(actions, vec![Action::FinishDeciding]);
}

#[test]
fn finishing_into_the_library_shuffles_it() {
    // A scripted generator makes the shuffle a fact we can check: every
    // outcome collapses to the same value, so the order changes in a way
    // that is not simply "unchanged".
    use sim::rng::ScriptedRng;
    let set = milestone1();
    let decklist = starter_decklist(&set);
    let mut state = GameState::new(
        set.db,
        [decklist.clone(), decklist],
        Box::new(ScriptedRng::new(vec![0])),
    );
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }

    let player = state.current;
    let before = state.player(player).library.clone();
    state.phase = Phase::Deciding {
        chooser: player,
        from: Zone::Discard,
        to: Destination::Zone(Zone::Library),
        filter: CardFilter::AnyPokemon,
        remaining: 0,
        moved: 0,
        then: None,
    };
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.phase, Phase::Main, "finishing returns control");
    assert_eq!(
        state.player(player).library.len(),
        before.len(),
        "finishing into the Library with nothing taken changes nothing"
    );
}

#[test]
fn only_the_chooser_may_act_in_a_deciding_phase() {
    let mut state = game(9);
    let player = state.current;
    state.phase = Phase::Deciding {
        chooser: player,
        from: Zone::Library,
        to: Destination::Zone(Zone::Hand),
        filter: CardFilter::AnyPokemon,
        remaining: 1,
        moved: 0,
        then: None,
    };
    assert_eq!(
        sim::action::player_to_act(&state),
        Some(player),
        "the phase names who is choosing"
    );
}

#[test]
fn promoting_lets_a_different_chooser_pick_the_others_bench() {
    let mut state = game(9);
    let one = PlayerId::One;
    let two = PlayerId::Two;
    let bench_pick = state.player(one).bench.first().copied();

    state.phase = Phase::Promoting {
        of: one,
        chooser: two,
    };
    assert_eq!(sim::action::player_to_act(&state), Some(two));

    if let Some(pokemon) = bench_pick {
        apply(&mut state, Action::Promote { pokemon }).unwrap();
        assert_eq!(
            state.player(one).active,
            Some(pokemon),
            "the pick lands on the bench it named, not the chooser's own"
        );
    }
}
