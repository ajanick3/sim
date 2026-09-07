# A Stadium's static effect is read from both sides alike

**Status:** Accepted — 2026-09-08

## Context

`Gravity Mountain` and `N's Castle` are the first Stadiums to reach a
`TrainerEffect`: -30 HP for every Stage 2 in play, and no Retreat Cost
for every N's Pokémon, "both yours and your opponent's." Every static
effect built so far (ADR 0042, Milestone 9) belonged to one Pokémon,
read from that Pokémon's own `attached` list. A Stadium belongs to
neither player — `state.stadium: Option<(PlayerId, CardId)>` already
holds it, but nothing before this read its effect at all.

## Decision

A new `GameState::stadium_effect()` reads whichever Stadium is in play,
if any, the same way `pokemon_def` reads a Pokémon's own definition —
by dereferencing `state.stadium` and pulling the card's `TrainerEffect`.
`effective_hp` and `effective_retreat_cost` each add one more term:
`stadium_effect()`, checked unconditionally, against the Pokémon's own
printed Stage or name, whichever side of the board it is on. Neither
function needed a `player` parameter to do this — a Stadium is read
identically for both sides, so the existing `PokemonId`-only signature
already carries what is needed.

Playing a Stadium still runs through `resolve_trainer` (unlike a Tool,
which `PlayTool` skips it for entirely — ADR 0041). `resolve_trainer`
gets a genuine no-op arm for these two effects, distinct from a Tool's
`unreachable!()` — there is a real dispatch here, it simply has nothing
to do at play time, since `state.stadium` already names the card and
every reader pulls the effect from there afterward.

## Consequences

A Stadium's static effect follows the same "derive at the read site"
discipline ADR 0042 set for a Tool's, extended to a fact both players
share rather than one Pokémon owns. A future Stadium modifying a stat
neither `effective_hp` nor `effective_retreat_cost` reads yet (damage,
a Special Condition) gets its own read site the same way, checking
`stadium_effect()` alongside whatever else that site already checks.
