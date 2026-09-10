# A card granting an effect over its whole side next turn stores it apart from the per-Pokémon restrictions

**Status:** Accepted — 2026-09-10

`opponent_next_turn_restriction` and `own_next_turn_restriction` each hold
`Option<(PokemonId, AttackEffect, …)>` — one Pokémon, an effect an attack
granted, read on the very next turn. `Jasmine's Gaze`, `Iron Defender`,
and `Roxie's Performance` grant an effect that is not an `AttackEffect`,
is not tied to a single Pokémon, and covers a whole side: "during your
opponent's next turn, all of your Pokémon take 30 less", "your {M}
Pokémon take 30 less", "their Poisoned Pokémon can't retreat".

## Decision

A third field, `GameState::side_shield_next_turn: Option<(PlayerId,
SideShield)>`, holds it — the player who granted it, and a `SideShield`:
`DamageReduction(u32)`, `DamageReductionForType(Type, u32)`, or
`OpponentPoisonedCannotRetreat`. `TrainerEffect::GrantSideShieldNextTurn`
sets it when the card resolves.

Its lifetime is the one the per-Pokémon restrictions already carry:
granted on the granting player's turn, live through the opponent's next
turn, cleared in `begin_turn` the moment the granting player's own turn
comes back — read from the stored `PlayerId`, not inferred.

The two damage shields are read in `damage_dealt_with`, after Weakness
and Resistance, for a defender the granting player owns while it is not
the granting player's turn. The retreat shield is read in the retreat
legality check, the same site the `DefenderCannotRetreat*` attack
restrictions are read.

## Consequences

- A per-Pokémon restriction and a side shield can both be live at once;
  they are separate fields and each read site checks its own.
- `SideShield` is a small closed enum. A fourth kind of side-wide
  next-turn effect adds a variant, not a new field.
- `Acerola's Mischief` (protect one chosen Pokémon from the opponent's
  ex) stays out: it is single-target, closer to the per-Pokémon
  restrictions than to a side shield.
