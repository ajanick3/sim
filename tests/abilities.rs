//! Milestone 8: the first Abilities. One fixture, shared across this
//! milestone's tickets, the same shape every earlier milestone's own
//! fixture already took.

use sim::action::{Action, legal_actions};
use sim::card::{Ability, Attack, CardDb, CardDef, Energy, Pokemon, Stage, Type};
use sim::engine::apply;
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

/// A game where the first player's Active carries the Ability under
/// test, and the second player's Active is a plain punching bag.
fn game(ability: Ability, seed: u64) -> (GameState, sim::ids::CardDefId) {
    let mut db = CardDb::new();
    let carrier = db.add(CardDef::Pokemon(Pokemon {
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
