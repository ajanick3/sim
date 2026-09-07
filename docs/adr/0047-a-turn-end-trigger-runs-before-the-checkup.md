# A turn-end trigger runs before the checkup, inside `settle` itself

**Status:** Accepted — 2026-09-08

## Context

`Powerglass` reads: *"At the end of your turn (after your attack), if
the Pokémon this card is attached to is in the Active Spot, you may
attach a Basic Energy card from your discard pile to it."* Every Tool
trigger built so far in this milestone fires on being attacked (ADR
0045, ADR 0046); this one fires on the turn itself ending, whether or
not an attack happened. `settle`'s loop already has exactly one place
that means "the turn just ended": the `state.pending_end_turn` branch,
which today goes straight to queuing the checkup and marking the next
turn owed.

## Decision

`settle` checks `powerglass_owner(state)` — the current player's Active,
if it carries `Powerglass` — before doing that work, and opens
`Phase::AttachingFromDiscardForPowerglass` instead when it applies. The
checkup-and-next-turn work itself moves into a new `end_the_turn`
helper, called both from `settle`'s own loop (when Powerglass does not
apply) and directly by whichever action resolves the Powerglass phase
(`AttachFromDiscardForPowerglass`, `DeclinePowerglass`) — so the turn
finishes ending exactly once, on whichever path was taken, rather than
`settle` re-deciding it a second time on re-entry.

This is the same shape ADR 0046 gave attacks: a phase can interrupt a
sequence `settle` would otherwise run straight through, with the
interrupted work resumed — here by `end_the_turn`, there by `settle`
itself — once the phase's own action resolves it.

## Consequences

A card that triggers on the turn ending, rather than on being attacked,
extends `settle`'s `pending_end_turn` branch the same way: check for it
before `end_the_turn` runs, open a phase if it applies, and have that
phase's own action call `end_the_turn` once resolved. Two such cards
active in the same turn end would need ordering decided explicitly —
none exists yet.
