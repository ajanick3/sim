//! Milestone 9: every Tool in the field. One fixture, shared across this
//! milestone's tickets, the same way `items.rs` served Milestone 8.

use sim::action::{Action, legal_actions};
use sim::card::{
    Attack, CardDb, CardDef, CardFilter, Destination, Energy, Pokemon, PromoteFollowUp,
    Requirement, Slot, Stage, TargetFilter, Trainer, TrainerEffect, TrainerKind, TurnBonusTarget,
    Type, Zone,
};
use sim::engine::apply;
use sim::ids::{CardDefId, CardId, PlayerId, PokemonId};
use sim::rng::SeededRng;
use sim::state::GameState;
use sim::state::Phase;

/// A pool wide enough for this milestone's filters: an ordinary Basic, a
/// Basic worth two Prizes, a Stage 1, a Stage 2, and a card with a Rule Box
/// but no evolution line (an ex Basic already covers that).
struct Set {
    db: CardDb,
    mon: CardDefId,
    mon_ex: CardDefId,
    stage1: CardDefId,
    stage2: CardDefId,
    energy: CardDefId,
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
        ability: None,
        attacks: vec![Attack {
            name: "Tackle",
            cost: vec![Type::Colorless],
            base_damage: 10,
            inflicts: None, effect: None,
        }],
    }))
}

fn build() -> Set {
    let mut db = CardDb::new();
    let mon = basic(&mut db, "test-mon", "Testmon", 100, 1, None);
    let mon_ex = basic(&mut db, "test-mon-ex", "Testmon ex", 200, 2, None);
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
        evolves_from_basic: Some("Smallmon"),
        ability: None,
        attacks: vec![Attack {
            name: "Slam",
            cost: vec![Type::Colorless, Type::Colorless],
            base_damage: 60,
            inflicts: None, effect: None,
        }],
    }));
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-energy",
        name: "Colorless Energy",
        kind: Type::Colorless,
    }));
    Set {
        db,
        mon,
        mon_ex,
        stage1,
        stage2,
        energy,
    }
}

/// A deck holding a few of each card, then Energy to sixty.
fn deck(set: &Set, extra: CardDefId) -> Vec<CardDefId> {
    let mut decklist = vec![set.mon; 6];
    decklist.extend([set.mon_ex; 4]);
    decklist.extend([set.stage1; 4]);
    decklist.extend([set.stage2; 2]);
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

/// Put a copy of `def` in this player's hand, taking it from wherever the
/// deal put it.
fn ensure_in_hand(state: &mut GameState, player: PlayerId, def: CardDefId) -> CardId {
    if let Some(card) = state
        .player(player)
        .hand
        .iter()
        .find(|c| state.cards[c.index()].def == def)
    {
        return *card;
    }
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

/// A physical card of `def`, for a definition that was never part of the
/// sixty-card decklist. Placed nowhere; the caller pushes it to whatever
/// zone the test needs.
fn deal_new_card(state: &mut GameState, player: PlayerId, def: CardDefId) -> CardId {
    let card = CardId(state.cards.len() as u32);
    state.cards.push(sim::state::Card { def, owner: player });
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

/// Put a copy of `def` into the discard pile, taking it from the library.
fn deal_to_discard(state: &mut GameState, player: PlayerId, def: CardDefId) -> CardId {
    let card = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.cards[c.index()].def == def)
        .expect("the deck holds this card");
    state.players[player.index()].library.retain(|c| *c != card);
    state.players[player.index()].discard.push(card);
    card
}


// --- Ticket 01: the attach mechanism ---

fn with_a_tool(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-a-tool",
        name: "A Tool",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::Nothing,
    }));
    (Set { db, ..set }, card)
}

#[test]
fn a_tool_attaches_immediately_with_a_target_named_at_play_time() {
    let (set, tool) = with_a_tool(build());
    let mut state = game(&set, tool, 3);
    let player = state.current;
    let card = ensure_in_hand(&mut state, player, tool);
    let target = state.player(player).active.unwrap();

    assert!(legal_actions(&state).contains(&Action::PlayTool { card, target }));
    apply(&mut state, Action::PlayTool { card, target }).unwrap();

    assert!(state.pokemon(target).attached.contains(&card));
    assert!(!state.player(player).hand.contains(&card));
    assert_eq!(state.phase, Phase::Main, "a Tool needs no phase, like Energy");
}

#[test]
fn a_pokemon_carries_at_most_one_tool() {
    let (set, tool) = with_a_tool(build());
    let mut state = game(&set, tool, 3);
    let player = state.current;
    let first = ensure_in_hand(&mut state, player, tool);
    let target = state.player(player).active.unwrap();
    apply(&mut state, Action::PlayTool { card: first, target }).unwrap();

    let second = deal_new_card(&mut state, player, tool);
    state.players[player.index()].hand.push(second);
    assert!(
        !legal_actions(&state).contains(&Action::PlayTool { card: second, target }),
        "the target already carries a Tool"
    );
    // A different, bare Pokémon may still take one.
    let bare = state.player(player).bench[0];
    assert!(legal_actions(&state).contains(&Action::PlayTool { card: second, target: bare }));
}

#[test]
fn a_tool_is_unlimited_per_turn_unlike_energy() {
    let (set, tool) = with_a_tool(build());
    let mut state = game(&set, tool, 3);
    let player = state.current;
    let first = ensure_in_hand(&mut state, player, tool);
    let first_target = state.player(player).active.unwrap();
    apply(&mut state, Action::PlayTool { card: first, target: first_target }).unwrap();

    let second = deal_new_card(&mut state, player, tool);
    state.players[player.index()].hand.push(second);
    let second_target = state.player(player).bench[0];
    assert!(
        legal_actions(&state).contains(&Action::PlayTool { card: second, target: second_target }),
        "no once-per-turn limit on playing a Tool"
    );
}

// --- Ticket 02: Air Balloon ---

fn with_air_balloon(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-air-balloon",
        name: "Air Balloon",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::ReducesRetreatCost(2),
    }));
    (Set { db, ..set }, card)
}

#[test]
fn air_balloon_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Air Balloon")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Air Balloon should play");
}

#[test]
fn air_balloon_reduces_the_retreat_cost_by_two() {
    let (set, balloon) = with_air_balloon(build());
    let mut state = game(&set, balloon, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    // The fixture's ordinary Basic prints a Retreat Cost of 1.
    assert_eq!(state.effective_retreat_cost(active), 1);

    let card = ensure_in_hand(&mut state, player, balloon);
    apply(&mut state, Action::PlayTool { card, target: active }).unwrap();

    assert_eq!(
        state.effective_retreat_cost(active),
        0,
        "floored at zero, not negative"
    );
    assert!(
        legal_actions(&state).contains(&Action::Retreat { to: state.player(player).bench[0] }),
        "retreating for free is now legal with no Energy attached"
    );
}

#[test]
fn air_balloon_does_nothing_for_a_pokemon_it_is_not_attached_to() {
    let (set, balloon) = with_air_balloon(build());
    let mut state = game(&set, balloon, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let bench_mon = state.player(player).bench[0];
    let card = ensure_in_hand(&mut state, player, balloon);
    apply(&mut state, Action::PlayTool { card, target: bench_mon }).unwrap();

    assert_eq!(
        state.effective_retreat_cost(active),
        1,
        "the Active's own cost is untouched"
    );
}

// --- Ticket 03: Hero's Cape ---

fn with_heros_cape(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-heros-cape",
        name: "Hero's Cape",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::IncreasesHp(100),
    }));
    (Set { db, ..set }, card)
}

#[test]
fn heros_cape_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Hero's Cape")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Hero's Cape should play");
}

#[test]
fn heros_cape_adds_a_hundred_effective_hp() {
    let (set, cape) = with_heros_cape(build());
    let mut state = game(&set, cape, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    // The fixture's ordinary Basic prints 100 HP.
    assert_eq!(state.effective_hp(active), 100);
    assert_eq!(state.remaining_hp(active), 100);

    let card = ensure_in_hand(&mut state, player, cape);
    apply(&mut state, Action::PlayTool { card, target: active }).unwrap();

    assert_eq!(state.effective_hp(active), 200);
    assert_eq!(
        state.remaining_hp(active),
        200,
        "no damage taken, so remaining HP grows with the cap"
    );
}

#[test]
fn heros_cape_keeps_a_pokemon_alive_past_its_printed_hp() {
    let (set, cape) = with_heros_cape(build());
    let mut state = game(&set, cape, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let card = ensure_in_hand(&mut state, player, cape);
    apply(&mut state, Action::PlayTool { card, target: active }).unwrap();

    // Damage past the printed 100 HP, but under the effective 200.
    state.pokemon[active.index()].damage = 150;
    assert_eq!(state.remaining_hp(active), 50, "still standing");
}

// --- Ticket 04: Brave Bangle & Binding Mochi ---

fn with_brave_bangle(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-brave-bangle",
        name: "Brave Bangle",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::BonusDamageWithoutRuleBoxVsEx(30),
    }));
    (Set { db, ..set }, card)
}

fn with_binding_mochi(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-binding-mochi",
        name: "Binding Mochi",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::BonusDamageIfPoisonedVsActive(40),
    }));
    (Set { db, ..set }, card)
}

#[test]
fn brave_bangle_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Brave Bangle")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Brave Bangle should play");
}

#[test]
fn brave_bangle_adds_damage_only_without_a_rule_box_against_an_ex() {
    let (set, bangle) = with_brave_bangle(build());
    let mut state = game(&set, bangle, 3);
    let player = state.current;
    let opponent = player.opponent();
    let attacker = state.player(player).active.unwrap();
    let defender = state.player(opponent).active.unwrap();

    let card = ensure_in_hand(&mut state, player, bangle);
    apply(&mut state, Action::PlayTool { card, target: attacker }).unwrap();

    // The defender is an ordinary Pokémon (1 Prize) by default: no bonus.
    assert_eq!(sim::engine::damage_dealt(&state, attacker, defender, 100), 100);

    // Replace the defender with an ex.
    let ex_card = *state
        .player(opponent)
        .library
        .iter()
        .find(|c| state.cards[c.index()].def == set.mon_ex)
        .unwrap();
    state.players[opponent.index()].library.retain(|c| *c != ex_card);
    state.pokemon[defender.index()].cards = vec![ex_card];
    assert_eq!(sim::engine::damage_dealt(&state, attacker, defender, 100), 130);
}

#[test]
fn brave_bangle_adds_nothing_carrying_a_rule_box_itself() {
    let (set, bangle) = with_brave_bangle(build());
    let mut state = game(&set, bangle, 3);
    let player = state.current;
    let opponent = player.opponent();
    let attacker = state.player(player).active.unwrap();
    let defender = state.player(opponent).active.unwrap();

    // The attacker itself becomes an ex — a Rule Box.
    let ex_card = *state
        .player(player)
        .library
        .iter()
        .find(|c| state.cards[c.index()].def == set.mon_ex)
        .unwrap();
    state.players[player.index()].library.retain(|c| *c != ex_card);
    state.pokemon[attacker.index()].cards = vec![ex_card];

    let card = ensure_in_hand(&mut state, player, bangle);
    apply(&mut state, Action::PlayTool { card, target: attacker }).unwrap();

    let defender_ex = *state
        .player(opponent)
        .library
        .iter()
        .find(|c| state.cards[c.index()].def == set.mon_ex)
        .unwrap();
    state.players[opponent.index()].library.retain(|c| *c != defender_ex);
    state.pokemon[defender.index()].cards = vec![defender_ex];

    assert_eq!(
        sim::engine::damage_dealt(&state, attacker, defender, 100),
        100,
        "the attacker's own Rule Box turns off the bonus"
    );
}

#[test]
fn binding_mochi_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Binding Mochi")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Binding Mochi should play");
}

#[test]
fn binding_mochi_adds_damage_only_while_poisoned() {
    let (set, mochi) = with_binding_mochi(build());
    let mut state = game(&set, mochi, 3);
    let player = state.current;
    let opponent = player.opponent();
    let attacker = state.player(player).active.unwrap();
    let defender = state.player(opponent).active.unwrap();

    let card = ensure_in_hand(&mut state, player, mochi);
    apply(&mut state, Action::PlayTool { card, target: attacker }).unwrap();
    assert_eq!(sim::engine::damage_dealt(&state, attacker, defender, 100), 100);

    state.inflict(attacker, sim::card::Condition::Poisoned);
    assert_eq!(sim::engine::damage_dealt(&state, attacker, defender, 100), 140);
}

// --- Ticket 05: Lillie's Pearl ---

/// A game where a Lillie's-named Basic, worth 1 Prize, carries
/// `Lillie's Pearl` and dies to one attack from the opponent's Basic.
fn lillies_pearl_game(with_pearl: bool) -> (GameState, u32) {
    let mut db = CardDb::new();
    let lillies_mon = db.add(CardDef::Pokemon(Pokemon {
        print_id: "test-lillies-mon",
        name: "Lillie's Testmon",
        hp: 30,
        kind: Type::Colorless,
        weakness: None,
        resistance: None,
        retreat_cost: 1,
        stage: Stage::Basic,
        evolve_from: None,
        evolves_from_basic: None,
        ability: None,
        prizes: 1,
        attacks: vec![Attack {
            name: "Nudge",
            cost: vec![Type::Colorless],
            base_damage: 30,
            inflicts: None, effect: None,
        }],
    }));
    let energy = db.add(CardDef::Energy(Energy {
        print_id: "test-colorless-energy-lp",
        name: "Colorless Energy",
        kind: Type::Colorless,
    }));
    let pearl = db.add(CardDef::Trainer(Trainer {
        print_id: "test-lillies-pearl",
        name: "Lillie's Pearl",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::FewerPrizeIfLilliesKnockedOutByAttack,
    }));

    let mut decklist = vec![lillies_mon; 12];
    while decklist.len() < 60 {
        decklist.push(energy);
    }
    let mut state = GameState::new(
        db,
        [decklist.clone(), decklist],
        Box::new(SeededRng::new(9)),
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

    let attacker_player = state.current;
    let defender_player = attacker_player.opponent();
    let defender = state.player(defender_player).active.unwrap();
    if with_pearl {
        let pearl_card = deal_new_card(&mut state, defender_player, pearl);
        state.pokemon[defender.index()].attached.push(pearl_card);
    }

    // Pay for the attack.
    let active = state.player(attacker_player).active.unwrap();
    let side = state.player(attacker_player);
    let card = side
        .hand
        .iter()
        .chain(side.library.iter())
        .find(|c| state.def_of(**c).is_energy())
        .copied()
        .expect("the deck is mostly Energy");
    state.remove_from_hand(attacker_player, card);
    state.players[attacker_player.index()].library.retain(|c| *c != card);
    state.pokemon[active.index()].attached.push(card);

    let before = state.player(attacker_player).prizes.len() as u32;
    let attack = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("a paid-for Active can attack");
    apply(&mut state, attack).unwrap();
    let after = state.player(attacker_player).prizes.len() as u32;
    (state, before - after)
}

#[test]
fn lillies_pearl_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Lillie's Pearl")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Lillie's Pearl should play");
}

#[test]
fn lillies_pearl_takes_one_fewer_prize_when_its_pokemon_is_knocked_out() {
    let (_, taken) = lillies_pearl_game(true);
    assert_eq!(taken, 0, "1 Prize's worth, minus 1, floored at zero");
}

#[test]
fn without_lillies_pearl_the_knockout_takes_its_usual_prize() {
    let (_, taken) = lillies_pearl_game(false);
    assert_eq!(taken, 1);
}

// --- Ticket 06: Punk Helmet & Lucky Helmet ---

fn with_punk_helmet(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-punk-helmet",
        name: "Punk Helmet",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::DamagesAttackerWhenDefenderIsHit(40),
    }));
    (Set { db, ..set }, card)
}

fn with_lucky_helmet(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-lucky-helmet",
        name: "Lucky Helmet",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::DrawsWhenDefenderIsHit(2),
    }));
    (Set { db, ..set }, card)
}

/// Pay for and use the Active's first attack.
fn pay_and_attack(state: &mut GameState, player: sim::ids::PlayerId) {
    let active = state.player(player).active.unwrap();
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
    let attack = legal_actions(state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("a paid-for Active can attack");
    apply(state, attack).unwrap();
}

#[test]
fn punk_helmet_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Punk Helmet")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Punk Helmet should play");
}

#[test]
fn punk_helmet_damages_the_attacker_back() {
    let (set, helmet) = with_punk_helmet(build());
    let mut state = game(&set, helmet, 3);
    let attacker_player = state.current;
    let defender_player = attacker_player.opponent();
    let attacker = state.player(attacker_player).active.unwrap();
    let defender = state.player(defender_player).active.unwrap();

    let card = deal_new_card(&mut state, defender_player, helmet);
    state.pokemon[defender.index()].attached.push(card);

    pay_and_attack(&mut state, attacker_player);

    assert_eq!(state.pokemon(attacker).damage, 40);
}

#[test]
fn lucky_helmet_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Lucky Helmet")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Lucky Helmet should play");
}

#[test]
fn lucky_helmet_draws_two_for_its_owner() {
    // The Active's turn ends with this attack, so the defender's own
    // turn — and its ordinary turn-start draw — starts in the same
    // `settle` call. Comparing against an identical game with no Lucky
    // Helmet attached isolates the card's own +2 from that draw, which
    // both games make identically off the same seed.
    let (set, helmet) = with_lucky_helmet(build());
    let without = {
        let mut state = game(&set, helmet, 3);
        let attacker_player = state.current;
        let defender_player = attacker_player.opponent();
        pay_and_attack(&mut state, attacker_player);
        state.player(defender_player).hand.len()
    };
    let with = {
        let mut state = game(&set, helmet, 3);
        let attacker_player = state.current;
        let defender_player = attacker_player.opponent();
        let defender = state.player(defender_player).active.unwrap();
        let card = deal_new_card(&mut state, defender_player, helmet);
        state.pokemon[defender.index()].attached.push(card);
        pay_and_attack(&mut state, attacker_player);
        state.player(defender_player).hand.len()
    };
    assert_eq!(with, without + 2, "Lucky Helmet draws 2 on top of the ordinary turn draw");
}

// --- Ticket 07: Handheld Fan ---

fn with_handheld_fan(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-handheld-fan",
        name: "Handheld Fan",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::MovesEnergyFromAttackerToTheirBench,
    }));
    (Set { db, ..set }, card)
}

#[test]
fn handheld_fan_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Handheld Fan")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Handheld Fan should play");
}

#[test]
fn handheld_fan_opens_a_choice_of_where_the_energy_goes() {
    let (set, fan) = with_handheld_fan(build());
    let mut state = game(&set, fan, 3);
    let attacker_player = state.current;
    let defender_player = attacker_player.opponent();
    let defender = state.player(defender_player).active.unwrap();
    let attacker = state.player(attacker_player).active.unwrap();
    let bench_target = state.player(attacker_player).bench[0];

    let card = deal_new_card(&mut state, defender_player, fan);
    state.pokemon[defender.index()].attached.push(card);

    let side = state.player(attacker_player);
    let energy = side
        .hand
        .iter()
        .chain(side.library.iter())
        .find(|c| state.def_of(**c).is_energy())
        .copied()
        .expect("the deck holds Energy");
    state.remove_from_hand(attacker_player, energy);
    state.players[attacker_player.index()].library.retain(|c| *c != energy);
    state.pokemon[attacker.index()].attached.push(energy);

    let attack = legal_actions(&state)
        .into_iter()
        .find(|a| matches!(a, Action::Attack { .. }))
        .expect("a paid-for Active can attack");
    apply(&mut state, attack).unwrap();

    assert!(matches!(
        state.phase,
        Phase::MovingEnergyForHandheldFan { .. }
    ));
    assert_eq!(sim::action::player_to_act(&state), Some(defender_player));
    assert!(legal_actions(&state).contains(&Action::MoveEnergyForHandheldFan {
        card: energy,
        target: bench_target,
    }));

    apply(
        &mut state,
        Action::MoveEnergyForHandheldFan { card: energy, target: bench_target },
    )
    .unwrap();

    assert_eq!(state.phase, Phase::Main, "the deferred attack resolution finally runs");
    assert!(!state.pokemon(attacker).attached.contains(&energy));
    assert!(state.pokemon(bench_target).attached.contains(&energy));
}

#[test]
fn handheld_fan_does_nothing_with_no_bench_to_receive_it() {
    let (set, fan) = with_handheld_fan(build());
    let mut state = game(&set, fan, 3);
    let attacker_player = state.current;
    let defender_player = attacker_player.opponent();
    let defender = state.player(defender_player).active.unwrap();

    let card = deal_new_card(&mut state, defender_player, fan);
    state.pokemon[defender.index()].attached.push(card);
    // The attacker has nowhere on their own Bench to receive the Energy.
    state.players[attacker_player.index()].bench.clear();

    pay_and_attack(&mut state, attacker_player);

    assert_eq!(
        state.phase,
        Phase::Main,
        "no Bench to move the Energy to, so the trigger does nothing"
    );
}

// --- Ticket 08: Powerglass ---

fn with_powerglass(set: Set) -> (Set, CardDefId) {
    let mut db = set.db.clone();
    let card = db.add(CardDef::Trainer(Trainer {
        print_id: "test-powerglass",
        name: "Powerglass",
        kind: TrainerKind::Tool,
        requirement: None,
        effect: TrainerEffect::MayAttachBasicEnergyFromDiscardAtTurnEnd,
    }));
    (Set { db, ..set }, card)
}

#[test]
fn powerglass_is_admitted_from_the_artifact() {
    let import = sim::import::load(
        &std::fs::read_to_string("data/cards.json").expect("the artifact is committed"),
    )
    .unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Powerglass")
        .expect("the artifact holds this card");
    assert!(card.playable.is_some(), "Powerglass should play");
}

#[test]
fn powerglass_offers_a_basic_energy_from_discard_at_turn_end() {
    let (set, glass) = with_powerglass(build());
    let mut state = game(&set, glass, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let card = ensure_in_hand(&mut state, player, glass);
    apply(&mut state, Action::PlayTool { card, target: active }).unwrap();

    let energy = deal_to_discard(&mut state, player, set.energy);

    apply(&mut state, Action::EndTurn).unwrap();
    assert!(matches!(
        state.phase,
        Phase::AttachingFromDiscardForPowerglass { .. }
    ));
    assert_eq!(sim::action::player_to_act(&state), Some(player));
    assert!(legal_actions(&state).contains(&Action::AttachFromDiscardForPowerglass { card: energy }));
    assert!(legal_actions(&state).contains(&Action::DeclinePowerglass));

    apply(&mut state, Action::AttachFromDiscardForPowerglass { card: energy }).unwrap();
    assert!(state.pokemon(active).attached.contains(&energy));
    assert!(!state.player(player).discard.contains(&energy));
    // Turn resolution kept going: it is now the opponent's turn.
    assert_ne!(state.current, player);
}

#[test]
fn powerglass_can_be_declined() {
    let (set, glass) = with_powerglass(build());
    let mut state = game(&set, glass, 3);
    let player = state.current;
    let active = state.player(player).active.unwrap();
    let card = ensure_in_hand(&mut state, player, glass);
    apply(&mut state, Action::PlayTool { card, target: active }).unwrap();
    let energy = deal_to_discard(&mut state, player, set.energy);

    apply(&mut state, Action::EndTurn).unwrap();
    apply(&mut state, Action::DeclinePowerglass).unwrap();

    assert!(state.player(player).discard.contains(&energy), "left in discard");
    assert!(!state.pokemon(active).attached.contains(&energy));
    assert_ne!(state.current, player);
}

#[test]
fn without_powerglass_the_turn_ends_without_a_phase() {
    let (set, glass) = with_powerglass(build());
    let mut state = game(&set, glass, 3);
    let player = state.current;

    apply(&mut state, Action::EndTurn).unwrap();
    assert_ne!(state.current, player, "the turn just ends, no Powerglass to ask about");
}
