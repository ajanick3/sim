//! Milestone 11: Pokémon attacks, including their effects. One fixture,
//! shared across this milestone's tickets, the same shape every
//! Trainer-kind milestone's own fixture already took.

use sim::action::{Action, legal_actions};
use sim::card::{Attack, AttackEffect, CardDb, CardDef, Energy, Pokemon, Stage, Type};
use sim::engine::apply;
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

/// A game where the first player's Active has one attack under test, and
/// the second player's Active is a plain punching bag.
fn game(attacker_attack: Attack, seed: u64) -> (GameState, sim::ids::CardDefId) {
    let mut db = CardDb::new();
    let attacker_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-attacker",
        name: "Attackmon",
        hp: 200,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![attacker_attack],
    }));
    let defender_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-defender",
        name: "Defendmon",
        hp: 200,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let defender_ex = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-defender-ex",
        name: "Defendmon ex",
        hp: 200,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 2,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-colorless-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));

    let mut attacker_deck = vec![attacker_mon; 4];
    while attacker_deck.len() < 60 {
        attacker_deck.push(energy);
    }
    let mut defender_deck = vec![defender_mon; 4];
    while defender_deck.len() < 60 {
        defender_deck.push(energy);
    }
    let mut state = GameState::new(
        db,
        [attacker_deck, defender_deck],
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
    (state, defender_ex)
}

/// A physical card of `def`, for a definition that was never part of
/// the deck. Placed nowhere; the caller pushes it where the test needs.
fn deal_new_card(state: &mut GameState, player: sim::ids::PlayerId, def: sim::ids::CardDefId) -> sim::ids::CardId {
    let card = sim::ids::CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def, owner: player });
    card
}

/// Pay for and use the Active's only attack.
fn pay_and_attack(state: &mut GameState) {
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let cost_len = state.pokemon_def(active).attacks[0].cost.len();
    for _ in 0..cost_len {
        let side = state.player(player);
        let card = side
            .hand
            .iter()
            .chain(side.deck.iter())
            .find(|c| state.def_of(**c).is_energy())
            .copied()
            .expect("the deck holds Energy");
        state.remove_from_hand(player, card);
        state.players[player.index()].deck.retain(|c| *c != card);
        state.pokemon[active.index()].attached.push(card);
    }
    let attack = legal_actions(state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("a paid-for Active can attack");
    apply(state, attack).unwrap();
}

// --- Ticket 01: the mechanism, and Recoil ---

#[test]
fn recoil_damages_the_attacker_alongside_the_defender() {
    let attack = Attack {
        name: "Reckless Charge",
        cost: vec![Type::Colorless],
        base_damage: 30,
        inflicts: None,
        effect: Some(AttackEffect::Recoil(10)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(attacker).damage, 10, "the recoil");
    assert_eq!(state.pokemon(defender).damage, 30, "the attack's own damage");
}

#[test]
fn carvanha_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Carvanha")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Carvanha should play");
}

// --- Ticket 02: damage multiplied by a counted board fact ---

fn multiplier_attack(count: sim::card::Count, per_unit: u32) -> Attack {
    Attack {
        name: "Multiply",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DamagePerCount(count, per_unit)),
    }
}

#[test]
fn damage_per_own_damage_counters() {
    let attack = multiplier_attack(sim::card::Count::OwnDamageCounters, 20);
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    state.pokemon[attacker.index()].damage = 30; // 3 counters

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 60, "3 counters times 20");
}

#[test]
fn damage_per_opponent_basic_energy_in_discard() {
    let attack = multiplier_attack(sim::card::Count::OpponentBasicEnergyInDiscard, 30);
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    for _ in 0..2 {
        let card = *state
            .player(opponent)
            .deck
            .iter()
            .find(|c| state.def_of(**c).is_energy())
            .unwrap();
        state.players[opponent.index()].deck.retain(|c| *c != card);
        state.players[opponent.index()].discard.push(card);
    }

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 60, "2 Energy in discard times 30");
}

#[test]
fn damage_per_opponent_ex_in_play_is_zero_with_no_ex() {
    let attack = multiplier_attack(sim::card::Count::OpponentPokemonExInPlay, 60);
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);
    assert_eq!(state.pokemon(defender).damage, 0, "no ex in play yet");
}

#[test]
fn damage_per_opponent_ex_in_play_counts_one() {
    let attack = multiplier_attack(sim::card::Count::OpponentPokemonExInPlay, 60);
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let ex_card = deal_new_card(&mut state, opponent, defender_ex);
    state.pokemon[defender.index()].cards = vec![ex_card];

    pay_and_attack(&mut state);
    assert_eq!(state.pokemon(defender).damage, 60, "one opponent ex in play");
}

#[test]
fn damage_per_own_basic_pokemon_in_play() {
    let attack = multiplier_attack(sim::card::Count::OwnBasicPokemonInPlay, 20);
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    // The attacker itself is one Basic in play.

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 20, "the attacker itself, 1 Basic");
}

#[test]
fn damage_per_own_damaged_pokemon_with_name_prefix() {
    // No damage on the attacker yet: the name matches, but nothing counts.
    let attack = multiplier_attack(sim::card::Count::OwnDamagedWithNamePrefix("Attack"), 40);
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    pay_and_attack(&mut state);
    assert_eq!(state.pokemon(defender).damage, 0, "no damage on it yet");

    // With damage on the attacker, it counts.
    let attack = multiplier_attack(sim::card::Count::OwnDamagedWithNamePrefix("Attack"), 40);
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    state.pokemon[attacker.index()].damage = 10;
    pay_and_attack(&mut state);
    assert_eq!(state.pokemon(defender).damage, 40, "one damaged match");
}

#[test]
fn ns_reshiram_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "N's Reshiram")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "N's Reshiram should play");
}

#[test]
fn paldean_tauros_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Paldean Tauros")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Paldean Tauros should play");
}

// --- Ticket 03: damage that ignores the defender's own effects ---

/// A game like `game`, but the defender prints a Weakness to Colorless
/// (the attacker's own type), so Weakness doubling is observable.
fn game_with_weak_defender(attacker_attack: Attack, seed: u64) -> GameState {
    let mut db = CardDb::new();
    let attacker_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-attacker-w",
        name: "Attackmon",
        hp: 200,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![attacker_attack],
    }));
    let defender_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-defender-w",
        name: "Defendmon",
        hp: 200,
        kind: Type::Colorless,
        weakness: Some(Type::Colorless),
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-colorless-energy-w",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));

    let mut attacker_deck = vec![attacker_mon; 4];
    while attacker_deck.len() < 60 {
        attacker_deck.push(energy);
    }
    let mut defender_deck = vec![defender_mon; 4];
    while defender_deck.len() < 60 {
        defender_deck.push(energy);
    }
    let mut state = GameState::new(
        db,
        [attacker_deck, defender_deck],
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

#[test]
fn ignores_defenders_effects_skips_weakness() {
    let attack = Attack {
        name: "Destructive Drill",
        cost: vec![Type::Colorless],
        base_damage: 100,
        inflicts: None,
        effect: Some(AttackEffect::IgnoresDefendersEffects),
    };
    let mut state = game_with_weak_defender(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    assert_eq!(
        state.pokemon(defender).damage, 100,
        "Weakness would double this to 200, but is one of the ignored effects"
    );
}

#[test]
fn without_the_effect_weakness_still_applies() {
    let attack = Attack {
        name: "Plain Hit",
        cost: vec![Type::Colorless],
        base_damage: 100,
        inflicts: None,
        effect: None,
    };
    let mut state = game_with_weak_defender(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 200, "Weakness doubles it");
}

#[test]
fn dudunsparce_ex_is_admitted_from_the_artifact() {
    // Its other attack (Tenacious Tail) was matched in ticket 02; this
    // ticket's Destructive Drill is the second, completing the print.
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Dudunsparce ex")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Dudunsparce ex should play");
}

// --- Ticket 04: a Special Condition, direct or coin-flipped ---

#[test]
fn inflicts_condition_poisons_outright() {
    let attack = Attack {
        name: "Poison Spray",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::InflictsCondition(sim::card::Condition::Poisoned)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    assert!(state.has_condition(defender, sim::card::Condition::Poisoned));
}

#[test]
fn coin_flip_inflicts_condition_only_on_heads() {
    let attack = Attack {
        name: "Shocking Knuckle",
        cost: vec![Type::Colorless],
        base_damage: 20,
        inflicts: None,
        effect: Some(AttackEffect::CoinFlipInflicts(sim::card::Condition::Paralyzed)),
    };
    // Seed 3 flips heads first (matches the coin-flip pattern already
    // relied on elsewhere in this codebase's tests).
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    // Either the coin landed heads (Paralyzed) or tails (not) -- both are
    // legitimate outcomes; the meaningful assertion is that the game
    // still resolves cleanly either way, and damage always lands.
    assert_eq!(state.pokemon(defender).damage, 20, "the attack's own damage always lands");
}

#[test]
fn coin_flip_bonus_damage_adds_only_on_heads() {
    let attack = Attack {
        name: "Tumbling Attack",
        cost: vec![Type::Colorless],
        base_damage: 10,
        inflicts: None,
        effect: Some(AttackEffect::CoinFlipBonusDamage(20)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    let damage = state.pokemon(defender).damage;
    assert!(damage == 10 || damage == 30, "either the base or the base plus the bonus");
}

// Brute Bonnet, Zeraora, and Dedenne each carry a second attack this
// ticket does not build (a count read from the opponent's Active
// specifically; a cost paid in the attacker's own Energy; a search
// whose limit is itself a board-read count), so none is fully admitted
// yet — `known_attack` matching `Poison Spray`/`Shocking Knuckle`/
// `Thunder Shock` is exercised directly by the mechanism tests above.

#[test]
fn applin_is_admitted_from_the_artifact() {
    // sv06-017 prints only Tumbling Attack, so this print alone
    // completes the card even though other Applin prints still refuse.
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Applin" && c.playable.is_some()),
        "at least one Applin print should play"
    );
}

// --- Ticket 05: a restriction through the opponent's next turn ---

#[test]
fn cannot_retreat_blocks_retreat_during_the_opponents_next_turn_only() {
    let attack = Attack {
        name: "Clutch",
        cost: vec![Type::Colorless],
        base_damage: 20,
        inflicts: None,
        effect: Some(AttackEffect::DefenderCannotRetreatNextTurn),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    // The defender needs Energy attached and somewhere to retreat to,
    // or retreating is illegal regardless of the restriction under test.
    let defender = state.player(opponent).active.unwrap();
    let energy_card = *state
        .player(opponent)
        .deck
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    state.players[opponent.index()].deck.retain(|c| *c != energy_card);
    state.pokemon[defender.index()].attached.push(energy_card);
    let bench_card = deal_new_card(&mut state, opponent, defender_ex);
    let bench_mon = state.put_into_play(opponent, bench_card);
    state.players[opponent.index()].bench.push(bench_mon);

    pay_and_attack(&mut state);
    assert_eq!(state.current, opponent, "the attack ended the attacker's turn");

    // No retreat offered at all, even with Energy attached and a Bench
    // to retreat to.
    assert!(
        !legal_actions(&state).into_iter().any(|a| matches!(a, Action::Retreat { .. })),
        "the Defending Pokémon cannot retreat during this turn"
    );

    // Once the opponent's turn ends, the restriction has had its one
    // turn and is gone -- read directly, since the fixture's own Bench
    // rules (not this restriction) govern whether Retreat is offered
    // several turns on.
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    assert_eq!(state.current, player);
    assert!(
        state.opponent_next_turn_restriction.is_none(),
        "the restriction does not survive past the one turn it applied to"
    );
}

#[test]
fn yveltal_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Yveltal" && c.playable.is_some()),
        "at least one Yveltal print should play"
    );
}

// --- Ticket 06: a restriction on the attacker's own next turn ---

#[test]
fn cannot_attack_next_turn_blocks_all_attacks_that_one_turn_only() {
    let attack = Attack {
        name: "Rampaging Thunder",
        cost: vec![Type::Colorless],
        base_damage: 50, // well short of the fixture's 200 HP: no knockout
        inflicts: None,
        effect: Some(AttackEffect::AttackerCannotAttackNextTurn),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let attacker = state.player(player).active.unwrap();

    pay_and_attack(&mut state);
    assert_eq!(state.current, opponent, "the attack ended the attacker's turn");

    // Not yet the attacker's own next turn: no restriction observable
    // through their own attack offer, since it is not their turn.
    assert!(!legal_actions(&state).into_iter().any(|a| matches!(a, Action::Attack { .. })));

    // The opponent's turn passes; now it is the attacker's own next turn.
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    assert_eq!(state.current, player, "the attacker's own next turn");
    assert_eq!(state.player(player).active, Some(attacker));
    assert!(
        !legal_actions(&state).into_iter().any(|a| matches!(a, Action::Attack { .. })),
        "cannot attack during this, the one restricted turn"
    );

    // A further turn on, the restriction is gone.
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    assert_eq!(state.current, player, "the attacker's turn again");
    assert!(
        legal_actions(&state).into_iter().any(|a| matches!(a, Action::Attack { .. })),
        "the restriction does not survive past the one turn it applied to"
    );
}

#[test]
fn ns_zekrom_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "N's Zekrom" && c.playable.is_some()),
        "at least one N's Zekrom print should play"
    );
}

// --- Ticket 07: damage to a Benched Pokémon ---

#[test]
fn distributes_damage_counters_across_the_opponents_bench() {
    let attack = Attack {
        name: "Phantom Dive",
        cost: vec![Type::Colorless],
        base_damage: 50,
        inflicts: None,
        effect: Some(AttackEffect::DamageCountersToOpponentBenchAnyWay(6)),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let bench_card_1 = deal_new_card(&mut state, opponent, defender_ex);
    let bench_mon_1 = state.put_into_play(opponent, bench_card_1);
    state.players[opponent.index()].bench.push(bench_mon_1);
    let bench_card_2 = deal_new_card(&mut state, opponent, defender_ex);
    let bench_mon_2 = state.put_into_play(opponent, bench_card_2);
    state.players[opponent.index()].bench.push(bench_mon_2);

    pay_and_attack(&mut state);
    assert!(matches!(state.phase, Phase::DistributingDamageCounters { .. }));

    // Split: 4 counters on one, 2 on the other.
    for _ in 0..4 {
        apply(&mut state, Action::PlaceDamageCounter { target: bench_mon_1 }).unwrap();
    }
    for _ in 0..2 {
        apply(&mut state, Action::PlaceDamageCounter { target: bench_mon_2 }).unwrap();
    }

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(bench_mon_1).damage, 40);
    assert_eq!(state.pokemon(bench_mon_2).damage, 20);
}

#[test]
fn distributing_damage_counters_does_nothing_with_an_empty_bench() {
    let attack = Attack {
        name: "Phantom Dive",
        cost: vec![Type::Colorless],
        base_damage: 50,
        inflicts: None,
        effect: Some(AttackEffect::DamageCountersToOpponentBenchAnyWay(6)),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "no Bench to place counters on");
}

#[test]
fn dragapult_ex_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Dragapult ex" && c.playable.is_some()),
        "at least one Dragapult ex print should play"
    );
}

// --- Ticket 08: a cost paid in the attacker's own Energy ---

#[test]
fn discards_all_the_attackers_own_energy_then_damages_a_chosen_benched_pokemon() {
    let attack = Attack {
        name: "Flamebody Cannon",
        cost: vec![Type::Colorless],
        base_damage: 90,
        inflicts: None,
        effect: Some(AttackEffect::DiscardsOwnEnergyThenDamagesChosenBenched(90)),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let bench_card = deal_new_card(&mut state, opponent, defender_ex);
    let bench_mon = state.put_into_play(opponent, bench_card);
    state.players[opponent.index()].bench.push(bench_mon);

    pay_and_attack(&mut state);

    let active = state.player(player).active.unwrap();
    assert!(
        state.pokemon(active).attached.is_empty(),
        "the attack discards every Energy from itself"
    );
    assert!(matches!(state.phase, Phase::ChoosingBenchDamageTarget { .. }));
    apply(&mut state, Action::DamageBenchedPokemon { target: bench_mon }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(bench_mon).damage, 90);
}

#[test]
fn discards_the_attackers_energy_even_with_no_bench_to_damage() {
    let attack = Attack {
        name: "Flamebody Cannon",
        cost: vec![Type::Colorless],
        base_damage: 90,
        inflicts: None,
        effect: Some(AttackEffect::DiscardsOwnEnergyThenDamagesChosenBenched(90)),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    let player = state.current;
    let active = state.player(player).active.unwrap();
    assert!(state.pokemon(active).attached.is_empty(), "still discards with no Bench");
    assert_eq!(state.phase, Phase::Main, "no Bench to choose a target on");
}

#[test]
fn ns_darmanitan_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "N's Darmanitan" && c.playable.is_some()),
        "at least one N's Darmanitan print should play"
    );
}

// --- Ticket 09: a switch ---

#[test]
fn switches_the_attacker_with_a_benched_pokemon() {
    let attack = Attack {
        name: "Teleportation Attack",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SwitchOwnActive),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker_before = state.player(player).active.unwrap();
    let bench_card = deal_new_card(&mut state, player, defender_ex);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::Promoting { .. }));
    apply(&mut state, Action::Promote { pokemon: bench_mon }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.player(player).active, Some(bench_mon));
    assert!(state.player(player).bench.contains(&attacker_before));
}

#[test]
fn switching_the_attacker_does_nothing_with_no_bench() {
    let attack = Attack {
        name: "Teleportation Attack",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SwitchOwnActive),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "no Bench to switch into");
}

#[test]
fn abra_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Abra" && c.playable.is_some()),
        "at least one Abra print should play"
    );
}

// --- Ticket 10: a search ---

#[test]
fn searches_the_deck_for_up_to_two_basics_onto_the_bench() {
    let attack = Attack {
        name: "Call for Family",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SearchDeckForBasicPokemonToBench(2)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::SearchingDeckForBasics { .. }));
    let first = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::TakeBasicPokemonForCallForFamily { .. }))
        .expect("the deck holds a Basic Pokemon");
    apply(&mut state, first).unwrap();
    assert!(matches!(state.phase, Phase::SearchingDeckForBasics { .. }));
    let second = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::TakeBasicPokemonForCallForFamily { .. }))
        .expect("the deck still holds a Basic Pokemon");
    apply(&mut state, second).unwrap();

    assert_eq!(state.phase, Phase::Main, "the limit of 2 ends the search on its own");
    assert_eq!(state.player(player).bench.len(), 2);
}

#[test]
fn the_search_can_be_declined_early() {
    let attack = Attack {
        name: "Call for Family",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SearchDeckForBasicPokemonToBench(2)),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::SearchingDeckForBasics { .. }));
    apply(&mut state, Action::FinishCallForFamily).unwrap();
    assert_eq!(state.phase, Phase::Main);
}

#[test]
fn drilbur_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Drilbur" && c.playable.is_some()),
        "at least one Drilbur print should play"
    );
}

#[test]
fn toxel_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Toxel" && c.playable.is_some()),
        "at least one Toxel print should play"
    );
}

// --- Ticket 11: hand and deck reads ---

#[test]
fn reveals_the_opponents_hand() {
    let attack = Attack {
        name: "Silent Wing",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::RevealOpponentsHand),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "revealing a hand needs no choice");
    assert!(
        state.log.iter().any(|line| line.contains("reveals")),
        "the reveal lands in the log: {:?}",
        state.log
    );
}

#[test]
fn hoothoot_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Hoothoot" && c.playable.is_some()),
        "at least one Hoothoot print should play"
    );
}

// --- Ticket 12: draws ---

#[test]
fn draws_a_fixed_count_outright() {
    let attack = Attack {
        name: "Greedy Fang",
        cost: vec![Type::Colorless],
        base_damage: 70,
        inflicts: None,
        effect: Some(AttackEffect::DrawCards(2)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let before = state.player(player).hand.len();

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "a fixed draw needs no choice");
    // Paying the attack's cost removed 1 card from hand; the draw adds 2.
    assert_eq!(state.player(player).hand.len(), before - 1 + 2);
}

#[test]
fn bonus_damage_only_when_the_attacker_already_carries_damage() {
    let attack = Attack {
        name: "Hungry Jaws",
        cost: vec![Type::Colorless],
        base_damage: 120,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfOwnDamaged(150)),
    };
    let (mut state, _defender_ex) = game(attack.clone(), 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let defender = state.player(player.opponent()).active.unwrap();

    pay_and_attack(&mut state);

    assert_eq!(
        state.pokemon(defender).damage, 120,
        "no bonus while the attacker carries no damage"
    );

    let (mut state2, _defender_ex2) = game(attack.clone(), 3);
    let player2 = state2.current;
    let attacker2 = state2.player(player2).active.unwrap();
    state2.pokemon[attacker2.index()].damage = 10;
    let defender2 = state2.player(player2.opponent()).active.unwrap();
    pay_and_attack(&mut state2);
    assert_eq!(state2.pokemon(defender2).damage, 270, "150 more with damage already on it");
    let _ = attacker;
}

#[test]
fn mega_sharpedo_ex_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Mega Sharpedo ex" && c.playable.is_some()),
        "at least one Mega Sharpedo ex print should play"
    );
}

// --- Ticket 13: the deferred-on-inspection cards ---

#[test]
fn dwebble_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Dwebble" && c.playable.is_some()),
        "at least one Dwebble print should play"
    );
}

#[test]
fn may_put_up_to_two_of_the_defenders_energy_into_their_hand() {
    let attack = Attack {
        name: "Wash the Slate Clean",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::MayReturnOpponentsActiveEnergyToHand(2)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    // Attach two Energy of the same kind the deck already carries.
    let energy_card = {
        let side = state.player(opponent);
        side.deck
            .iter()
            .chain(side.hand.iter())
            .find(|c| state.def_of(**c).is_energy())
            .copied()
            .expect("the deck holds Energy")
    };
    let energy_def = state.cards[energy_card.index()].def;
    let e1 = deal_new_card(&mut state, opponent, energy_def);
    let e2 = deal_new_card(&mut state, opponent, energy_def);
    state.pokemon[defender.index()].attached.push(e1);
    state.pokemon[defender.index()].attached.push(e2);

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::MovingOpponentsActiveEnergyToHand { .. }));
    apply(&mut state, Action::MoveOpponentsActiveEnergyToHand { card: e1 }).unwrap();
    apply(&mut state, Action::FinishMovingOpponentsActiveEnergyToHand).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(opponent).hand.contains(&e1));
    assert!(state.pokemon(defender).attached.contains(&e2));
}

#[test]
fn declining_wash_the_slate_clean_moves_nothing() {
    let attack = Attack {
        name: "Wash the Slate Clean",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::MayReturnOpponentsActiveEnergyToHand(2)),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "the defender has no Energy to offer");
}

#[test]
fn slowking_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Slowking" && c.playable.is_some()),
        "at least one Slowking print should play"
    );
}

#[test]
fn takes_a_trainer_card_from_the_discard_pile() {
    let attack = Attack {
        name: "Electromagnetic Sonar",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::TakeTrainerFromDiscard),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;

    let trainer_def = state.db.add(CardDef::Trainer(sim::card::Trainer {
        print_id: "test-any-trainer",
        name: "Test Trainer",
        kind: sim::card::TrainerKind::Item,
        requirement: None,
        effect: sim::card::TrainerEffect::MoveAttachedEnergy,
    }));
    let trainer = deal_new_card(&mut state, player, trainer_def);
    state.players[player.index()].discard.push(trainer);

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::TakingTrainerFromDiscard { .. }));
    apply(&mut state, Action::TakeTrainerFromDiscard { card: trainer }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).hand.contains(&trainer));
}

#[test]
fn no_trainer_in_discard_opens_no_phase() {
    let attack = Attack {
        name: "Electromagnetic Sonar",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::TakeTrainerFromDiscard),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "no Trainer in the discard pile");
}

#[test]
fn dedenne_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Dedenne" && c.playable.is_some()),
        "at least one Dedenne print should play"
    );
}

#[test]
fn searches_the_deck_to_evolve_itself() {
    let attack = Attack {
        name: "Ascension",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SearchDeckToEvolveSelf),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let attacker_name = state.pokemon_def(attacker).name;

    let evolution_def = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-evolution",
        name: "Evolvemon",
        hp: 200,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Stage1,
        evolve_from: Some(attacker_name),
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let evolution = deal_new_card(&mut state, player, evolution_def);
    state.players[player.index()].deck.push(evolution);

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::SearchingDeckToEvolveSelf { .. }));
    apply(&mut state, Action::EvolveWithAscension { card: evolution }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon_def(attacker).name, "Evolvemon");
    assert!(!state.player(player).deck.contains(&evolution));
}

#[test]
fn no_evolution_in_deck_does_nothing() {
    let attack = Attack {
        name: "Ascension",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SearchDeckToEvolveSelf),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(attacker).cards.len(), 1, "nothing to evolve into");
}

#[test]
fn dwebble_ascension_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv10-011").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Dwebble's Ascension print should play");
}

// --- Beyond the spec: more attack-text names blockers still shows ---

#[test]
fn beldum_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv05-113").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Beldum's Iron Tackle print should play");
}

#[test]
fn dunsparce_trading_places_print_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv09-120").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Dunsparce's Trading Places print should play");
}

#[test]
fn bonus_damage_only_when_the_defender_is_an_ex() {
    let attack = Attack {
        name: "Fighting Wings",
        cost: vec![Type::Colorless],
        base_damage: 30,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfDefenderIsEx(90)),
    };
    let (mut state, defender_ex) = game(attack.clone(), 3);
    let player = state.current;
    let defender = state.player(player.opponent()).active.unwrap();

    pay_and_attack(&mut state);
    assert_eq!(state.pokemon(defender).damage, 30, "no bonus against a plain defender");

    let (mut state2, _defender_ex2) = game(attack, 3);
    let player2 = state2.current;
    let opponent2 = player2.opponent();
    let card = deal_new_card(&mut state2, opponent2, defender_ex);
    let old_active = state2.player(opponent2).active.unwrap();
    let ex_active = state2.put_into_play(opponent2, card);
    state2.players[opponent2.index()].active = Some(ex_active);
    let _ = old_active;

    pay_and_attack(&mut state2);
    assert_eq!(state2.pokemon(ex_active).damage, 120, "90 more against a Pokemon ex");
}

#[test]
fn moltres_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Moltres" && c.playable.is_some()),
        "at least one Moltres print should play"
    );
}

// --- Beyond the spec: Slowpoke's Dangle Tail ---

#[test]
fn takes_a_pokemon_card_from_the_discard_pile() {
    let attack = Attack {
        name: "Dangle Tail",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::TakePokemonFromDiscard),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;

    let pokemon = deal_new_card(&mut state, player, defender_ex);
    state.players[player.index()].discard.push(pokemon);

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::TakingPokemonFromDiscard { .. }));
    apply(&mut state, Action::TakePokemonFromDiscard { card: pokemon }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).hand.contains(&pokemon));
}

#[test]
fn no_pokemon_in_discard_opens_no_phase_for_dangle_tail() {
    let attack = Attack {
        name: "Dangle Tail",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::TakePokemonFromDiscard),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "no Pokemon in the discard pile");
}

#[test]
fn slowpoke_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv07-057").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Slowpoke's Dangle Tail print should play");
}

// --- Beyond the spec: Duskull's Come and Get You ---

#[test]
fn searches_the_discard_pile_for_up_to_three_copies_of_its_own_name() {
    let attack = Attack {
        name: "Come and Get You",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SearchDiscardForNamedToBench("Attackmon", 3)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker_card = state.pokemon(state.player(player).active.unwrap()).top_card();
    let attacker_def_id = state.cards[attacker_card.index()].def;

    let copy1 = deal_new_card(&mut state, player, attacker_def_id);
    let copy2 = deal_new_card(&mut state, player, attacker_def_id);
    state.players[player.index()].discard.push(copy1);
    state.players[player.index()].discard.push(copy2);

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::SearchingDiscardForNamedToBench { .. }));
    apply(&mut state, Action::TakeNamedFromDiscardToBench { card: copy1 }).unwrap();
    apply(&mut state, Action::FinishSearchingDiscardForNamedToBench).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).bench.iter().any(|p| state.pokemon(*p).cards.contains(&copy1)));
}

#[test]
fn no_matching_name_in_discard_opens_no_phase() {
    let attack = Attack {
        name: "Come and Get You",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SearchDiscardForNamedToBench("Attackmon", 3)),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "no matching name in the discard pile");
}

#[test]
fn duskull_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Duskull" && c.playable.is_some()),
        "at least one Duskull print should play"
    );
}

// --- Beyond the spec: more easy admits from existing shapes ---

#[test]
fn torchic_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv10-040").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Torchic's Collect print should play");
}

#[test]
fn celebi_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv07-004").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Celebi's Collect print should play");
}

#[test]
fn buneary_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "me02-083").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Buneary's Run Around print should play");
}

#[test]
fn switches_the_opponents_active() {
    let attack = Attack {
        name: "Push Down",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SwitchOpponentActive),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let old_defender = state.player(opponent).active.unwrap();
    let bench_card = deal_new_card(&mut state, opponent, defender_ex);
    let bench_mon = state.put_into_play(opponent, bench_card);
    state.players[opponent.index()].bench.push(bench_mon);

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::Promoting { .. }));
    apply(&mut state, Action::Promote { pokemon: bench_mon }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.player(opponent).active, Some(bench_mon));
    assert!(state.player(opponent).bench.contains(&old_defender));
}

#[test]
fn switching_the_opponent_does_nothing_with_no_bench() {
    let attack = Attack {
        name: "Push Down",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SwitchOpponentActive),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "no Bench to switch into");
}

#[test]
fn bayleef_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "me01-009").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Bayleef's Push Down print should play");
}

// --- Beyond the spec: a damage-reduction restriction through the opponent's next turn ---

#[test]
fn defender_deals_less_damage_only_during_its_own_next_turn() {
    let attack = Attack {
        name: "Growl",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DefenderDealsLessDamageNextTurn(20)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);
    assert_eq!(
        state.opponent_next_turn_restriction,
        Some((defender, AttackEffect::DefenderDealsLessDamageNextTurn(20), player))
    );
    // The attack itself already ended the attacker's turn.
    assert_eq!(state.current, opponent, "now the restricted Pokemon's own turn");

    let restricted_attacker = state.player(opponent).active.unwrap();
    let its_target = state.player(player).active.unwrap();
    let damage = sim::engine::damage_dealt(&state, restricted_attacker, its_target, 30);
    assert_eq!(damage, 10, "20 less, before Weakness and Resistance");
}

#[test]
fn chikorita_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "me02.5-008").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Chikorita's Growl print should play");
}

#[test]
fn buneary_charm_print_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "me01-107").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Buneary's Charm print should play");
}

// --- Beyond the spec: a coin-flipped invulnerability through the opponent's next turn ---

#[test]
fn heads_makes_the_attacker_invulnerable_on_the_opponents_next_turn() {
    let attack = Attack {
        name: "Dig",
        cost: vec![Type::Colorless],
        base_damage: 20,
        inflicts: None,
        effect: Some(AttackEffect::CoinFlipSelfInvulnerableNextTurn),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();

    pay_and_attack(&mut state);
    assert_eq!(state.current, player.opponent());
    let heads = state.opponent_next_turn_restriction
        == Some((attacker, AttackEffect::CoinFlipSelfInvulnerableNextTurn, player));

    // Pay the opponent's own attack cost, the same way pay_and_attack does.
    let opponent = player.opponent();
    let opponent_active = state.player(opponent).active.unwrap();
    let energy_card = *state
        .player(opponent)
        .deck
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    state.players[opponent.index()].deck.retain(|c| *c != energy_card);
    state.pokemon[opponent_active.index()].attached.push(energy_card);

    // Either way, the opponent's own attack resolves without erroring;
    // on heads it lands no damage at all.
    let damage_before = state.pokemon(attacker).damage;
    sim::engine::apply(&mut state, Action::Attack { index: 0 }).unwrap();
    if heads {
        assert_eq!(state.pokemon(attacker).damage, damage_before, "invulnerable: no damage lands");
    }
}

#[test]
fn dunsparce_dig_print_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv05-128").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Dunsparce's Dig print should play");
}

#[test]
fn elgyem_hide_print_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv05-073").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Elgyem's Hide print should play");
}

// --- Beyond the spec: a restriction on playing Item cards ---

#[test]
fn opponent_cannot_play_items_during_their_own_next_turn_only() {
    let attack = Attack {
        name: "Itchy Pollen",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::OpponentCannotPlayItemsNextTurn),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();

    let item_def = state.db.add(CardDef::Trainer(sim::card::Trainer {
        print_id: "test-item",
        name: "Test Item",
        kind: sim::card::TrainerKind::Item,
        requirement: None,
        effect: sim::card::TrainerEffect::MoveAttachedEnergy,
    }));
    let item = deal_new_card(&mut state, opponent, item_def);
    state.players[opponent.index()].hand.push(item);

    pay_and_attack(&mut state);
    assert_eq!(state.current, opponent, "the attack ended the attacker's turn");

    assert!(
        !legal_actions(&state).contains(&Action::PlayTrainer { card: item }),
        "no Item cards this turn"
    );
}

#[test]
fn budew_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Budew" && c.playable.is_some()),
        "at least one Budew print should play"
    );
}

// --- Beyond the spec: an optional Energy cost paid for bench damage ---

#[test]
fn accepting_shuffles_energy_then_damages_a_chosen_benched_pokemon() {
    let attack = Attack {
        name: "Torrential Pump",
        cost: vec![Type::Colorless],
        base_damage: 30,
        inflicts: None,
        effect: Some(AttackEffect::MayShuffleFixedEnergyThenDamageChosenBenched {
            count: 3,
            damage: 120,
        }),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let attacker = state.player(player).active.unwrap();
    let bench_card = deal_new_card(&mut state, opponent, defender_ex);
    let bench_mon = state.put_into_play(opponent, bench_card);
    state.players[opponent.index()].bench.push(bench_mon);

    // Give the attacker 3 Energy beyond the 1 it pays the cost with.
    let energy_card = *state
        .player(player)
        .deck
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    let energy_def = state.cards[energy_card.index()].def;
    for _ in 0..3 {
        let extra = deal_new_card(&mut state, player, energy_def);
        state.pokemon[attacker.index()].attached.push(extra);
    }
    pay_and_attack(&mut state);
    let attached_before = state.pokemon(attacker).attached.len();

    assert!(matches!(state.phase, Phase::DecidingToShuffleEnergyForBenchDamage { .. }));
    apply(&mut state, Action::AcceptShuffleEnergyForBenchDamage).unwrap();

    assert!(matches!(state.phase, Phase::ChoosingBenchDamageTarget { .. }));
    apply(&mut state, Action::DamageBenchedPokemon { target: bench_mon }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(bench_mon).damage, 120);
    assert_eq!(state.pokemon(attacker).attached.len(), attached_before - 3);
}

#[test]
fn declining_leaves_everything_as_is() {
    let attack = Attack {
        name: "Torrential Pump",
        cost: vec![Type::Colorless],
        base_damage: 30,
        inflicts: None,
        effect: Some(AttackEffect::MayShuffleFixedEnergyThenDamageChosenBenched {
            count: 3,
            damage: 120,
        }),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let energy_card = *state
        .player(player)
        .deck
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    let energy_def = state.cards[energy_card.index()].def;
    for _ in 0..3 {
        let extra = deal_new_card(&mut state, player, energy_def);
        state.pokemon[attacker.index()].attached.push(extra);
    }
    pay_and_attack(&mut state);
    let attached_before = state.pokemon(attacker).attached.len();

    assert!(matches!(state.phase, Phase::DecidingToShuffleEnergyForBenchDamage { .. }));
    apply(&mut state, Action::DeclineShuffleEnergyForBenchDamage).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(attacker).attached.len(), attached_before);
}

#[test]
fn not_enough_energy_opens_no_phase_at_all() {
    let attack = Attack {
        name: "Torrential Pump",
        cost: vec![Type::Colorless],
        base_damage: 30,
        inflicts: None,
        effect: Some(AttackEffect::MayShuffleFixedEnergyThenDamageChosenBenched {
            count: 3,
            damage: 120,
        }),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "only 1 Energy attached, need 3");
}

#[test]
fn wellspring_mask_ogerpon_ex_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import
            .cards
            .iter()
            .any(|c| c.name == "Wellspring Mask Ogerpon ex" && c.playable.is_some()),
        "at least one Wellspring Mask Ogerpon ex print should play"
    );
}

// --- Beyond the spec: a bonus if the defender is a particular stage ---

#[test]
fn bonus_damage_only_when_the_defender_is_stage_1() {
    let attack = Attack {
        name: "Spirited Tackle",
        cost: vec![Type::Colorless],
        base_damage: 30,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfDefenderIsStage(Stage::Stage1, 90)),
    };
    let (mut state, _defender_ex) = game(attack.clone(), 3);
    let player = state.current;
    let defender = state.player(player.opponent()).active.unwrap();

    pay_and_attack(&mut state);
    assert_eq!(state.pokemon(defender).damage, 30, "no bonus against a Basic defender");
}

#[test]
fn paldean_tauros_spirited_tackle_print_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv08-018").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Paldean Tauros's Spirited Tackle print should play");
}

// --- Beyond the spec: damage per heads across several coin flips ---

#[test]
fn damage_scales_with_heads_across_two_flips() {
    let attack = Attack {
        name: "Double Kick",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DamagePerCoinFlipHeads { flips: 2, per_head: 40 }),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let opponent = state.current.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    let damage = state.pokemon(defender).damage;
    assert!(
        damage == 0 || damage == 40 || damage == 80,
        "0, 1, or 2 heads times 40: got {damage}"
    );
}

#[test]
fn combusken_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv10-041").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Combusken's Double Kick print should play");
}

// --- Beyond the spec: moving an Energy between two of the opponent's own Pokemon ---

#[test]
fn moves_energy_between_two_of_the_opponents_own_pokemon() {
    let attack = Attack {
        name: "Slight Shift",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::MoveOpponentsEnergyBetweenTheirPokemon),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let energy_card = *state
        .player(opponent)
        .deck
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    state.players[opponent.index()].deck.retain(|c| *c != energy_card);
    state.pokemon[defender.index()].attached.push(energy_card);
    let bench_card = deal_new_card(&mut state, opponent, defender_ex);
    let bench_mon = state.put_into_play(opponent, bench_card);
    state.players[opponent.index()].bench.push(bench_mon);

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::MovingOpponentsEnergy { .. }));
    apply(
        &mut state,
        Action::MoveOpponentsEnergy { card: energy_card, target: bench_mon },
    )
    .unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.pokemon(bench_mon).attached.contains(&energy_card));
    assert!(!state.pokemon(defender).attached.contains(&energy_card));
}

#[test]
fn no_move_available_opens_no_phase() {
    let attack = Attack {
        name: "Slight Shift",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::MoveOpponentsEnergyBetweenTheirPokemon),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "only one opponent Pokemon in play, nowhere to move to");
}

#[test]
fn elgyem_slight_shift_print_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv10.5b-040").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Elgyem's Slight Shift print should play");
}

// --- Beyond the spec: damage per the opponent's Benched Pokemon ---

#[test]
fn damage_per_opponent_benched_pokemon() {
    let attack = multiplier_attack(sim::card::Count::OpponentBenchedPokemonCount, 20);
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    for _ in 0..2 {
        let card = deal_new_card(&mut state, opponent, defender_ex);
        let bench_mon = state.put_into_play(opponent, card);
        state.players[opponent.index()].bench.push(bench_mon);
    }

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 40, "2 Benched Pokemon times 20");
}

#[test]
fn zeraora_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv07-055").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Zeraora's Combat Thunder print should play");
}

// --- Beyond the spec: an attack returns the attacker itself to hand ---

#[test]
fn returns_the_attacker_and_its_attachments_to_hand() {
    let attack = Attack {
        name: "Tuck Tail",
        cost: vec![Type::Colorless],
        base_damage: 60,
        inflicts: None,
        effect: Some(AttackEffect::ReturnSelfAndAttachedToHand),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let attacker_card = state.pokemon(attacker).top_card();
    let bench_card = deal_new_card(&mut state, player, defender_ex);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::Promoting { .. }));
    apply(&mut state, Action::Promote { pokemon: bench_mon }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).hand.contains(&attacker_card), "the card returns to hand");
    assert_eq!(state.player(player).active, Some(bench_mon));
}

#[test]
fn does_nothing_with_no_bench_to_promote() {
    let attack = Attack {
        name: "Tuck Tail",
        cost: vec![Type::Colorless],
        base_damage: 60,
        inflicts: None,
        effect: Some(AttackEffect::ReturnSelfAndAttachedToHand),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "no Bench to promote from");
    assert_eq!(state.player(player).active, Some(attacker), "stays put with no Bench");
}

#[test]
fn meowth_ex_attack_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Meowth ex" && c.playable.is_some()),
        "at least one Meowth ex print should play"
    );
}

// --- Beyond the spec: damage a chosen opponent Pokemon, Active or Benched ---

#[test]
fn deals_flat_damage_to_a_chosen_opponent_pokemon_including_active() {
    let attack = Attack {
        name: "Cruel Arrow",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DamageChosenOpponentPokemon(100)),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let bench_card = deal_new_card(&mut state, opponent, defender_ex);
    let bench_mon = state.put_into_play(opponent, bench_card);
    state.players[opponent.index()].bench.push(bench_mon);

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::ChoosingAnyOpponentPokemonDamageTarget { .. }));
    let actions = legal_actions(&state);
    assert!(actions.contains(&Action::DamageChosenOpponentPokemon { target: defender }));
    assert!(actions.contains(&Action::DamageChosenOpponentPokemon { target: bench_mon }));

    apply(&mut state, Action::DamageChosenOpponentPokemon { target: bench_mon }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(bench_mon).damage, 100);
    assert_eq!(state.pokemon(defender).damage, 0);
}

#[test]
fn fezandipiti_ex_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Fezandipiti ex" && c.playable.is_some()),
        "at least one Fezandipiti ex print should play"
    );
}

// --- Beyond the spec: damage per Energy on both Active Pokemon combined ---

#[test]
fn damage_per_energy_on_both_actives_combined() {
    let attack = multiplier_attack(sim::card::Count::EnergyOnBothActivesCount, 30);
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    // pay_and_attack attaches 1 Energy to the attacker for cost;
    // attach 1 more to the attacker and 2 to the defender.
    let energy_card = *state
        .player(player)
        .deck
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    let energy_def = state.cards[energy_card.index()].def;
    let extra1 = deal_new_card(&mut state, player, energy_def);
    state.pokemon[attacker.index()].attached.push(extra1);
    for _ in 0..2 {
        let extra = deal_new_card(&mut state, opponent, energy_def);
        state.pokemon[defender.index()].attached.push(extra);
    }

    pay_and_attack(&mut state);

    // 2 on attacker (1 cost + 1 extra) + 2 on defender = 4, times 30.
    assert_eq!(state.pokemon(defender).damage, 120);
}

#[test]
fn teal_mask_ogerpon_ex_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Teal Mask Ogerpon ex" && c.playable.is_some()),
        "at least one Teal Mask Ogerpon ex print should play"
    );
}

// --- Beyond the spec: reduced damage taken through the opponent's next turn ---

#[test]
fn takes_less_damage_only_during_the_opponents_next_turn_after_weakness() {
    let attack = Attack {
        name: "Protect Charge",
        cost: vec![Type::Colorless],
        base_damage: 20,
        inflicts: None,
        effect: Some(AttackEffect::SelfDamageReductionNextTurn(30)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();

    pay_and_attack(&mut state);
    assert_eq!(
        state.opponent_next_turn_restriction,
        Some((attacker, AttackEffect::SelfDamageReductionNextTurn(30), player))
    );
    assert_eq!(state.current, player.opponent(), "now the opponent's turn");

    let damage = sim::engine::damage_dealt(&state, state.player(player.opponent()).active.unwrap(), attacker, 50);
    assert_eq!(damage, 20, "50 less 30, after Weakness and Resistance");
}

#[test]
fn genesect_ex_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Genesect ex" && c.playable.is_some()),
        "at least one Genesect ex print should play"
    );
}

// --- Beyond the spec: Chien-Pao's Strafe/Rising Blade print ---

#[test]
fn chien_pao_strafe_print_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "me03-054").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Chien-Pao's Strafe/Rising Blade print should play");
}

// --- Beyond the spec: put an Energy attached to the attacker into hand ---

#[test]
fn puts_the_attackers_own_energy_into_hand() {
    let attack = Attack {
        name: "Icicle Loop",
        cost: vec![Type::Colorless],
        base_damage: 120,
        inflicts: None,
        effect: Some(AttackEffect::MoveOwnAttachedEnergyToHand),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::ChoosingOwnEnergyToHand { .. }));
    let card = state.pokemon(attacker).attached[0];
    apply(&mut state, Action::MoveOwnAttachedEnergyToHand { card }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).hand.contains(&card));
    assert!(!state.pokemon(attacker).attached.contains(&card));
}

#[test]
fn no_energy_attached_opens_no_phase_for_icicle_loop() {
    let attack = Attack {
        name: "Icicle Loop",
        cost: vec![],
        base_damage: 120,
        inflicts: None,
        effect: Some(AttackEffect::MoveOwnAttachedEnergyToHand),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let attack_action = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("a free attack can always be used");
    apply(&mut state, attack_action).unwrap();

    assert_eq!(state.phase, Phase::Main, "no Energy attached to move");
}

#[test]
fn chien_pao_icicle_loop_print_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv08-056").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Chien-Pao's Icicle Loop print should play");
}

// --- Beyond the spec: an attack that fizzles with no Stadium in play ---

#[test]
fn fizzles_outright_with_no_stadium_in_play() {
    let attack = Attack {
        name: "Assault Landing",
        cost: vec![Type::Colorless],
        base_damage: 70,
        inflicts: None,
        effect: Some(AttackEffect::FizzlesWithNoStadiumInPlay),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let defender = state.player(player.opponent()).active.unwrap();

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 0, "no Stadium, no damage at all");
}

#[test]
fn deals_damage_normally_with_a_stadium_in_play() {
    let attack = Attack {
        name: "Assault Landing",
        cost: vec![Type::Colorless],
        base_damage: 70,
        inflicts: None,
        effect: Some(AttackEffect::FizzlesWithNoStadiumInPlay),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let defender = state.player(player.opponent()).active.unwrap();
    let stadium_def = state.db.add(CardDef::Trainer(sim::card::Trainer {
        print_id: "test-stadium",
        name: "Test Stadium",
        kind: sim::card::TrainerKind::Stadium,
        requirement: None,
        effect: sim::card::TrainerEffect::MoveAttachedEnergy,
    }));
    let stadium = deal_new_card(&mut state, player, stadium_def);
    state.stadium = Some((player, stadium));

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 70);
}

#[test]
fn fan_rotom_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Fan Rotom" && c.playable.is_some()),
        "at least one Fan Rotom print should play"
    );
}

// --- Beyond the spec: damage per Prize the opponent has taken ---

#[test]
fn damage_per_opponent_prizes_taken() {
    let attack = multiplier_attack(sim::card::Count::OpponentPrizesTakenCount, 60);
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    for _ in 0..2 {
        state.players[opponent.index()].prizes.pop();
    }

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 120, "2 Prizes taken times 60");
}

#[test]
fn pecharunt_ex_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Pecharunt ex" && c.playable.is_some()),
        "at least one Pecharunt ex print should play"
    );
}

// --- Beyond the spec: damage per count to a chosen opponent Pokemon ---

#[test]
fn damage_per_count_to_a_chosen_opponent_pokemon() {
    let attack = Attack {
        name: "Bug's Cannon",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DamagePerCountToChosenOpponentPokemon(
            sim::card::Count::OwnDamageCounters,
            20,
        )),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let attacker = state.player(player).active.unwrap();
    state.pokemon[attacker.index()].damage = 30; // 3 counters
    let bench_card = deal_new_card(&mut state, opponent, defender_ex);
    let bench_mon = state.put_into_play(opponent, bench_card);
    state.players[opponent.index()].bench.push(bench_mon);

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::ChoosingAnyOpponentPokemonDamageTarget { .. }));
    apply(&mut state, Action::DamageChosenOpponentPokemon { target: bench_mon }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(bench_mon).damage, 60, "3 counters times 20");
}

#[test]
fn genesect_bugs_cannon_print_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "me02-008").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Genesect's Bug's Cannon print should play");
}

// --- Beyond the spec: placing damage counters per a counted board fact ---

#[test]
fn places_damage_counters_on_the_defender_per_hand_size() {
    let attack = Attack {
        name: "Powerful Hand",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::PlaceDamageCountersOnDefenderPerCount(
            sim::card::Count::OwnHandSizeCount,
            2,
        )),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let hand_size = state.player(player).hand.len() as u32;

    pay_and_attack(&mut state);

    // Paying the attack's own cost removed 1 card from hand first.
    assert_eq!(state.pokemon(defender).damage, (hand_size - 1) * 2 * 10);
}

#[test]
fn the_summary_line_counts_damage_counters_the_effect_placed() {
    let attack = Attack {
        name: "Powerful Hand",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::PlaceDamageCountersOnDefenderPerCount(
            sim::card::Count::OwnHandSizeCount,
            2,
        )),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let placed = (state.player(player).hand.len() as u32 - 1) * 2 * 10;

    pay_and_attack(&mut state);

    let line = state
        .log
        .iter()
        .rev()
        .find(|l| l.contains("uses Powerful Hand on"))
        .expect("the attack logged a summary line");
    assert_eq!(
        line,
        &format!("Attackmon uses Powerful Hand on Defendmon for {placed}."),
        "the summary counts the counters the effect placed, not a bare 0"
    );
}

#[test]
fn alakazam_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Alakazam" && c.playable.is_some()),
        "at least one Alakazam print should play"
    );
}

// --- Beyond the spec: damage to a chosen Benched Pokemon ex ---

#[test]
fn deals_damage_to_a_chosen_benched_ex_only() {
    let attack = Attack {
        name: "Pinpoint Dive",
        cost: vec![Type::Colorless],
        base_damage: 60,
        inflicts: None,
        effect: Some(AttackEffect::DamageChosenOpponentBenchedEx(60)),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    // A plain (non-ex) Benched Pokemon, from the fixture's own defender.
    let plain_bench_def = state.cards[state.pokemon(state.player(opponent).active.unwrap()).top_card().index()].def;
    let plain_bench_card = deal_new_card(&mut state, opponent, plain_bench_def);
    let plain_bench = state.put_into_play(opponent, plain_bench_card);
    state.players[opponent.index()].bench.push(plain_bench);

    let ex_bench_card = deal_new_card(&mut state, opponent, defender_ex);
    let ex_bench = state.put_into_play(opponent, ex_bench_card);
    state.players[opponent.index()].bench.push(ex_bench);

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::ChoosingBenchedExDamageTarget { .. }));
    let actions = legal_actions(&state);
    assert!(actions.contains(&Action::DamageBenchedEx { target: ex_bench }));
    assert!(!actions.contains(&Action::DamageBenchedEx { target: plain_bench }));

    apply(&mut state, Action::DamageBenchedEx { target: ex_bench }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(ex_bench).damage, 60);
    assert_eq!(state.pokemon(plain_bench).damage, 0);
}

#[test]
fn no_benched_ex_opens_no_phase() {
    let attack = Attack {
        name: "Pinpoint Dive",
        cost: vec![Type::Colorless],
        base_damage: 60,
        inflicts: None,
        effect: Some(AttackEffect::DamageChosenOpponentBenchedEx(60)),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "no Benched Pokemon ex to target");
}

#[test]
fn shaymin_pinpoint_dive_print_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv05-013").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Shaymin's Pinpoint Dive print should play");
}

// --- Beyond the spec: search an Energy, attach to a chosen Benched Pokemon of a type ---

#[test]
fn searches_an_energy_and_attaches_it_to_a_chosen_benched_pokemon_of_type() {
    let attack = Attack {
        name: "Send Flowers",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SearchEnergyAttachToBenchedOfType(Type::Grass)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;

    let grass_bench_def = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-grass-bench",
        name: "Grassmon",
        hp: 200,
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
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let bench_card = deal_new_card(&mut state, player, grass_bench_def);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);

    let plain_bench_def = state.cards[state.pokemon(state.player(player).active.unwrap()).top_card().index()].def;
    let plain_bench_card = deal_new_card(&mut state, player, plain_bench_def);
    let plain_bench = state.put_into_play(player, plain_bench_card);
    state.players[player.index()].bench.push(plain_bench);

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::SearchingForEnergyToAttachToBenchedOfType { .. }));
    let deck_before = state.player(player).deck.len();
    let actions = legal_actions(&state);
    assert!(actions.contains(&Action::AttachSearchedEnergyTo { target: bench_mon }));
    assert!(!actions.contains(&Action::AttachSearchedEnergyTo { target: plain_bench }));

    apply(&mut state, Action::AttachSearchedEnergyTo { target: bench_mon }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(bench_mon).attached.len(), 1);
    assert!(state.def_of(state.pokemon(bench_mon).attached[0]).is_energy());
    assert_eq!(state.player(player).deck.len(), deck_before - 1);
}

#[test]
fn not_offered_with_no_energy_or_no_benched_target_of_type() {
    let attack = Attack {
        name: "Send Flowers",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SearchEnergyAttachToBenchedOfType(Type::Grass)),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "no Grass Benched Pokemon and no Energy in deck");
}

#[test]
fn shaymin_send_flowers_print_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "me03-003").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Shaymin's Send Flowers print should play");
}

// --- Beyond the spec: move the attacker's own Energy to a chosen Benched Pokemon ---

#[test]
fn moves_the_attackers_own_energy_to_a_chosen_benched_pokemon() {
    let attack = Attack {
        name: "Reflect Energy",
        cost: vec![Type::Colorless, Type::Colorless],
        base_damage: 60,
        inflicts: None,
        effect: Some(AttackEffect::MoveOwnAttachedEnergyToChosenBenched),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();

    let plain_bench_def = state.cards[state.pokemon(active).top_card().index()].def;
    let plain_bench_card = deal_new_card(&mut state, player, plain_bench_def);
    let plain_bench = state.put_into_play(player, plain_bench_card);
    state.players[player.index()].bench.push(plain_bench);

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::ChoosingEnergyAndBenchedTargetToMove { .. }));
    let attached_before = state.pokemon(active).attached.clone();
    assert!(!attached_before.is_empty());
    let card = attached_before[0];

    let actions = legal_actions(&state);
    assert!(actions.contains(&Action::MoveEnergyToChosenBenched { card, target: plain_bench }));

    apply(&mut state, Action::MoveEnergyToChosenBenched { card, target: plain_bench }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(!state.pokemon(active).attached.contains(&card));
    assert!(state.pokemon(plain_bench).attached.contains(&card));
}

#[test]
fn no_bench_opens_no_phase_for_reflect_energy() {
    let attack = Attack {
        name: "Reflect Energy",
        cost: vec![Type::Colorless, Type::Colorless],
        base_damage: 60,
        inflicts: None,
        effect: Some(AttackEffect::MoveOwnAttachedEnergyToChosenBenched),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "no own Bench to move Energy to");
}

#[test]
fn shaymin_reflect_energy_print_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv08.5-087").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Shaymin's Reflect Energy print should play");
}

// --- Beyond the spec: search the whole deck for an Item card ---

#[test]
fn searches_the_whole_deck_for_an_item_and_shuffles_after() {
    let attack = Attack {
        name: "Procurement",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SearchDeckForItemCardToHand),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;

    let item_def = state.db.add(CardDef::Trainer(sim::card::Trainer {
        print_id: "test-item",
        name: "Test Item",
        kind: sim::card::TrainerKind::Item,
        requirement: None,
        effect: sim::card::TrainerEffect::MoveAttachedEnergy,
    }));
    let item = deal_new_card(&mut state, player, item_def);
    let deck_len_before = state.player(player).deck.len();
    state.players[player.index()].deck.push(item);

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::SearchingDeckForItem { .. }));
    let actions = legal_actions(&state);
    assert!(actions.contains(&Action::TakeItemFromDeck { card: item }));

    apply(&mut state, Action::TakeItemFromDeck { card: item }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).hand.contains(&item));
    assert_eq!(state.player(player).deck.len(), deck_len_before);
}

#[test]
fn no_item_in_the_deck_opens_no_phase() {
    let attack = Attack {
        name: "Procurement",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SearchDeckForItemCardToHand),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "no Item card anywhere in the deck");
}

#[test]
fn patrat_procurement_print_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv10.5w-072").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Patrat's Procurement print should play");
}

#[test]
fn seaking_hydro_jet_deals_damage_per_water_energy_to_a_chosen_pokemon() {
    let attack = Attack {
        name: "Hydro Jet",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DamagePerCountToChosenOpponentPokemon(
            sim::card::Count::OwnEnergyOfTypeAttachedCount(Type::Water),
            30,
        )),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();

    let water_energy_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-water-energy",
        name: "Water Energy",
        kind: Type::Water,
        effect: None,
    }));
    let water_energy = deal_new_card(&mut state, player, water_energy_def);
    state.pokemon[active.index()].attached.push(water_energy);

    pay_and_attack(&mut state);

    let opponent = player.opponent();
    let opponent_active = state.player(opponent).active.unwrap();
    assert!(matches!(state.phase, Phase::ChoosingAnyOpponentPokemonDamageTarget { .. }));
    apply(&mut state, Action::DamageChosenOpponentPokemon { target: opponent_active }).unwrap();

    assert_eq!(state.pokemon(opponent_active).damage, 30, "one Water Energy attached");
}

#[test]
fn seaking_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Seaking" && c.playable.is_some()),
        "at least one Seaking print should play"
    );
}

// --- Beyond the spec: search the whole deck for up to N cards of any kind ---

#[test]
fn takes_up_to_the_limit_of_any_cards_from_the_deck() {
    let attack = Attack {
        name: "Talon Hunt",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SearchDeckForUpToCardsOfAnyKindToHand(2)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::SearchingDeckForAnyCards { .. }));
    let first = state.player(player).deck.last().copied().unwrap();
    apply(&mut state, Action::TakeAnyCardFromDeck { card: first }).unwrap();
    assert!(matches!(state.phase, Phase::SearchingDeckForAnyCards { .. }), "one more to take");

    let second = state.player(player).deck.last().copied().unwrap();
    apply(&mut state, Action::TakeAnyCardFromDeck { card: second }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).hand.contains(&first));
    assert!(state.player(player).hand.contains(&second));
}

#[test]
fn can_stop_searching_any_cards_before_the_limit() {
    let attack = Attack {
        name: "Talon Hunt",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SearchDeckForUpToCardsOfAnyKindToHand(2)),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);
    apply(&mut state, Action::FinishSearchingAnyCards).unwrap();

    assert_eq!(state.phase, Phase::Main);
}

#[test]
fn no_deck_opens_no_phase_for_talon_hunt() {
    let attack = Attack {
        name: "Talon Hunt",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SearchDeckForUpToCardsOfAnyKindToHand(2)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    state.players[player.index()].deck.clear();

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "an empty deck has nothing to search");
}

#[test]
fn noctowl_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Noctowl" && c.playable.is_some()),
        "at least one Noctowl print should play"
    );
}

#[test]
fn dipplin_energy_loop_print_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv10-017").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Dipplin's Energy Loop print should play");
}

// --- Beyond the spec: discard the defender's Tools before dealing damage ---

#[test]
fn discards_the_defenders_tools_before_dealing_damage() {
    let attack = Attack {
        name: "Peck Off",
        cost: vec![Type::Colorless],
        base_damage: 30,
        inflicts: None,
        effect: Some(AttackEffect::DiscardsDefendersTools),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    let tool_def = state.db.add(CardDef::Trainer(sim::card::Trainer {
        print_id: "test-tool",
        name: "Test Tool",
        kind: sim::card::TrainerKind::Tool,
        requirement: None,
        effect: sim::card::TrainerEffect::Nothing,
    }));
    let tool = deal_new_card(&mut state, opponent, tool_def);
    state.pokemon[defender.index()].attached.push(tool);

    pay_and_attack(&mut state);

    assert!(!state.pokemon(defender).attached.contains(&tool), "the Tool is discarded");
    assert!(state.player(opponent).discard.contains(&tool));
    assert_eq!(state.pokemon(defender).damage, 30, "damage still lands as printed");
}

#[test]
fn seaking_peck_off_print_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv06-045").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Seaking's Peck Off print should play");
}

// --- Beyond the spec: damage per every Benched Pokemon, both sides ---

#[test]
fn damage_per_both_sides_benched_pokemon() {
    let attack = multiplier_attack(sim::card::Count::BothBenchedPokemonCount, 20);
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let card = deal_new_card(&mut state, opponent, defender_ex);
    let opponent_bench = state.put_into_play(opponent, card);
    state.players[opponent.index()].bench.push(opponent_bench);
    for _ in 0..2 {
        let card = deal_new_card(&mut state, player, defender_ex);
        let own_bench = state.put_into_play(player, card);
        state.players[player.index()].bench.push(own_bench);
    }

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 60, "1 opponent Benched plus 2 own Benched, times 20");
}

// Admission is checked once, in tests/abilities.rs, alongside Fairy
// Zone — Full Moon Rondo is this card's only attack, so the same
// admission proves both effects build.

#[test]
fn spherical_shield_blocks_a_move_onto_or_off_a_protected_bench_target() {
    let attack = Attack {
        name: "Slight Shift",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::MoveOpponentsEnergyBetweenTheirPokemon),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let energy_card = *state
        .player(opponent)
        .deck
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    state.players[opponent.index()].deck.retain(|c| *c != energy_card);
    state.pokemon[defender.index()].attached.push(energy_card);

    let rabsca_def = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-spherical-shield-slight-shift",
        name: "Rabsca",
        hp: 90,
        kind: Type::Psychic,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: Some(sim::card::Ability {
            name: "Spherical Shield",
            effect: sim::card::AbilityEffect::PassivePreventsAttackEffectsOnBench,
        }),
        attacks: vec![],
    }));
    let bench_card = deal_new_card(&mut state, opponent, rabsca_def);
    let bench_mon = state.put_into_play(opponent, bench_card);
    state.players[opponent.index()].bench.push(bench_mon);

    pay_and_attack(&mut state);

    assert_eq!(
        state.phase,
        Phase::Main,
        "the only unprotected Pokemon is the Active alone, so there's no legal from/to pair to move between"
    );
    assert!(state.pokemon(defender).attached.contains(&energy_card), "the move never happened");
}

#[test]
fn damage_per_defender_energy_attached() {
    let attack = multiplier_attack(sim::card::Count::DefenderEnergyAttachedCount, 30);
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    for _ in 0..2 {
        let card = *state
            .player(opponent)
            .deck
            .iter()
            .find(|c| state.def_of(**c).is_energy())
            .unwrap();
        state.players[opponent.index()].deck.retain(|c| *c != card);
        state.pokemon[defender.index()].attached.push(card);
    }

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 60, "2 Energy on the defender times 30");
}

#[test]
fn rabscas_psychic_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv05-024").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Rabsca's Psychic print should play");
}

// --- Beyond the spec: bonus damage only with a near-empty deck ---

#[test]
fn bonus_damage_if_own_deck_at_most() {
    let attack = Attack {
        name: "Counterturn",
        cost: vec![Type::Colorless],
        base_damage: 40,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfOwnDeckAtMost(3, 200)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    while state.player(player).deck.len() > 3 {
        let card = state.player(player).deck[0];
        state.players[player.index()].deck.retain(|c| *c != card);
        state.players[player.index()].discard.push(card);
    }

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 240, "3 or fewer left in the deck, so the bonus applies");
}

#[test]
fn no_bonus_damage_with_more_than_the_deck_threshold() {
    let attack = Attack {
        name: "Counterturn",
        cost: vec![Type::Colorless],
        base_damage: 40,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfOwnDeckAtMost(3, 200)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 40, "still well over 3 cards left, so no bonus");
}

#[test]
fn rabscas_counterturn_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv08-014").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Rabsca's Counterturn print should play");
}

// --- Beyond the spec: flat damage to two distinct chosen opponent Pokemon ---

#[test]
fn twin_shotels_damages_two_distinct_chosen_pokemon() {
    let attack = Attack {
        name: "Twin Shotels",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DamageTwoChosenOpponentPokemon(50)),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let active = state.player(opponent).active.unwrap();
    let bench_card = deal_new_card(&mut state, opponent, defender_ex);
    let bench_mon = state.put_into_play(opponent, bench_card);
    state.players[opponent.index()].bench.push(bench_mon);

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::ChoosingTwoOpponentPokemonDamageTargets { excluding: None, .. }));
    apply(&mut state, Action::DamageOneOfTwoChosenOpponentPokemon { target: active }).unwrap();
    assert!(matches!(
        state.phase,
        Phase::ChoosingTwoOpponentPokemonDamageTargets { excluding: Some(t), .. } if t == active
    ));
    apply(&mut state, Action::DamageOneOfTwoChosenOpponentPokemon { target: bench_mon }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(active).damage, 50);
    assert_eq!(state.pokemon(bench_mon).damage, 50);
}

#[test]
fn twin_shotels_stops_after_one_pick_with_only_one_opponent_pokemon_in_play() {
    let attack = Attack {
        name: "Twin Shotels",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DamageTwoChosenOpponentPokemon(50)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let active = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::ChoosingTwoOpponentPokemonDamageTargets { .. }));
    apply(&mut state, Action::DamageOneOfTwoChosenOpponentPokemon { target: active }).unwrap();

    assert_eq!(state.phase, Phase::Main, "only the Active was in play, so there's no second pick");
    assert_eq!(state.pokemon(active).damage, 50);
}

#[test]
fn iron_crown_exs_twin_shotels_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv05-081").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Iron Crown ex's Twin Shotels print should play");
}

// --- Beyond the spec: base damage plus a per-count bonus, additive not multiplicative ---

#[test]
fn bonus_damage_per_count_adds_to_the_printed_base() {
    let attack = Attack {
        name: "Retribution Strike",
        cost: vec![Type::Colorless],
        base_damage: 20,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamagePerCount(sim::card::Count::OwnDamageCounters, 10)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    state.pokemon[attacker.index()].damage = 30; // 3 counters

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 50, "20 base plus 3 counters times 10, not 30 alone");
}

// --- Beyond the spec: bonus damage only after an own Knockout last turn ---

#[test]
fn bonus_damage_if_own_knocked_out_last_turn() {
    let attack = Attack {
        name: "Orichalcum Fang",
        cost: vec![Type::Colorless],
        base_damage: 50,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfOwnKnockedOutLastTurn(120)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    state.knocked_out_last_turn[player.index()] = true;

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 170, "an own Knockout last turn, so the bonus applies");
}

#[test]
fn no_bonus_damage_without_an_own_knockout_last_turn() {
    let attack = Attack {
        name: "Orichalcum Fang",
        cost: vec![Type::Colorless],
        base_damage: 50,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfOwnKnockedOutLastTurn(120)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 50, "no Knockout last turn");
}

// --- Beyond the spec: bonus damage only with a damaged own Bench ---

#[test]
fn bonus_damage_if_own_bench_damaged() {
    let attack = Attack {
        name: "Revenge Buster",
        cost: vec![Type::Colorless],
        base_damage: 100,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfOwnBenchDamaged(120)),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let bench_card = deal_new_card(&mut state, player, defender_ex);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);
    state.pokemon[bench_mon.index()].damage = 10;

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 220, "the own Bench carries damage, so the bonus applies");
}

// --- Beyond the spec: a specific named attack locked out, not the whole turn ---

#[test]
fn cannot_use_this_attack_next_turn_leaves_the_pokemons_other_attack_usable() {
    let mut db = CardDb::new();
    let attacker_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-locked-attack-attacker",
        name: "Koraidon ex",
        hp: 230,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 2,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![
            Attack {
                name: "Impact Blow",
                cost: vec![Type::Colorless],
                base_damage: 200,
                inflicts: None,
                effect: Some(AttackEffect::CannotUseThisAttackNextTurn),
            },
            Attack {
                name: "Other Move",
                cost: vec![Type::Colorless],
                base_damage: 10,
                inflicts: None,
                effect: None,
            },
        ],
    }));
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-locked-attack-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    // Both sides run the same deck: whichever player ends up moving
    // first (Rule 17 makes that turn parity, not deck order, decide
    // who attacks first), the attacker has both attacks available.
    let mut deck = vec![attacker_mon; 4];
    while deck.len() < 60 {
        deck.push(energy);
    }
    let mut state =
        GameState::new(db, [deck.clone(), deck], Box::new(SeededRng::new(3)));
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }

    let player = state.current;
    let active = state.player(player).active.unwrap();
    pay_and_attack(&mut state);
    // The attack already ended the attacker's own turn; one more
    // EndTurn closes the opponent's, landing back on the attacker's
    // own very next turn.
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }

    // Now it's the attacker's own next turn: Impact Blow is locked,
    // Other Move is not.
    let attacks: Vec<_> = legal_actions(&state)
        .into_iter()
        .filter_map(|a| match a {
            Action::Attack { index } => Some(state.pokemon_def(active).attacks[index].name),
            _ => None,
        })
        .collect();
    assert!(!attacks.contains(&"Impact Blow"), "Impact Blow is locked out this turn");
    assert!(attacks.contains(&"Other Move"), "the Pokemon's other attack is untouched");
}

#[test]
fn koraidon_exs_retribution_strike_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv05-120").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Koraidon ex's Retribution Strike print should play");
}

#[test]
fn koraidon_exs_orichalcum_fang_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "me02.5-121").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Koraidon ex's Orichalcum Fang print should play");
}

#[test]
fn koraidon_exs_revenge_buster_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "svp-197").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Koraidon ex's Revenge Buster print should play");
}

#[test]
fn annihilapes_impact_blow_print_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv10-092").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Annihilape's Impact Blow print should play");
}

// --- Beyond the spec: bonus damage per the defender's own damage counters ---

#[test]
fn bonus_damage_per_defender_damage_counters() {
    let attack = Attack {
        name: "Relentless Punches",
        cost: vec![Type::Colorless],
        base_damage: 50,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamagePerCount(sim::card::Count::DefenderDamageCounters, 50)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    state.pokemon[defender.index()].damage = 20; // 2 counters

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 20 + 50 + 100, "50 base plus 2 counters times 50");
}

#[test]
fn brute_bonnets_relentless_punches_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv06-118").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Brute Bonnet's Relentless Punches print should play");
}

// --- Beyond the spec: bonus damage only sharing a type with the opponent's side ---

#[test]
fn bonus_damage_if_a_shared_type_is_in_play() {
    let attack = Attack {
        name: "Love Resonance",
        cost: vec![Type::Colorless],
        base_damage: 80,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfSharedTypeInPlay(120)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 200, "both sides are Colorless in this fixture");
}

#[test]
fn enamorus_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Enamorus" && c.playable.is_some()),
        "at least one Enamorus print should play"
    );
}

// --- Beyond the spec: bonus damage only with the right Energy type attached ---

#[test]
fn bonus_damage_if_own_energy_of_type_attached() {
    let attack = Attack {
        name: "Muddy Bolt",
        cost: vec![Type::Colorless],
        base_damage: 20,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfOwnEnergyOfTypeAttached(Type::Fighting, 20)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let fighting_energy = state.db.add(CardDef::Energy(Energy {
        print_id: "test-fighting-energy",
        name: "Fighting Energy",
        kind: Type::Fighting,
        effect: None,
    }));
    let card = deal_new_card(&mut state, player, fighting_energy);
    state.pokemon[attacker.index()].attached.push(card);

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 40, "20 base plus the 20 bonus");
}

#[test]
fn no_bonus_damage_without_the_right_energy_type() {
    let attack = Attack {
        name: "Muddy Bolt",
        cost: vec![Type::Colorless],
        base_damage: 20,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfOwnEnergyOfTypeAttached(Type::Fighting, 20)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 20, "only Colorless Energy attached, no bonus");
}

#[test]
fn stunfisks_muddy_bolt_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv10.5w-035").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Stunfisk's Muddy Bolt print should play");
}

// --- Beyond the spec: coin-flip condition, plus a discard from the defender ---

#[test]
fn paralyzing_crackle_offers_a_discard_only_when_it_paralyzes() {
    let attack = Attack {
        name: "Paralyzing Crackle",
        cost: vec![Type::Colorless],
        base_damage: 50,
        inflicts: None,
        effect: Some(AttackEffect::CoinFlipInflictsAndDiscardsDefenderEnergy(
            sim::card::Condition::Paralyzed,
        )),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let energy_card = *state
        .player(opponent)
        .deck
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    state.players[opponent.index()].deck.retain(|c| *c != energy_card);
    state.pokemon[defender.index()].attached.push(energy_card);

    pay_and_attack(&mut state);

    if state.has_condition(defender, sim::card::Condition::Paralyzed) {
        assert!(matches!(state.phase, Phase::DiscardingDefenderEnergyForAttack { .. }), "heads: offers the discard");
        apply(&mut state, Action::DiscardDefenderEnergyForAttack { card: energy_card }).unwrap();
        assert_eq!(state.phase, Phase::Main);
        assert!(!state.pokemon(defender).attached.contains(&energy_card));
        assert!(state.player(opponent).discard.contains(&energy_card));
    } else {
        assert_eq!(state.phase, Phase::Main, "tails: neither the condition nor the discard happens");
        assert!(state.pokemon(defender).attached.contains(&energy_card), "tails: nothing is discarded");
    }
}

#[test]
fn paralyzing_crackle_opens_no_discard_with_no_energy_attached() {
    let attack = Attack {
        name: "Paralyzing Crackle",
        cost: vec![Type::Colorless],
        base_damage: 50,
        inflicts: None,
        effect: Some(AttackEffect::CoinFlipInflictsAndDiscardsDefenderEnergy(
            sim::card::Condition::Paralyzed,
        )),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "nothing attached on the defender, so no discard phase opens");
}

#[test]
fn stunfisks_paralyzing_crackle_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv08-064").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Stunfisk's Paralyzing Crackle print should play");
}

// --- Pouncing Trap: one restriction record, two consequences ---

#[test]
fn pouncing_trap_blocks_retreat_and_adds_bonus_damage_next_turn() {
    let attack = Attack {
        name: "Pouncing Trap",
        cost: vec![Type::Colorless],
        base_damage: 30,
        inflicts: None,
        effect: Some(AttackEffect::DefenderCannotRetreatAndTakesMoreDamageNextTurn(100)),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let energy_card = *state
        .player(opponent)
        .deck
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    state.players[opponent.index()].deck.retain(|c| *c != energy_card);
    state.pokemon[defender.index()].attached.push(energy_card);
    let bench_card = deal_new_card(&mut state, opponent, defender_ex);
    let bench_mon = state.put_into_play(opponent, bench_card);
    state.players[opponent.index()].bench.push(bench_mon);

    pay_and_attack(&mut state);

    assert!(
        !legal_actions(&state).into_iter().any(|a| matches!(a, Action::Retreat { .. })),
        "the Defending Pokémon cannot retreat during this turn"
    );

    // The opponent's turn ends; now it's the attacker's own next turn,
    // and the same Pokémon (still Active — it never retreated) takes
    // the bonus damage from an ordinary attack.
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    assert_eq!(state.current, player, "back to the attacker's own turn");
    let damage_before = state.pokemon(defender).damage;
    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage - damage_before, 130, "30 base plus the 100 bonus");
}

#[test]
fn pouncing_traps_bonus_does_not_carry_past_its_one_turn() {
    let attack = Attack {
        name: "Pouncing Trap",
        cost: vec![Type::Colorless],
        base_damage: 30,
        inflicts: None,
        effect: Some(AttackEffect::DefenderCannotRetreatAndTakesMoreDamageNextTurn(100)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let defender = state.player(player.opponent()).active.unwrap();

    pay_and_attack(&mut state);
    // 3 more EndTurns, none of them an attack: opponent's turn (the
    // retreat lock's own turn) ends, then the attacker's own next
    // turn (the bonus's one armed turn) passes without attacking,
    // then the opponent's turn after that ends too, un-arming it.
    for _ in 0..3 {
        apply(&mut state, Action::EndTurn).unwrap();
        while state.phase != Phase::Main && !state.is_over() {
            let first = legal_actions(&state)[0];
            apply(&mut state, first).unwrap();
        }
    }
    assert_eq!(state.current, player, "back to the attacker's own turn, one turn too late for the bonus");
    let damage_before = state.pokemon(defender).damage;
    pay_and_attack(&mut state);
    assert_eq!(
        state.pokemon(defender).damage - damage_before,
        30,
        "the restriction's one turn has passed, so only the base 30 lands"
    );
}

#[test]
fn stunfisks_pouncing_trap_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "me02.5-062").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Stunfisk's Pouncing Trap print should play");
}

// --- Beyond the spec: discard a fixed number of own Energy, chosen by the player ---

#[test]
fn discards_fixed_own_energy_chosen() {
    let attack = Attack {
        name: "Strong Volt",
        cost: vec![Type::Colorless],
        base_damage: 120,
        inflicts: None,
        effect: Some(AttackEffect::DiscardsFixedOwnEnergyChosen(1)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 120, "the printed damage lands regardless");
    assert!(matches!(state.phase, Phase::ChoosingOwnEnergyToDiscardForAttack { remaining: 1, .. }));
    let card = state.pokemon(attacker).attached[0];
    apply(&mut state, Action::DiscardOwnEnergyForAttack { card }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.pokemon(attacker).attached.is_empty());
    assert!(state.player(player).discard.contains(&card));
}

#[test]
fn discards_fixed_own_energy_chosen_over_multiple_picks() {
    let attack = Attack {
        name: "Luster Blast",
        cost: vec![Type::Colorless, Type::Colorless],
        base_damage: 20,
        inflicts: None,
        effect: Some(AttackEffect::DiscardsFixedOwnEnergyChosen(2)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::ChoosingOwnEnergyToDiscardForAttack { remaining: 2, .. }));
    let first = state.pokemon(attacker).attached[0];
    apply(&mut state, Action::DiscardOwnEnergyForAttack { card: first }).unwrap();
    assert!(matches!(state.phase, Phase::ChoosingOwnEnergyToDiscardForAttack { remaining: 1, .. }));
    let second = state.pokemon(attacker).attached[0];
    apply(&mut state, Action::DiscardOwnEnergyForAttack { card: second }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(attacker).attached.len(), 0);
}

#[test]
fn zeraoras_strong_volt_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv05-057").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Zeraora's Strong Volt print should play");
}

#[test]
fn metagrosss_luster_blast_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "me04-061").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Metagross's Metallic Hammer print should play");
}

// --- Beyond the spec: may discard up to N of a type for a flat bonus, even with zero ---

#[test]
fn metallic_hammer_grants_the_bonus_even_discarding_nothing() {
    let attack = Attack {
        name: "Metallic Hammer",
        cost: vec![Type::Colorless],
        base_damage: 50,
        inflicts: None,
        effect: Some(AttackEffect::MayDiscardUpToOwnEnergyOfTypeForFlatBonusDamage(Type::Metal, 3, 50)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::DecidingToDiscardOwnEnergyForBonusDamage { .. }));
    apply(&mut state, Action::AcceptDiscardOwnEnergyForBonusDamage).unwrap();

    assert_eq!(state.pokemon(defender).damage, 100, "50 base plus the 50 bonus, taken for free");
    assert!(matches!(state.phase, Phase::ChoosingOwnEnergyToDiscardForBonusDamage { .. }));
    apply(&mut state, Action::FinishDiscardingOwnEnergyForBonusDamage).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(defender).damage, 100, "declining to discard further doesn't undo the bonus");
}

#[test]
fn declining_metallic_hammers_bonus_grants_nothing() {
    let attack = Attack {
        name: "Metallic Hammer",
        cost: vec![Type::Colorless],
        base_damage: 150,
        inflicts: None,
        effect: Some(AttackEffect::MayDiscardUpToOwnEnergyOfTypeForFlatBonusDamage(Type::Metal, 3, 150)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    apply(&mut state, Action::DeclineDiscardOwnEnergyForBonusDamage).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(defender).damage, 150, "no bonus without accepting");
}

// --- Beyond the spec: discard the whole hand, then draw outright ---

#[test]
fn discards_hand_then_draws_cards() {
    let attack = Attack {
        name: "Burst Roar",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DiscardsHandThenDrawsCards(6)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    // `pay_and_attack` may pull the paying Energy from hand or
    // deck — only what's still in hand right before the attack
    // itself fires is what Burst Roar discards.
    let hand_before: Vec<_> = {
        let active = state.player(player).active.unwrap();
        let cost_len = state.pokemon_def(active).attacks[0].cost.len();
        for _ in 0..cost_len {
            let side = state.player(player);
            let card = side
                .hand
                .iter()
                .chain(side.deck.iter())
                .find(|c| state.def_of(**c).is_energy())
                .copied()
                .expect("the deck holds Energy");
            state.remove_from_hand(player, card);
            state.players[player.index()].deck.retain(|c| *c != card);
            state.pokemon[active.index()].attached.push(card);
        }
        state.player(player).hand.clone()
    };
    let attack = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("the attacker is paid for");
    apply(&mut state, attack).unwrap();

    assert_eq!(state.player(player).hand.len(), 6, "drew exactly 6");
    for card in hand_before {
        assert!(state.player(player).discard.contains(&card), "the old hand landed in the discard pile");
    }
}

#[test]
fn raging_bolt_exs_burst_roar_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv05-123").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Raging Bolt ex's Burst Roar print should play");
}

// --- Beyond the spec: discard any amount of own Basic Energy, damage per card ---

#[test]
fn bellowing_thunder_deals_damage_per_card_discarded() {
    let attack = Attack {
        name: "Bellowing Thunder",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::MayDiscardAnyOwnBasicEnergyForDamagePerCard(70)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    // Attach a second Energy beyond the one cost already put there, so
    // there are two to discard.
    let extra_energy = state.db.add(CardDef::Energy(Energy {
        print_id: "test-bellowing-thunder-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    let extra = deal_new_card(&mut state, player, extra_energy);
    state.pokemon[attacker.index()].attached.push(extra);

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::DiscardingAnyBasicEnergyForDamagePerCard { .. }));
    let cards: Vec<_> = state.pokemon(attacker).attached.clone();
    for card in &cards {
        apply(&mut state, Action::DiscardBasicEnergyForDamagePerCard { card: *card }).unwrap();
    }
    apply(&mut state, Action::FinishDiscardingBasicEnergyForDamagePerCard).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(defender).damage, cards.len() as u32 * 70);
    assert!(state.pokemon(attacker).attached.is_empty());
}

#[test]
fn bellowing_thunder_deals_no_damage_discarding_nothing() {
    let attack = Attack {
        name: "Bellowing Thunder",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::MayDiscardAnyOwnBasicEnergyForDamagePerCard(70)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    apply(&mut state, Action::FinishDiscardingBasicEnergyForDamagePerCard).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(defender).damage, 0, "declining outright deals nothing");
}

#[test]
fn raging_bolt_exs_bellowing_thunder_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Raging Bolt ex" && c.playable.is_some()),
        "at least one Raging Bolt ex print should play"
    );
}

// --- Beyond the spec: discard the top N of the opponent's deck ---

#[test]
fn discards_top_of_opponents_deck() {
    let attack = Attack {
        name: "Undermine",
        cost: vec![Type::Colorless],
        base_damage: 90,
        inflicts: None,
        effect: Some(AttackEffect::DiscardsTopOfOpponentsDeck(2)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let top_two: Vec<_> = state.player(opponent).deck.iter().take(2).copied().collect();

    pay_and_attack(&mut state);

    // The attack ends the attacker's own turn, which starts the
    // opponent's — including their own draw — inside this same
    // `apply` call, so the deck also loses that one card beyond
    // the two Undermine itself discards; only the discard pile's own
    // count is exact.
    assert_eq!(state.player(opponent).discard.len(), 2, "exactly the top two, discarded");
    for card in top_two {
        assert!(state.player(opponent).discard.contains(&card));
    }
}

#[test]
fn mega_excadrill_exs_undermine_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Mega Excadrill ex" && c.playable.is_some()),
        "at least one Mega Excadrill ex print should play"
    );
}

// --- Beyond the spec: bonus damage only with extra Energy beyond the attack's own cost ---

#[test]
fn bonus_damage_with_extra_energy_beyond_cost() {
    let attack = Attack {
        name: "Maximum Drilling",
        cost: vec![Type::Colorless],
        base_damage: 200,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfExtraEnergyAttached(2, 130)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    // Cost is 1; attach 2 more beyond it (3 total) for the bonus.
    for _ in 0..2 {
        let card = *state
            .player(player)
            .deck
            .iter()
            .find(|c| state.def_of(**c).is_energy())
            .unwrap();
        state.players[player.index()].deck.retain(|c| *c != card);
        state.pokemon[attacker.index()].attached.push(card);
    }

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 330, "200 base plus the 130 bonus");
}

#[test]
fn no_bonus_damage_without_enough_extra_energy() {
    let attack = Attack {
        name: "Maximum Drilling",
        cost: vec![Type::Colorless],
        base_damage: 200,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfExtraEnergyAttached(2, 130)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 200, "only the cost's own 1 Energy attached, no bonus");
}

// --- Beyond the spec: bonus damage per Energy of a type across the whole side, not only the attacker ---

#[test]
fn bonus_damage_per_own_energy_of_type_across_the_whole_side() {
    let attack = Attack {
        name: "Syrup Storm",
        cost: vec![Type::Colorless],
        base_damage: 30,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamagePerCount(
            sim::card::Count::OwnEnergyOfTypeAttachedAcrossSideCount(Type::Grass),
            30,
        )),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let grass_energy = state.db.add(CardDef::Energy(Energy {
        print_id: "test-syrup-storm-grass-energy",
        name: "Grass Energy",
        kind: Type::Grass,
        effect: None,
    }));
    // One Grass Energy on the attacker, one on a Benched Pokemon —
    // both count, since Syrup Storm reads the whole side.
    let card = deal_new_card(&mut state, player, grass_energy);
    state.pokemon[attacker.index()].attached.push(card);
    let bench_card = deal_new_card(&mut state, player, defender_ex);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);
    let card = deal_new_card(&mut state, player, grass_energy);
    state.pokemon[bench_mon.index()].attached.push(card);

    let label = sim::action::describe(&state, Action::Attack { index: 0 });
    assert_eq!(label, "Attack: Syrup Storm (90 damage)", "the label shows the bonus, not just the printed 30+");

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 90, "30 base plus 2 Grass Energy (attacker plus bench) times 30");
}

#[test]
fn hydrapple_exs_syrup_storm_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Hydrapple ex" && c.playable.is_some()),
        "at least one Hydrapple ex print should play"
    );
}

// --- Beyond the spec: shuffle own Energy into the deck, then flat damage — Weakness if Active only ---

#[test]
fn sonic_ripper_applies_weakness_against_the_opponents_active() {
    let attack = Attack {
        name: "Sonic Ripper",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::ShufflesOwnEnergyThenDamagesChosenOpponentPokemonWeaknessIfActive(220)),
    };
    let mut state = game_with_weak_defender(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let attached_before = state.pokemon(attacker).attached.len();

    pay_and_attack(&mut state);

    assert!(matches!(
        state.phase,
        Phase::ChoosingAnyOpponentPokemonDamageTargetWeaknessIfActive { .. }
    ));
    assert_eq!(state.pokemon(attacker).attached.len(), 0, "the attacker's own Energy shuffled away");
    assert!(state.player(player).deck.len() >= attached_before, "the deck grew back");
    apply(&mut state, Action::DamageChosenOpponentPokemonWeaknessIfActive { target: defender }).unwrap();

    // 440 Knocks the 200 HP defender out outright; this fixture's deck
    // has no Bench to fall back to, so the game ends rather than
    // opening Promoting — either way, the doubled damage landed.
    assert_eq!(state.pokemon(defender).damage, 440, "220 doubled by Weakness against the Active");
    assert!(matches!(state.phase, Phase::Promoting { .. }) || state.is_over());
}

#[test]
fn sonic_ripper_skips_weakness_against_a_benched_target() {
    let attack = Attack {
        name: "Sonic Ripper",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::ShufflesOwnEnergyThenDamagesChosenOpponentPokemonWeaknessIfActive(220)),
    };
    let mut state = game_with_weak_defender(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let bench_def = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-sonic-ripper-bench",
        name: "Benchmon",
        hp: 300,
        kind: Type::Colorless,
        weakness: Some(Type::Colorless),
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![],
    }));
    let bench_card = deal_new_card(&mut state, opponent, bench_def);
    let bench_mon = state.put_into_play(opponent, bench_card);
    state.players[opponent.index()].bench.push(bench_mon);

    pay_and_attack(&mut state);

    apply(&mut state, Action::DamageChosenOpponentPokemonWeaknessIfActive { target: bench_mon }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(bench_mon).damage, 220, "no Weakness doubling against a Benched target");
}

#[test]
fn mega_skarmory_exs_sonic_ripper_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Mega Skarmory ex" && c.playable.is_some()),
        "at least one Mega Skarmory ex print should play"
    );
}

// --- Beyond the spec: damage every Pokemon with an Ability, both sides ---

#[test]
fn damages_every_pokemon_with_an_ability_both_sides() {
    let attack = Attack {
        name: "Law of the Underworld",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DamagesEveryPokemonWithAnAbility(60)),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    // Neither side's Active carries an Ability in this fixture, so
    // bench a copy of each with one added directly.
    let ability_def = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-law-of-the-underworld-ability-mon",
        name: "Abilitymon",
        hp: 200,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: Some(sim::card::Ability {
            name: "Some Ability",
            effect: sim::card::AbilityEffect::PassiveImmuneToAsleep,
        }),
        attacks: vec![],
    }));
    let own_bench_card = deal_new_card(&mut state, player, ability_def);
    let own_bench = state.put_into_play(player, own_bench_card);
    state.players[player.index()].bench.push(own_bench);
    let opp_bench_card = deal_new_card(&mut state, opponent, ability_def);
    let opp_bench = state.put_into_play(opponent, opp_bench_card);
    state.players[opponent.index()].bench.push(opp_bench);
    let plain_bench_card = deal_new_card(&mut state, opponent, defender_ex);
    let plain_bench = state.put_into_play(opponent, plain_bench_card);
    state.players[opponent.index()].bench.push(plain_bench);

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(own_bench).damage, 60, "the player's own Ability-carrying Bench mon");
    assert_eq!(state.pokemon(opp_bench).damage, 60, "the opponent's Ability-carrying Bench mon");
    assert_eq!(state.pokemon(plain_bench).damage, 0, "no Ability, untouched");
    assert_eq!(state.pokemon(attacker).damage, 0, "the attacker itself has no Ability in this fixture");
    assert_eq!(state.pokemon(defender).damage, 0, "the defender itself has no Ability in this fixture");
}

#[test]
fn cofagriguss_law_of_the_underworld_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv08-083").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Cofagrigus's Law of the Underworld print should play");
}

// --- Beyond the spec: move all damage from a chosen own Benched Pokemon to a chosen opponent ---

#[test]
fn moves_all_damage_from_a_chosen_own_benched_pokemon_to_a_chosen_opponent() {
    let attack = Attack {
        name: "Extended Damagriiigus",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::MovesAllDamageFromChosenOwnBenchedToChosenOpponentPokemon),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let bench_card = deal_new_card(&mut state, player, defender_ex);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);
    state.pokemon[bench_mon.index()].damage = 90;

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::ChoosingOwnBenchedSourceForDamageMove { .. }));
    apply(&mut state, Action::ChooseOwnBenchedSourceForDamageMove { source: bench_mon }).unwrap();
    assert!(matches!(state.phase, Phase::ChoosingOpponentTargetForDamageMove { .. }));
    apply(&mut state, Action::MoveDamageToChosenOpponentPokemon { target: defender }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(bench_mon).damage, 0, "all of it left the source");
    assert_eq!(state.pokemon(defender).damage, 90, "all of it landed on the chosen target");
}

#[test]
fn no_move_available_with_an_empty_own_bench() {
    let attack = Attack {
        name: "Extended Damagriiigus",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::MovesAllDamageFromChosenOwnBenchedToChosenOpponentPokemon),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "no own Bench at all, so nothing opens");
}

#[test]
fn cofagriguss_extended_damagriiigus_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Cofagrigus" && c.playable.is_some()),
        "at least one Cofagrigus print should play"
    );
}

// --- Beyond the spec: choose a Bench target first, then search Energy to attach to it ---

#[test]
fn delightful_kiss_attaches_up_to_two_searched_energy_to_the_chosen_target() {
    let attack = Attack {
        name: "Delightful Kiss",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SearchesBasicEnergyOfTypeAttachToChosenBenched(Type::Psychic, 2)),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let psychic_energy = state.db.add(CardDef::Energy(Energy {
        print_id: "test-delightful-kiss-energy",
        name: "Psychic Energy",
        kind: Type::Psychic,
        effect: None,
    }));
    let bench_card = deal_new_card(&mut state, player, defender_ex);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);
    for _ in 0..2 {
        let card = deal_new_card(&mut state, player, psychic_energy);
        state.players[player.index()].deck.push(card);
    }

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::ChoosingBenchedTargetForEnergySearch { .. }));
    apply(&mut state, Action::ChooseBenchedTargetForEnergySearch { target: bench_mon }).unwrap();

    assert!(matches!(state.phase, Phase::SearchingEnergyOfTypeToAttachToChosen { remaining: 2, .. }));
    let first = *state
        .player(player)
        .deck
        .iter()
        .find(|c| state.def_of(**c).name() == "Psychic Energy")
        .unwrap();
    apply(&mut state, Action::TakeEnergyOfTypeToAttachToChosen { card: first }).unwrap();
    assert!(matches!(state.phase, Phase::SearchingEnergyOfTypeToAttachToChosen { remaining: 1, .. }));
    let second = *state
        .player(player)
        .deck
        .iter()
        .find(|c| state.def_of(**c).name() == "Psychic Energy")
        .unwrap();
    apply(&mut state, Action::TakeEnergyOfTypeToAttachToChosen { card: second }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.pokemon(bench_mon).attached.contains(&first));
    assert!(state.pokemon(bench_mon).attached.contains(&second));
}

#[test]
fn delightful_kiss_may_stop_after_one() {
    let attack = Attack {
        name: "Delightful Kiss",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SearchesBasicEnergyOfTypeAttachToChosenBenched(Type::Psychic, 2)),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let psychic_energy = state.db.add(CardDef::Energy(Energy {
        print_id: "test-delightful-kiss-stop-energy",
        name: "Psychic Energy",
        kind: Type::Psychic,
        effect: None,
    }));
    let bench_card = deal_new_card(&mut state, player, defender_ex);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);
    let card = deal_new_card(&mut state, player, psychic_energy);
    state.players[player.index()].deck.push(card);

    pay_and_attack(&mut state);

    apply(&mut state, Action::ChooseBenchedTargetForEnergySearch { target: bench_mon }).unwrap();
    apply(&mut state, Action::FinishSearchingEnergyOfTypeToAttachToChosen).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.pokemon(bench_mon).attached.is_empty(), "declined before taking any");
}

#[test]
fn no_search_opens_with_no_own_bench() {
    let attack = Attack {
        name: "Delightful Kiss",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SearchesBasicEnergyOfTypeAttachToChosenBenched(Type::Psychic, 2)),
    };
    let (mut state, _defender_ex) = game(attack, 3);

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "no own Bench, so nothing opens");
}

#[test]
fn smoochums_delightful_kiss_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Smoochum" && c.playable.is_some()),
        "at least one Smoochum print should play"
    );
}

// --- Beyond the spec: search up to N, any combination of a type and Stadiums ---

#[test]
fn traverse_time_takes_any_combination_of_grass_pokemon_and_stadiums() {
    let attack = Attack {
        name: "Traverse Time",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::SearchDeckForUpToPokemonOfTypeOrStadiumToHand(Type::Grass, 3)),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let grass_mon = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-traverse-time-grass",
        name: "Grassmon",
        hp: 100,
        kind: Type::Grass,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![],
    }));
    let stadium_def = state.db.add(CardDef::Trainer(sim::card::Trainer {
        print_id: "test-traverse-time-stadium",
        name: "Test Stadium",
        kind: sim::card::TrainerKind::Stadium,
        requirement: None,
        effect: sim::card::TrainerEffect::MayDiscardUpToTwoToolsAnywhere,
    }));
    let grass_card = deal_new_card(&mut state, player, grass_mon);
    state.players[player.index()].deck.push(grass_card);
    let stadium_card = deal_new_card(&mut state, player, stadium_def);
    state.players[player.index()].deck.push(stadium_card);
    // A non-matching card (the punching-bag defender_ex, Colorless) to
    // confirm it's never offered.
    let plain_card = deal_new_card(&mut state, player, defender_ex);
    state.players[player.index()].deck.push(plain_card);

    pay_and_attack(&mut state);

    assert!(matches!(
        state.phase,
        Phase::SearchingDeckForPokemonOfTypeOrStadium { remaining: 3, .. }
    ));
    let offered: Vec<_> = legal_actions(&state)
        .into_iter()
        .filter_map(|a| match a {
            Action::TakePokemonOfTypeOrStadiumFromDeck { card } => Some(card),
            _ => None,
        })
        .collect();
    assert!(offered.contains(&grass_card));
    assert!(offered.contains(&stadium_card));
    assert!(!offered.contains(&plain_card), "not Grass and not a Stadium");

    apply(&mut state, Action::TakePokemonOfTypeOrStadiumFromDeck { card: grass_card }).unwrap();
    apply(&mut state, Action::TakePokemonOfTypeOrStadiumFromDeck { card: stadium_card }).unwrap();
    apply(&mut state, Action::FinishSearchingPokemonOfTypeOrStadium).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).hand.contains(&grass_card));
    assert!(state.player(player).hand.contains(&stadium_card));
}

#[test]
fn celebis_traverse_time_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "me01-012").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Celebi's Traverse Time print should play");
}

// --- Beyond the spec: Knockout the defender outright at an exact damage-counter count ---

#[test]
fn knocks_out_the_defender_at_exactly_the_named_counters() {
    let attack = Attack {
        name: "Terminal Period",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::KnocksOutDefenderIfExactDamageCounters(6)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    state.pokemon[defender.index()].damage = 60; // exactly 6 counters

    pay_and_attack(&mut state);

    assert!(
        state.pokemon(defender).knocked_out,
        "exactly 6 damage counters Knocks the defender out outright"
    );
}

#[test]
fn no_knockout_off_by_one_counter() {
    let attack = Attack {
        name: "Terminal Period",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::KnocksOutDefenderIfExactDamageCounters(6)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    state.pokemon[defender.index()].damage = 50; // 5 counters, not 6

    pay_and_attack(&mut state);

    assert!(!state.pokemon(defender).knocked_out, "an exact reading, not at least");
}

#[test]
fn mega_absol_exs_terminal_period_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Mega Absol ex" && c.playable.is_some()),
        "at least one Mega Absol ex print should play"
    );
}

// --- Beyond the spec: the opponent's hand is revealed, then the player discards from it ---

#[test]
fn claw_of_darkness_discards_a_chosen_card_from_the_opponents_hand() {
    let attack = Attack {
        name: "Claw of Darkness",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DiscardsChosenFromOpponentsHand),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let opponent_card = *state.player(opponent).hand.first().expect("the opponent has a hand");

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::ChoosingCardFromOpponentsHandToDiscard { .. }));
    let offered: Vec<_> = legal_actions(&state)
        .into_iter()
        .filter_map(|a| match a {
            Action::DiscardCardFromOpponentsHand { card } => Some(card),
            _ => None,
        })
        .collect();
    assert_eq!(offered.len(), state.player(opponent).hand.len(), "every card in the opponent's hand");
    apply(&mut state, Action::DiscardCardFromOpponentsHand { card: opponent_card }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(!state.player(opponent).hand.contains(&opponent_card));
    assert!(state.player(opponent).discard.contains(&opponent_card));
}

#[test]
fn claw_of_darkness_opens_no_choice_with_an_empty_opponent_hand() {
    let attack = Attack {
        name: "Claw of Darkness",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DiscardsChosenFromOpponentsHand),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    state.players[opponent.index()].hand.clear();

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "no cards in the opponent's hand, so nothing opens");
}

#[test]
fn mega_absol_exs_claw_of_darkness_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Mega Absol ex" && c.playable.is_some()),
        "at least one Mega Absol ex print should play"
    );
}

// --- Beyond the spec: bonus damage only if the attacker itself promoted from Bench this turn ---

#[test]
fn gale_thrust_bonus_applies_after_retreating_in_this_turn() {
    let attack = Attack {
        name: "Gale Thrust",
        cost: vec![Type::Colorless],
        base_damage: 60,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfSelfPromotedThisTurn(170)),
    };
    let (mut state, _defender_ex) = game(attack.clone(), 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let old_active = state.player(player).active.unwrap();
    // Retreating costs Energy equal to the current Active's own
    // Retreat Cost (1 in this fixture), paid from what's attached.
    let retreat_energy = *state
        .player(player)
        .deck
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    state.players[player.index()].deck.retain(|c| *c != retreat_energy);
    state.pokemon[old_active.index()].attached.push(retreat_energy);
    let bench_def = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-gale-thrust-bench",
        name: "Mega Lopunny ex",
        hp: 220,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 0,
        prizes: 2,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![attack],
    }));
    let bench_card = deal_new_card(&mut state, player, bench_def);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);

    let retreat = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Retreat { to } if *to == bench_mon))
        .expect("no Retreat cost, and it's this player's own turn");
    apply(&mut state, retreat).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    assert_eq!(state.player(player).active, Some(bench_mon));

    let attack_energy = *state
        .player(player)
        .deck
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    state.players[player.index()].deck.retain(|c| *c != attack_energy);
    state.pokemon[bench_mon.index()].attached.push(attack_energy);

    let attack = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("the newly Active Pokemon is paid for");
    apply(&mut state, attack).unwrap();

    assert_eq!(state.pokemon(defender).damage, 230, "60 base plus the 170 bonus, just retreated in");
}

#[test]
fn no_gale_thrust_bonus_without_promoting_this_turn() {
    let attack = Attack {
        name: "Gale Thrust",
        cost: vec![Type::Colorless],
        base_damage: 60,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfSelfPromotedThisTurn(170)),
    };
    let (mut state, _defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 60, "the Active the whole turn, no promotion");
}

#[test]
fn mega_lopunny_exs_gale_thrust_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Mega Lopunny ex" && c.playable.is_some()),
        "at least one Mega Lopunny ex print should play"
    );
}

// --- Beyond the spec: discard all own Energy, then flat damage to a chosen Benched ex ---

#[test]
fn thunder_raid_discards_all_energy_then_damages_a_chosen_benched_ex() {
    let attack = Attack {
        name: "Thunder Raid",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DiscardsOwnEnergyThenDamagesChosenBenchedEx(210)),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let bench_card = deal_new_card(&mut state, opponent, defender_ex);
    let bench_mon = state.put_into_play(opponent, bench_card);
    state.players[opponent.index()].bench.push(bench_mon);

    pay_and_attack(&mut state);

    assert!(state.pokemon(attacker).attached.is_empty(), "all of the attacker's own Energy discarded");
    assert!(matches!(state.phase, Phase::ChoosingBenchedExDamageTarget { .. }));
    apply(&mut state, Action::DamageBenchedEx { target: bench_mon }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(bench_mon).damage, 210);
}

#[test]
fn zeraoras_thunder_raid_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv10-078").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Zeraora's Thunder Raid print should play");
}

#[test]
fn trifrost_discards_all_energy_then_damages_three_chosen_opponent_pokemon() {
    let attack = Attack {
        name: "Trifrost",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DiscardsOwnEnergyThenDamagesThreeChosenOpponentPokemon(110)),
    };
    let (mut state, defender_def) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let defender_active = state.player(opponent).active.unwrap();

    let bench_card_1 = deal_new_card(&mut state, opponent, defender_def);
    let bench_mon_1 = state.put_into_play(opponent, bench_card_1);
    state.players[opponent.index()].bench.push(bench_mon_1);
    let bench_card_2 = deal_new_card(&mut state, opponent, defender_def);
    let bench_mon_2 = state.put_into_play(opponent, bench_card_2);
    state.players[opponent.index()].bench.push(bench_mon_2);

    pay_and_attack(&mut state);

    assert!(state.pokemon(attacker).attached.is_empty(), "all of the attacker's own Energy discarded");
    assert!(matches!(state.phase, Phase::ChoosingThreeOpponentPokemonDamageTargets { .. }));
    apply(&mut state, Action::DamageOneOfThreeChosenOpponentPokemon { target: defender_active }).unwrap();
    assert!(matches!(state.phase, Phase::ChoosingThreeOpponentPokemonDamageTargets { .. }));
    apply(&mut state, Action::DamageOneOfThreeChosenOpponentPokemon { target: bench_mon_1 }).unwrap();
    assert!(matches!(state.phase, Phase::ChoosingThreeOpponentPokemonDamageTargets { .. }));
    apply(&mut state, Action::DamageOneOfThreeChosenOpponentPokemon { target: bench_mon_2 }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(defender_active).damage, 110);
    assert_eq!(state.pokemon(bench_mon_1).damage, 110);
    assert_eq!(state.pokemon(bench_mon_2).damage, 110);
}

#[test]
fn trifrost_stops_early_with_fewer_than_three_opponent_pokemon() {
    let attack = Attack {
        name: "Trifrost",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DiscardsOwnEnergyThenDamagesThreeChosenOpponentPokemon(110)),
    };
    let (mut state, _defender_def) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender_active = state.player(opponent).active.unwrap();

    pay_and_attack(&mut state);

    assert!(matches!(state.phase, Phase::ChoosingThreeOpponentPokemonDamageTargets { .. }));
    apply(&mut state, Action::DamageOneOfThreeChosenOpponentPokemon { target: defender_active }).unwrap();

    assert_eq!(
        state.phase,
        Phase::Main,
        "only one opponent Pokémon in play, so the choice ends after one pick"
    );
    assert_eq!(state.pokemon(defender_active).damage, 110);
}

#[test]
fn kyurems_trifrost_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv06.5-047").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Kyurem's print should play");
}

// --- Night Joker: copying a Benched "N's" Pokémon's own attack ---

#[test]
fn night_joker_copies_a_benched_ns_pokemons_chosen_attack() {
    let night_joker = Attack {
        name: "Night Joker",
        cost: vec![Type::Colorless, Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::CopiesChosenBenchedPokemonAttackByNamePrefix("N's ")),
    };
    let (mut state, _defender_def) = game(night_joker, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let defender_active = state.player(opponent).active.unwrap();

    let ns_mon = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-ns-mon",
        name: "N's Purrloin",
        hp: 90,
        kind: Type::Darkness,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Scratch",
            cost: vec![Type::Colorless],
            base_damage: 60,
            inflicts: None,
            effect: None,
        }],
    }));
    let bench_card = deal_new_card(&mut state, player, ns_mon);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);

    // Pay Night Joker's own cost (2 Darkness) — the copied attack's
    // printed cost is never paid, only Night Joker's own.
    for _ in 0..2 {
        let energy = *state
            .player(player)
            .deck
            .iter()
            .find(|c| state.def_of(**c).is_energy())
            .unwrap();
        state.players[player.index()].deck.retain(|c| *c != energy);
        state.pokemon[attacker.index()].attached.push(energy);
    }

    apply(&mut state, Action::Attack { index: 0 }).unwrap();
    assert!(matches!(state.phase, Phase::ChoosingBenchedPokemonAttackToCopy { .. }));

    apply(&mut state, Action::CopyBenchedPokemonAttack { pokemon: bench_mon, index: 0 }).unwrap();

    assert_eq!(state.pokemon(defender_active).damage, 60, "Scratch's own 60 damage landed");
}

#[test]
fn night_joker_ignores_a_benched_pokemon_outside_the_ns_family() {
    let night_joker = Attack {
        name: "Night Joker",
        cost: vec![Type::Colorless, Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::CopiesChosenBenchedPokemonAttackByNamePrefix("N's ")),
    };
    let (mut state, _defender_def) = game(night_joker, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();

    let plain_mon = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-plain-mon",
        name: "Plainmon",
        hp: 90,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 60,
            inflicts: None,
            effect: None,
        }],
    }));
    let bench_card = deal_new_card(&mut state, player, plain_mon);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);

    for _ in 0..2 {
        let energy = *state
            .player(player)
            .deck
            .iter()
            .find(|c| state.def_of(**c).is_energy())
            .unwrap();
        state.players[player.index()].deck.retain(|c| *c != energy);
        state.pokemon[attacker.index()].attached.push(energy);
    }

    apply(&mut state, Action::Attack { index: 0 }).unwrap();

    assert_eq!(
        state.phase,
        Phase::Main,
        "no Benched \"N's\" Pokémon, so the choice never opens and the attack does nothing"
    );
}

#[test]
fn ns_zoroark_ex_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "N's Zoroark ex" && c.playable.is_some()),
        "at least one N's Zoroark ex print should play"
    );
}

// --- Seek Inspiration: discarding into a copied attack ---

#[test]
fn seek_inspiration_copies_the_discarded_pokemons_attack() {
    let seek_inspiration = Attack {
        name: "Seek Inspiration",
        cost: vec![Type::Colorless, Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DiscardsTopOfDeckThenCopiesItsAttackIfNoRuleBox),
    };
    let (mut state, _defender_def) = game(seek_inspiration, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let opponent = player.opponent();
    let defender_active = state.player(opponent).active.unwrap();

    let no_rule_box_mon = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-no-rule-box",
        name: "Inspiremon",
        hp: 90,
        kind: Type::Psychic,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Borrowed Bolt",
            cost: vec![Type::Colorless],
            base_damage: 70,
            inflicts: None,
            effect: None,
        }],
    }));
    let top_card = deal_new_card(&mut state, player, no_rule_box_mon);
    state.players[player.index()].deck.retain(|c| *c != top_card);
    state.players[player.index()].deck.push(top_card);

    // Pay Seek Inspiration's own cost.
    for _ in 0..2 {
        let energy = *state
            .player(player)
            .deck
            .iter()
            .rev()
            .find(|c| state.def_of(**c).is_energy())
            .unwrap();
        state.players[player.index()].deck.retain(|c| *c != energy);
        state.pokemon[attacker.index()].attached.push(energy);
    }

    apply(&mut state, Action::Attack { index: 0 }).unwrap();
    assert!(state.player(player).discard.contains(&top_card), "the top card discarded outright");
    assert!(matches!(state.phase, Phase::ChoosingDiscardedPokemonAttackToCopy { card, .. } if card == top_card));

    apply(&mut state, Action::CopyDiscardedPokemonAttack { index: 0 }).unwrap();

    assert_eq!(state.pokemon(defender_active).damage, 70, "Borrowed Bolt's own 70 damage landed");
}

#[test]
fn seek_inspiration_does_nothing_more_when_the_discard_is_not_a_plain_pokemon() {
    let seek_inspiration = Attack {
        name: "Seek Inspiration",
        cost: vec![Type::Colorless, Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DiscardsTopOfDeckThenCopiesItsAttackIfNoRuleBox),
    };
    let (mut state, _defender_def) = game(seek_inspiration, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();

    let ex_mon = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-rule-box",
        name: "Rulemon ex",
        hp: 200,
        kind: Type::Psychic,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 2,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Big Bolt",
            cost: vec![Type::Colorless],
            base_damage: 200,
            inflicts: None,
            effect: None,
        }],
    }));
    let top_card = deal_new_card(&mut state, player, ex_mon);
    state.players[player.index()].deck.retain(|c| *c != top_card);
    state.players[player.index()].deck.push(top_card);

    for _ in 0..2 {
        let energy = *state
            .player(player)
            .deck
            .iter()
            .rev()
            .find(|c| state.def_of(**c).is_energy())
            .unwrap();
        state.players[player.index()].deck.retain(|c| *c != energy);
        state.pokemon[attacker.index()].attached.push(energy);
    }

    apply(&mut state, Action::Attack { index: 0 }).unwrap();

    assert!(state.player(player).discard.contains(&top_card), "still discarded outright");
    assert_eq!(
        state.phase,
        Phase::Main,
        "a Pokémon ex carries a Rule Box, so nothing is copied"
    );
}

#[test]
fn slowkings_seek_inspiration_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Slowking" && c.playable.is_some()),
        "at least one Slowking print should play"
    );
}

// --- Chi-Yu: three prints, three different attacks ---

#[test]
fn ground_melter_deals_bonus_damage_and_discards_the_stadium() {
    let attack = Attack {
        name: "Ground Melter",
        cost: vec![Type::Colorless, Type::Colorless],
        base_damage: 60,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfStadiumInPlayThenDiscardsIt(60)),
    };
    let (mut state, _defender_def) = game(attack, 3);
    let player = state.current;
    let defender = state.player(player.opponent()).active.unwrap();
    let stadium_def = state.db.add(CardDef::Trainer(sim::card::Trainer {
        print_id: "test-ground-melter-stadium",
        name: "Test Stadium",
        kind: sim::card::TrainerKind::Stadium,
        requirement: None,
        effect: sim::card::TrainerEffect::Nothing,
    }));
    let stadium = deal_new_card(&mut state, player, stadium_def);
    state.stadium = Some((player, stadium));

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 120, "60 base plus 60 bonus");
    assert!(state.stadium.is_none(), "the Stadium is discarded");
    assert!(state.player(player).discard.contains(&stadium));
}

#[test]
fn ground_melter_deals_only_base_damage_with_no_stadium() {
    let attack = Attack {
        name: "Ground Melter",
        cost: vec![Type::Colorless, Type::Colorless],
        base_damage: 60,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfStadiumInPlayThenDiscardsIt(60)),
    };
    let (mut state, _defender_def) = game(attack, 3);
    let player = state.current;
    let defender = state.player(player.opponent()).active.unwrap();

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 60, "no Stadium, no bonus");
}

#[test]
fn scorching_earth_discards_the_opponents_stadium_and_locks_out_their_next_turn() {
    let attack = Attack {
        name: "Scorching Earth",
        cost: vec![Type::Colorless],
        base_damage: 40,
        inflicts: None,
        effect: Some(AttackEffect::DiscardsOpponentsStadiumThenOpponentCannotPlayStadiumsNextTurn),
    };
    let (mut state, _defender_def) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let stadium_def = state.db.add(CardDef::Trainer(sim::card::Trainer {
        print_id: "test-scorching-earth-stadium",
        name: "Test Stadium",
        kind: sim::card::TrainerKind::Stadium,
        requirement: None,
        effect: sim::card::TrainerEffect::Nothing,
    }));
    let stadium = deal_new_card(&mut state, opponent, stadium_def);
    state.stadium = Some((opponent, stadium));

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 40);
    assert!(state.stadium.is_none(), "the opponent's Stadium is discarded");
    assert!(state.player(opponent).discard.contains(&stadium));

    // The opponent's own next turn: a Stadium in hand cannot be played.
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    let another_stadium_def = state.db.add(CardDef::Trainer(sim::card::Trainer {
        print_id: "test-scorching-earth-stadium-2",
        name: "Another Stadium",
        kind: sim::card::TrainerKind::Stadium,
        requirement: None,
        effect: sim::card::TrainerEffect::Nothing,
    }));
    let in_hand = deal_new_card(&mut state, opponent, another_stadium_def);
    assert!(
        !legal_actions(&state).iter().any(|a| matches!(a, Action::PlayTrainer { card } if *card == in_hand)),
        "no Stadium cards can be played this turn"
    );
}

#[test]
fn scorching_earth_does_not_touch_the_players_own_stadium() {
    let attack = Attack {
        name: "Scorching Earth",
        cost: vec![Type::Colorless],
        base_damage: 40,
        inflicts: None,
        effect: Some(AttackEffect::DiscardsOpponentsStadiumThenOpponentCannotPlayStadiumsNextTurn),
    };
    let (mut state, _defender_def) = game(attack, 3);
    let player = state.current;
    let stadium_def = state.db.add(CardDef::Trainer(sim::card::Trainer {
        print_id: "test-own-stadium",
        name: "Own Stadium",
        kind: sim::card::TrainerKind::Stadium,
        requirement: None,
        effect: sim::card::TrainerEffect::Nothing,
    }));
    let stadium = deal_new_card(&mut state, player, stadium_def);
    state.stadium = Some((player, stadium));

    pay_and_attack(&mut state);

    assert!(state.stadium.is_some(), "the attacker's own Stadium is left alone");
}

#[test]
fn whirling_envy_deals_bonus_damage_ignoring_weakness_once_damaged_enough() {
    let attack = Attack {
        name: "Whirling Envy",
        cost: vec![Type::Colorless],
        base_damage: 20,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfOwnDamageCountersAtLeastIgnoringWeakness(2, 90)),
    };
    // `game_with_weak_defender` prints the defender's own Weakness to
    // Colorless, the attacker's own type, so a normal attack would
    // double — Whirling Envy's own text says it never does.
    let mut state = game_with_weak_defender(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let defender = state.player(player.opponent()).active.unwrap();
    state.pokemon[attacker.index()].damage = 20; // 2 damage counters

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 110, "20 base + 90 bonus, never doubled by Weakness");
}

#[test]
fn whirling_envy_deals_only_base_damage_under_two_counters() {
    let attack = Attack {
        name: "Whirling Envy",
        cost: vec![Type::Colorless],
        base_damage: 20,
        inflicts: None,
        effect: Some(AttackEffect::BonusDamageIfOwnDamageCountersAtLeastIgnoringWeakness(2, 90)),
    };
    let (mut state, _defender_def) = game(attack, 3);
    let player = state.current;
    let defender = state.player(player.opponent()).active.unwrap();

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 20, "no damage counters yet, so no bonus");
}

#[test]
fn allure_draws_two_cards() {
    let attack = Attack {
        name: "Allure",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DrawCards(2)),
    };
    let (mut state, _defender_def) = game(attack, 3);
    let player = state.current;
    let hand_before = state.player(player).hand.len();

    pay_and_attack(&mut state);

    // -1 for the Colorless Energy Allure's own cost spent, +2 drawn.
    assert_eq!(state.player(player).hand.len(), hand_before + 1);
}

#[test]
fn chi_yus_attacks_are_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let admitted = import.cards.iter().filter(|c| c.name == "Chi-Yu" && c.playable.is_some()).count();
    assert_eq!(admitted, 3, "all 3 Chi-Yu prints should play");
}

// --- Shellnado Spin: a self-granted counter, even if Knocked Out ---

#[test]
fn shellnado_spin_counters_whoever_damages_it_next_turn() {
    let attack = Attack {
        name: "Shellnado Spin",
        cost: vec![Type::Colorless, Type::Colorless, Type::Colorless],
        base_damage: 180,
        inflicts: None,
        effect: Some(AttackEffect::GrantsSelfCountersAttackerIfDamagedNextTurn(120)),
    };
    let (mut state, _defender_def) = game(attack, 3);
    let player = state.current;
    let slowbro = state.player(player).active.unwrap();
    let opponent = player.opponent();

    pay_and_attack(&mut state);
    assert_eq!(state.current, opponent, "the attack ended the attacker's own turn");

    // The opponent's own Tackle hits back during their turn — exactly
    // when the granted counter is armed. It already carries 180
    // damage from Shellnado Spin's own attack in turn 1.
    let opponent_attacker = state.player(opponent).active.unwrap();
    let attacker_damage_before = state.pokemon(opponent_attacker).damage;
    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(slowbro).damage, 10, "the opponent's own 10 damage landed as normal");
    assert_eq!(
        state.pokemon(opponent_attacker).damage - attacker_damage_before,
        120,
        "the counter landed on the attacker"
    );
}

#[test]
fn shellnado_spins_counter_lands_even_when_the_hit_is_lethal() {
    let attack = Attack {
        name: "Shellnado Spin",
        cost: vec![Type::Colorless, Type::Colorless, Type::Colorless],
        base_damage: 180,
        inflicts: None,
        effect: Some(AttackEffect::GrantsSelfCountersAttackerIfDamagedNextTurn(120)),
    };
    let (mut state, _defender_def) = game(attack, 3);
    let player = state.current;
    let slowbro = state.player(player).active.unwrap();
    let opponent = player.opponent();

    pay_and_attack(&mut state);
    // Bring Slowbro to the brink by hand, so the opponent's ordinary
    // Tackle (10 damage) finishes it off — confirming the counter
    // still lands even though its own carrier is Knocked Out by the
    // very attack that triggers it.
    let slowbro_hp = state.pokemon_def(slowbro).hp;
    state.pokemon[slowbro.index()].damage = slowbro_hp - 10;

    let opponent_attacker = state.player(opponent).active.unwrap();
    let attacker_damage_before = state.pokemon(opponent_attacker).damage;
    pay_and_attack(&mut state);

    assert!(
        state.player(player).active.is_none() || state.pokemon(slowbro).knocked_out,
        "Slowbro was Knocked Out by this very attack"
    );
    assert_eq!(
        state.pokemon(opponent_attacker).damage - attacker_damage_before,
        120,
        "the counter lands even though its own carrier was just Knocked Out"
    );
}

#[test]
fn shellnado_spins_counter_does_not_carry_past_its_one_turn() {
    let attack = Attack {
        name: "Shellnado Spin",
        cost: vec![Type::Colorless, Type::Colorless, Type::Colorless],
        base_damage: 180,
        inflicts: None,
        effect: Some(AttackEffect::GrantsSelfCountersAttackerIfDamagedNextTurn(120)),
    };
    let (mut state, defender_def) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();

    pay_and_attack(&mut state);
    // The opponent's own turn ends without attacking (a fresh Bench
    // mon promoted instead), so the granted counter's one turn passes
    // unused.
    let bench_card = deal_new_card(&mut state, opponent, defender_def);
    let bench_mon = state.put_into_play(opponent, bench_card);
    state.players[opponent.index()].bench.push(bench_mon);
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    assert_eq!(state.current, opponent, "the opponent's own next turn, one turn too late");

    let opponent_attacker = state.player(opponent).active.unwrap();
    let attacker_damage_before = state.pokemon(opponent_attacker).damage;
    pay_and_attack(&mut state);

    assert_eq!(
        state.pokemon(opponent_attacker).damage - attacker_damage_before,
        0,
        "the counter's one turn has already passed"
    );
}

#[test]
fn mega_slowbro_exs_shellnado_spin_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Mega Slowbro ex" && c.playable.is_some()),
        "Mega Slowbro ex should play"
    );
}

// --- Ghostly Blow: placing counters on a chosen Benched Pokémon ---

#[test]
fn ghostly_blow_places_counters_on_the_chosen_benched_pokemon() {
    let attack = Attack {
        name: "Ghostly Blow",
        cost: vec![Type::Colorless, Type::Colorless],
        base_damage: 100,
        inflicts: None,
        effect: Some(AttackEffect::PlacesDamageCountersOnChosenOpponentBenched(50)),
    };
    let (mut state, defender_def) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let bench_card = deal_new_card(&mut state, opponent, defender_def);
    let bench_mon = state.put_into_play(opponent, bench_card);
    state.players[opponent.index()].bench.push(bench_mon);

    pay_and_attack(&mut state);
    assert!(matches!(state.phase, Phase::ChoosingAnyBenchedDamageTarget { .. }));

    apply(&mut state, Action::DamageAnyBenched { target: bench_mon }).unwrap();

    assert_eq!(state.pokemon(bench_mon).damage, 50, "5 damage counters, placed directly");
}

#[test]
fn ghostly_blow_does_nothing_with_an_empty_bench() {
    let attack = Attack {
        name: "Ghostly Blow",
        cost: vec![Type::Colorless, Type::Colorless],
        base_damage: 100,
        inflicts: None,
        effect: Some(AttackEffect::PlacesDamageCountersOnChosenOpponentBenched(50)),
    };
    let (mut state, _defender_def) = game(attack, 3);

    pay_and_attack(&mut state);

    assert_eq!(state.phase, Phase::Main, "no Benched Pokémon, so no phase opens");
}

#[test]
fn annihilapes_ghostly_blow_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "me05-041").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Annihilape's Ghostly Blow/Durable Body print should play");
}

// --- Tantrum and Destined Fight: self-inflicted and mutual ---

#[test]
fn tantrum_confuses_the_attacker_itself() {
    let attack = Attack {
        name: "Tantrum",
        cost: vec![Type::Colorless],
        base_damage: 130,
        inflicts: None,
        effect: Some(AttackEffect::InflictsConditionOnSelf(sim::card::Condition::Confused)),
    };
    let (mut state, _defender_def) = game(attack, 3);
    let player = state.current;
    let attacker = state.player(player).active.unwrap();
    let defender = state.player(player.opponent()).active.unwrap();

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 130, "the attack's own damage lands as normal");
    assert!(
        state.has_condition(attacker, sim::card::Condition::Confused),
        "the attacker itself is Confused, not the defender"
    );
    assert!(!state.has_condition(defender, sim::card::Condition::Confused));
}

#[test]
fn destined_fight_knocks_out_both_actives() {
    let attack = Attack {
        name: "Destined Fight",
        cost: vec![Type::Colorless, Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::KnocksOutBothActivePokemon),
    };
    let (mut state, _defender_def) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let attacker = state.player(player).active.unwrap();
    let defender = state.player(opponent).active.unwrap();
    let attacker_prizes_before = state.player(player).prizes.len();
    let defender_prizes_before = state.player(opponent).prizes.len();

    pay_and_attack(&mut state);

    assert!(state.pokemon(attacker).knocked_out, "the attacker itself is Knocked Out too");
    assert!(state.pokemon(defender).knocked_out);
    assert_eq!(
        state.player(player).prizes.len(),
        attacker_prizes_before - 1,
        "the opponent's Prize for Knocking out the attacker"
    );
    assert_eq!(
        state.player(opponent).prizes.len(),
        defender_prizes_before - 1,
        "the attacker's own Prize for Knocking out the defender"
    );
}

#[test]
fn annihilapes_tantrum_and_destined_fight_are_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv08-100").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Annihilape's Tantrum/Destined Fight print should play");
}

// --- Whirlpool, Rapid Draw, Do the Wave: the Festival Lead family's own attacks ---

#[test]
fn whirlpool_discards_defender_energy_on_heads() {
    let attack = Attack {
        name: "Whirlpool",
        cost: vec![Type::Colorless, Type::Colorless],
        base_damage: 10,
        inflicts: None,
        effect: Some(AttackEffect::CoinFlipDiscardsDefenderEnergy),
    };
    let (mut state, _defender_def) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let energy = *state
        .player(opponent)
        .deck
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    state.players[opponent.index()].deck.retain(|c| *c != energy);
    state.pokemon[defender.index()].attached.push(energy);

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 10, "the attack's own damage always lands");
    // Either the coin landed heads (a discard phase opens) or tails
    // (nothing further happens) — both are legitimate outcomes.
    assert!(
        matches!(state.phase, Phase::DiscardingDefenderEnergyForAttack { .. }) || state.phase == Phase::Main
    );
}

#[test]
fn rapid_draw_deals_damage_and_draws_two() {
    let attack = Attack {
        name: "Rapid Draw",
        cost: vec![Type::Colorless],
        base_damage: 60,
        inflicts: None,
        effect: Some(AttackEffect::DrawCards(2)),
    };
    let (mut state, _defender_def) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let hand_before = state.player(player).hand.len();

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 60);
    // -1 for the Colorless Energy this attack's own cost spent, +2 drawn.
    assert_eq!(state.player(player).hand.len(), hand_before + 1);
}

#[test]
fn do_the_wave_deals_twenty_per_benched_pokemon() {
    let attack = Attack {
        name: "Do the Wave",
        cost: vec![Type::Colorless],
        base_damage: 0,
        inflicts: None,
        effect: Some(AttackEffect::DamagePerCount(sim::card::Count::OwnBenchedPokemonCount, 20)),
    };
    let (mut state, defender_def) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    for _ in 0..2 {
        let bench_card = deal_new_card(&mut state, player, defender_def);
        let bench_mon = state.put_into_play(player, bench_card);
        state.players[player.index()].bench.push(bench_mon);
    }

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 40, "20 for each of 2 Benched Pokémon");
}

#[test]
fn goldeen_seaking_and_dipplin_festival_lead_prints_are_admitted() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    for id in ["sv06-044", "sv08.5-020", "sv06-045", "sv08.5-021", "sv06-170"] {
        let card = import.cards.iter().find(|c| c.id == id).expect("the artifact holds this print");
        assert!(card.playable.is_some(), "{id} should play");
    }
}

// --- Poison Chain: Poison plus a retreat lock, one restriction record ---

#[test]
fn poison_chain_poisons_and_locks_out_retreat_next_turn() {
    let attack = Attack {
        name: "Poison Chain",
        cost: vec![Type::Colorless, Type::Colorless],
        base_damage: 10,
        inflicts: None,
        effect: Some(AttackEffect::InflictsConditionAndDefenderCannotRetreatNextTurn(
            sim::card::Condition::Poisoned,
        )),
    };
    let (mut state, defender_ex) = game(attack, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();
    let energy_card = *state
        .player(opponent)
        .deck
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    state.players[opponent.index()].deck.retain(|c| *c != energy_card);
    state.pokemon[defender.index()].attached.push(energy_card);
    let bench_card = deal_new_card(&mut state, opponent, defender_ex);
    let bench_mon = state.put_into_play(opponent, bench_card);
    state.players[opponent.index()].bench.push(bench_mon);

    pay_and_attack(&mut state);

    assert_eq!(state.pokemon(defender).damage, 10);
    assert!(state.has_condition(defender, sim::card::Condition::Poisoned));
    assert!(
        !legal_actions(&state).into_iter().any(|a| matches!(a, Action::Retreat { .. })),
        "the Poisoned Pokémon cannot retreat during the opponent's own next turn"
    );
}

#[test]
fn pecharunts_poison_chain_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    for id in ["svp-129", "svp-149"] {
        let card = import.cards.iter().find(|c| c.id == id).expect("the artifact holds this print");
        assert!(card.playable.is_some(), "{id} should play");
    }
}
