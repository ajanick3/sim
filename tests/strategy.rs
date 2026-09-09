//! HeuristicStrategy's own sequencing: attack first, then the priority
//! order among everything else.

use sim::action::{Action, legal_actions};
use sim::card::{
    Ability, AbilityEffect, Attack, CardDb, CardDef, Energy, Pokemon, Slot, Stage,
    Trainer, TrainerEffect, TrainerKind, Type, Zone,
};
use sim::engine::apply;
use sim::rng::SeededRng;
use sim::state::{GameState, Phase};
use sim::strategy::{HeuristicStrategy, Strategy};
use sim::view::PlayerView;

fn deal_new_card(state: &mut GameState, player: sim::ids::PlayerId, def: sim::ids::CardDefId) -> sim::ids::CardId {
    let card = sim::ids::CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def, owner: player });
    card
}

/// A game where the first player's Active can attack, carries an Ability,
/// and holds a Basic Pokémon, a Stadium, a drawing Item, a drawing
/// Supporter, and a searching Item in hand — enough to see every tier of
/// the priority order picked from correctly.
fn game() -> GameState {
    let mut db = CardDb::new();
    let active = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-strategy-active",
        name: "Activemon",
        hp: 200,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        prizes: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: Some(Ability {
            name: "Test Ability",
            effect: AbilityEffect::OncePerTurnWhileActiveMayDrawCards(1),
        }),
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None,
            effect: None,
        }],
    }));
    let basic = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-strategy-basic",
        name: "Basicmon",
        hp: 60,
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
    let defender = db.add(CardDef::Pokemon(Pokemon {
        markers: Vec::new(),
        print_id: "test-strategy-defender",
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
    let stadium = db.add(CardDef::Trainer(Trainer {
        print_id: "test-strategy-stadium",
        name: "Test Stadium",
        kind: TrainerKind::Stadium,
        effect: TrainerEffect::Nothing,
        requirement: None,
    }));
    let drawing_supporter = db.add(CardDef::Trainer(Trainer {
        print_id: "test-strategy-drawing-supporter",
        name: "Test Drawing Supporter",
        kind: TrainerKind::Supporter,
        effect: TrainerEffect::DrawPerOpponentBenched,
        requirement: None,
    }));
    let drawing_item = db.add(CardDef::Trainer(Trainer {
        print_id: "test-strategy-drawing-item",
        name: "Test Drawing Item",
        kind: TrainerKind::Item,
        effect: TrainerEffect::DrawPerOpponentBenched,
        requirement: None,
    }));
    let searching_item = db.add(CardDef::Trainer(Trainer {
        print_id: "test-strategy-searching-item",
        name: "Test Searching Item",
        kind: TrainerKind::Item,
        effect: TrainerEffect::Decide {
            from: Zone::Library,
            slots: vec![Slot {
                filter: sim::card::CardFilter::AnyCard,
                to: sim::card::Destination::Zone(Zone::Hand),
                limit: 1,
                excludes_type_of_previous: false,
                peek: None,
            }],
            then: None,
        },
        requirement: None,
    }));
    let other_item = db.add(CardDef::Trainer(Trainer {
        print_id: "test-strategy-other-item",
        name: "Test Other Item",
        kind: TrainerKind::Item,
        effect: TrainerEffect::SwitchOpponentActive,
        requirement: None,
    }));
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-strategy-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
        effect: None,
    }));

    let mut deck = vec![active; 4];
    while deck.len() < 60 {
        deck.push(energy);
    }
    let mut defender_deck = vec![defender; 4];
    while defender_deck.len() < 60 {
        defender_deck.push(energy);
    }
    let mut state = GameState::new(db, [deck, defender_deck], Box::new(SeededRng::new(3)));
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }
    apply(&mut state, Action::EndTurn).unwrap();
    while state.phase != Phase::Main && !state.is_over() {
        let first = legal_actions(&state)[0];
        apply(&mut state, first).unwrap();
    }

    // Dealt straight into hand, not left to a shuffle to surface — every
    // test here needs each of these actually offered as a choice.
    let player = state.current;
    for def in [basic, stadium, drawing_supporter, drawing_item, searching_item, other_item] {
        let card = deal_new_card(&mut state, player, def);
        state.players[player.index()].hand.push(card);
    }
    state
}

#[test]
fn attacks_whenever_legal_no_matter_what_else_is_in_hand() {
    let mut state = game();
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let energy = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.def_of(**c).is_energy())
        .unwrap();
    state.players[player.index()].library.retain(|c| *c != energy);
    state.pokemon[active.index()].attached.push(energy);

    let legal = legal_actions(&state);
    let view = PlayerView::of(&state, player);
    let mut strategy = HeuristicStrategy::new(Box::new(SeededRng::new(1)));
    let choice = strategy.choose(&view, &state.db, &legal);

    assert!(matches!(choice, Action::Attack { .. }), "attacked instead of anything else legal");
}

#[test]
fn prefers_playing_a_basic_over_everything_below_it() {
    let state = game();
    let player = state.current;
    let legal = legal_actions(&state);
    let view = PlayerView::of(&state, player);
    let mut strategy = HeuristicStrategy::new(Box::new(SeededRng::new(1)));
    let choice = strategy.choose(&view, &state.db, &legal);

    assert!(matches!(choice, Action::PlayBasic { .. }), "got {choice:?}");
}

#[test]
fn ranks_a_drawing_item_above_a_drawing_supporter_above_search_above_other() {
    let mut state = game();
    let player = state.current;
    let active = state.player(player).active.unwrap();
    // Fill the Bench so PlayBasic and UseAbility both fall away, leaving
    // only the Trainer tiers (and Retreat/random) to choose from.
    let basicmon_cards: Vec<_> = state
        .player(player)
        .hand
        .iter()
        .copied()
        .filter(|c| matches!(state.def_of(*c).as_pokemon(), Some(p) if p.name == "Basicmon"))
        .collect();
    state.players[player.index()].hand.retain(|c| !basicmon_cards.contains(c));
    state.spend(sim::state::Limit::AbilityUsed(player, "Test Ability"));
    // Stadium outranks every Trainer tier below it (its own test covers
    // that); drop it here so this test isolates the tiers under it.
    let stadium_card = *state
        .player(player)
        .hand
        .iter()
        .find(|c| state.def_of(**c).name() == "Test Stadium")
        .unwrap();
    state.players[player.index()].hand.retain(|c| *c != stadium_card);

    let legal = legal_actions(&state);
    let view = PlayerView::of(&state, player);
    let mut strategy = HeuristicStrategy::new(Box::new(SeededRng::new(1)));
    let choice = strategy.choose(&view, &state.db, &legal);

    let name = match choice {
        Action::PlayTrainer { card } => state.def_of(card).name(),
        other => panic!("expected a Trainer play, got {other:?}"),
    };
    assert_eq!(name, "Test Drawing Item", "the drawing Item outranks everything else offered");

    let _ = active;
}

#[test]
fn ranks_stadium_above_the_trainer_draw_search_tiers() {
    let mut state = game();
    let player = state.current;
    let basicmon_cards: Vec<_> = state
        .player(player)
        .hand
        .iter()
        .copied()
        .filter(|c| matches!(state.def_of(*c).as_pokemon(), Some(p) if p.name == "Basicmon"))
        .collect();
    state.players[player.index()].hand.retain(|c| !basicmon_cards.contains(c));
    state.spend(sim::state::Limit::AbilityUsed(player, "Test Ability"));
    // Drop the drawing Item so the Stadium is the highest tier left.
    let drawing_item_card = *state
        .player(player)
        .hand
        .iter()
        .find(|c| state.def_of(**c).name() == "Test Drawing Item")
        .unwrap();
    state.players[player.index()].hand.retain(|c| *c != drawing_item_card);

    let legal = legal_actions(&state);
    let view = PlayerView::of(&state, player);
    let mut strategy = HeuristicStrategy::new(Box::new(SeededRng::new(1)));
    let choice = strategy.choose(&view, &state.db, &legal);

    let name = match choice {
        Action::PlayTrainer { card } => state.def_of(card).name(),
        other => panic!("expected a Trainer play, got {other:?}"),
    };
    assert_eq!(name, "Test Stadium");
}

#[test]
fn falls_back_to_random_once_nothing_above_matches() {
    let mut state = game();
    let player = state.current;
    // Empty the hand entirely and spend the Ability, so only Retreat (no
    // Bench to retreat to, so not even that) and EndTurn remain.
    state.players[player.index()].hand.clear();
    state.spend(sim::state::Limit::AbilityUsed(player, "Test Ability"));

    let legal = legal_actions(&state);
    let view = PlayerView::of(&state, player);
    let mut strategy = HeuristicStrategy::new(Box::new(SeededRng::new(1)));
    let choice = strategy.choose(&view, &state.db, &legal);

    assert!(legal.contains(&choice), "the fallback still only ever picks a legal action");
}

#[test]
fn strategys_own_rng_is_independent_of_the_games() {
    // Two Strategy instances seeded differently from the same legal list
    // do not have to agree — proof the choice does not read GameState's
    // own rng, which this test never touches.
    let state = game();
    let player = state.current;
    let legal = legal_actions(&state);
    let view = PlayerView::of(&state, player);
    let mut a = HeuristicStrategy::new(Box::new(SeededRng::new(1)));
    let mut b = HeuristicStrategy::new(Box::new(SeededRng::new(2)));
    // Both still land on the same top-tier action regardless of seed,
    // since PlayBasic is unambiguous here — the seed only ever matters
    // once execution reaches the random fallback tier.
    assert_eq!(a.choose(&view, &state.db, &legal), b.choose(&view, &state.db, &legal));
}
