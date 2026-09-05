//! Milestone 1: two synthetic Basics attack until someone wins.

use sim::action::{Action, legal_actions, player_to_act};
use sim::cards::{Milestone1, milestone1, starter_decklist};
use sim::engine::{apply, damage_dealt};
use sim::ids::PlayerId;
use sim::rng::SeededRng;
use sim::state::{GameState, WinReason};

fn game(seed: u64) -> (Milestone1, GameState) {
    let set = milestone1();
    let decklist = starter_decklist(&set);
    let db = set.db.clone();
    let state = GameState::new(
        db,
        [decklist.clone(), decklist],
        Box::new(SeededRng::new(seed)),
    );
    (set, state)
}

/// Attack when you can, otherwise build up. Enough to drive a game to its end.
fn choose(state: &GameState) -> Action {
    let actions = legal_actions(state);
    let preferred = actions
        .iter()
        .find(|a| matches!(a, Action::Promote { .. }))
        .or_else(|| actions.iter().find(|a| matches!(a, Action::Attack { .. })))
        .or_else(|| {
            actions
                .iter()
                .find(|a| matches!(a, Action::AttachEnergy { .. }))
        });
    *preferred.unwrap_or(&Action::EndTurn)
}

#[test]
fn setup_deals_a_legal_opening_board() {
    let (_, state) = game(7);
    for player in [PlayerId::First, PlayerId::Second] {
        let side = state.player(player);
        assert!(side.active.is_some(), "each player starts with an Active");
        assert_eq!(side.prizes.len(), 6, "each player sets 6 Prizes aside");
        assert!(side.bench.len() <= 5, "the Bench holds 5");
        let total = side.library.len() + side.hand.len() + side.prizes.len() + side.in_play().len();
        assert_eq!(total, 60, "every card is somewhere");
    }
}

#[test]
fn the_first_player_cannot_attack_on_the_first_turn() {
    let (_, state) = game(7);
    assert!(
        !legal_actions(&state)
            .iter()
            .any(|a| matches!(a, Action::Attack { .. })),
        "rule 17: the player going first skips their attack step"
    );
}

#[test]
fn weakness_doubles_and_damage_lands_in_tens() {
    let (_, state) = game(3);
    let attacker = state.player(PlayerId::First).active.unwrap();
    let defender = state.player(PlayerId::Second).active.unwrap();

    let attacker_type = state.pokemon_def(attacker).kind;
    let weak = state.pokemon_def(defender).weakness == Some(attacker_type);
    let expected = if weak { 60 } else { 30 };
    assert_eq!(damage_dealt(&state, attacker, defender, 30), expected);

    // Rule 35: 1 counter per 10 damage, so 35 damage is 3 counters.
    let odd = damage_dealt(&state, attacker, defender, 35);
    assert_eq!(odd % 10, 0, "damage lands in tens");
}

#[test]
fn an_action_outside_the_legal_list_is_refused() {
    let (_, mut state) = game(5);
    let illegal = Action::Attack { index: 0 };
    assert!(
        !legal_actions(&state).contains(&illegal),
        "the first turn has no attack to make"
    );
    assert!(apply(&mut state, illegal).is_err());
}

#[test]
fn a_knockout_takes_a_prize() {
    let (_, mut state) = game(11);

    // Give the first player a turn that can attack, and a defender at 10 HP.
    while state.is_first_turn_of_game() {
        apply(&mut state, Action::EndTurn).unwrap();
    }
    while state.current != PlayerId::First {
        let action = choose(&state);
        apply(&mut state, action).unwrap();
    }

    let attacker = state.player(PlayerId::First).active.unwrap();
    let defender = state.player(PlayerId::Second).active.unwrap();
    let hp = state.pokemon_def(defender).hp;
    state.pokemon[defender.index()].damage = hp - 10;

    // Pay for the strongest attack the Active has.
    let cost = state
        .pokemon_def(attacker)
        .attacks
        .iter()
        .map(|a| a.cost)
        .max()
        .unwrap();
    while state.energy_attached(attacker) < cost {
        let energy = *state
            .player(PlayerId::First)
            .hand
            .iter()
            .find(|c| state.def_of(**c).is_energy())
            .expect("the deck is mostly Energy");
        state.remove_from_hand(PlayerId::First, energy);
        state.pokemon[attacker.index()].attached.push(energy);
    }

    let before = state.player(PlayerId::First).prizes.len();
    let attack = legal_actions(&state)
        .into_iter()
        .filter(|a| matches!(a, Action::Attack { .. }))
        .next_back()
        .expect("a paid-for Active can attack");
    apply(&mut state, attack).unwrap();

    assert!(
        state.pokemon(defender).knocked_out,
        "damage past HP knocks a Pokémon out"
    );
    assert_eq!(
        state.player(PlayerId::First).prizes.len(),
        before - 1,
        "rule 39: the opponent of the knocked-out player takes a Prize"
    );
}

#[test]
fn a_game_of_attacks_reaches_a_winner() {
    for seed in 1..20 {
        let (_, mut state) = game(seed);
        let mut steps = 0;
        while !state.is_over() {
            assert!(steps < 20_000, "seed {seed} did not finish");
            assert!(player_to_act(&state).is_some());
            let action = choose(&state);
            apply(&mut state, action).unwrap();
            steps += 1;
        }
        let outcome = state.outcome.unwrap();
        assert!(matches!(
            outcome.reason,
            WinReason::AllPrizesTaken | WinReason::NoPokemonInPlay | WinReason::CouldNotDraw
        ));
    }
}

#[test]
fn an_empty_library_loses_the_game() {
    let (_, mut state) = game(2);
    let loser = state.current.opponent();
    state.players[loser.index()].library.clear();
    apply(&mut state, Action::EndTurn).unwrap();
    let outcome = state.outcome.expect("a player who cannot draw loses");
    assert_eq!(outcome.winner, loser.opponent());
    assert_eq!(outcome.reason, WinReason::CouldNotDraw);
}
