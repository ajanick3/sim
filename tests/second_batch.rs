//! The second batch of Trainers: the nine most-played cards the engine
//! refused after milestone 3, and the primitives each one needs.

use sim::action::{Action, legal_actions};
use sim::card::{
    Attack, CardDb, CardDef, CardFilter, Destination, Energy, Pokemon, Requirement, Slot, Stage,
    TargetFilter, Trainer, TrainerEffect, TrainerKind, Type, Zone,
};
use sim::engine::apply;
use sim::ids::{CardDefId, CardId, PlayerId, PokemonId};
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

/// A pool wide enough to tell the new filters apart: an ordinary Basic, a
/// Basic worth two Prizes, a small Basic, and a Stage 1.
struct Set {
    db: CardDb,
    mon: CardDefId,
    mon_ex: CardDefId,
    small: CardDefId,
    stage1: CardDefId,
    stage2: CardDefId,
    energy: CardDefId,
    fire_energy: CardDefId,
    cyrano: CardDefId,
}

fn basic(
    db: &mut CardDb,
    print_id: &'static str,
    name: &'static str,
    hp: u32,
    prizes: u32,
    evolve_from: Option<&'static str>,
) -> CardDefId {
    db.add(CardDef::Pokemon(Pokemon {
        print_id,
        name,
        hp,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes,
        stage: match evolve_from {
            None => Stage::Basic,
            Some(_) => Stage::Stage1,
        },
        evolve_from,
        evolves_from_basic: None,
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
        }],
    }))
}

fn build() -> Set {
    let mut db = CardDb::new();
    let mon = basic(&mut db, "test-mon", "Testmon", 100, 1, None);
    let mon_ex = basic(&mut db, "test-mon-ex", "Testmon ex", 200, 2, None);
    let small = basic(&mut db, "test-small", "Smallmon", 70, 1, None);
    let stage1 = basic(&mut db, "test-stage1", "Bigmon", 120, 1, Some("Smallmon"));
    let stage2 = db.add(CardDef::Pokemon(Pokemon {
        print_id: "test-stage2",
        name: "Hugemon",
        hp: 180,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 2,
        prizes: 1,
        stage: Stage::Stage2,
        evolve_from: Some("Bigmon"),
        // The chain this ticket walks: Hugemon evolves from Bigmon, which
        // evolves from Smallmon, so Rare Candy may put Hugemon straight onto
        // a Smallmon in play.
        evolves_from_basic: Some("Smallmon"),
        attacks: vec![Attack {
            name: "Slam",
            cost: vec![Type::Colorless, Type::Colorless],
            base_damage: 60,
            inflicts: None,
        }],
    }));
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
    }));
    let fire_energy = db.add(CardDef::Energy(Energy {
        print_id: "test-fire-energy",
        name: "Fire Energy",
        kind: Type::Fire,
    }));
    let cyrano = db.add(CardDef::Trainer(Trainer {
        print_id: "test-cyrano",
        name: "Cyrano",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![Slot {
                filter: CardFilter::PokemonEx,
                to: Destination::Zone(Zone::Hand),
                limit: 3,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    }));
    Set {
        db,
        mon,
        mon_ex,
        small,
        stage1,
        stage2,
        energy,
        fire_energy,
        cyrano,
    }
}

/// A deck holding a few of each card, then Energy to sixty. A handful of
/// Fire Energy sits among the Colorless, so a card that wants two different
/// types has two to choose between.
fn deck(set: &Set, extra: CardDefId) -> Vec<CardDefId> {
    let mut decklist = vec![set.mon; 6];
    decklist.extend([set.mon_ex; 4]);
    decklist.extend([set.small; 4]);
    decklist.extend([set.stage1; 4]);
    decklist.extend([set.stage2; 2]);
    decklist.extend([set.fire_energy; 4]);
    // A second Trainer, distinct from whatever `extra` is, so a search for
    // "any Trainer" has something left to find once `extra` is played.
    decklist.extend([set.cyrano; 2]);
    decklist.push(extra);
    while decklist.len() < 60 {
        decklist.push(set.energy);
    }
    decklist
}

/// Play to the second player's first Main phase, so no turn-one rule is in
/// the way of playing a card.
fn game(set: &Set, extra: CardDefId, seed: u64) -> GameState {
    let decklist = deck(set, extra);
    let mut state = GameState::new(
        set.db.clone(),
        [decklist.clone(), decklist],
        Box::new(SeededRng::new(seed)),
    );
    for _ in 0..2 {
        while state.phase != Phase::Main && !state.is_over() {
            let first = legal_actions(&state)[0];
            apply(&mut state, first).unwrap();
        }
        if state.turn_number > 1 {
            break;
        }
        apply(&mut state, Action::EndTurn).unwrap();
    }
    state
}

/// Put a copy of `def` in this player's hand, taking it from the deck.
fn ensure_in_hand(state: &mut GameState, player: PlayerId, def: CardDefId) -> CardId {
    if let Some(card) = state
        .player(player)
        .hand
        .iter()
        .find(|c| state.cards[c.index()].def == def)
    {
        return *card;
    }
    // A single copy can be dealt into the Prizes, so every face-down zone is
    // searched, not only the deck.
    let side = state.player(player);
    let card = *side
        .library
        .iter()
        .chain(side.prizes.iter())
        .chain(side.discard.iter())
        .find(|c| state.cards[c.index()].def == def)
        .expect("the deal put this card somewhere face down");
    let side = &mut state.players[player.index()];
    side.library.retain(|c| *c != card);
    side.prizes.retain(|c| *c != card);
    side.discard.retain(|c| *c != card);
    side.hand.push(card);
    card
}

/// Every card the current `Deciding` phase offers.
fn offered(state: &GameState) -> Vec<CardId> {
    legal_actions(state)
        .into_iter()
        .filter_map(|a| match a {
            Action::TakeCard { card } => Some(card),
            _ => None,
        })
        .collect()
}

// --- Ticket 02: a filter on stage and HP ---

#[test]
fn the_ex_filter_offers_only_a_pokemon_worth_more_than_one_prize() {
    let set = build();
    let mut state = game(&set, set.cyrano, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, set.cyrano);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let choices = offered(&state);
    assert!(!choices.is_empty(), "the deck holds four Testmon ex");
    for card in &choices {
        let def = state.def_of(*card);
        let pokemon = def.as_pokemon().expect("only a Pokémon is offered");
        assert!(
            pokemon.prizes > 1,
            "{} is not a Pokémon ex and must not be offered",
            pokemon.name
        );
    }
}

#[test]
fn cyrano_takes_up_to_three_and_may_stop_early() {
    let set = build();
    let mut state = game(&set, set.cyrano, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, set.cyrano);
    let before = state.player(player).hand.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    for _ in 0..3 {
        let take = offered(&state)[0];
        apply(&mut state, Action::TakeCard { card: take }).unwrap();
    }
    assert!(
        offered(&state).is_empty(),
        "three is the limit the card prints"
    );
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.phase, Phase::Main);
    // Three taken, and the Supporter itself left the hand.
    assert_eq!(state.player(player).hand.len(), before + 2);
}

#[test]
fn the_small_basic_filter_reads_both_the_stage_and_the_hp() {
    let set = build();
    let mut db = set.db.clone();
    let poffin = db.add(CardDef::Trainer(Trainer {
        print_id: "test-poffin",
        name: "Poffin",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![Slot {
                filter: CardFilter::BasicPokemonWithHpAtMost(70),
                to: Destination::Zone(Zone::Hand),
                limit: 2,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    }));
    let set = Set { db, ..set };
    let mut state = game(&set, poffin, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, poffin);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let choices = offered(&state);
    assert!(!choices.is_empty(), "the deck holds four Smallmon");
    for card in &choices {
        let pokemon = state
            .def_of(*card)
            .as_pokemon()
            .expect("only a Pokémon is offered")
            .clone();
        assert!(pokemon.hp <= 70, "{} has too much HP", pokemon.name);
        assert!(
            pokemon.evolve_from.is_none(),
            "{} is not a Basic",
            pokemon.name
        );
    }
}

// --- Ticket 01: a search that puts a Pokémon into play ---

/// `Buddy-Buddy Poffin` as printed: up to 2 small Basics, deck to Bench.
fn with_poffin(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let poffin = db.add(CardDef::Trainer(Trainer {
        print_id: "test-poffin",
        name: "Buddy-Buddy Poffin",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![Slot {
                filter: CardFilter::BasicPokemonWithHpAtMost(70),
                to: Destination::Bench,
                limit: 2,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    }));
    (Set { db, ..set }, poffin)
}

#[test]
fn a_search_can_put_a_pokemon_into_play() {
    let (set, poffin) = with_poffin(build());
    let mut state = game(&set, poffin, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, poffin);
    let bench_before = state.player(player).bench.len();
    let hand_before = state.player(player).hand.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let take = offered(&state)[0];
    apply(&mut state, Action::TakeCard { card: take }).unwrap();

    assert_eq!(
        state.player(player).bench.len(),
        bench_before + 1,
        "the searched card is a Pokémon in play, not a card in a zone"
    );
    let benched = *state.player(player).bench.last().unwrap();
    assert_eq!(
        state.pokemon(benched).cards,
        vec![take],
        "the Pokémon in play is the card that was taken"
    );
    assert_eq!(
        state.pokemon(benched).played_on_turn,
        state.turn_number,
        "it came into play this turn, the same as a Basic from hand"
    );
    assert!(
        !state.player(player).hand.contains(&take),
        "and it never passed through the hand"
    );
    // The Item itself left the hand; nothing else joined it.
    assert_eq!(state.player(player).hand.len(), hand_before - 1);
    assert!(
        !state.player(player).library.contains(&take),
        "the card left the deck"
    );
}

#[test]
fn a_full_bench_offers_nothing_to_take() {
    let (set, poffin) = with_poffin(build());
    let mut state = game(&set, poffin, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, poffin);

    // Fill the Bench from the deck, the way an ordinary turn would.
    while state.player(player).bench.len() < sim::state::BENCH_LIMIT {
        let basic = *state
            .player(player)
            .library
            .iter()
            .find(|c| state.def_of(**c).is_basic_pokemon())
            .expect("the deck holds Basics");
        state.players[player.index()]
            .library
            .retain(|c| *c != basic);
        let pokemon = state.put_into_play(player, basic);
        state.players[player.index()].bench.push(pokemon);
    }

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    assert!(
        offered(&state).is_empty(),
        "there is nowhere to put a Pokémon"
    );
    assert!(
        legal_actions(&state).contains(&Action::FinishDeciding),
        "and the choice can still be ended"
    );
    apply(&mut state, Action::FinishDeciding).unwrap();
    assert_eq!(state.phase, Phase::Main);
}

#[test]
fn a_bench_that_fills_part_way_through_ends_the_choice() {
    let (set, poffin) = with_poffin(build());
    let mut state = game(&set, poffin, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, poffin);

    // One space left, and the card offers two.
    while state.player(player).bench.len() < sim::state::BENCH_LIMIT - 1 {
        let basic = *state
            .player(player)
            .library
            .iter()
            .find(|c| state.def_of(**c).is_basic_pokemon())
            .expect("the deck holds Basics");
        state.players[player.index()]
            .library
            .retain(|c| *c != basic);
        let pokemon = state.put_into_play(player, basic);
        state.players[player.index()].bench.push(pokemon);
    }

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let take = offered(&state)[0];
    apply(&mut state, Action::TakeCard { card: take }).unwrap();
    assert_eq!(state.player(player).bench.len(), sim::state::BENCH_LIMIT);
    assert!(
        offered(&state).is_empty(),
        "the second card has nowhere to go"
    );
}

#[test]
fn a_search_that_ends_in_the_library_still_shuffles() {
    // The destination is a value now. The rule that a card put back into the
    // deck shuffles it must still read the value and not a `Zone` field that
    // no longer exists.
    let set = build();
    let mut db = set.db.clone();
    let ash = db.add(CardDef::Trainer(Trainer {
        print_id: "test-ash",
        name: "Sacred Ash",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Discard,
            slots: vec![Slot {
                filter: CardFilter::AnyPokemon,
                to: Destination::Zone(Zone::Library),
                limit: 5,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    }));
    let set = Set { db, ..set };
    let mut state = game(&set, ash, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, ash);

    // Put a Pokémon in the discard for it to find.
    let mon = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.def_of(**c).as_pokemon().is_some())
        .unwrap();
    state.players[player.index()].library.retain(|c| *c != mon);
    state.players[player.index()].discard.push(mon);
    let order_before = state.player(player).library.clone();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::TakeCard { card: mon }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert!(state.player(player).library.contains(&mon));
    assert_ne!(
        state.player(player).library,
        order_before,
        "the deck is shuffled once the choice ends"
    );
}

// --- Ticket 03: a requirement paid to play a card ---

/// `Ultra Ball` and `Special Red Card` as printed.
fn with_requirements(set: Set) -> (Set, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let ultra_ball = db.add(CardDef::Trainer(Trainer {
        print_id: "test-ultra-ball",
        name: "Ultra Ball",
        kind: TrainerKind::Item,
        requirement: Some(Requirement::DiscardOtherCardsFromHand(2)),
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![Slot {
                filter: CardFilter::AnyPokemon,
                to: Destination::Zone(Zone::Hand),
                limit: 1,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    }));
    let red_card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-special-red-card",
        name: "Special Red Card",
        kind: TrainerKind::Item,
        requirement: Some(Requirement::OpponentPrizesAtMost(3)),
        effect: TrainerEffect::OpponentHandToBottomThenDraw { count: 3 },
    }));
    (Set { db, ..set }, ultra_ball, red_card)
}

#[test]
fn ultra_ball_cannot_be_played_holding_nothing_else() {
    let (set, ultra_ball, _) = with_requirements(build());
    let mut state = game(&set, ultra_ball, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, ultra_ball);

    // Hold the card and one other: one short of the cost.
    let keep: Vec<CardId> = state
        .player(player)
        .hand
        .iter()
        .filter(|c| **c != card)
        .take(1)
        .copied()
        .collect();
    state.players[player.index()].hand = keep.clone();
    state.players[player.index()].hand.push(card);
    assert!(
        !legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "one other card does not pay for two"
    );

    // A second other card, and it may be played.
    let extra = state.players[player.index()].library.pop().unwrap();
    state.players[player.index()].hand.push(extra);
    assert!(
        legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "two other cards pay for it"
    );
}

#[test]
fn a_cost_is_paid_as_a_phase_before_the_effect_runs() {
    let (set, ultra_ball, _) = with_requirements(build());
    let mut state = game(&set, ultra_ball, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, ultra_ball);
    let hand_before = state.player(player).hand.len();
    let discard_before = state.player(player).discard.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    assert!(
        matches!(state.phase, Phase::Paying { .. }),
        "the cost comes first: {:?}",
        state.phase
    );
    // Every card still in hand may pay, and nothing else is offered.
    let paying = legal_actions(&state);
    assert!(!paying.is_empty());
    for action in &paying {
        assert!(
            matches!(action, Action::PayWithCard { .. }),
            "the cost is the only choice: {action:?}"
        );
    }

    for _ in 0..2 {
        let pay = match legal_actions(&state)[0] {
            Action::PayWithCard { card } => card,
            other => panic!("expected a payment: {other:?}"),
        };
        apply(&mut state, Action::PayWithCard { card: pay }).unwrap();
    }

    assert!(
        matches!(state.phase, Phase::Deciding { .. }),
        "the cost paid, the effect runs: {:?}",
        state.phase
    );
    // The Ultra Ball itself and the two cards that paid for it.
    assert_eq!(state.player(player).discard.len(), discard_before + 3);
    assert_eq!(state.player(player).hand.len(), hand_before - 3);
}

#[test]
fn special_red_card_waits_for_the_opponents_prizes() {
    let (set, _, red_card) = with_requirements(build());
    let mut state = game(&set, red_card, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, red_card);

    assert_eq!(state.player(player.opponent()).prizes.len(), 6);
    assert!(
        !legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "six Prizes is more than three"
    );

    state.players[player.opponent().index()].prizes.truncate(3);
    assert!(
        legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "three Prizes is not more than three"
    );
}

#[test]
fn special_red_card_puts_the_hand_under_the_deck_and_deals_three() {
    let (set, _, red_card) = with_requirements(build());
    let mut state = game(&set, red_card, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, red_card);
    state.players[player.opponent().index()].prizes.truncate(3);

    let opponent = player.opponent();
    let their_hand = state.player(opponent).hand.clone();
    let held = their_hand.len();
    assert!(held > 0, "the opponent holds a hand to lose");
    let library_before = state.player(opponent).library.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert_eq!(state.phase, Phase::Main, "no choice is left in the card");
    assert_eq!(
        state.player(opponent).hand.len(),
        3,
        "they draw three for the hand they gave up"
    );
    assert_eq!(
        state.player(opponent).library.len(),
        library_before + held - 3
    );
    // The bottom of the deck is where a draw reaches last.
    let bottom = &state.player(opponent).library[..held];
    for card in &their_hand {
        assert!(
            bottom.contains(card),
            "every card of the old hand went under the deck"
        );
    }
}

#[test]
fn a_card_with_no_requirement_still_plays() {
    let set = build();
    let mut state = game(&set, set.cyrano, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, set.cyrano);
    assert!(legal_actions(&state).contains(&Action::PlayTrainer { card }));
    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    assert!(
        matches!(state.phase, Phase::Deciding { .. }),
        "no cost stands between the card and its effect"
    );
}

// --- Ticket 04: move an Energy between Pokémon ---

fn with_energy_switch(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let switch = db.add(CardDef::Trainer(Trainer {
        print_id: "test-energy-switch",
        name: "Energy Switch",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::MoveAttachedEnergy,
    }));
    (Set { db, ..set }, switch)
}

/// Bench a Pokémon and give the Active an Energy, so there is something to
/// move and somewhere to move it.
fn board_with_an_energy(state: &mut GameState, player: PlayerId) -> (CardId, PokemonId, PokemonId) {
    let basic = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.def_of(**c).is_basic_pokemon())
        .expect("the deck holds Basics");
    state.players[player.index()]
        .library
        .retain(|c| *c != basic);
    let benched = state.put_into_play(player, basic);
    state.players[player.index()].bench.push(benched);

    let active = state.player(player).active.expect("setup placed an Active");
    let energy = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .expect("the deck is mostly Energy");
    state.players[player.index()]
        .library
        .retain(|c| *c != energy);
    state.pokemon[active.index()].attached.push(energy);
    (energy, active, benched)
}

#[test]
fn an_attached_energy_moves_to_another_pokemon() {
    let (set, switch) = with_energy_switch(build());
    let mut state = game(&set, switch, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, switch);
    let (energy, active, benched) = board_with_an_energy(&mut state, player);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    assert!(
        matches!(state.phase, Phase::MovingEnergy { .. }),
        "the card asks where: {:?}",
        state.phase
    );

    let moves: Vec<Action> = legal_actions(&state)
        .into_iter()
        .filter(|a| matches!(a, Action::MoveEnergy { .. }))
        .collect();
    assert!(
        moves.contains(&Action::MoveEnergy {
            card: energy,
            target: benched
        }),
        "the Active's Energy may go to the Bench"
    );
    assert!(
        !moves.contains(&Action::MoveEnergy {
            card: energy,
            target: active
        }),
        "a Pokémon is not another Pokémon"
    );

    apply(
        &mut state,
        Action::MoveEnergy {
            card: energy,
            target: benched,
        },
    )
    .unwrap();

    assert!(!state.pokemon(active).attached.contains(&energy));
    assert!(state.pokemon(benched).attached.contains(&energy));
    assert_eq!(state.phase, Phase::Main);
}

#[test]
fn only_an_energy_moves_and_only_between_your_own_pokemon() {
    let (set, switch) = with_energy_switch(build());
    let mut state = game(&set, switch, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, switch);
    let (energy, _, _) = board_with_an_energy(&mut state, player);

    // Give the opponent a Pokémon with an Energy of its own.
    let opponent = player.opponent();
    let theirs = state
        .player(opponent)
        .active
        .expect("both players placed an Active");
    let their_energy = *state
        .player(opponent)
        .library
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    state.players[opponent.index()]
        .library
        .retain(|c| *c != their_energy);
    state.pokemon[theirs.index()].attached.push(their_energy);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    for action in legal_actions(&state) {
        let Action::MoveEnergy { card, target } = action else {
            continue;
        };
        assert_eq!(card, energy, "only the player's own Energy moves");
        assert_eq!(
            state.pokemon(target).owner,
            player,
            "and only onto the player's own Pokémon"
        );
    }
    assert!(state.pokemon(theirs).attached.contains(&their_energy));
}

#[test]
fn energy_switch_needs_an_energy_and_somewhere_to_put_it() {
    let (set, switch) = with_energy_switch(build());
    let mut state = game(&set, switch, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, switch);

    state.players[player.index()].bench.clear();
    assert!(
        !legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "nothing is attached, so nothing can move"
    );

    let (_, _, benched) = board_with_an_energy(&mut state, player);
    assert!(
        legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "an Energy and a second Pokémon are enough"
    );

    // Take the second Pokémon away and there is nowhere left to move to.
    state.players[player.index()]
        .bench
        .retain(|p| *p != benched);
    assert!(
        !legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "one Pokémon cannot pass an Energy to itself"
    );
}

// --- Ticket 05, part one: the printed stage, and the shuffle a search owes ---

#[test]
fn a_pokemon_carries_the_stage_it_prints() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let stage_of = |name: &str| {
        import
            .admitted
            .iter()
            .filter_map(|id| import.db.get(*id).as_pokemon())
            .find(|p| p.name == name)
            .map(|p| p.stage)
    };
    assert_eq!(stage_of("Chikorita"), Some(Stage::Basic));
    assert_eq!(stage_of("Bayleef"), Some(Stage::Stage1));
    assert_eq!(stage_of("Ampharos"), Some(Stage::Stage2));
}

#[test]
fn the_stage_and_the_name_it_evolves_from_agree() {
    // A Basic names nothing to evolve from, and every evolution names one.
    // The two facts came from different fields of the artifact, so the
    // engine checks that they tell the same story.
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    for pokemon in import
        .admitted
        .iter()
        .filter_map(|id| import.db.get(*id).as_pokemon())
    {
        assert_eq!(
            pokemon.stage == Stage::Basic,
            pokemon.evolve_from.is_none(),
            "{} is a {:?} and evolves from {:?}",
            pokemon.name,
            pokemon.stage,
            pokemon.evolve_from
        );
    }
}

#[test]
fn the_stage_filters_offer_what_they_name() {
    let set = build();
    let mut db = set.db.clone();
    let hunt = |db: &mut CardDb, id: &'static str, filter| {
        db.add(CardDef::Trainer(Trainer {
            print_id: id,
            name: "Search",
            kind: TrainerKind::Item,
            requirement: None,
            effect: TrainerEffect::Decide {
                from: Zone::Library,
                slots: vec![Slot {
                    filter,
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        }))
    };
    let evolutions = hunt(&mut db, "test-evolutions", CardFilter::EvolutionPokemon);
    let stage_ones = hunt(
        &mut db,
        "test-stage-ones",
        CardFilter::PokemonOfStage(Stage::Stage1),
    );
    let energy = hunt(&mut db, "test-energy-search", CardFilter::BasicEnergy);
    let set = Set { db, ..set };

    for (card_def, check) in [
        (evolutions, "evolution"),
        (stage_ones, "stage 1"),
        (energy, "energy"),
    ] {
        let mut state = game(&set, card_def, 3);
        let player = state.current;
        let card = ensure_in_hand(&mut state, player, card_def);
        apply(&mut state, Action::PlayTrainer { card }).unwrap();
        let choices = offered(&state);
        assert!(!choices.is_empty(), "the deck holds a {check}");
        for taken in &choices {
            let def = state.def_of(*taken);
            match check {
                "evolution" => assert!(
                    def.as_pokemon().is_some_and(|p| p.stage != Stage::Basic),
                    "{} is not an Evolution",
                    def.name()
                ),
                "stage 1" => assert!(
                    def.as_pokemon().is_some_and(|p| p.stage == Stage::Stage1),
                    "{} is not a Stage 1",
                    def.name()
                ),
                _ => assert!(def.is_energy(), "{} is not an Energy", def.name()),
            }
        }
    }
}

#[test]
fn a_deck_search_shuffles_the_deck_when_it_ends() {
    // Every card that searches the deck prints "Then, shuffle your deck."
    // The player saw the whole deck while choosing, so the order they saw
    // must not survive the search.
    let set = build();
    let mut state = game(&set, set.cyrano, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, set.cyrano);
    let before = state.player(player).library.clone();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    let after = state.player(player).library.clone();
    assert_eq!(before.len(), after.len(), "nothing was taken");
    assert_ne!(before, after, "but the order the player saw is gone");
}

// --- Ticket 05, part two: a search of several slots ---

fn slot(filter: CardFilter) -> Slot {
    Slot {
        filter,
        to: Destination::Zone(Zone::Hand),
        limit: 1,
        excludes_type_of_previous: false,
        peek: None,
    }
}

fn with_hilda_and_dawn(set: Set) -> (Set, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let hilda = db.add(CardDef::Trainer(Trainer {
        print_id: "test-hilda",
        name: "Hilda",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![
                slot(CardFilter::EvolutionPokemon),
                slot(CardFilter::BasicEnergy),
            ],
            then: None,
        },
    }));
    let dawn = db.add(CardDef::Trainer(Trainer {
        print_id: "test-dawn",
        name: "Dawn",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![
                slot(CardFilter::PokemonOfStage(Stage::Basic)),
                slot(CardFilter::PokemonOfStage(Stage::Stage1)),
                slot(CardFilter::PokemonOfStage(Stage::Stage2)),
            ],
            then: None,
        },
    }));
    (Set { db, ..set }, hilda, dawn)
}

/// Which slot the search is on, or `None` once it has finished.
fn step_of(state: &GameState) -> Option<u32> {
    match state.phase {
        Phase::Deciding { step, .. } => Some(step),
        _ => None,
    }
}

#[test]
fn hilda_asks_for_one_of_each_in_turn() {
    let (set, hilda, _) = with_hilda_and_dawn(build());
    let mut state = game(&set, hilda, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, hilda);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    assert_eq!(step_of(&state), Some(0));
    for offered in offered(&state) {
        assert!(
            state
                .def_of(offered)
                .as_pokemon()
                .is_some_and(|p| p.stage != Stage::Basic),
            "the first slot wants an Evolution"
        );
    }

    let evolution = offered(&state)[0];
    apply(&mut state, Action::TakeCard { card: evolution }).unwrap();
    assert!(offered(&state).is_empty(), "the slot wanted one card");
    // ADR 0012 keeps the choice to stop with the player, slot by slot.
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(step_of(&state), Some(1), "the search moves on");
    let second = offered(&state);
    assert!(!second.is_empty());
    for offered in second {
        assert!(
            state.def_of(offered).is_energy(),
            "the second slot wants an Energy"
        );
    }

    let energy = offered(&state)[0];
    apply(&mut state, Action::TakeCard { card: energy }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();
    assert_eq!(state.phase, Phase::Main, "two slots, two cards, done");
    assert!(state.player(player).hand.contains(&evolution));
    assert!(state.player(player).hand.contains(&energy));
}

#[test]
fn a_slot_declined_still_leaves_the_next_one() {
    // "Search for an Evolution Pokémon and an Energy card" does not demand
    // that the deck holds both, or that the player wants both.
    let (set, hilda, _) = with_hilda_and_dawn(build());
    let mut state = game(&set, hilda, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, hilda);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(
        step_of(&state),
        Some(1),
        "declining moves on, it does not end"
    );
    let energy = offered(&state)[0];
    apply(&mut state, Action::TakeCard { card: energy }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();
    assert_eq!(state.phase, Phase::Main);
    assert!(state.player(player).hand.contains(&energy));
}

#[test]
fn the_last_slot_declined_ends_the_search() {
    let (set, hilda, _) = with_hilda_and_dawn(build());
    let mut state = game(&set, hilda, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, hilda);
    let hand_before = state.player(player).hand.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.phase, Phase::Main);
    // Only the Supporter left the hand.
    assert_eq!(state.player(player).hand.len(), hand_before - 1);
}

#[test]
fn dawn_walks_three_slots_in_the_order_printed() {
    let (set, _, dawn) = with_hilda_and_dawn(build());
    let mut state = game(&set, dawn, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, dawn);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let wanted = [Stage::Basic, Stage::Stage1, Stage::Stage2];
    for (step, stage) in wanted.iter().enumerate() {
        assert_eq!(step_of(&state), Some(step as u32));
        for offered in offered(&state) {
            assert_eq!(
                state.def_of(offered).as_pokemon().map(|p| p.stage),
                Some(*stage),
                "slot {step} wants a {stage:?}"
            );
        }
        // This deck holds no Stage 2, so the last slot finds nothing.
        apply(&mut state, Action::FinishDeciding).unwrap();
    }
    assert_eq!(state.phase, Phase::Main);
}

#[test]
fn the_deck_is_shuffled_once_the_whole_search_ends() {
    let (set, _, dawn) = with_hilda_and_dawn(build());
    let mut state = game(&set, dawn, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, dawn);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let mid = state.player(player).library.clone();
    apply(&mut state, Action::FinishDeciding).unwrap();
    assert_eq!(
        state.player(player).library,
        mid,
        "a slot ending is not the search ending"
    );

    apply(&mut state, Action::FinishDeciding).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();
    assert_ne!(
        state.player(player).library,
        mid,
        "the shuffle comes once, at the end"
    );
}

#[test]
fn a_one_slot_search_is_the_same_path_as_the_rest() {
    // Every card built before this ticket is a search of one slot. Nothing
    // about them changed, and this holds them to it.
    let set = build();
    let mut state = game(&set, set.cyrano, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, set.cyrano);
    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    assert_eq!(step_of(&state), Some(0));
    let take = offered(&state)[0];
    apply(&mut state, Action::TakeCard { card: take }).unwrap();
    assert_eq!(step_of(&state), Some(0), "three of one card is one slot");
    apply(&mut state, Action::FinishDeciding).unwrap();
    assert_eq!(state.phase, Phase::Main);
}

#[test]
fn hilda_and_dawn_are_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let trainer = |name: &str| {
        import
            .admitted
            .iter()
            .map(|id| import.db.get(*id))
            .filter_map(|def| def.as_trainer())
            .find(|t| t.name == name)
            .map(|t| t.effect.clone())
    };
    assert_eq!(
        trainer("Hilda"),
        Some(TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![
                slot(CardFilter::EvolutionPokemon),
                slot(CardFilter::BasicEnergy),
            ],
            then: None,
        })
    );
    assert_eq!(
        trainer("Dawn"),
        Some(TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![
                slot(CardFilter::PokemonOfStage(Stage::Basic)),
                slot(CardFilter::PokemonOfStage(Stage::Stage1)),
                slot(CardFilter::PokemonOfStage(Stage::Stage2)),
            ],
            then: None,
        })
    );
}

// --- Ticket 05, part three: Crispin ---

fn with_crispin(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let crispin = db.add(CardDef::Trainer(Trainer {
        print_id: "test-crispin",
        name: "Crispin",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![
                Slot {
                    filter: CardFilter::BasicEnergy,
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                },
                Slot {
                    filter: CardFilter::BasicEnergy,
                    to: Destination::Attach(TargetFilter::AnyInPlay),
                    limit: 1,
                    excludes_type_of_previous: true,
                    peek: None,
                },
            ],
            then: None,
        },
    }));
    (Set { db, ..set }, crispin)
}

#[test]
fn the_second_slot_excludes_the_type_the_first_slot_took() {
    let (set, crispin) = with_crispin(build());
    let mut state = game(&set, crispin, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, crispin);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let first = offered(&state)[0];
    let first_kind = match state.def_of(first) {
        sim::card::CardDef::Energy(e) => e.kind,
        _ => panic!("the first slot only offers Energy"),
    };
    apply(&mut state, Action::TakeCard { card: first }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    // The second slot attaches, so its offers come as TakeCardOnto.
    let second_choices: Vec<CardId> = legal_actions(&state)
        .into_iter()
        .filter_map(|a| match a {
            Action::TakeCardOnto { card, .. } => Some(card),
            _ => None,
        })
        .collect();
    assert!(
        !second_choices.is_empty(),
        "a different type is in the deck"
    );
    for card in second_choices {
        let kind = match state.def_of(card) {
            sim::card::CardDef::Energy(e) => e.kind,
            _ => panic!("the second slot only offers Energy"),
        };
        assert_ne!(kind, first_kind, "the second Energy must differ in type");
    }
}

#[test]
fn crispin_attaches_the_second_card_to_the_chosen_pokemon() {
    let (set, crispin) = with_crispin(build());
    let mut state = game(&set, crispin, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, crispin);
    let active = state.player(player).active.expect("setup placed an Active");

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let first = offered(&state)[0];
    apply(&mut state, Action::TakeCard { card: first }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    let (second, target) = legal_actions(&state)
        .into_iter()
        .find_map(|a| match a {
            Action::TakeCardOnto { card, target } => Some((card, target)),
            _ => None,
        })
        .expect("the second slot offers somewhere to attach");
    assert_eq!(target, active, "the only Pokémon in play is the Active");

    apply(
        &mut state,
        Action::TakeCardOnto {
            card: second,
            target,
        },
    )
    .unwrap();
    // ADR 0012 keeps the choice to stop with the player, slot by slot.
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.phase, Phase::Main, "two slots, two cards, done");
    assert!(state.player(player).hand.contains(&first));
    assert!(state.pokemon(active).attached.contains(&second));
    assert!(!state.player(player).hand.contains(&second));
    assert!(!state.player(player).library.contains(&second));
}

#[test]
fn crispin_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let crispin = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Crispin")
        .expect("Crispin plays");
    assert_eq!(
        crispin.effect,
        TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![
                Slot {
                    filter: CardFilter::BasicEnergy,
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                },
                Slot {
                    filter: CardFilter::BasicEnergy,
                    to: Destination::Attach(TargetFilter::AnyInPlay),
                    limit: 1,
                    excludes_type_of_previous: true,
                    peek: None,
                },
            ],
            then: None,
        }
    );
}

// --- Ticket 06: Rare Candy and the evolution chain ---

fn with_rare_candy(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let rare_candy = db.add(CardDef::Trainer(Trainer {
        print_id: "test-rare-candy",
        name: "Rare Candy",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::EvolveSkippingOneStage,
    }));
    (Set { db, ..set }, rare_candy)
}

/// Put a copy of `def` into play for `player`, as though it had been placed
/// on an earlier turn — the way `Rare Candy` and ordinary evolution both
/// require.
/// A physical card of `def`, for a definition that was never part of the
/// sixty-card decklist — a target or an Energy the fixture needs but the
/// deal never dealt. Placed nowhere; the caller pushes it to whatever zone
/// or play the test needs.
fn deal_new_card(state: &mut GameState, player: PlayerId, def: CardDefId) -> CardId {
    let card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def, owner: player });
    card
}

fn put_in_play_from_an_earlier_turn(
    state: &mut GameState,
    player: PlayerId,
    def: CardDefId,
) -> PokemonId {
    let card = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.cards[c.index()].def == def)
        .expect("the deck holds this card");
    state.players[player.index()].library.retain(|c| *c != card);
    let pokemon = state.put_into_play(player, card);
    state.players[player.index()].bench.push(pokemon);
    state.pokemon[pokemon.index()].played_on_turn = 0;
    pokemon
}

#[test]
fn rare_candy_offers_only_a_chain_that_matches() {
    let (set, rare_candy) = with_rare_candy(build());
    let mut state = game(&set, rare_candy, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, rare_candy);
    let small = put_in_play_from_an_earlier_turn(&mut state, player, set.small);
    let stage2 = ensure_in_hand(&mut state, player, set.stage2);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    assert!(
        matches!(state.phase, Phase::EvolvingWithRareCandy { .. }),
        "the card asks which pair: {:?}",
        state.phase
    );
    let pairs: Vec<Action> = legal_actions(&state)
        .into_iter()
        .filter(|a| matches!(a, Action::EvolveSkippingOneStage { .. }))
        .collect();
    assert!(
        pairs.contains(&Action::EvolveSkippingOneStage {
            card: stage2,
            target: small,
        }),
        "the Stage 2 evolves from the Basic two links down"
    );
    // The ordinary Testmon in play does not sit under this Stage 2's chain.
    let mon = put_in_play_from_an_earlier_turn(&mut state, player, set.mon);
    let pairs_after: Vec<Action> = legal_actions(&state)
        .into_iter()
        .filter(|a| matches!(a, Action::EvolveSkippingOneStage { .. }))
        .collect();
    assert!(
        !pairs_after.contains(&Action::EvolveSkippingOneStage {
            card: stage2,
            target: mon
        }),
        "a Pokémon outside the chain is never offered"
    );
}

#[test]
fn rare_candy_evolves_the_basic_straight_to_the_stage_2() {
    let (set, rare_candy) = with_rare_candy(build());
    let mut state = game(&set, rare_candy, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, rare_candy);
    let small = put_in_play_from_an_earlier_turn(&mut state, player, set.small);
    let basic_card = state.pokemon(small).cards[0];
    let stage2 = ensure_in_hand(&mut state, player, set.stage2);
    let hand_before = state.player(player).hand.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(
        &mut state,
        Action::EvolveSkippingOneStage {
            card: stage2,
            target: small,
        },
    )
    .unwrap();

    assert_eq!(state.phase, Phase::Main, "no choice is left in the card");
    assert_eq!(
        state.pokemon(small).cards,
        vec![basic_card, stage2],
        "the Stage 1 is skipped: the stack holds the Basic, then the Stage 2"
    );
    assert_eq!(state.pokemon_def(small).name, "Hugemon");
    assert!(state.is_spent(sim::state::Limit::Evolved(small)));
    // Rare Candy itself, and the Stage 2 it played, both left the hand.
    assert_eq!(state.player(player).hand.len(), hand_before - 2);
}

#[test]
fn rare_candy_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let rare_candy = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Rare Candy")
        .expect("Rare Candy plays");
    assert_eq!(rare_candy.effect, TrainerEffect::EvolveSkippingOneStage);
}

#[test]
fn the_real_ampharos_chain_resolves_to_mareep() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let ampharos = import
        .admitted
        .iter()
        .filter_map(|id| import.db.get(*id).as_pokemon())
        .find(|p| p.name == "Ampharos")
        .expect("Ampharos plays");
    assert_eq!(ampharos.stage, Stage::Stage2);
    assert_eq!(ampharos.evolve_from, Some("Flaaffy"));
    assert_eq!(
        ampharos.evolves_from_basic,
        Some("Mareep"),
        "the chain is walked through Flaaffy's own evolveFrom, read from \
         the raw name table rather than from an admitted CardDef"
    );
}

// --- Milestone 6, ticket 01: a search for any Trainer card ---

fn with_petrel(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let petrel = db.add(CardDef::Trainer(Trainer {
        print_id: "test-petrel",
        name: "Team Rocket's Petrel",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![Slot {
                filter: CardFilter::AnyTrainer,
                to: Destination::Zone(Zone::Hand),
                limit: 1,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    }));
    (Set { db, ..set }, petrel)
}

#[test]
fn the_any_trainer_filter_offers_only_trainers() {
    let (set, petrel) = with_petrel(build());
    let mut state = game(&set, petrel, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, petrel);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let choices = offered(&state);
    assert!(!choices.is_empty(), "the deck holds a second Petrel");
    for card in &choices {
        assert!(
            state.def_of(*card).as_trainer().is_some(),
            "{:?} is not a Trainer and must not be offered",
            state.def_of(*card)
        );
    }
}

#[test]
fn the_any_trainer_filter_never_offers_a_pokemon_or_an_energy() {
    let (set, petrel) = with_petrel(build());
    let mut state = game(&set, petrel, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, petrel);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let offered_mons_and_energy = state
        .player(player)
        .library
        .iter()
        .filter(|c| {
            let def = state.def_of(**c);
            def.as_pokemon().is_some() || def.is_energy()
        })
        .any(|c| legal_actions(&state).contains(&Action::TakeCard { card: *c }));
    assert!(!offered_mons_and_energy);
}

#[test]
fn team_rockets_petrel_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let petrel = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Team Rocket's Petrel")
        .expect("Team Rocket's Petrel plays");
    assert_eq!(
        petrel.effect,
        TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![Slot {
                filter: CardFilter::AnyTrainer,
                to: Destination::Zone(Zone::Hand),
                limit: 1,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        }
    );
}

// --- Milestone 6, ticket 02: an attach with a filtered target ---

fn with_pp_up(set: Set) -> (Set, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let ns_mon = basic(&mut db, "test-ns-mon", "N's Testmon", 90, 1, None);
    let pp_up = db.add(CardDef::Trainer(Trainer {
        print_id: "test-pp-up",
        name: "N's PP Up",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Discard,
            slots: vec![Slot {
                filter: CardFilter::BasicEnergy,
                to: Destination::Attach(TargetFilter::BenchedNameStartsWith("N's")),
                limit: 1,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    }));
    (Set { db, ..set }, pp_up, ns_mon)
}

#[test]
fn pp_up_offers_only_a_benched_pokemon_whose_name_starts_with_ns() {
    let (set, pp_up, ns_mon) = with_pp_up(build());
    let mut state = game(&set, pp_up, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, pp_up);

    // Bench the N's Pokémon, and an ordinary one to prove it is excluded.
    // The N's Pokémon was never part of the decklist, so it is dealt
    // straight into play rather than pulled from the library.
    let ns_card = deal_new_card(&mut state, player, ns_mon);
    let benched_ns = state.put_into_play(player, ns_card);
    state.players[player.index()].bench.push(benched_ns);
    state.pokemon[benched_ns.index()].played_on_turn = 0;
    let benched_ordinary = put_in_play_from_an_earlier_turn(&mut state, player, set.small);

    // A Basic Energy sitting in the discard pile, for the search to find.
    let energy = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    state.players[player.index()]
        .library
        .retain(|c| *c != energy);
    state.players[player.index()].discard.push(energy);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let targets: Vec<PokemonId> = legal_actions(&state)
        .into_iter()
        .filter_map(|a| match a {
            Action::TakeCardOnto { target, .. } => Some(target),
            _ => None,
        })
        .collect();
    assert!(targets.contains(&benched_ns), "the N's Pokémon is offered");
    assert!(
        !targets.contains(&benched_ordinary),
        "an ordinary Pokémon is not"
    );
    // Whichever Pokémon setup placed as Active does not start with "N's",
    // so it must never appear even though it is in play.
    let active = state.player(player).active.unwrap();
    assert!(!targets.contains(&active), "the Active is not Benched");
}

fn with_wondrous_patch(set: Set) -> (Set, CardDefId, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let psychic_mon = db.add(CardDef::Pokemon(Pokemon {
        print_id: "test-psychic-mon",
        name: "Psymon",
        hp: 90,
        kind: Type::Psychic,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        attacks: vec![Attack {
            name: "Zap",
            cost: vec![Type::Psychic],
            base_damage: 10,
            inflicts: None,
        }],
    }));
    let psychic_energy = db.add(CardDef::Energy(Energy {
        print_id: "test-psychic-energy",
        name: "Psychic Energy",
        kind: Type::Psychic,
    }));
    let patch = db.add(CardDef::Trainer(Trainer {
        print_id: "test-wondrous-patch",
        name: "Wondrous Patch",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Discard,
            slots: vec![Slot {
                filter: CardFilter::BasicEnergyOfType(Type::Psychic),
                to: Destination::Attach(TargetFilter::BenchedOfType(Type::Psychic)),
                limit: 1,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    }));
    (Set { db, ..set }, patch, psychic_mon, psychic_energy)
}

#[test]
fn wondrous_patch_offers_only_a_psychic_energy_onto_a_psychic_pokemon() {
    let (set, patch, psychic_mon, psychic_energy) = with_wondrous_patch(build());
    let mut state = game(&set, patch, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, patch);

    // Neither the Psychic Pokémon nor the Psychic Energy was part of the
    // decklist, so both are dealt straight to where the test needs them.
    let psychic_card = deal_new_card(&mut state, player, psychic_mon);
    let psychic_bench = state.put_into_play(player, psychic_card);
    state.players[player.index()].bench.push(psychic_bench);
    state.pokemon[psychic_bench.index()].played_on_turn = 0;
    let ordinary_bench = put_in_play_from_an_earlier_turn(&mut state, player, set.small);

    // Both a Psychic and a Colorless Energy sit in the discard, so the
    // wrong-typed one can be proven excluded, not merely unmentioned.
    let colorless_energy = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    state.players[player.index()]
        .library
        .retain(|c| *c != colorless_energy);
    state.players[player.index()].discard.push(colorless_energy);
    let psychic_energy_card = deal_new_card(&mut state, player, psychic_energy);
    state.players[player.index()]
        .discard
        .push(psychic_energy_card);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let offers: Vec<Action> = legal_actions(&state)
        .into_iter()
        .filter(|a| matches!(a, Action::TakeCardOnto { .. }))
        .collect();
    assert!(
        offers.contains(&Action::TakeCardOnto {
            card: psychic_energy_card,
            target: psychic_bench,
        }),
        "a Psychic Energy onto a Psychic Pokémon is offered"
    );
    assert!(
        !offers.iter().any(|a| matches!(a,
            Action::TakeCardOnto { card, .. } if *card == colorless_energy
        )),
        "a Colorless Energy is never offered, even to a Psychic Pokémon"
    );
    assert!(
        !offers.iter().any(|a| matches!(a,
            Action::TakeCardOnto { target, .. } if *target == ordinary_bench
        )),
        "an ordinary Pokémon is never offered, even the right Energy type"
    );
}

#[test]
fn pp_up_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let pp_up = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "N's PP Up")
        .expect("N's PP Up plays");
    assert_eq!(
        pp_up.effect,
        TrainerEffect::Decide {
            from: Zone::Discard,
            slots: vec![Slot {
                filter: CardFilter::BasicEnergy,
                to: Destination::Attach(TargetFilter::BenchedNameStartsWith("N's")),
                limit: 1,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        }
    );
}

#[test]
fn wondrous_patch_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let patch = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Wondrous Patch")
        .expect("Wondrous Patch plays");
    assert_eq!(
        patch.effect,
        TrainerEffect::Decide {
            from: Zone::Discard,
            slots: vec![Slot {
                filter: CardFilter::BasicEnergyOfType(Type::Psychic),
                to: Destination::Attach(TargetFilter::BenchedOfType(Type::Psychic)),
                limit: 1,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        }
    );
}

// --- Milestone 6, ticket 03: a search bounded to the top of the deck ---

fn with_pokegear(set: Set) -> (Set, CardDefId, CardDefId, CardDefId) {
    let mut db = set.db.clone();
    let supporter_a = db.add(CardDef::Trainer(Trainer {
        print_id: "test-supporter-a",
        name: "Test Supporter A",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::Nothing,
    }));
    let supporter_b = db.add(CardDef::Trainer(Trainer {
        print_id: "test-supporter-b",
        name: "Test Supporter B",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::Nothing,
    }));
    let pokegear = db.add(CardDef::Trainer(Trainer {
        print_id: "test-pokegear",
        name: "Pokégear 3.0",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![Slot {
                filter: CardFilter::TrainerOfKind(TrainerKind::Supporter),
                to: Destination::Zone(Zone::Hand),
                limit: 1,
                excludes_type_of_previous: false,
                peek: Some(7),
            }],
            then: None,
        },
    }));
    (Set { db, ..set }, pokegear, supporter_a, supporter_b)
}

/// Library order, from bottom (index 0) to top (the end — `draw` pops from
/// there). `top` lands within the peek window; `bottom` lands well below
/// it, but is never lost from the deck.
fn arrange_library_with_one_card_near_the_top(
    state: &mut GameState,
    player: PlayerId,
    top: CardId,
    bottom: CardId,
) {
    let mut library = std::mem::take(&mut state.players[player.index()].library);
    library.insert(0, bottom);
    library.push(top);
    state.players[player.index()].library = library;
}

#[test]
fn a_peeked_search_offers_only_the_top_of_the_deck() {
    let (set, pokegear, supporter_a, supporter_b) = with_pokegear(build());
    let mut state = game(&set, pokegear, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, pokegear);

    let near_top = deal_new_card(&mut state, player, supporter_a);
    let buried = deal_new_card(&mut state, player, supporter_b);
    arrange_library_with_one_card_near_the_top(&mut state, player, near_top, buried);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let choices = offered(&state);
    assert!(
        choices.contains(&near_top),
        "a Supporter within the top 7 is offered"
    );
    assert!(
        !choices.contains(&buried),
        "a Supporter buried below the top 7 is not, even though it matches"
    );
}

#[test]
fn a_peeked_search_still_shuffles_the_deck_when_it_ends() {
    let (set, pokegear, supporter_a, supporter_b) = with_pokegear(build());
    let mut state = game(&set, pokegear, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, pokegear);

    let near_top = deal_new_card(&mut state, player, supporter_a);
    let buried = deal_new_card(&mut state, player, supporter_b);
    arrange_library_with_one_card_near_the_top(&mut state, player, near_top, buried);
    let before = state.player(player).library.clone();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    let after = &state.player(player).library;
    assert_eq!(before.len(), after.len(), "nothing was taken");
    assert!(after.contains(&buried), "the buried card is not lost");
    assert_ne!(&before, after, "the deck is shuffled once the search ends");
}

fn with_bug_catching_set(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let bug_catching_set = db.add(CardDef::Trainer(Trainer {
        print_id: "test-bug-catching-set",
        name: "Bug Catching Set",
        kind: TrainerKind::Item,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![Slot {
                filter: CardFilter::PokemonOfTypeOrBasicEnergyOfType(Type::Grass),
                to: Destination::Zone(Zone::Hand),
                limit: 2,
                excludes_type_of_previous: false,
                peek: Some(7),
            }],
            then: None,
        },
    }));
    (Set { db, ..set }, bug_catching_set)
}

#[test]
fn bug_catching_sets_filter_admits_grass_pokemon_and_grass_energy_alike() {
    let (set, bug_catching_set) = with_bug_catching_set(build());
    let mut state = game(&set, bug_catching_set, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, bug_catching_set);

    // A Grass Pokémon and a Grass Energy, both dealt near the top; a
    // Colorless Pokémon dealt alongside them to prove the type still
    // matters, not only "any Pokémon or any Energy".
    let grass_mon = db_add_grass_mon(&mut state);
    let grass_energy = db_add_grass_energy(&mut state);
    let grass_mon_card = deal_new_card(&mut state, player, grass_mon);
    let grass_energy_card = deal_new_card(&mut state, player, grass_energy);
    let colorless_card = deal_new_card(&mut state, player, set.mon);

    let mut library = std::mem::take(&mut state.players[player.index()].library);
    library.push(colorless_card);
    library.push(grass_mon_card);
    library.push(grass_energy_card);
    state.players[player.index()].library = library;

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let choices = offered(&state);
    assert!(
        choices.contains(&grass_mon_card),
        "a Grass Pokémon is offered"
    );
    assert!(
        choices.contains(&grass_energy_card),
        "a Grass Energy is offered"
    );
    assert!(
        !choices.contains(&colorless_card),
        "a Colorless Pokémon is not"
    );
}

/// A Grass-typed Basic Pokémon, added directly to the running game's own
/// database — `Set` is built once, before the game starts, so a filter
/// this specific gets its fixture dealt in rather than pre-built.
fn db_add_grass_mon(state: &mut GameState) -> CardDefId {
    state.db.add(CardDef::Pokemon(Pokemon {
        print_id: "test-grass-mon",
        name: "Grassmon",
        hp: 60,
        kind: Type::Grass,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        attacks: vec![Attack {
            name: "Vine",
            cost: vec![Type::Grass],
            base_damage: 10,
            inflicts: None,
        }],
    }))
}

fn db_add_grass_energy(state: &mut GameState) -> CardDefId {
    state.db.add(CardDef::Energy(Energy {
        print_id: "test-grass-energy",
        name: "Grass Energy",
        kind: Type::Grass,
    }))
}

#[test]
fn pokegear_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let pokegear = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Pokégear 3.0")
        .expect("Pokégear 3.0 plays");
    assert_eq!(
        pokegear.effect,
        TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![Slot {
                filter: CardFilter::TrainerOfKind(TrainerKind::Supporter),
                to: Destination::Zone(Zone::Hand),
                limit: 1,
                excludes_type_of_previous: false,
                peek: Some(7),
            }],
            then: None,
        }
    );
}

#[test]
fn bug_catching_set_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let bug_catching_set = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Bug Catching Set")
        .expect("Bug Catching Set plays");
    assert_eq!(
        bug_catching_set.effect,
        TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![Slot {
                filter: CardFilter::PokemonOfTypeOrBasicEnergyOfType(Type::Grass),
                to: Destination::Zone(Zone::Hand),
                limit: 2,
                excludes_type_of_previous: false,
                peek: Some(7),
            }],
            then: None,
        }
    );
}

// --- Milestone 6, ticket 04: a search with no filter, returned in order ---

fn with_codebreaking(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let codebreaking = db.add(CardDef::Trainer(Trainer {
        print_id: "test-codebreaking",
        name: "Ciphermaniac's Codebreaking",
        kind: TrainerKind::Supporter,
        requirement: None,
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![Slot {
                filter: CardFilter::AnyCard,
                to: Destination::TopOfLibraryInOrder,
                limit: 2,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
    }));
    (Set { db, ..set }, codebreaking)
}

#[test]
fn the_any_card_filter_admits_every_kind_at_once() {
    let (set, codebreaking) = with_codebreaking(build());
    let mut state = game(&set, codebreaking, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, codebreaking);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let choices = offered(&state);
    let has = |pred: fn(&sim::card::CardDef) -> bool| {
        choices.iter().any(|c| pred(state.def_of(*c)))
    };
    assert!(
        has(|def| def.as_pokemon().is_some()),
        "a Pokémon is offered"
    );
    assert!(has(|def| def.is_energy()), "an Energy is offered");
    assert!(
        has(|def| def.as_trainer().is_some()),
        "a Trainer is offered"
    );
}

#[test]
fn codebreaking_returns_the_two_taken_to_the_top_in_the_order_taken() {
    let (set, codebreaking) = with_codebreaking(build());
    let mut state = game(&set, codebreaking, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, codebreaking);
    let library_before = state.player(player).library.len();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let first = offered(&state)[0];
    apply(&mut state, Action::TakeCard { card: first }).unwrap();
    let second = offered(&state)[0];
    apply(&mut state, Action::TakeCard { card: second }).unwrap();
    // ADR 0012 keeps the choice to stop with the player, even at the limit.
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert_eq!(state.phase, Phase::Main, "two taken, the limit is spent");
    assert_ne!(first, second);
    let library = &state.player(player).library;
    assert_eq!(
        library.len(),
        library_before,
        "the two return to the deck; nothing leaves it"
    );
    // `draw` pops from the end, so the end is the top: the second card
    // taken sits above the first, the same order the player chose.
    assert_eq!(
        &library[library.len() - 2..],
        &[first, second],
        "the two land on top, in the order they were taken"
    );
}

#[test]
fn codebreaking_shuffles_what_is_left_before_placing_the_two_on_top() {
    let (set, codebreaking) = with_codebreaking(build());
    let mut state = game(&set, codebreaking, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, codebreaking);
    let before = state.player(player).library.clone();

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    let first = offered(&state)[0];
    apply(&mut state, Action::TakeCard { card: first }).unwrap();
    let second = offered(&state)[0];
    apply(&mut state, Action::TakeCard { card: second }).unwrap();

    let after = &state.player(player).library;
    let beneath_the_top_two = &after[..after.len() - 2];
    let original_beneath = before
        .iter()
        .filter(|c| **c != first && **c != second)
        .copied()
        .collect::<Vec<_>>();
    assert_ne!(
        beneath_the_top_two, original_beneath,
        "the rest of the deck is shuffled, not merely missing two cards"
    );
}

#[test]
fn codebreaking_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let codebreaking = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Ciphermaniac's Codebreaking")
        .expect("Ciphermaniac's Codebreaking plays");
    assert_eq!(
        codebreaking.effect,
        TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![Slot {
                filter: CardFilter::AnyCard,
                to: Destination::TopOfLibraryInOrder,
                limit: 2,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        }
    );
}

// --- Milestone 6, ticket 05: a requirement read from last turn ---

fn with_unfair_stamp(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let unfair_stamp = db.add(CardDef::Trainer(Trainer {
        print_id: "test-unfair-stamp",
        name: "Unfair Stamp",
        kind: TrainerKind::Item,
        requirement: Some(Requirement::KnockedOutDuringOpponentsLastTurn),
        effect: TrainerEffect::BothShuffleHandThenDraw {
            you: 5,
            opponent: 2,
        },
    }));
    (Set { db, ..set }, unfair_stamp)
}

#[test]
fn unfair_stamp_cannot_be_played_with_no_knockout_to_point_to() {
    let (set, unfair_stamp) = with_unfair_stamp(build());
    let mut state = game(&set, unfair_stamp, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, unfair_stamp);
    assert!(
        !legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "nothing was Knocked Out; the card has nothing to point to"
    );
}

/// Run `EndTurn`, then every action `legal_actions` offers first until a
/// Main phase is reached — the ordinary way this file already drives setup
/// and Promoting through to a stopping point.
fn end_turn_and_advance(state: &mut GameState) {
    apply(state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(state)[0];
        apply(state, first).unwrap();
    }
}

/// Damage a Pokémon in play to exactly its own HP — not by an attack, but
/// by the same fact `knock_out_the_dead` reads to decide a Knockout: how
/// much damage it already carries. The next `settle` (any `EndTurn` runs
/// one) finds it exactly the way a lethal attack would have left it.
fn make_lethally_damaged(state: &mut GameState, pokemon: PokemonId) {
    let hp = state.pokemon_def(pokemon).hp;
    state.pokemon[pokemon.index()].damage = hp;
}

#[test]
fn unfair_stamp_plays_once_the_opponent_knocked_something_out_last_turn() {
    let (set, unfair_stamp) = with_unfair_stamp(build());
    let mut state = game(&set, unfair_stamp, 3);
    let player = state.current;
    let opponent = player.opponent();

    // Into the opponent's turn, where the Knockout belongs.
    end_turn_and_advance(&mut state);
    assert_eq!(state.current, opponent);

    let victim = state
        .player(player)
        .active
        .expect("the player has an Active to lose");
    make_lethally_damaged(&mut state, victim);

    // The opponent's turn ends; `settle` finds the lethal damage and
    // Knocks the Pokémon out, then the player promotes a new Active,
    // then it becomes the player's turn — where the fact belongs.
    end_turn_and_advance(&mut state);
    assert_eq!(state.current, player, "back to the player who lost it");

    let card = ensure_in_hand(&mut state, player, unfair_stamp);
    assert!(
        legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "the opponent's last turn Knocked this player's Pokémon out"
    );
    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert_eq!(state.phase, Phase::Main, "no choice is left in the card");
    assert_eq!(
        state.player(player).hand.len(),
        5,
        "the player who played it draws 5"
    );
    assert_eq!(
        state.player(opponent).hand.len(),
        2,
        "the opponent draws only 2"
    );
}

#[test]
fn the_fact_does_not_survive_the_players_own_next_turn() {
    let (set, unfair_stamp) = with_unfair_stamp(build());
    let mut state = game(&set, unfair_stamp, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, unfair_stamp);

    end_turn_and_advance(&mut state); // into the opponent's turn
    let victim = state.player(player).active.unwrap();
    make_lethally_damaged(&mut state, victim);
    end_turn_and_advance(&mut state); // Knockout, promote, into the player's turn
    assert_eq!(state.current, player);
    assert!(
        legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "true on the turn right after the Knockout"
    );

    // One full cycle further, with no new Knockout: the old one is now two
    // turns stale and must not still count.
    end_turn_and_advance(&mut state); // into the opponent's turn again
    end_turn_and_advance(&mut state); // back to the player, no Knockout this time
    assert_eq!(state.current, player, "back to the same player's turn");

    let card = ensure_in_hand(&mut state, player, unfair_stamp);
    assert!(
        !legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "the old Knockout is two turns stale and must not still count"
    );
}

#[test]
fn unfair_stamp_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let unfair_stamp = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Unfair Stamp")
        .expect("Unfair Stamp plays");
    assert_eq!(
        unfair_stamp.requirement,
        Some(Requirement::KnockedOutDuringOpponentsLastTurn)
    );
    assert_eq!(
        unfair_stamp.effect,
        TrainerEffect::BothShuffleHandThenDraw {
            you: 5,
            opponent: 2,
        }
    );
}

// --- The card data ---

#[test]
fn buddy_buddy_poffin_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let poffin = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Buddy-Buddy Poffin")
        .expect("Buddy-Buddy Poffin plays");
    assert_eq!(
        poffin.effect,
        TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![Slot {
                filter: CardFilter::BasicPokemonWithHpAtMost(70),
                to: Destination::Bench,
                limit: 2,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        }
    );
}

#[test]
fn cyrano_is_admitted_from_the_artifact() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = sim::import::load(&json).unwrap();
    let cyrano = import
        .admitted
        .iter()
        .map(|id| import.db.get(*id))
        .filter_map(|def| def.as_trainer())
        .find(|t| t.name == "Cyrano")
        .expect("Cyrano plays");
    assert_eq!(
        cyrano.effect,
        TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![Slot {
                filter: CardFilter::PokemonEx,
                to: Destination::Zone(Zone::Hand),
                limit: 3,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        }
    );
}
