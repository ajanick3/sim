//! Ticket 03: the first Trainer effects.

use sim::action::{Action, legal_actions};
use sim::card::{
    Attack, CardDb, CardDef, Energy, Pokemon, Trainer, TrainerEffect, TrainerKind, Type,
};
use sim::engine::apply;
use sim::ids::{CardDefId, PlayerId};
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};

struct Set {
    db: CardDb,
    mon: CardDefId,
    energy: CardDefId,
    judge: CardDefId,
    lillie: CardDefId,
}

fn build() -> Set {
    let mut db = CardDb::new();
    let mon = db.add(CardDef::Pokemon(Pokemon {
        print_id: "test-mon",
        name: "Testmon",
        hp: 60,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        evolve_from: None,
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
        }],
    }));
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
    }));
    let judge = db.add(CardDef::Trainer(Trainer {
        print_id: "test-judge",
        name: "Judge",
        kind: TrainerKind::Supporter,
        effect: TrainerEffect::BothShuffleHandThenDraw { count: 4 },
    }));
    let lillie = db.add(CardDef::Trainer(Trainer {
        print_id: "test-lillie",
        name: "Lillie's Determination",
        kind: TrainerKind::Supporter,
        effect: TrainerEffect::ShuffleHandThenDraw {
            normal: 6,
            at_six_prizes: 8,
        },
    }));
    Set {
        db,
        mon,
        energy,
        judge,
        lillie,
    }
}

fn deck(set: &Set, supporter: CardDefId) -> Vec<CardDefId> {
    let mut decklist = vec![set.mon; 6];
    decklist.push(supporter);
    while decklist.len() < 60 {
        decklist.push(set.energy);
    }
    decklist
}

fn game(set: &Set, supporter: CardDefId, seed: u64) -> GameState {
    let decklist = deck(set, supporter);
    let mut state = GameState::new(
        set.db.clone(),
        [decklist.clone(), decklist],
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

fn ensure_in_hand(state: &mut GameState, player: PlayerId, def: CardDefId) -> sim::ids::CardId {
    if let Some(card) = state
        .player(player)
        .hand
        .iter()
        .find(|c| state.cards[c.index()].def == def)
    {
        return *card;
    }
    let card = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.cards[c.index()].def == def)
        .expect("the deck holds this card");
    state.players[player.index()].library.retain(|c| *c != card);
    state.players[player.index()].hand.push(card);
    card
}

#[test]
fn a_supporter_is_played_once_per_turn_then_discarded() {
    let set = build();
    let mut state = game(&set, set.judge, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, set.judge);

    assert!(
        legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "a Supporter in hand is playable"
    );
    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert!(
        !state.player(player).hand.contains(&card),
        "played, it leaves the hand"
    );
    assert!(
        state.player(player).discard.contains(&card),
        "a Supporter discards once played"
    );
    assert!(
        state.is_spent(sim::state::Limit::SupporterPlayed(player)),
        "rule 13: one Supporter a turn"
    );
    assert!(
        !legal_actions(&state)
            .iter()
            .any(|a| matches!(a, Action::PlayTrainer { .. })),
        "a second Supporter cannot be played the same turn"
    );
}

#[test]
fn judge_reshuffles_both_hands_and_draws_four() {
    let set = build();
    let mut state = game(&set, set.judge, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, set.judge);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    assert_eq!(state.player(PlayerId::One).hand.len(), 4);
    assert_eq!(state.player(PlayerId::Two).hand.len(), 4);
    assert_eq!(state.phase, Phase::Main, "no choice, resolves at once");
}

#[test]
fn lillies_determination_draws_six_normally() {
    // A fresh game starts at exactly 6 Prizes (rule 10) — the bonus case by
    // default. The normal count needs a Prize already taken.
    let set = build();
    let mut state = game(&set, set.lillie, 3);
    let player = state.current;
    let taken = state.players[player.index()].prizes.pop().unwrap();
    state.players[player.index()].hand.push(taken);
    assert_eq!(state.player(player).prizes.len(), 5, "not the bonus case");

    let card = ensure_in_hand(&mut state, player, set.lillie);
    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    assert_eq!(state.player(player).hand.len(), 6);
}

#[test]
fn lillies_determination_draws_eight_at_six_prizes() {
    let set = build();
    let mut state = game(&set, set.lillie, 3);
    let player = state.current;
    state.players[player.index()].prizes = (0..6)
        .map(|_| state.player(player).library[0])
        .collect::<Vec<_>>();
    let card = ensure_in_hand(&mut state, player, set.lillie);
    assert_eq!(state.player(player).prizes.len(), 6);

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    assert_eq!(state.player(player).hand.len(), 8);
}

// --- The remaining six cards ---

struct Set2 {
    db: CardDb,
    mon: CardDefId,
    mon_ex: CardDefId,
    energy: CardDefId,
    boss: CardDefId,
    poke_pad: CardDefId,
    night_stretcher: CardDefId,
    sacred_ash: CardDefId,
    gwynn: CardDefId,
    crushing_hammer: CardDefId,
}

fn build2() -> Set2 {
    use sim::card::{CardFilter, Then, Zone};
    let mut db = CardDb::new();
    let mon = db.add(CardDef::Pokemon(Pokemon {
        print_id: "test-mon",
        name: "Testmon",
        hp: 60,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        evolve_from: None,
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
        }],
    }));
    let mon_ex = db.add(CardDef::Pokemon(Pokemon {
        print_id: "test-mon-ex",
        name: "Testmon ex",
        hp: 150,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 2,
        prizes: 2,
        evolve_from: None,
        attacks: vec![Attack {
            name: "Big Tackle",
            cost: vec![Type::Colorless, Type::Colorless],
            base_damage: 50,
            inflicts: None,
        }],
    }));
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
    }));
    let boss = db.add(CardDef::Trainer(Trainer {
        print_id: "test-boss",
        name: "Boss's Orders",
        kind: TrainerKind::Supporter,
        effect: TrainerEffect::SwitchOpponentActive,
    }));
    let poke_pad = db.add(CardDef::Trainer(Trainer {
        print_id: "test-poke-pad",
        name: "Poke Pad",
        kind: TrainerKind::Item,
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            to: Zone::Hand,
            filter: CardFilter::PokemonWithoutRuleBox,
            limit: 1,
            then: None,
        },
    }));
    let night_stretcher = db.add(CardDef::Trainer(Trainer {
        print_id: "test-night-stretcher",
        name: "Night Stretcher",
        kind: TrainerKind::Item,
        effect: TrainerEffect::Decide {
            from: Zone::Discard,
            to: Zone::Hand,
            filter: CardFilter::PokemonOrBasicEnergy,
            limit: 1,
            then: None,
        },
    }));
    let sacred_ash = db.add(CardDef::Trainer(Trainer {
        print_id: "test-sacred-ash",
        name: "Sacred Ash",
        kind: TrainerKind::Item,
        effect: TrainerEffect::Decide {
            from: Zone::Discard,
            to: Zone::Library,
            filter: CardFilter::AnyPokemon,
            limit: 5,
            then: None,
        },
    }));
    let gwynn = db.add(CardDef::Trainer(Trainer {
        print_id: "test-gwynn",
        name: "Gwynn",
        kind: TrainerKind::Supporter,
        effect: TrainerEffect::Decide {
            from: Zone::Hand,
            to: Zone::Discard,
            filter: CardFilter::PokemonWithoutRuleBox,
            limit: 2,
            then: Some(Then::DrawPerCardMoved(3)),
        },
    }));
    let crushing_hammer = db.add(CardDef::Trainer(Trainer {
        print_id: "test-crushing-hammer",
        name: "Crushing Hammer",
        kind: TrainerKind::Item,
        effect: TrainerEffect::CoinFlipDiscardOpponentEnergy,
    }));
    Set2 {
        db,
        mon,
        mon_ex,
        energy,
        boss,
        poke_pad,
        night_stretcher,
        sacred_ash,
        gwynn,
        crushing_hammer,
    }
}

fn deck2(set: &Set2, extra: CardDefId) -> Vec<CardDefId> {
    let mut decklist = vec![set.mon; 6];
    decklist.push(set.mon_ex);
    decklist.push(extra);
    while decklist.len() < 60 {
        decklist.push(set.energy);
    }
    decklist
}

fn game2(set: &Set2, extra: CardDefId, seed: u64) -> GameState {
    let decklist = deck2(set, extra);
    let mut state = GameState::new(
        set.db.clone(),
        [decklist.clone(), decklist],
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

fn ensure_in_hand2(state: &mut GameState, player: PlayerId, def: CardDefId) -> sim::ids::CardId {
    if let Some(card) = state
        .player(player)
        .hand
        .iter()
        .find(|c| state.cards[c.index()].def == def)
    {
        return *card;
    }
    let card = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.cards[c.index()].def == def)
        .expect("the deck holds this card");
    state.players[player.index()].library.retain(|c| *c != card);
    state.players[player.index()].hand.push(card);
    card
}

#[test]
fn boss_orders_switches_the_opponents_active() {
    let set = build2();
    let mut state = game2(&set, set.boss, 3);
    let player = state.current;
    let card = ensure_in_hand2(&mut state, player, set.boss);
    let opponent_bench = state
        .player(player.opponent())
        .bench
        .first()
        .copied()
        .expect("setup filled the Bench");

    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    assert_eq!(
        state.phase,
        Phase::Promoting {
            of: player.opponent(),
            chooser: player,
        }
    );
    apply(
        &mut state,
        Action::Promote {
            pokemon: opponent_bench,
        },
    )
    .unwrap();
    assert_eq!(state.player(player.opponent()).active, Some(opponent_bench));
}

#[test]
fn boss_orders_is_not_playable_with_no_benched_opponent() {
    let set = build2();
    let mut state = game2(&set, set.boss, 3);
    let player = state.current;
    let card = ensure_in_hand2(&mut state, player, set.boss);
    state.players[player.opponent().index()].bench.clear();

    assert!(
        !legal_actions(&state).contains(&Action::PlayTrainer { card }),
        "nowhere to switch to"
    );
}

#[test]
fn poke_pad_finds_a_pokemon_without_a_rule_box_but_not_one_with() {
    let set = build2();
    let mut state = game2(&set, set.poke_pad, 3);
    let player = state.current;
    let card = ensure_in_hand2(&mut state, player, set.poke_pad);
    apply(&mut state, Action::PlayTrainer { card }).unwrap();

    let offered: Vec<_> = legal_actions(&state)
        .into_iter()
        .filter_map(|a| match a {
            Action::TakeCard { card } => Some(card),
            _ => None,
        })
        .collect();
    assert!(
        offered
            .iter()
            .all(|c| state.cards[c.index()].def != set.mon_ex),
        "Testmon ex has a Rule Box and is never offered"
    );
    assert!(
        offered
            .iter()
            .any(|c| state.cards[c.index()].def == set.mon),
        "Testmon has none and is offered"
    );
}

#[test]
fn night_stretcher_recovers_a_pokemon_or_a_basic_energy() {
    let set = build2();
    let mut state = game2(&set, set.night_stretcher, 3);
    let player = state.current;
    let discarded_mon = state.player(player).library[0];
    state.players[player.index()]
        .library
        .retain(|c| *c != discarded_mon);
    state.players[player.index()].discard.push(discarded_mon);

    let card = ensure_in_hand2(&mut state, player, set.night_stretcher);
    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(
        &mut state,
        Action::TakeCard {
            card: discarded_mon,
        },
    )
    .unwrap();
    // remaining reaches 0 without auto-finishing (ADR 0012): the choice to
    // stop is still the player's, same as PlacingBench.
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert!(state.player(player).hand.contains(&discarded_mon));
    assert_eq!(state.phase, Phase::Main);
}

#[test]
fn sacred_ash_shuffles_up_to_five_pokemon_back_in() {
    let set = build2();
    let mut state = game2(&set, set.sacred_ash, 3);
    let player = state.current;
    let discarded = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.def_of(**c).as_pokemon().is_some())
        .expect("the deck holds Pokémon");
    state.players[player.index()]
        .library
        .retain(|c| *c != discarded);
    state.players[player.index()].discard.push(discarded);

    let card = ensure_in_hand2(&mut state, player, set.sacred_ash);
    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::TakeCard { card: discarded }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert!(state.player(player).library.contains(&discarded));
    assert!(!state.player(player).discard.contains(&discarded));
}

#[test]
fn gwynn_discards_up_to_two_and_draws_three_each() {
    let set = build2();
    let mut state = game2(&set, set.gwynn, 3);
    let player = state.current;
    let extra_mon = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.cards[c.index()].def == set.mon)
        .expect("another Testmon is in the deck");
    state.players[player.index()]
        .library
        .retain(|c| *c != extra_mon);
    state.players[player.index()].hand.push(extra_mon);

    let card = ensure_in_hand2(&mut state, player, set.gwynn);
    let hand_before = state.player(player).hand.len();
    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    apply(&mut state, Action::TakeCard { card: extra_mon }).unwrap();
    apply(&mut state, Action::FinishDeciding).unwrap();

    assert!(state.player(player).discard.contains(&extra_mon));
    // Played the Supporter (-1), discarded one Pokémon (-1), drew 3: net +1.
    assert_eq!(state.player(player).hand.len(), hand_before - 2 + 3);
}

#[test]
fn crushing_hammer_discards_opponent_energy_on_heads() {
    use sim::rng::ScriptedRng;
    let set = build2();
    let decklist = deck2(&set, set.crushing_hammer);
    let mut state = GameState::new(
        set.db.clone(),
        [decklist.clone(), decklist],
        Box::new(ScriptedRng::new(vec![1])),
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

    let player = state.current;
    let opp_active = state.player(player.opponent()).active.unwrap();
    let energy = *state
        .player(player.opponent())
        .hand
        .iter()
        .chain(state.player(player.opponent()).library.iter())
        .find(|c| state.def_of(**c).is_energy())
        .expect("the opponent's deck holds Energy");
    state.players[player.opponent().index()]
        .hand
        .retain(|c| *c != energy);
    state.players[player.opponent().index()]
        .library
        .retain(|c| *c != energy);
    state.pokemon[opp_active.index()].attached.push(energy);

    let card = ensure_in_hand2(&mut state, player, set.crushing_hammer);
    apply(&mut state, Action::PlayTrainer { card }).unwrap();
    assert_eq!(
        state.phase,
        Phase::DiscardingOpponentEnergy {
            chooser: player,
            of: player.opponent(),
        }
    );
    apply(&mut state, Action::DiscardOpponentEnergy { card: energy }).unwrap();

    assert!(!state.pokemon(opp_active).attached.contains(&energy));
    assert!(state.player(player.opponent()).discard.contains(&energy));
}
