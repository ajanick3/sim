# A Stadium's own search reuses `enter_slot`, entered directly

**Status:** Accepted — 2026-09-08

## Context

`Lumiose City` reads: *"Once during each player's turn, that player may
search their deck for a Basic Pokémon and put it onto their Bench.
Then, that player shuffles their deck. If a player searches their deck
in this way, their turn ends."* The search itself is exactly the shape
`TrainerEffect::Decide` already runs — one slot, `Zone::Library` to
`Destination::Bench`, limit 1 — but unlike every `Decide` built so far,
nothing is *played* to trigger it: the Stadium is already in play, and
either player may use its search once on their own turn, the same way
`Academy at Night`'s action is offered directly (ADR 0049).

Storing `TrainerEffect::Decide { .. }` as `Lumiose City`'s own `effect`
field was tried first, and rejected: `resolve_trainer` dispatches
`Decide` the moment the Stadium is *played*, opening the search
immediately rather than leaving it as a standing action either player
can use once a turn. A Stadium's effect needs to stay a no-op at play
time (ADR 0048), the same as `Gravity Mountain` and `N's Castle`.

## Decision

`TrainerEffect::MaySearchBasicToBenchThenMaybeEndTurn` is the stored,
no-op-at-play effect. `Trainer::slots()` grows one more match arm
returning a fixed, `const` single-slot search shape for this variant —
the search itself needs no per-print data, unlike a played card's own
`Decide`, which carries its slots inline. `Action::UseLumioseCity`
calls `enter_slot` directly — the same function `resolve_trainer`'s own
`Decide` arm calls — passing `Then::EndTurnIfMoved` (new: ends the turn
only if the search actually moved a card, not on a decline or an empty
deck) rather than routing through `resolve_trainer` at all.

## Consequences

A Stadium's own search reuses the full `Decide`/`enter_slot` machinery
— peeking, exclusion, the end-of-search shuffle — without needing to be
played to enter it. A future Stadium with a *different* fixed search
shape extends `Trainer::slots()` the same way; one whose search varies
by print (the way a played card's `Decide` does) would need its slots
threaded through some other way, not yet needed here.
