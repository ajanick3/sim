//! Ticket 03: the Special Conditions and the Pokémon Checkup.

use sim::action::{Action, legal_actions};
use sim::card::{Attack, CardDb, CardDef, Condition, Energy, Pokemon, Stage, Type};
use sim::engine::apply;
use sim::rng::{Rng, ScriptedRng, SeededRng};
use sim::state::{GameState, Phase};

/// A card set whose one attack inflicts the condition under test and deals no
/// damage, so a test reads the condition and nothing else.
fn game(inflicts: Condition, rng: Box<dyn Rng>) -> GameState {
    let mut db = CardDb::new();
    let stinger = db.add(CardDef::Pokemon(Pokemon {
        print_id: "test-stinger",
        name: "Stinger",
        hp: 100,
        kind: Type::Grass,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        evolve_from: None,
        stage: Stage::Basic,
        prizes: 1,
        attacks: vec![Attack {
            name: "Sting",
            cost: vec![Type::Colorless],
            base_damage: 0,
            inflicts: Some(inflicts),
        }],
    }));
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-grass-energy",
        name: "Grass Energy",
        kind: Type::Grass,
    }));

    // Enough Pokémon that an opening hand fills a Bench, so a test can retreat.
    let mut decklist = vec![stinger; 12];
    decklist.extend(vec![energy; 48]);
    let mut state = GameState::new(db, [decklist.clone(), decklist], rng);
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    // Rule 17: the first turn has no attack step.
    apply(&mut state, Action::EndTurn).unwrap();
    drive_setup_choices(&mut state);
    state
}

/// A game whose setup left both players a Bench, so a knockout does not end
/// the game. It reads no flip, so a real shuffle is what it wants.
fn game_with_benches(inflicts: Condition) -> GameState {
    for seed in 1..200 {
        let state = game(inflicts, Box::new(SeededRng::new(seed)));
        let benched = [sim::ids::PlayerId::One, sim::ids::PlayerId::Two]
            .iter()
            .all(|p| !state.player(*p).bench.is_empty());
        if benched {
            return state;
        }
    }
    panic!("no seed under 200 dealt both players a Bench");
}

/// Take the first choice through any phase that is not a player's main turn.
fn drive_setup_choices(state: &mut GameState) {
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(state)[0];
        apply(state, first).unwrap();
    }
}

/// Attach one Energy to the Active, without spending the turn's attachment.
fn force_attach(state: &mut GameState) {
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let energy = *state
        .player(player)
        .hand
        .iter()
        .chain(state.player(player).library.iter())
        .find(|c| state.def_of(**c).is_energy())
        .expect("the deck is mostly Energy");
    state.remove_from_hand(player, energy);
    state.players[player.index()]
        .library
        .retain(|c| *c != energy);
    state.pokemon[active.index()].attached.push(energy);
}

/// Attack with the Active, paying for it first.
fn sting(state: &mut GameState) {
    force_attach(state);
    let attack = legal_actions(state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("a paid-for Active can attack");
    apply(state, attack).unwrap();
}

#[test]
fn poison_puts_one_counter_at_the_checkup() {
    let mut state = game(Condition::Poisoned, Box::new(SeededRng::new(9)));
    let poisoned = state.player(state.current.opponent()).active.unwrap();

    sting(&mut state);
    assert!(
        state.has_condition(poisoned, Condition::Poisoned),
        "the attack inflicts Poison"
    );
    assert_eq!(
        state.pokemon(poisoned).damage,
        0,
        "Poison damages at the checkup, not when it lands"
    );

    // The attack ended the turn, so the checkup has run.
    drive_setup_choices(&mut state);
    assert_eq!(
        state.pokemon(poisoned).damage,
        10,
        "rule 54: Poison puts 1 damage counter at the checkup"
    );
    assert!(
        state.has_condition(poisoned, Condition::Poisoned),
        "Poison stays until something removes it"
    );
}

#[test]
fn a_scripted_generator_makes_a_flip_an_assertion() {
    let state = game(Condition::Poisoned, Box::new(ScriptedRng::new(vec![1])));
    assert!(!state.is_over());
}

#[test]
fn burn_puts_two_counters_and_a_heads_removes_it() {
    // Every flip is heads.
    let mut state = game(Condition::Burned, Box::new(ScriptedRng::new(vec![1])));
    let burned = state.player(state.current.opponent()).active.unwrap();

    sting(&mut state);
    drive_setup_choices(&mut state);

    assert_eq!(
        state.pokemon(burned).damage,
        20,
        "rule 53: Burn puts 2 damage counters"
    );
    assert!(
        !state.has_condition(burned, Condition::Burned),
        "heads removes the Burn"
    );
}

#[test]
fn a_tails_keeps_the_burn() {
    // Every flip is tails.
    let mut state = game(Condition::Burned, Box::new(ScriptedRng::new(vec![0])));
    let burned = state.player(state.current.opponent()).active.unwrap();

    sting(&mut state);
    drive_setup_choices(&mut state);

    assert_eq!(state.pokemon(burned).damage, 20);
    assert!(
        state.has_condition(burned, Condition::Burned),
        "tails keeps the Burn, and it burns again next checkup"
    );
}

#[test]
fn asleep_cannot_attack_or_retreat() {
    // Tails, so the sleeper does not wake at the first checkup.
    let mut state = game(Condition::Asleep, Box::new(ScriptedRng::new(vec![0])));
    sting(&mut state);
    drive_setup_choices(&mut state);

    let sleeper = state.player(state.current).active.unwrap();
    assert!(state.has_condition(sleeper, Condition::Asleep));

    // Pay for the attack, so only the sleep can stop it.
    force_attach(&mut state);
    let actions = legal_actions(&state);
    assert!(
        !actions.iter().any(|a| matches!(a, Action::Attack { .. })),
        "rule 50: Asleep cannot attack"
    );
    assert!(
        !actions.iter().any(|a| matches!(a, Action::Retreat { .. })),
        "rule 50: Asleep cannot retreat"
    );
}

#[test]
fn a_heads_wakes_the_sleeper() {
    let mut state = game(Condition::Asleep, Box::new(ScriptedRng::new(vec![1])));
    let sleeper = state.player(state.current.opponent()).active.unwrap();
    sting(&mut state);
    drive_setup_choices(&mut state);

    assert!(
        !state.has_condition(sleeper, Condition::Asleep),
        "rule 50: heads at the checkup wakes it"
    );
}

#[test]
fn paralysis_lasts_until_the_checkup_after_its_owners_turn() {
    let mut state = game(Condition::Paralyzed, Box::new(ScriptedRng::new(vec![0])));
    let victim = state.player(state.current.opponent()).active.unwrap();
    sting(&mut state);
    drive_setup_choices(&mut state);

    assert!(
        state.has_condition(victim, Condition::Paralyzed),
        "it is still Paralyzed for its owner's turn"
    );
    force_attach(&mut state);
    assert!(
        !legal_actions(&state)
            .iter()
            .any(|a| matches!(a, Action::Attack { .. })),
        "rule 51: Paralyzed cannot attack"
    );

    // The owner takes their turn; the checkup after it clears the paralysis.
    apply(&mut state, Action::EndTurn).unwrap();
    drive_setup_choices(&mut state);
    assert!(
        !state.has_condition(victim, Condition::Paralyzed),
        "rule 51: it recovers at the checkup after its owner's next turn"
    );
}

#[test]
fn confusion_flips_before_the_attack_and_tails_hurts_you() {
    // The flip that inflicts nothing, then tails for the confusion flip.
    let mut state = game(Condition::Confused, Box::new(ScriptedRng::new(vec![0])));
    sting(&mut state);
    drive_setup_choices(&mut state);

    let confused = state.player(state.current).active.unwrap();
    let defender = state.player(state.current.opponent()).active.unwrap();
    assert!(state.has_condition(confused, Condition::Confused));

    // Rule 52: Confused may still attack. The flip decides what happens.
    force_attach(&mut state);
    let attack = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("rule 52: Confused may still attack");
    apply(&mut state, attack).unwrap();

    assert_eq!(
        state.pokemon(confused).damage,
        30,
        "rule 52: tails puts 3 damage counters on your own Pokémon"
    );
    assert!(
        !state.has_condition(defender, Condition::Confused),
        "rule 52: on tails the attack does not happen"
    );
}

#[test]
fn a_heads_lets_a_confused_pokemon_attack() {
    let mut state = game(Condition::Confused, Box::new(ScriptedRng::new(vec![1])));
    sting(&mut state);
    drive_setup_choices(&mut state);

    let confused = state.player(state.current).active.unwrap();
    let defender = state.player(state.current.opponent()).active.unwrap();

    force_attach(&mut state);
    let attack = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("rule 52: Confused may still attack");
    apply(&mut state, attack).unwrap();

    assert_eq!(state.pokemon(confused).damage, 0, "heads costs nothing");
    assert!(
        state.has_condition(defender, Condition::Confused),
        "heads lets the attack happen"
    );
}

#[test]
fn a_rotation_condition_replaces_the_last_one() {
    let mut state = game(Condition::Asleep, Box::new(ScriptedRng::new(vec![0])));
    let victim = state.player(state.current.opponent()).active.unwrap();

    state.inflict(victim, Condition::Asleep);
    state.inflict(victim, Condition::Confused);
    assert!(
        !state.has_condition(victim, Condition::Asleep),
        "rule 55: Confused replaces Asleep, because both rotate the card"
    );
    assert!(state.has_condition(victim, Condition::Confused));

    state.inflict(victim, Condition::Paralyzed);
    assert!(!state.has_condition(victim, Condition::Confused));
    assert!(state.has_condition(victim, Condition::Paralyzed));
}

#[test]
fn burn_and_poison_coexist_with_each_other_and_with_a_rotation() {
    let mut state = game(Condition::Asleep, Box::new(ScriptedRng::new(vec![0])));
    let victim = state.player(state.current.opponent()).active.unwrap();

    state.inflict(victim, Condition::Burned);
    state.inflict(victim, Condition::Poisoned);
    state.inflict(victim, Condition::Asleep);

    assert!(state.has_condition(victim, Condition::Burned));
    assert!(state.has_condition(victim, Condition::Poisoned));
    assert!(
        state.has_condition(victim, Condition::Asleep),
        "rule 56: Burned and Poisoned are independent of a rotation"
    );
    assert_eq!(state.pokemon(victim).conditions.len(), 3);
}

#[test]
fn a_second_burn_replaces_the_first() {
    let mut state = game(Condition::Asleep, Box::new(ScriptedRng::new(vec![0])));
    let victim = state.player(state.current.opponent()).active.unwrap();

    state.inflict(victim, Condition::Burned);
    state.inflict(victim, Condition::Burned);
    assert_eq!(
        state.pokemon(victim).conditions.len(),
        1,
        "rule 57: a second Burn replaces the first rather than stacking"
    );
}

#[test]
fn retreating_removes_every_condition() {
    // A real shuffle, because this test reads no flip and needs a full Bench.
    let mut state = game(Condition::Asleep, Box::new(SeededRng::new(9)));
    let player = state.current;
    let active = state.player(player).active.unwrap();
    state.inflict(active, Condition::Poisoned);
    state.inflict(active, Condition::Burned);

    force_attach(&mut state);
    let bench = state.player(player).bench[0];
    apply(&mut state, Action::Retreat { to: bench }).unwrap();
    drive_setup_choices(&mut state);

    assert!(
        state.pokemon(active).conditions.is_empty(),
        "rule 27: moving to the Bench removes all Special Conditions"
    );
}

#[test]
fn only_the_active_carries_a_condition() {
    let state = game(Condition::Asleep, Box::new(ScriptedRng::new(vec![0])));
    for player in [sim::ids::PlayerId::One, sim::ids::PlayerId::Two] {
        for benched in &state.player(player).bench {
            assert!(
                state.pokemon(*benched).conditions.is_empty(),
                "rule 49: only the Active can carry a Special Condition"
            );
        }
    }
}

#[test]
fn a_checkup_knockout_takes_a_prize_before_the_next_turn() {
    let mut state = game_with_benches(Condition::Poisoned);
    let attacker = state.current;
    let victim = state.player(attacker.opponent()).active.unwrap();

    // One counter from Poison is enough to knock it out.
    let hp = state.pokemon_def(victim).hp;
    state.pokemon[victim.index()].damage = hp - 10;
    let prizes_before = state.player(attacker).prizes.len();
    let turn_before = state.turn_number;

    sting(&mut state);

    // Rule 47: the Poison is the victim's own effect, so they resolve it.
    assert_eq!(
        state.phase,
        Phase::Checkup {
            player: attacker.opponent()
        }
    );
    let poison = legal_actions(&state)[0];
    apply(&mut state, poison).unwrap();

    // Rule 48: the checkup knocks it out, and the owner promotes before the
    // next turn begins.
    assert_eq!(
        state.phase,
        Phase::Promoting {
            of: attacker.opponent(),
            chooser: attacker.opponent(),
        },
        "the owner of the knocked-out Pokémon chooses the next Active"
    );
    assert!(state.pokemon(victim).knocked_out);
    assert_eq!(
        state.player(attacker).prizes.len(),
        prizes_before - 1,
        "the opponent of the knocked-out player takes a Prize"
    );
    assert_eq!(
        state.turn_number, turn_before,
        "the next turn waits for the promotion"
    );

    let promote = legal_actions(&state)[0];
    apply(&mut state, promote).unwrap();
    assert_eq!(state.phase, Phase::Main);
    assert!(state.turn_number > turn_before, "then the next turn starts");
}
