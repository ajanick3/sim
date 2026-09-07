# A restriction through the opponent's next turn outlives one `begin_turn`

**Status:** Accepted — 2026-09-08

## Context

`Yveltal` and `Wellspring Mask Ogerpon ex` both print "During your
opponent's next turn, the Defending Pokémon can't retreat." Every
"this turn" fact built so far — `turn_bonus`, `cannot_evolve_this_turn`,
`played_a_team_rocket_supporter_this_turn` — is cleared at *every*
`begin_turn`, unconditionally: that lifetime ends the moment any turn
starts, whoever's. A restriction granted "during your opponent's next
turn" needs to survive exactly one `begin_turn` (the boundary into the
target's own next turn) and clear at the one after that (the boundary
back into the granting player's turn) — two `begin_turn` calls, not
zero.

## Decision

`GameState.opponent_next_turn_restriction: Option<(PokemonId,
AttackEffect)>` — reusing `AttackEffect` itself as the stored value
rather than a dedicated restriction enum, since only restriction-shaped
variants are ever placed there and a reader only ever matches the one
it cares about. `begin_turn` clears it with a condition, not
unconditionally: only when `self.pokemon[target.index()].owner !=
self.current` — read after `start_next_turn` has already flipped
`state.current`, so this is true exactly when the *granting* player's
turn is starting, meaning the target's one covered turn already ran.

`AttackEffect::DefenderCannotRetreatNextTurn` is the first variant
read this way, checked in `legal_actions`' Retreat-offering block
alongside the existing Energy-cost and Bench checks.

## Consequences

`opponent_next_turn_restriction` is a single slot — two attacks
granting restrictions on two different targets in overlapping windows
would collide, the same limitation `turn_bonus`'s own single slot
already carries for "this turn" bonuses; nothing in the sample stacks
two at once. A restriction on the *attacker's* own next turn (ticket
06, `N's Zekrom`'s "can't use attacks," `Koraidon ex`'s "can't use this
attack") needs the opposite clearing condition — cleared once it is the
attacker's own turn again, not the opponent's — and gets its own field
rather than reusing this one, since the two lifetimes clear on opposite
turns.
