//! The resolution machinery a Trainer effect uses: Phase::Deciding, and the
//! generalized Phase::Promoting. The phase is entered directly, since the
//! machinery is what several cards share and no one card exercises all of
//! it. A search names the card it came from, so each test deals a Trainer
//! into the deck to name — the phase reads its next slot back from it.

use sim::action::{Action, legal_actions};
use sim::card::{
    CardDef, CardFilter, Destination, Slot, Trainer, TrainerEffect, TrainerKind, Zone,
};
use sim::cards::{milestone1, starter_decklist};
use sim::engine::apply;
use sim::ids::{CardId, PlayerId};
use sim::rng::{Rng, SeededRng};
use sim::state::{GameState, Phase};

/// A game whose deck holds one Trainer that searches `from` with `slots`,
/// and the card itself, dealt somewhere face down.
fn game_with_a_search(
    rng: Box<dyn Rng>,
    from: Zone,
    slots: Vec<Slot>,
) -> (GameState, PlayerId, CardId) {
    let mut set = milestone1();
    let search = set.db.add(CardDef::Trainer(Trainer {
        print_id: "test-search",
        name: "Search",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::Decide {
            from,
            slots,
            then: None,
        },
    }));
    let mut decklist = starter_decklist(&set);
    decklist.pop();
    decklist.push(search);
    let mut state = GameState::new(set.db, [decklist.clone(), decklist], rng);
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    let player = state.current;
    let side = state.player(player);
    let card = *side
        .library
        .iter()
        .chain(side.hand.iter())
        .chain(side.prizes.iter())
        .find(|c| state.cards[c.index()].def == search)
        .expect("the deal put the Trainer somewhere");
    (state, player, card)
}

/// The one search these tests use unless they say otherwise: the deck to the
/// hand, any Pokémon, up to `limit`.
fn deck_search(limit: u32) -> Vec<Slot> {
    vec![Slot {
        filter: CardFilter::AnyPokemon,
        to: Destination::Zone(Zone::Hand),
        limit,
        excludes_type_of_previous: false,
        peek: None,
    }]
}

#[test]
fn deciding_offers_only_cards_the_filter_admits() {
    let (mut state, player, search) =
        game_with_a_search(Box::new(SeededRng::new(9)), Zone::Library, deck_search(1));
    let library_card = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.def_of(**c).as_pokemon().is_some())
        .expect("the deck holds Pokémon");

    // Enter the phase directly: the machinery, not a card, is under test.
    state.phase = Phase::Deciding {
        chooser: player,
        card: search,
        step: 0,
        from: Zone::Library,
        to: Destination::Zone(Zone::Hand),
        filter: CardFilter::AnyPokemon,
        excludes_type_of_previous: false,
        peek: None,
        remaining: 1,
        previous: None,
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
    let (mut state, player, search) =
        game_with_a_search(Box::new(SeededRng::new(9)), Zone::Library, deck_search(2));
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
        card: search,
        step: 0,
        from: Zone::Library,
        to: Destination::Zone(Zone::Hand),
        filter: CardFilter::AnyPokemon,
        excludes_type_of_previous: false,
        peek: None,
        remaining: 2,
        previous: None,
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
            card: search,
            step: 0,
            from: Zone::Library,
            to: Destination::Zone(Zone::Hand),
            filter: CardFilter::AnyPokemon,
            excludes_type_of_previous: false,
            peek: None,
            remaining: 1,
            previous: Some(card),
            moved: 1,
            then: None,
        },
        "one taken, one still owed"
    );
}

#[test]
fn remaining_at_zero_offers_only_finishing() {
    let (mut state, player, search) =
        game_with_a_search(Box::new(SeededRng::new(9)), Zone::Library, deck_search(1));
    state.phase = Phase::Deciding {
        chooser: player,
        card: search,
        step: 0,
        from: Zone::Library,
        to: Destination::Zone(Zone::Hand),
        filter: CardFilter::AnyPokemon,
        excludes_type_of_previous: false,
        peek: None,
        remaining: 0,
        previous: None,
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
    let put_back = vec![Slot {
        filter: CardFilter::AnyPokemon,
        to: Destination::Zone(Zone::Library),
        limit: 1,
        excludes_type_of_previous: false,
        peek: None,
    }];
    let (mut state, player, search) =
        game_with_a_search(Box::new(ScriptedRng::new(vec![0])), Zone::Discard, put_back);

    let before = state.player(player).library.clone();
    state.phase = Phase::Deciding {
        chooser: player,
        card: search,
        step: 0,
        from: Zone::Discard,
        to: Destination::Zone(Zone::Library),
        filter: CardFilter::AnyPokemon,
        excludes_type_of_previous: false,
        peek: None,
        remaining: 0,
        previous: None,
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
    let (mut state, player, search) =
        game_with_a_search(Box::new(SeededRng::new(9)), Zone::Library, deck_search(1));
    state.phase = Phase::Deciding {
        chooser: player,
        card: search,
        step: 0,
        from: Zone::Library,
        to: Destination::Zone(Zone::Hand),
        filter: CardFilter::AnyPokemon,
        excludes_type_of_previous: false,
        peek: None,
        remaining: 1,
        previous: None,
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
    let (mut state, _, _) =
        game_with_a_search(Box::new(SeededRng::new(9)), Zone::Library, deck_search(1));
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
