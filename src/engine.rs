//! Applying an action to the state.

use crate::action::{Action, legal_actions};
use crate::ids::{PlayerId, PokemonId};
use crate::state::{GameState, Outcome, Phase, WinReason};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IllegalAction;

/// Apply one action. An action outside [`legal_actions`] is refused and the
/// state does not move.
pub fn apply(state: &mut GameState, action: Action) -> Result<(), IllegalAction> {
    if !legal_actions(state).contains(&action) {
        return Err(IllegalAction);
    }

    match action {
        Action::PlayBasic { card } => {
            let player = state.current;
            state.remove_from_hand(player, card);
            let pokemon = state.put_into_play(player, card);
            state.players[player.index()].bench.push(pokemon);
            let name = state.def_of(card).name();
            state.log.push(format!("{player:?} benches {name}."));
        }

        Action::AttachEnergy { card, target } => {
            let player = state.current;
            state.remove_from_hand(player, card);
            state.pokemon[target.index()].attached.push(card);
            state.players[player.index()].energy_attached_this_turn = true;
            let energy = state.def_of(card).name();
            let name = state.pokemon_def(target).name;
            state
                .log
                .push(format!("{player:?} attaches {energy} to {name}."));
        }

        Action::Retreat { to } => retreat(state, to),

        Action::Attack { index } => {
            attack(state, index);
            state.pending_end_turn = true;
            settle(state);
        }

        Action::EndTurn => {
            state.pending_end_turn = true;
            settle(state);
        }

        Action::Promote { pokemon } => {
            let player = match state.phase {
                Phase::Promoting(player) => player,
                _ => return Err(IllegalAction),
            };
            let side = &mut state.players[player.index()];
            side.bench.retain(|p| *p != pokemon);
            side.active = Some(pokemon);
            state.phase = Phase::Main;
            let name = state.pokemon_def(pokemon).name;
            state.log.push(format!("{player:?} promotes {name}."));
            settle(state);
        }
    }

    Ok(())
}

fn retreat(state: &mut GameState, to: PokemonId) {
    let player = state.current;
    let active = state.players[player.index()]
        .active
        .expect("retreating needs an Active");
    let cost = state.pokemon_def(active).retreat_cost as usize;

    // Rule 23: discard Energy equal to the Retreat Cost. Which Energy is a
    // choice; Milestone 1 discards from the front.
    for _ in 0..cost {
        let attached = &mut state.pokemon[active.index()].attached;
        let position = attached
            .iter()
            .position(|c| state.db.get(state.cards[c.index()].def).is_energy());
        if let Some(at) = position {
            let card = attached.remove(at);
            state.players[player.index()].discard.push(card);
        }
    }

    let side = &mut state.players[player.index()];
    side.bench.retain(|p| *p != to);
    side.bench.push(active);
    side.active = Some(to);
    side.retreated_this_turn = true;

    let name = state.pokemon_def(to).name;
    state.log.push(format!("{player:?} retreats to {name}."));
}

/// Rules 29-37, in the order the rulebook sets out.
fn attack(state: &mut GameState, index: usize) {
    let player = state.current;
    let attacker = state.players[player.index()]
        .active
        .expect("attacking needs an Active");
    let Some(defender) = state.players[player.opponent().index()].active else {
        return;
    };

    let attack = state.pokemon_def(attacker).attacks[index].clone();
    let damage = damage_dealt(state, attacker, defender, attack.base_damage);

    state.pokemon[defender.index()].damage += damage;

    let attacker_name = state.pokemon_def(attacker).name;
    let defender_name = state.pokemon_def(defender).name;
    state.log.push(format!(
        "{attacker_name} uses {} on {defender_name} for {damage}.",
        attack.name
    ));
}

/// Step 31 to 35 of the damage order. Milestone 1 has no effects that change
/// damage, so steps 32 and 34 are the identity — the shape is here so a card
/// that does have one has a place to act.
pub fn damage_dealt(state: &GameState, attacker: PokemonId, defender: PokemonId, base: u32) -> u32 {
    let mut damage = base;

    // Step 32: effects on the attacking player's Pokémon. Stop at 0.
    if damage == 0 {
        return 0;
    }

    // Step 33: Weakness, then Resistance. Both read the attacker's type.
    let attacker_type = state.pokemon_def(attacker).kind;
    let defender_def = state.pokemon_def(defender);
    if defender_def.weakness == Some(attacker_type) {
        damage *= 2;
    }
    if defender_def.resistance == Some(attacker_type) {
        damage = damage.saturating_sub(30);
    }

    // Step 34: effects on the defending Pokémon.
    // Step 35: 1 counter per 10 damage, so damage lands in tens.
    damage - damage % 10
}

/// Carry the state forward until it is waiting on a player again: settle
/// knockouts, check for a winner, ask for a promotion, and end the turn if one
/// is owed.
fn settle(state: &mut GameState) {
    loop {
        if state.is_over() {
            state.phase = Phase::Over;
            return;
        }

        knock_out_the_dead(state);
        if state.is_over() {
            state.phase = Phase::Over;
            return;
        }

        // Rule 40: the player whose Active was knocked out chooses the next one.
        for player in [PlayerId::First, PlayerId::Second] {
            if state.player(player).active.is_none() {
                if state.player(player).bench.is_empty() {
                    // Rule 42: no Pokémon to promote loses the game.
                    win(state, player.opponent(), WinReason::NoPokemonInPlay);
                    state.phase = Phase::Over;
                    return;
                }
                state.phase = Phase::Promoting(player);
                return;
            }
        }

        if !state.pending_end_turn {
            state.phase = Phase::Main;
            return;
        }

        state.pending_end_turn = false;
        end_turn(state);
        if state.is_over() {
            state.phase = Phase::Over;
            return;
        }
    }
}

fn knock_out_the_dead(state: &mut GameState) {
    for player in [PlayerId::First, PlayerId::Second] {
        for pokemon in state.player(player).in_play() {
            if state.remaining_hp(pokemon) > 0 {
                continue;
            }
            knock_out(state, pokemon);
            // Rule 39: the opponent of the knocked-out player takes a Prize.
            // The count starts at 1 and a card effect adjusts it; Milestone 1
            // has no such card.
            take_prizes(state, player.opponent(), 1);
            if state.is_over() {
                return;
            }
        }
    }
}

fn knock_out(state: &mut GameState, pokemon: PokemonId) {
    let owner = state.pokemon(pokemon).owner;
    let name = state.pokemon_def(pokemon).name;

    // Rule 38: the Pokémon and everything attached go to its owner's discard.
    let card = state.pokemon(pokemon).card;
    let attached = std::mem::take(&mut state.pokemon[pokemon.index()].attached);
    let side = &mut state.players[owner.index()];
    side.discard.push(card);
    side.discard.extend(attached);
    if side.active == Some(pokemon) {
        side.active = None;
    }
    side.bench.retain(|p| *p != pokemon);

    state.pokemon[pokemon.index()].knocked_out = true;
    state.log.push(format!("{name} is Knocked Out."));
}

fn take_prizes(state: &mut GameState, player: PlayerId, count: usize) {
    for _ in 0..count {
        match state.players[player.index()].prizes.pop() {
            Some(card) => state.players[player.index()].hand.push(card),
            None => break,
        }
    }
    state.log.push(format!("{player:?} takes {count} Prize."));
    // Rule 41, and rule 44: this check comes before any other, so taking the
    // last Prize wins even when a knockout would end the game another way.
    if state.player(player).prizes.is_empty() {
        win(state, player, WinReason::AllPrizesTaken);
    }
}

fn end_turn(state: &mut GameState) {
    // Milestone 1 has no Special Conditions, so Pokémon Checkup (rules 45-48)
    // has nothing to do between the turns yet.
    state.current = state.current.opponent();
    state.turn_number += 1;
    state.begin_turn();
    state
        .log
        .push(format!("Turn {} begins.", state.turn_number + 1));

    // Rule 12: draw, and lose if you cannot.
    if !state.draw(state.current) {
        win(state, state.current.opponent(), WinReason::CouldNotDraw);
    }
}

fn win(state: &mut GameState, winner: PlayerId, reason: WinReason) {
    if state.outcome.is_none() {
        state.outcome = Some(Outcome { winner, reason });
        state.log.push(format!("{winner:?} wins: {reason:?}."));
    }
}
