# A static effect is read wherever it applies, never dispatched

**Status:** Accepted — 2026-09-08

## Context

`Air Balloon` reads: *"The Retreat Cost of the Pokémon this card is
attached to is {C}{C} less."* Every `TrainerEffect` built so far runs
once, through `resolve_trainer`, at the moment the card is played.
`Air Balloon` does nothing at attach time — its effect only exists as a
question asked later: "what does this Pokémon's Retreat Cost read as
right now?"

Two shapes were live. First: mutate `PokemonInPlay.retreat_cost` (a new
field) when the Tool attaches, and unmutate it when the Tool leaves
play (discarded, `Tool Scrapper`, a knockout). Second: derive the
answer at read time, computing from the printed value and whatever
Tools happen to be attached right now, storing nothing.

## Decision

Derive. `GameState::effective_retreat_cost(id)` reads
`pokemon_def(id).retreat_cost` (still the printed value —
`Pokemon::retreat_cost` is unchanged) and subtracts the sum of every
attached Tool's `TrainerEffect::ReducesRetreatCost` amount, floored at
zero with `saturating_sub`. Both sites that used to read the printed
value directly (`legal_actions`' retreat-offering check, `retreat`'s
cost calculation) now call this instead.

This is the same call the codebase has made twice already: `damage_dealt`
reads `state.turn_bonus` at "step 32" rather than mutating stored damage
when a bonus is granted, and ADR 0010 already decided a Prize value is
read at knockout time rather than stored, "so a card that adjusts the
count has somewhere to act." A mutate-on-attach field would need exact,
symmetric unmutation on every path a Tool can leave play — discard,
`Tool Scrapper`, a knockout, `Transformation Tome`'s identity swap — and
the engine's purity and replay guarantees (ADR 0002, ADR 0009) make a
value computed once and cached the more fragile shape, not the simpler
one.

`resolve_trainer` still needs an arm for `ReducesRetreatCost`, since
`TrainerEffect` is one enum matched exhaustively — it is `unreachable!()`,
since `PlayTool` (ADR 0041) never calls `resolve_trainer` at all.

## Consequences

Every stat a Tool can modify follows this shape: a `GameState` method
named for what it computes, called wherever the printed value used to
be read directly, deriving from whatever is attached right now. HP and
Prizes are next, and read from more sites than Retreat Cost's two — the
split matters most where a zone-card read (printed) and an in-play read
(effective) coexist, which Retreat Cost never has: nothing reads a
card's Retreat Cost while it sits in a deck or hand.
