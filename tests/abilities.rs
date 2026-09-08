//! Milestone 8: the first Abilities. One fixture, shared across this
//! milestone's tickets, the same shape every earlier milestone's own
//! fixture already took.

use sim::action::{Action, legal_actions};
use sim::card::{
    Ability, Attack, CardDb, CardDef, Energy, Pokemon, Stage, Trainer, TrainerEffect,
    TrainerKind, Type,
};
use sim::engine::apply;
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

/// A game where the first player's Active carries the Ability under
/// test, and the second player's Active is a plain punching bag.
fn game(ability: Ability, seed: u64) -> (GameState, sim::ids::CardDefId) {
    let mut db = CardDb::new();
    let carrier = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-carrier",
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
        ability: Some(ability),
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
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
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-colorless-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));

    let mut carrier_deck = vec![carrier; 4];
    while carrier_deck.len() < 60 {
        carrier_deck.push(energy);
    }
    let mut defender_deck = vec![defender_mon; 4];
    while defender_deck.len() < 60 {
        defender_deck.push(energy);
    }
    let mut state = GameState::new(
        db,
        [carrier_deck, defender_deck],
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
    (state, carrier)
}

/// A physical card of `def`, for a definition that was never part of
/// the deck. Placed nowhere; the caller pushes it where the test needs.
fn deal_new_card(state: &mut GameState, player: sim::ids::PlayerId, def: sim::ids::CardDefId) -> sim::ids::CardId {
    let card = sim::ids::CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def, owner: player });
    card
}

// --- Ticket 01: an Ability a player opts into ---

#[test]
fn draws_once_while_the_carrier_is_active() {
    let ability = Ability {
        name: "Run Errand",
        effect: sim::card::AbilityEffect::OncePerTurnWhileActiveMayDrawCards(2),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let carrier = state.player(player).active.unwrap();
    let before = state.player(player).hand.len();

    apply(&mut state, Action::UseAbility { pokemon: carrier }).unwrap();

    assert_eq!(state.player(player).hand.len(), before + 2);
    assert_eq!(state.phase, Phase::Main, "no phase, no cost, no target");
}

#[test]
fn cannot_use_the_ability_a_second_time_the_same_turn() {
    let ability = Ability {
        name: "Run Errand",
        effect: sim::card::AbilityEffect::OncePerTurnWhileActiveMayDrawCards(2),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let carrier = state.player(player).active.unwrap();

    apply(&mut state, Action::UseAbility { pokemon: carrier }).unwrap();
    let result = apply(&mut state, Action::UseAbility { pokemon: carrier });

    assert!(result.is_err(), "Run Errand is once per turn");
}

#[test]
fn cannot_use_the_ability_from_the_bench() {
    let ability = Ability {
        name: "Run Errand",
        effect: sim::card::AbilityEffect::OncePerTurnWhileActiveMayDrawCards(2),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let active_card = state.pokemon(state.player(player).active.unwrap()).top_card();
    let carrier_def = state.cards[active_card.index()].def;

    // Bench a second copy and try to use its Ability instead.
    let card = sim::ids::CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def: carrier_def, owner: player });
    let bench_mon = state.put_into_play(player, card);
    state.players[player.index()].bench.push(bench_mon);

    let result = apply(&mut state, Action::UseAbility { pokemon: bench_mon });
    assert!(result.is_err(), "Run Errand only works from the Active Spot");
}

#[test]
fn cannot_use_the_ability_on_the_opponents_turn() {
    let ability = Ability {
        name: "Run Errand",
        effect: sim::card::AbilityEffect::OncePerTurnWhileActiveMayDrawCards(2),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let owner = state.current;
    let carrier = state.player(owner).active.unwrap();

    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    assert_eq!(state.current, owner.opponent(), "now the opponent's turn");

    let result = apply(&mut state, Action::UseAbility { pokemon: carrier });
    assert!(result.is_err(), "not this player's turn");
}

#[test]
fn mega_kangaskhan_ex_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Mega Kangaskhan ex" && c.playable.is_some()),
        "at least one Mega Kangaskhan ex print should play"
    );
}

// --- Ticket 03: an Ability tied to a moment ---

#[test]
fn benching_from_hand_offers_the_search() {
    let ability = Ability {
        name: "Last-Ditch Catch",
        effect: sim::card::AbilityEffect::WhenBenchedFromHandMaySearchSupporter,
    };
    let (mut state, carrier_def) = game(ability, 3);
    let player = state.current;

    // A Supporter to find, and a second copy of the carrier in hand.
    let supporter_def = state.db.add(CardDef::Trainer(sim::card::Trainer {
        print_id: "test-supporter",
        name: "Test Supporter",
        kind: sim::card::TrainerKind::Supporter,
        requirement: None,
        effect: sim::card::TrainerEffect::MoveAttachedEnergy,
    }));
    let supporter = deal_new_card(&mut state, player, supporter_def);
    state.players[player.index()].library.push(supporter);
    let second_copy = deal_new_card(&mut state, player, carrier_def);
    state.players[player.index()].hand.push(second_copy);

    apply(&mut state, Action::PlayBasic { card: second_copy }).unwrap();

    assert!(matches!(state.phase, Phase::DecidingToUseLastDitchCatch { .. }));
    apply(
        &mut state,
        Action::TakeSupporterForLastDitchCatch { card: supporter },
    )
    .unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).hand.contains(&supporter));
}

#[test]
fn declining_the_search_leaves_the_library_alone() {
    let ability = Ability {
        name: "Last-Ditch Catch",
        effect: sim::card::AbilityEffect::WhenBenchedFromHandMaySearchSupporter,
    };
    let (mut state, carrier_def) = game(ability, 3);
    let player = state.current;
    let supporter_def = state.db.add(CardDef::Trainer(sim::card::Trainer {
        print_id: "test-supporter",
        name: "Test Supporter",
        kind: sim::card::TrainerKind::Supporter,
        requirement: None,
        effect: sim::card::TrainerEffect::MoveAttachedEnergy,
    }));
    let supporter = deal_new_card(&mut state, player, supporter_def);
    state.players[player.index()].library.push(supporter);
    let second_copy = deal_new_card(&mut state, player, carrier_def);
    state.players[player.index()].hand.push(second_copy);

    apply(&mut state, Action::PlayBasic { card: second_copy }).unwrap();
    apply(&mut state, Action::DeclineLastDitchCatch).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).library.contains(&supporter));
}

#[test]
fn no_supporter_in_library_opens_no_phase() {
    let ability = Ability {
        name: "Last-Ditch Catch",
        effect: sim::card::AbilityEffect::WhenBenchedFromHandMaySearchSupporter,
    };
    let (mut state, carrier_def) = game(ability, 3);
    let player = state.current;
    let second_copy = deal_new_card(&mut state, player, carrier_def);
    state.players[player.index()].hand.push(second_copy);

    apply(&mut state, Action::PlayBasic { card: second_copy }).unwrap();

    assert_eq!(state.phase, Phase::Main, "no Supporter in the library");
}

#[test]
fn does_not_trigger_a_second_time_the_same_turn() {
    let ability = Ability {
        name: "Last-Ditch Catch",
        effect: sim::card::AbilityEffect::WhenBenchedFromHandMaySearchSupporter,
    };
    let (mut state, carrier_def) = game(ability, 3);
    let player = state.current;
    let supporter_def = state.db.add(CardDef::Trainer(sim::card::Trainer {
        print_id: "test-supporter",
        name: "Test Supporter",
        kind: sim::card::TrainerKind::Supporter,
        requirement: None,
        effect: sim::card::TrainerEffect::MoveAttachedEnergy,
    }));
    let first_supporter = deal_new_card(&mut state, player, supporter_def);
    let second_supporter = deal_new_card(&mut state, player, supporter_def);
    state.players[player.index()].library.push(first_supporter);
    state.players[player.index()].library.push(second_supporter);
    let second_copy = deal_new_card(&mut state, player, carrier_def);
    let third_copy = deal_new_card(&mut state, player, carrier_def);
    state.players[player.index()].hand.push(second_copy);
    state.players[player.index()].hand.push(third_copy);

    apply(&mut state, Action::PlayBasic { card: second_copy }).unwrap();
    apply(
        &mut state,
        Action::TakeSupporterForLastDitchCatch { card: first_supporter },
    )
    .unwrap();

    apply(&mut state, Action::PlayBasic { card: third_copy }).unwrap();
    assert_eq!(state.phase, Phase::Main, "already used a Last-Ditch Ability this turn");
}

#[test]
fn meowth_ex_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Meowth ex" && c.playable.is_some()),
        "at least one Meowth ex print should play"
    );
}

// --- Ticket 04: the same Ability name, two effects ---

#[test]
fn evolving_offers_the_draw() {
    let ability = Ability {
        name: "Psychic Draw",
        effect: sim::card::AbilityEffect::WhenEvolvedFromHandMayDrawCards(2),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let basic = state.player(player).active.unwrap();
    let basic_name = state.pokemon_def(basic).name;

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
        evolve_from: Some(basic_name),
        evolves_from_basic: None,
        ability: Some(ability),
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let evolution = deal_new_card(&mut state, player, evolution_def);
    state.players[player.index()].hand.push(evolution);
    let before = state.player(player).hand.len();

    apply(&mut state, Action::Evolve { card: evolution, target: basic }).unwrap();

    assert!(matches!(state.phase, Phase::DecidingToUsePsychicDraw { .. }));
    apply(&mut state, Action::AcceptPsychicDraw).unwrap();

    assert_eq!(state.phase, Phase::Main);
    // The evolution card itself left the hand to evolve; the draw adds 2.
    assert_eq!(state.player(player).hand.len(), before - 1 + 2);
}

#[test]
fn kadabra_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "me01-055").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Kadabra's Super Psy Bolt print should play");
}

// --- Ticket 05: a fact read from last turn ---

#[test]
fn draws_only_if_a_pokemon_was_knocked_out_last_turn() {
    let ability = Ability {
        name: "Flip the Script",
        effect: sim::card::AbilityEffect::OncePerTurnIfKnockedOutLastTurnMayDrawCards(3),
    };
    let (mut state, carrier_def) = game(ability, 3);
    let player = state.current;
    let carrier = state.player(player).active.unwrap();

    // Not knocked out last turn: not offered at all.
    let result = apply(&mut state, Action::UseAbility { pokemon: carrier });
    assert!(result.is_err(), "nothing was Knocked Out last turn");

    state.knocked_out_last_turn[player.index()] = true;
    let before = state.player(player).hand.len();
    apply(&mut state, Action::UseAbility { pokemon: carrier }).unwrap();
    assert_eq!(state.player(player).hand.len(), before + 3);
    let _ = carrier_def;
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

// --- Ticket 06: attaching Energy from hand as the effect itself ---

#[test]
fn attaches_energy_from_hand_and_draws() {
    let ability = Ability {
        name: "Teal Dance",
        effect: sim::card::AbilityEffect::OncePerTurnMayAttachBasicEnergyOfTypeThenDraw(Type::Grass),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let carrier = state.player(player).active.unwrap();

    let grass_energy_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-grass-energy",
        name: "Grass Energy",
        kind: Type::Grass,
        effect: None,
    }));
    let energy = deal_new_card(&mut state, player, grass_energy_def);
    state.players[player.index()].hand.push(energy);
    let before = state.player(player).hand.len();

    apply(&mut state, Action::UseAbility { pokemon: carrier }).unwrap();

    assert!(matches!(state.phase, Phase::DecidingToUseTealDance { .. }));
    apply(&mut state, Action::AttachEnergyForTealDance { card: energy }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.pokemon(carrier).attached.contains(&energy));
    // The Energy left the hand; the draw adds 1 back.
    assert_eq!(state.player(player).hand.len(), before - 1 + 1);
}

#[test]
fn declining_teal_dance_draws_nothing() {
    let ability = Ability {
        name: "Teal Dance",
        effect: sim::card::AbilityEffect::OncePerTurnMayAttachBasicEnergyOfTypeThenDraw(Type::Grass),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let carrier = state.player(player).active.unwrap();
    let grass_energy_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-grass-energy",
        name: "Grass Energy",
        kind: Type::Grass,
        effect: None,
    }));
    let energy = deal_new_card(&mut state, player, grass_energy_def);
    state.players[player.index()].hand.push(energy);
    let before = state.player(player).hand.len();

    apply(&mut state, Action::UseAbility { pokemon: carrier }).unwrap();
    apply(&mut state, Action::DeclineTealDance).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.player(player).hand.len(), before, "no attach, no draw");
}

#[test]
fn teal_dance_is_not_offered_with_no_grass_energy_in_hand() {
    let ability = Ability {
        name: "Teal Dance",
        effect: sim::card::AbilityEffect::OncePerTurnMayAttachBasicEnergyOfTypeThenDraw(Type::Grass),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let carrier = state.player(player).active.unwrap();

    let result = apply(&mut state, Action::UseAbility { pokemon: carrier });
    assert!(result.is_err(), "no Basic Grass Energy in hand to attach");
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

// --- Ticket 07: a Pokemon that returns to the deck ---

#[test]
fn draws_then_shuffles_itself_into_the_deck_from_the_bench() {
    let ability = Ability {
        name: "Run Away Draw",
        effect: sim::card::AbilityEffect::OncePerTurnMayDrawThenShuffleSelfIntoDeck(3),
    };
    let (mut state, carrier_def) = game(ability, 3);
    let player = state.current;
    let bench_card = deal_new_card(&mut state, player, carrier_def);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);
    let before = state.player(player).hand.len();
    let library_before = state.player(player).library.len();

    apply(&mut state, Action::UseAbility { pokemon: bench_mon }).unwrap();

    assert_eq!(state.player(player).hand.len(), before + 3);
    assert!(!state.player(player).bench.contains(&bench_mon), "left the Bench");
    assert_eq!(
        state.player(player).library.len(),
        library_before - 3 + 1,
        "3 drawn out, itself shuffled back in"
    );
    assert_eq!(state.phase, Phase::Main);
}

#[test]
fn draws_then_shuffles_itself_from_active_promoting_from_bench() {
    let ability = Ability {
        name: "Run Away Draw",
        effect: sim::card::AbilityEffect::OncePerTurnMayDrawThenShuffleSelfIntoDeck(3),
    };
    let (mut state, carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let bench_card = deal_new_card(&mut state, player, carrier_def);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);

    apply(&mut state, Action::UseAbility { pokemon: active }).unwrap();

    assert!(matches!(state.phase, Phase::Promoting { .. }));
    apply(&mut state, Action::Promote { pokemon: bench_mon }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.player(player).active, Some(bench_mon));
}

#[test]
fn dudunsparce_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Dudunsparce" && c.playable.is_some()),
        "at least one Dudunsparce print should play"
    );
}

// --- Beyond the map: Abra's own shuffle-self ability ---

#[test]
fn shuffles_itself_into_the_deck_while_active_no_draw() {
    let ability = Ability {
        name: "Teleporter",
        effect: sim::card::AbilityEffect::OncePerTurnWhileActiveMayShuffleSelfIntoDeck,
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let bench_card_def = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-bench-filler",
        name: "Fillermon",
        hp: 100,
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
    let bench_card = deal_new_card(&mut state, player, bench_card_def);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);
    let library_before = state.player(player).library.len();

    apply(&mut state, Action::UseAbility { pokemon: active }).unwrap();

    assert!(matches!(state.phase, Phase::Promoting { .. }));
    apply(&mut state, Action::Promote { pokemon: bench_mon }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.player(player).active, Some(bench_mon));
    assert_eq!(state.player(player).library.len(), library_before + 1);
}

#[test]
fn cannot_shuffle_self_from_the_bench() {
    let ability = Ability {
        name: "Teleporter",
        effect: sim::card::AbilityEffect::OncePerTurnWhileActiveMayShuffleSelfIntoDeck,
    };
    let (mut state, carrier_def) = game(ability, 3);
    let player = state.current;
    let bench_card = deal_new_card(&mut state, player, carrier_def);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);

    let result = apply(&mut state, Action::UseAbility { pokemon: bench_mon });
    assert!(result.is_err(), "only works from the Active Spot");
}

#[test]
fn abra_beam_print_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv06-080").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Abra's Beam print should play");
}

// --- Beyond the map: an Ability that knocks out its own carrier ---

#[test]
fn damages_the_opponent_then_knocks_out_the_carrier() {
    let ability = Ability {
        name: "Cursed Blast",
        effect: sim::card::AbilityEffect::OncePerTurnMayDamageOpponentThenKnockOutSelf(5),
    };
    let (mut state, carrier_def) = game(ability, 3);
    let player = state.current;
    let opponent = player.opponent();
    let active = state.player(player).active.unwrap();
    let defender = state.player(opponent).active.unwrap();
    // A Bench Pokemon of the player's own, so a promotion has somewhere to go.
    let bench_card = deal_new_card(&mut state, player, carrier_def);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);

    apply(&mut state, Action::UseAbility { pokemon: active }).unwrap();

    assert!(matches!(state.phase, Phase::DecidingCursedBlastTarget { .. }));
    apply(&mut state, Action::DamageOpponentForCursedBlast { target: defender }).unwrap();

    assert_eq!(state.pokemon(defender).damage, 50);
    assert!(state.pokemon(active).knocked_out, "the carrier is Knocked Out too");
}

#[test]
fn battle_cage_stops_cursed_blast_landing_on_a_benched_target_but_the_carrier_still_ko_s() {
    let ability = Ability {
        name: "Cursed Blast",
        effect: sim::card::AbilityEffect::OncePerTurnMayDamageOpponentThenKnockOutSelf(13),
    };
    let (mut state, carrier_def) = game(ability, 3);
    let player = state.current;
    let opponent = player.opponent();
    let active = state.player(player).active.unwrap();
    // A Bench Pokemon of the player's own, so a promotion has somewhere to go.
    let bench_card = deal_new_card(&mut state, player, carrier_def);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);

    let cage = state.db.add(CardDef::Trainer(Trainer {
        print_id: "test-battle-cage-cursed-blast",
        name: "Battle Cage",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::PreventsDamageCountersOnBench,
    }));
    let cage_card = deal_new_card(&mut state, player, cage);
    state.stadium = Some((player, cage_card));

    let opponent_bench_def = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-cursed-blast-bench-target",
        name: "Benchmon",
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
        attacks: vec![],
    }));
    let opponent_bench_card = deal_new_card(&mut state, opponent, opponent_bench_def);
    let opponent_bench = state.put_into_play(opponent, opponent_bench_card);
    state.players[opponent.index()].bench.push(opponent_bench);

    apply(&mut state, Action::UseAbility { pokemon: active }).unwrap();
    assert!(matches!(state.phase, Phase::DecidingCursedBlastTarget { .. }));
    apply(&mut state, Action::DamageOpponentForCursedBlast { target: opponent_bench }).unwrap();

    assert_eq!(state.pokemon(opponent_bench).damage, 0, "Battle Cage stops the counters landing");
    assert!(state.pokemon(active).knocked_out, "the Ability was still used, so the carrier still KO's");
}

#[test]
fn dusclops_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Dusclops" && c.playable.is_some()),
        "at least one Dusclops print should play"
    );
}

#[test]
fn dusknoir_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Dusknoir" && c.playable.is_some()),
        "at least one Dusknoir print should play"
    );
}

// --- Beyond the map: a standing search for Evolution Pokemon of a type ---

#[test]
fn searches_for_up_to_two_evolution_pokemon_of_a_type() {
    let ability = Ability {
        name: "Metallic Signal",
        effect: sim::card::AbilityEffect::OncePerTurnMaySearchEvolutionPokemonOfType(Type::Metal, 2),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();

    let evolution_def = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-metal-evolution",
        name: "Metalmon",
        hp: 120,
        kind: Type::Metal,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Stage1,
        evolve_from: Some("Anything"),
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
    state.players[player.index()].library.push(evolution);
    let before = state.player(player).hand.len();

    apply(&mut state, Action::UseAbility { pokemon: active }).unwrap();

    assert!(matches!(state.phase, Phase::SearchingLibraryForEvolutionPokemonOfType { .. }));
    apply(
        &mut state,
        Action::TakeEvolutionPokemonOfType { card: evolution },
    )
    .unwrap();
    apply(&mut state, Action::FinishSearchingEvolutionPokemonOfType).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.player(player).hand.len(), before + 1);
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

// --- Beyond the map: attaching Energy from discard to a chosen own Pokemon ---

#[test]
fn attaches_energy_from_discard_to_a_chosen_own_pokemon() {
    let ability = Ability {
        name: "Seething Spirit",
        effect: sim::card::AbilityEffect::OncePerTurnMayAttachBasicEnergyFromDiscardToChosen,
    };
    let (mut state, carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let bench_card = deal_new_card(&mut state, player, carrier_def);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);

    let energy_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-basic-energy",
        name: "Basic Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    let energy = deal_new_card(&mut state, player, energy_def);
    state.players[player.index()].discard.push(energy);

    apply(&mut state, Action::UseAbility { pokemon: active }).unwrap();

    assert!(matches!(state.phase, Phase::DecidingToUseSeethingSpirit { .. }));
    apply(
        &mut state,
        Action::AttachEnergyForSeethingSpirit { card: energy, target: bench_mon },
    )
    .unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.pokemon(bench_mon).attached.contains(&energy));
    assert!(!state.player(player).discard.contains(&energy));
}

#[test]
fn seething_spirit_not_offered_with_no_energy_in_discard() {
    let ability = Ability {
        name: "Seething Spirit",
        effect: sim::card::AbilityEffect::OncePerTurnMayAttachBasicEnergyFromDiscardToChosen,
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();

    let result = apply(&mut state, Action::UseAbility { pokemon: active });
    assert!(result.is_err(), "no Basic Energy in the discard pile");
}

#[test]
fn blaziken_ex_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Blaziken ex" && c.playable.is_some()),
        "at least one Blaziken ex print should play"
    );
}

// --- Beyond the map: a play-triggered may-discard-the-Stadium Ability ---

#[test]
fn benching_from_hand_offers_discarding_the_stadium() {
    let ability = Ability {
        name: "Snow Sink",
        effect: sim::card::AbilityEffect::WhenBenchedFromHandMayDiscardStadium,
    };
    let (mut state, carrier_def) = game(ability, 3);
    let player = state.current;

    let stadium_def = state.db.add(CardDef::Trainer(sim::card::Trainer {
        print_id: "test-stadium",
        name: "Test Stadium",
        kind: sim::card::TrainerKind::Stadium,
        requirement: None,
        effect: sim::card::TrainerEffect::MoveAttachedEnergy,
    }));
    let stadium = deal_new_card(&mut state, player, stadium_def);
    state.stadium = Some((player.opponent(), stadium));

    let second_copy = deal_new_card(&mut state, player, carrier_def);
    state.players[player.index()].hand.push(second_copy);

    apply(&mut state, Action::PlayBasic { card: second_copy }).unwrap();

    assert!(matches!(state.phase, Phase::DecidingToUseSnowSink { .. }));
    apply(&mut state, Action::AcceptSnowSink).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.stadium, None);
    assert!(state.player(player.opponent()).discard.contains(&stadium));
}

#[test]
fn no_stadium_in_play_opens_no_phase_for_snow_sink() {
    let ability = Ability {
        name: "Snow Sink",
        effect: sim::card::AbilityEffect::WhenBenchedFromHandMayDiscardStadium,
    };
    let (mut state, carrier_def) = game(ability, 3);
    let player = state.current;
    let second_copy = deal_new_card(&mut state, player, carrier_def);
    state.players[player.index()].hand.push(second_copy);

    apply(&mut state, Action::PlayBasic { card: second_copy }).unwrap();

    assert_eq!(state.phase, Phase::Main, "no Stadium in play");
}

#[test]
fn chien_pao_snow_sink_print_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import.cards.iter().find(|c| c.id == "sv08-056").expect("the artifact holds this print");
    assert!(card.playable.is_some(), "Chien-Pao's Snow Sink print should play");
}

// --- Beyond the map: switch in on entry, then move any amount of Energy ---

#[test]
fn switches_in_then_moves_energy_from_other_pokemon() {
    let ability = Ability {
        name: "Rapid Vernier",
        effect: sim::card::AbilityEffect::WhenBenchedFromHandMaySwitchThenMoveAnyEnergy,
    };
    let (mut state, carrier_def) = game(ability, 3);
    let player = state.current;
    let old_active = state.player(player).active.unwrap();

    let energy_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-energy",
        name: "Test Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    let energy = deal_new_card(&mut state, player, energy_def);
    state.pokemon[old_active.index()].attached.push(energy);

    let second_copy = deal_new_card(&mut state, player, carrier_def);
    state.players[player.index()].hand.push(second_copy);

    apply(&mut state, Action::PlayBasic { card: second_copy }).unwrap();

    assert!(matches!(state.phase, Phase::DecidingToSwitchInForRapidVernier { .. }));
    apply(&mut state, Action::AcceptRapidVernierSwitch).unwrap();

    let new_active = state.player(player).active.unwrap();
    assert_ne!(new_active, old_active, "switched in");
    assert!(state.player(player).bench.contains(&old_active));

    assert!(matches!(state.phase, Phase::MovingAnyEnergyForRapidVernier { .. }));
    apply(&mut state, Action::MoveEnergyForRapidVernier { card: energy }).unwrap();
    apply(&mut state, Action::FinishMovingEnergyForRapidVernier).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.pokemon(new_active).attached.contains(&energy));
    assert!(!state.pokemon(old_active).attached.contains(&energy));
}

#[test]
fn declining_the_switch_ends_it_there() {
    let ability = Ability {
        name: "Rapid Vernier",
        effect: sim::card::AbilityEffect::WhenBenchedFromHandMaySwitchThenMoveAnyEnergy,
    };
    let (mut state, carrier_def) = game(ability, 3);
    let player = state.current;
    let old_active = state.player(player).active.unwrap();
    let second_copy = deal_new_card(&mut state, player, carrier_def);
    state.players[player.index()].hand.push(second_copy);

    apply(&mut state, Action::PlayBasic { card: second_copy }).unwrap();
    apply(&mut state, Action::DeclineRapidVernierSwitch).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.player(player).active, Some(old_active), "no switch");
}

#[test]
fn iron_leaves_ex_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Iron Leaves ex" && c.playable.is_some()),
        "at least one Iron Leaves ex print should play"
    );
}

// --- Beyond the map: search + attach + self-damage as one Ability ---

#[test]
fn searches_energy_attaches_to_benched_of_type_then_damages_it() {
    let ability = Ability {
        name: "Sinister Surge",
        effect: sim::card::AbilityEffect::OncePerTurnMaySearchBasicEnergyOfTypeAttachToBenchedThenDamage(
            Type::Darkness,
            20,
        ),
    };
    let (mut state, carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();

    let dark_bench_def = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-dark-bench",
        name: "Darkmon",
        hp: 200,
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
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let bench_card = deal_new_card(&mut state, player, dark_bench_def);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);

    let energy_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-dark-energy",
        name: "Darkness Energy",
        kind: Type::Darkness,
        effect: None,
    }));
    let energy = deal_new_card(&mut state, player, energy_def);
    state.players[player.index()].library.push(energy);

    apply(&mut state, Action::UseAbility { pokemon: active }).unwrap();

    assert!(matches!(state.phase, Phase::SearchingForSinisterSurgeTarget { .. }));
    apply(
        &mut state,
        Action::AttachSinisterSurgeEnergyTo { target: bench_mon },
    )
    .unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.pokemon(bench_mon).attached.contains(&energy));
    assert_eq!(state.pokemon(bench_mon).damage, 20);
}

#[test]
fn sinister_surge_not_offered_with_no_energy_or_no_benched_target() {
    let ability = Ability {
        name: "Sinister Surge",
        effect: sim::card::AbilityEffect::OncePerTurnMaySearchBasicEnergyOfTypeAttachToBenchedThenDamage(
            Type::Darkness,
            20,
        ),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();

    let result = apply(&mut state, Action::UseAbility { pokemon: active });
    assert!(result.is_err(), "no Darkness Energy and no Benched Darkness Pokemon");
}

#[test]
fn toxtricity_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Toxtricity" && c.playable.is_some()),
        "at least one Toxtricity print should play"
    );
}

// --- Beyond the map: a search restricted to the player's own first turn ---

#[test]
fn searches_on_the_first_turn_for_colorless_low_hp_pokemon() {
    let ability = Ability {
        name: "Fan Call",
        effect: sim::card::AbilityEffect::OnceDuringFirstTurnMaySearchPokemonOfTypeWithHpAtMost(
            Type::Colorless,
            100,
            3,
        ),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();

    let target_def = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-low-hp-colorless",
        name: "Smallmon",
        hp: 80,
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
    let target = deal_new_card(&mut state, player, target_def);
    state.players[player.index()].library.push(target);
    let before = state.player(player).hand.len();

    apply(&mut state, Action::UseAbility { pokemon: active }).unwrap();

    assert!(matches!(state.phase, Phase::SearchingForFanCall { .. }));
    apply(&mut state, Action::TakeCardForFanCall { card: target }).unwrap();
    apply(&mut state, Action::FinishFanCall).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.player(player).hand.len(), before + 1);
}

#[test]
fn fan_call_not_offered_after_the_first_turn() {
    let ability = Ability {
        name: "Fan Call",
        effect: sim::card::AbilityEffect::OnceDuringFirstTurnMaySearchPokemonOfTypeWithHpAtMost(
            Type::Colorless,
            100,
            3,
        ),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();

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

    let result = apply(&mut state, Action::UseAbility { pokemon: active });
    assert!(result.is_err(), "past the player's own first turn");
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

// --- Beyond the map: a standing switch that Poisons the new Active ---

#[test]
fn switches_a_benched_dark_pokemon_in_and_poisons_it() {
    let ability = Ability {
        name: "Subjugating Chains",
        effect: sim::card::AbilityEffect::OncePerTurnMaySwitchBenchedOfTypeExcludingNamedThenPoison(
            Type::Darkness,
            "Pecharunt ex",
        ),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let old_active = state.player(player).active.unwrap();

    let dark_bench_def = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-dark-bench",
        name: "Darkmon",
        hp: 200,
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
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let bench_card = deal_new_card(&mut state, player, dark_bench_def);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);

    apply(&mut state, Action::UseAbility { pokemon: old_active }).unwrap();

    assert!(matches!(state.phase, Phase::DecidingToUseSubjugatingChains { .. }));
    apply(
        &mut state,
        Action::SwitchForSubjugatingChains { target: bench_mon },
    )
    .unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.player(player).active, Some(bench_mon));
    assert!(state.player(player).bench.contains(&old_active));
    assert!(state.has_condition(bench_mon, sim::card::Condition::Poisoned));
}

#[test]
fn subjugating_chains_not_offered_with_no_qualifying_bench() {
    let ability = Ability {
        name: "Subjugating Chains",
        effect: sim::card::AbilityEffect::OncePerTurnMaySwitchBenchedOfTypeExcludingNamedThenPoison(
            Type::Darkness,
            "Pecharunt ex",
        ),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();

    let result = apply(&mut state, Action::UseAbility { pokemon: active });
    assert!(result.is_err(), "no Benched Darkness Pokemon to switch in");
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

// --- Beyond the map: a standing Ability read directly, never used as an action ---

#[test]
fn skyliner_removes_retreat_cost_from_every_own_basic_in_play() {
    let ability = Ability {
        name: "Skyliner",
        effect: sim::card::AbilityEffect::PassiveOwnBasicPokemonHaveNoRetreatCost,
    };
    let (mut state, carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    assert_eq!(state.effective_retreat_cost(active), 0, "the carrier itself is a Basic");

    let bench_card = deal_new_card(&mut state, player, carrier_def);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);
    assert_eq!(state.effective_retreat_cost(bench_mon), 0, "a Benched Basic too");

    // Never offered as a standing choice.
    let actions = legal_actions(&state);
    assert!(!actions.contains(&Action::UseAbility { pokemon: active }));
}

#[test]
fn skyliner_does_not_touch_a_stage_1_or_the_opponents_pokemon() {
    let ability = Ability {
        name: "Skyliner",
        effect: sim::card::AbilityEffect::PassiveOwnBasicPokemonHaveNoRetreatCost,
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let opponent = player.opponent();
    let opponent_active = state.player(opponent).active.unwrap();
    assert_eq!(state.effective_retreat_cost(opponent_active), 1, "not this player's own Pokemon");

    let evolution_def = state.db.add(sim::card::CardDef::Pokemon(sim::card::Pokemon {
        print_id: "test-stage-1",
        name: "Evolvemon",
        hp: 200,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 2,
        prizes: 1,
        stage: Stage::Stage1,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        markers: Vec::new(),
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let evolution_card = deal_new_card(&mut state, player, evolution_def);
    let evolution = state.put_into_play(player, evolution_card);
    state.players[player.index()].bench.push(evolution);
    assert_eq!(state.effective_retreat_cost(evolution), 2, "not a Basic Pokemon");
}

#[test]
fn latias_ex_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Latias ex" && c.playable.is_some()),
        "at least one Latias ex print should play"
    );
}

// --- Beyond the map: move damage counters from an own Pokemon to the opponent's ---

#[test]
fn moves_up_to_the_limit_of_damage_counters_to_the_opponent() {
    let ability = Ability {
        name: "Adrena-Brain",
        effect: sim::card::AbilityEffect::OncePerTurnIfEnergyOfTypeAttachedMayMoveDamageCountersToOpponent(
            Type::Darkness,
            3,
        ),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();

    let dark_energy_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-dark-energy",
        name: "Darkness Energy",
        kind: Type::Darkness,
        effect: None,
    }));
    let dark_energy = deal_new_card(&mut state, player, dark_energy_def);
    state.pokemon[active.index()].attached.push(dark_energy);
    state.pokemon[active.index()].damage = 50;

    let opponent = player.opponent();
    let opponent_active = state.player(opponent).active.unwrap();

    let result = apply(&mut state, Action::UseAbility { pokemon: active });
    assert!(result.is_ok(), "carries Darkness Energy and has an own damaged Pokemon");
    assert!(matches!(state.phase, Phase::MovingDamageCountersFromOwnToOpponent { .. }));

    let actions = legal_actions(&state);
    assert!(actions.contains(&Action::MoveDamageCountersFromOwnToOpponent {
        source: active,
        target: opponent_active,
        count: 30,
    }));
    assert!(
        !actions.contains(&Action::MoveDamageCountersFromOwnToOpponent {
            source: active,
            target: opponent_active,
            count: 40,
        }),
        "at most 3 counters, the Ability's own limit"
    );

    apply(
        &mut state,
        Action::MoveDamageCountersFromOwnToOpponent { source: active, target: opponent_active, count: 30 },
    )
    .unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert_eq!(state.pokemon(active).damage, 20);
    assert_eq!(state.pokemon(opponent_active).damage, 30);
}

#[test]
fn battle_cage_lets_adrena_brains_counters_vanish_rather_than_land_on_the_bench() {
    let ability = Ability {
        name: "Adrena-Brain",
        effect: sim::card::AbilityEffect::OncePerTurnIfEnergyOfTypeAttachedMayMoveDamageCountersToOpponent(
            Type::Darkness,
            3,
        ),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();

    let cage = state.db.add(CardDef::Trainer(Trainer {
        print_id: "test-battle-cage",
        name: "Battle Cage",
        kind: TrainerKind::Stadium,
        requirement: None,
        effect: TrainerEffect::PreventsDamageCountersOnBench,
    }));
    let cage_card = deal_new_card(&mut state, player, cage);
    state.stadium = Some((player, cage_card));

    let dark_energy_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-dark-energy-cage",
        name: "Darkness Energy",
        kind: Type::Darkness,
        effect: None,
    }));
    let dark_energy = deal_new_card(&mut state, player, dark_energy_def);
    state.pokemon[active.index()].attached.push(dark_energy);
    state.pokemon[active.index()].damage = 50;

    let opponent = player.opponent();
    let bench_def = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-opponent-bench",
        name: "Benchmon",
        hp: 100,
        kind: Type::Colorless,
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
    let bench_card = deal_new_card(&mut state, opponent, bench_def);
    let opponent_bench = state.put_into_play(opponent, bench_card);
    state.players[opponent.index()].bench.push(opponent_bench);

    apply(&mut state, Action::UseAbility { pokemon: active }).unwrap();
    assert!(matches!(state.phase, Phase::MovingDamageCountersFromOwnToOpponent { .. }));
    assert!(
        legal_actions(&state).contains(&Action::MoveDamageCountersFromOwnToOpponent {
            source: active,
            target: opponent_bench,
            count: 30,
        }),
        "the move is still offered even though Battle Cage will stop it landing"
    );

    apply(
        &mut state,
        Action::MoveDamageCountersFromOwnToOpponent {
            source: active,
            target: opponent_bench,
            count: 30,
        },
    )
    .unwrap();

    assert_eq!(state.pokemon(active).damage, 20, "the counters still leave the source");
    assert_eq!(state.pokemon(opponent_bench).damage, 0, "Battle Cage stops them landing on the Bench");
}

#[test]
fn adrena_brain_not_offered_without_darkness_energy_or_own_damage() {
    let ability = Ability {
        name: "Adrena-Brain",
        effect: sim::card::AbilityEffect::OncePerTurnIfEnergyOfTypeAttachedMayMoveDamageCountersToOpponent(
            Type::Darkness,
            3,
        ),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();

    let result = apply(&mut state, Action::UseAbility { pokemon: active });
    assert!(result.is_err(), "no Darkness Energy attached and no own damage");
}

#[test]
fn munkidori_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Munkidori" && c.playable.is_some()),
        "at least one Munkidori print should play"
    );
}

// --- Beyond the map: peek at the top of the library, take one, bury the rest ---

#[test]
fn takes_one_of_the_top_cards_seen_and_buries_the_rest() {
    let ability = Ability {
        name: "Recon Directive",
        effect: sim::card::AbilityEffect::OncePerTurnMayLookAtTopCardsTakeOneRestToBottom(2),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();

    let library_before = state.player(player).library.clone();
    let top = *library_before.last().unwrap();
    let second_from_top = library_before[library_before.len() - 2];
    let bottom_before = library_before[0];

    apply(&mut state, Action::UseAbility { pokemon: active }).unwrap();
    assert!(matches!(state.phase, Phase::LookingAtTopCardsToTakeOne { .. }));

    let actions = legal_actions(&state);
    assert!(actions.contains(&Action::TakeCardFromTopPeek { card: top }));
    assert!(actions.contains(&Action::TakeCardFromTopPeek { card: second_from_top }));

    apply(&mut state, Action::TakeCardFromTopPeek { card: top }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).hand.contains(&top));
    assert_eq!(state.player(player).library.len(), library_before.len() - 1);
    assert_eq!(
        state.player(player).library[0], second_from_top,
        "the card not taken goes to the bottom"
    );
    assert_eq!(state.player(player).library[1], bottom_before, "the old bottom shifts up");
}

#[test]
fn recon_directive_not_offered_with_an_empty_library() {
    let ability = Ability {
        name: "Recon Directive",
        effect: sim::card::AbilityEffect::OncePerTurnMayLookAtTopCardsTakeOneRestToBottom(2),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    state.players[player.index()].library.clear();

    let result = apply(&mut state, Action::UseAbility { pokemon: active });
    assert!(result.is_err(), "an empty library has nothing to peek at");
}

#[test]
fn drakloak_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Drakloak" && c.playable.is_some()),
        "at least one Drakloak print should play"
    );
}

// --- Beyond the map: peek at the top of the library, attach found Energy one card at a time ---

#[test]
fn attaches_found_energy_and_buries_everything_else() {
    let ability = Ability {
        name: "Metal Maker",
        effect: sim::card::AbilityEffect::OncePerTurnMayLookAtTopCardsAttachFoundBasicEnergyOfType(
            2,
            Type::Metal,
        ),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();

    let metal_energy_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-metal-energy",
        name: "Metal Energy",
        kind: Type::Metal,
        effect: None,
    }));
    let metal_energy = deal_new_card(&mut state, player, metal_energy_def);
    let library_len_before = state.player(player).library.len();
    let other_top_card = *state.player(player).library.last().unwrap();
    state.players[player.index()].library.push(metal_energy);

    apply(&mut state, Action::UseAbility { pokemon: active }).unwrap();
    assert!(matches!(state.phase, Phase::ResolvingEnergyFoundInTopPeek { .. }));

    let actions = legal_actions(&state);
    assert!(actions.contains(&Action::AttachFoundEnergyTo { card: metal_energy, target: active }));
    assert!(actions.contains(&Action::PutFoundCardOnBottom { card: metal_energy }));
    assert!(actions.contains(&Action::PutFoundCardOnBottom { card: other_top_card }));
    assert!(!actions.contains(&Action::AttachFoundEnergyTo { card: other_top_card, target: active }));

    apply(&mut state, Action::AttachFoundEnergyTo { card: metal_energy, target: active }).unwrap();
    assert!(matches!(state.phase, Phase::ResolvingEnergyFoundInTopPeek { .. }), "one card left to resolve");

    apply(&mut state, Action::PutFoundCardOnBottom { card: other_top_card }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.pokemon(active).attached.contains(&metal_energy));
    assert_eq!(state.player(player).library[0], other_top_card, "buried at the bottom");
    assert_eq!(state.player(player).library.len(), library_len_before);
}

#[test]
fn metal_maker_not_offered_with_an_empty_library() {
    let ability = Ability {
        name: "Metal Maker",
        effect: sim::card::AbilityEffect::OncePerTurnMayLookAtTopCardsAttachFoundBasicEnergyOfType(
            4,
            Type::Metal,
        ),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    state.players[player.index()].library.clear();

    let result = apply(&mut state, Action::UseAbility { pokemon: active });
    assert!(result.is_err(), "an empty library has nothing to peek at");
}

#[test]
fn metang_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Metang" && c.playable.is_some()),
        "at least one Metang print should play"
    );
}

// --- Beyond the map: a standing Ability that blocks an ex attacker's damage ---

#[test]
fn takes_no_damage_from_an_ex_attacker() {
    let ability = Ability {
        name: "Mysterious Rock Inn",
        effect: sim::card::AbilityEffect::PassiveImmuneToDamageFromOpponentEx,
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let opponent = player.opponent();
    let carrier = state.player(player).active.unwrap();

    let ex_attacker_def = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-ex-attacker",
        name: "Attackmon ex",
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
            base_damage: 50,
            inflicts: None,
            effect: None,
        }],
    }));
    let ex_card = deal_new_card(&mut state, opponent, ex_attacker_def);
    let ex_attacker = state.put_into_play(opponent, ex_card);
    state.players[opponent.index()].active = Some(ex_attacker);

    let energy_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-colorless-energy-ex-atk",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    let energy = deal_new_card(&mut state, opponent, energy_def);
    state.pokemon[ex_attacker.index()].attached.push(energy);

    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    let attack = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("the ex attacker is paid for");
    apply(&mut state, attack).unwrap();

    assert_eq!(state.pokemon(carrier).damage, 0, "Mysterious Rock Inn blocks an ex attacker outright");
}

#[test]
fn takes_the_usual_damage_from_a_non_ex_attacker() {
    let ability = Ability {
        name: "Mysterious Rock Inn",
        effect: sim::card::AbilityEffect::PassiveImmuneToDamageFromOpponentEx,
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let opponent = player.opponent();
    let carrier = state.player(player).active.unwrap();
    let opponent_active = state.player(opponent).active.unwrap();

    let energy_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-colorless-energy-non-ex",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));
    let energy = deal_new_card(&mut state, opponent, energy_def);
    state.pokemon[opponent_active.index()].attached.push(energy);

    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    let attack = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("the plain defender is paid for");
    apply(&mut state, attack).unwrap();

    assert_eq!(state.pokemon(carrier).damage, 10, "a plain (non-ex) attacker still deals its damage");
}

#[test]
fn crustle_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Crustle" && c.playable.is_some()),
        "at least one Crustle print should play"
    );
}

// --- Beyond the map: peek at the top of the library for a Supporter, shuffle back ---

#[test]
fn takes_a_supporter_seen_at_the_top_and_shuffles_the_rest_back() {
    let ability = Ability {
        name: "Attract Customers",
        effect: sim::card::AbilityEffect::OncePerTurnWhileActiveMayLookAtTopCardsTakeASupporter(6),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();

    let supporter_def = state.db.add(CardDef::Trainer(sim::card::Trainer {
        print_id: "test-supporter",
        name: "Test Supporter",
        kind: sim::card::TrainerKind::Supporter,
        requirement: None,
        effect: sim::card::TrainerEffect::MoveAttachedEnergy,
    }));
    let supporter = deal_new_card(&mut state, player, supporter_def);
    let library_len_before = state.player(player).library.len();
    state.players[player.index()].library.push(supporter);

    apply(&mut state, Action::UseAbility { pokemon: active }).unwrap();
    assert!(matches!(state.phase, Phase::LookingAtTopCardsForSupporter { .. }));

    let actions = legal_actions(&state);
    assert!(actions.contains(&Action::TakeSupporterFromTopPeek { card: supporter }));
    assert!(actions.contains(&Action::DeclineTopPeekSupporter));

    apply(&mut state, Action::TakeSupporterFromTopPeek { card: supporter }).unwrap();

    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).hand.contains(&supporter));
    assert_eq!(state.player(player).library.len(), library_len_before);
}

#[test]
fn attract_customers_only_works_from_the_active_spot() {
    let ability = Ability {
        name: "Attract Customers",
        effect: sim::card::AbilityEffect::OncePerTurnWhileActiveMayLookAtTopCardsTakeASupporter(6),
    };
    let (mut state, carrier_def) = game(ability, 3);
    let player = state.current;

    let bench_card = deal_new_card(&mut state, player, carrier_def);
    let bench_mon = state.put_into_play(player, bench_card);
    state.players[player.index()].bench.push(bench_mon);

    let result = apply(&mut state, Action::UseAbility { pokemon: bench_mon });
    assert!(result.is_err(), "Attract Customers only works from the Active Spot");
}

#[test]
fn attract_customers_not_offered_with_no_supporter_in_the_peek() {
    let ability = Ability {
        name: "Attract Customers",
        effect: sim::card::AbilityEffect::OncePerTurnWhileActiveMayLookAtTopCardsTakeASupporter(6),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    state.players[player.index()].library.clear();

    let result = apply(&mut state, Action::UseAbility { pokemon: active });
    assert!(result.is_err(), "no Supporter in the deck at all, let alone the peek");
}

#[test]
fn tatsugiri_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Tatsugiri" && c.playable.is_some()),
        "at least one Tatsugiri print should play"
    );
}

// --- Beyond the map: a standing Ability that blocks damage counter movement everywhere ---

#[test]
fn watchful_eye_blocks_damage_counter_movement_even_from_the_opponents_side() {
    let ability = Ability {
        name: "Adrena-Brain",
        effect: sim::card::AbilityEffect::OncePerTurnIfEnergyOfTypeAttachedMayMoveDamageCountersToOpponent(
            Type::Darkness,
            3,
        ),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let opponent = player.opponent();
    let active = state.player(player).active.unwrap();

    let dark_energy_def = state.db.add(CardDef::Energy(Energy {
        print_id: "test-dark-energy-watchful",
        name: "Darkness Energy",
        kind: Type::Darkness,
        effect: None,
    }));
    let dark_energy = deal_new_card(&mut state, player, dark_energy_def);
    state.pokemon[active.index()].attached.push(dark_energy);
    state.pokemon[active.index()].damage = 50;

    let watcher_def = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-watcher",
        name: "Watchmon",
        hp: 100,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: Some(Ability {
            name: "Watchful Eye",
            effect: sim::card::AbilityEffect::PassiveBlocksDamageCounterMovement,
        }),
        attacks: vec![Attack {
            name: "Bite",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let watcher_card = deal_new_card(&mut state, opponent, watcher_def);
    let watcher = state.put_into_play(opponent, watcher_card);
    state.players[opponent.index()].bench.push(watcher);

    let result = apply(&mut state, Action::UseAbility { pokemon: active });
    assert!(result.is_err(), "Watchful Eye blocks damage counter movement on either side");
}

#[test]
fn patrat_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Patrat" && c.playable.is_some()),
        "at least one Patrat print should play"
    );
}

// --- Beyond the map: a standing Ability that disables self-knockout Abilities everywhere ---

#[test]
fn damp_disables_a_self_knockout_ability_even_from_the_opponents_side() {
    let ability = Ability {
        name: "Cursed Blast",
        effect: sim::card::AbilityEffect::OncePerTurnMayDamageOpponentThenKnockOutSelf(5),
    };
    let (mut state, _carrier_def) = game(ability, 3);
    let player = state.current;
    let opponent = player.opponent();
    let active = state.player(player).active.unwrap();

    let damp_def = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-damp",
        name: "Psyduck",
        hp: 60,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: Some(Ability { name: "Damp", effect: sim::card::AbilityEffect::PassiveDisablesSelfKnockOutAbilities }),
        attacks: vec![Attack {
            name: "Ram",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let damp_card = deal_new_card(&mut state, opponent, damp_def);
    let damp_mon = state.put_into_play(opponent, damp_card);
    state.players[opponent.index()].bench.push(damp_mon);

    let result = apply(&mut state, Action::UseAbility { pokemon: active });
    assert!(result.is_err(), "Damp disables Cursed Blast on either side");
}

#[test]
fn psyduck_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Psyduck" && c.playable.is_some()),
        "at least one Psyduck print should play"
    );
}

// --- Beyond the map: an Ability that overrides an opponent type's Weakness ---

/// A game where the first player's Active carries `Fairy Zone`, and
/// the second player's Active prints a Dragon Weakness — so Fairy
/// Zone's override (Dragon to Psychic) is observable against a
/// Psychic attacker.
fn fairy_zone_game(carrier_kind: Type, seed: u64) -> GameState {
    let mut db = CardDb::new();
    let carrier = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-fairy-zone-carrier",
        name: "Lillie's Clefairy ex",
        hp: 190,
        kind: carrier_kind,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 2,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: Some(Ability {
            name: "Fairy Zone",
            effect: sim::card::AbilityEffect::PassiveSetsOpponentTypeWeaknessTo(
                Type::Dragon,
                Type::Psychic,
            ),
        }),
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 50,
            inflicts: None,
            effect: None,
        }],
    }));
    let defender_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-fairy-zone-defender",
        name: "Dragonmon",
        hp: 200,
        kind: Type::Dragon,
        weakness: Some(Type::Dragon),
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
        print_id: "test-fairy-zone-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));

    let mut carrier_deck = vec![carrier; 4];
    while carrier_deck.len() < 60 {
        carrier_deck.push(energy);
    }
    let mut defender_deck = vec![defender_mon; 4];
    while defender_deck.len() < 60 {
        defender_deck.push(energy);
    }
    let mut state =
        GameState::new(db, [carrier_deck, defender_deck], Box::new(SeededRng::new(seed)));
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

fn pay_and_attack_fairy_zone(state: &mut GameState) {
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let cost_len = state.pokemon_def(active).attacks[0].cost.len();
    for _ in 0..cost_len {
        let side = state.player(player);
        let card = side
            .hand
            .iter()
            .chain(side.library.iter())
            .find(|c| state.def_of(**c).is_energy())
            .copied()
            .expect("the deck holds Energy");
        state.remove_from_hand(player, card);
        state.players[player.index()].library.retain(|c| *c != card);
        state.pokemon[active.index()].attached.push(card);
    }
    let attack = legal_actions(state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("the attacker is paid for");
    apply(state, attack).unwrap();
}

#[test]
fn fairy_zone_makes_a_dragon_weak_to_psychic() {
    let mut state = fairy_zone_game(Type::Psychic, 3);
    let player = state.current;
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack_fairy_zone(&mut state);

    assert_eq!(
        state.pokemon(defender).damage, 100,
        "Fairy Zone overrides the Dragon Weakness to Psychic, so the Psychic attacker doubles it"
    );
}

#[test]
fn without_fairy_zone_a_dragon_weakness_ignores_a_psychic_attacker() {
    // The carrier's own type still matters for the attack itself, but
    // giving it no Ability at all removes Fairy Zone's override —
    // Dragon Weakness (the printed value) never matches a Psychic
    // attacker, so no doubling happens either way.
    let mut db = CardDb::new();
    let attacker_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-no-fairy-zone-attacker",
        name: "Attackmon",
        hp: 190,
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
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 50,
            inflicts: None,
            effect: None,
        }],
    }));
    let defender_mon = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-no-fairy-zone-defender",
        name: "Dragonmon",
        hp: 200,
        kind: Type::Dragon,
        weakness: Some(Type::Dragon),
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
        print_id: "test-no-fairy-zone-energy",
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
    let mut state =
        GameState::new(db, [attacker_deck, defender_deck], Box::new(SeededRng::new(3)));
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
    let opponent = player.opponent();
    let defender = state.player(opponent).active.unwrap();

    pay_and_attack_fairy_zone(&mut state);

    assert_eq!(state.pokemon(defender).damage, 50, "no Fairy Zone in play, so no override applies");
}

#[test]
fn lillies_clefairy_ex_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    assert!(
        import.cards.iter().any(|c| c.name == "Lillie's Clefairy ex" && c.playable.is_some()),
        "at least one Lillie's Clefairy ex print should play"
    );
}

#[test]
fn fairy_zone_applies_from_the_bench_when_a_different_psychic_pokemon_attacks() {
    // Fairy Zone reads every Pokemon on the carrier's own side, not
    // only the one attacking — so a second Psychic Pokemon (paired
    // here with Slowking, since it prints the same Psychic type)
    // gets the same Dragon-to-Psychic Weakness override even while
    // Lillie's Clefairy ex itself sits on the Bench, not attacking.
    let mut state = fairy_zone_game(Type::Psychic, 3);
    let player = state.current;
    let opponent = player.opponent();
    let carrier = state.player(player).active.unwrap();

    let slowking_def = state.db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-fairy-zone-slowking",
        name: "Slowking",
        hp: 120,
        kind: Type::Psychic,
        weakness: None,
        resistance: None,
        retreat_cost: 3,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 50,
            inflicts: None,
            effect: None,
        }],
    }));
    let slowking_card = deal_new_card(&mut state, player, slowking_def);
    let slowking = state.put_into_play(player, slowking_card);

    // Bench the carrier, promote Slowking: the carrier's own Ability
    // still reads from the Bench, but it is no longer the attacker.
    state.players[player.index()].bench.push(carrier);
    state.players[player.index()].active = Some(slowking);

    let defender = state.player(opponent).active.unwrap();
    pay_and_attack_fairy_zone(&mut state);

    assert_eq!(
        state.pokemon(defender).damage, 100,
        "Fairy Zone still overrides the Weakness from the Bench, even though Slowking (not the carrier) is attacking"
    );
}
