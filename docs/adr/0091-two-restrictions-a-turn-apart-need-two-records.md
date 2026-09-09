# Two restrictions a turn apart need two state records

**Status:** Accepted — 2026-09-09

`Stunfisk`'s `Pouncing Trap` prints two consequences from one attack:
"During your opponent's next turn, the Defending Pokémon can't
retreat. During your next turn, the Defending Pokémon takes 100 more
damage from attacks." The first names the opponent's very next
turn — exactly `DefenderCannotRetreatNextTurn`'s own lifetime, reusing
`opponent_next_turn_restriction` as-is. The second names the granting
player's own very next turn, one turn later: by the time it is
"your next turn," `opponent_next_turn_restriction` has already
cleared (its own clearing rule fires the instant it becomes the
granting player's turn again — the same turn boundary the bonus
should *start* being armed on, not end).

Packing both into the one restriction record was tried first and
produced a bonus that fired the wrong turn (the retreat-locked turn,
not the one after). The two consequences need two separate lifetimes,
so `AttackEffect::DefenderCannotRetreatAndTakesMoreDamageNextTurn`
writes to two fields: `opponent_next_turn_restriction` for the
retreat lock, and a new
`bonus_damage_to_pokemon_on_granting_players_next_turn` for the bonus,
delayed-armed the same way `own_next_turn_restriction` already is —
`armed` flips `true` the first `begin_turn` after granting, marking
that the *next* time it is the granting player's own turn is the one
bonus turn. Unlike `own_next_turn_restriction`, which keys off the
*target*'s own owner, this field keys off the *granting player*
explicitly, since the target here (the opponent's own Pokémon) never
shares a turn with the player whose turn actually grants the bonus.
`Attack` still holds only one `AttackEffect`; both consequences read
from that one variant, at two different sites.
