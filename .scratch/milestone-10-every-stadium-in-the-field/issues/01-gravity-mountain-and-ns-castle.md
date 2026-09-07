# Gravity Mountain & N's Castle

Type: task
Status: resolved

*"Each Stage 2 Pokémon in play (both yours and your opponent's) gets
-30 HP."* (`Gravity Mountain`) and *"N's Pokémon in play (both yours
and your opponent's) have no Retreat Cost."* (`N's Castle`)

New: `GameState::stadium_effect()`, read from `effective_hp` and
`effective_retreat_cost` alongside the Tool-derived terms already
there — the first Stadium effects either function reads. Recorded in
[ADR 0048](../../../docs/adr/0048-a-stadium-is-read-from-both-sides-alike.md).

- [x] Both apply to either player's matching Pokémon, not only the
      Stadium's owner's

## Resolution

One new `GameState` method, two new `TrainerEffect` variants, two read
sites extended.

Coverage: `admitted` 498 -> 501 (2 prints of Gravity Mountain, 1 of
N's Castle); `trainers` (refused) 293 -> 290.
