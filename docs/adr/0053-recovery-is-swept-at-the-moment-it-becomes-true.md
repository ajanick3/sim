# "Recovers" is swept at the moment it becomes true, not derived at read time

**Status:** Accepted — 2026-09-08

## Context

`Festival Grounds` reads: *"Each Pokémon that has any Energy attached
(both yours and your opponent's) recovers from all Special Conditions
and can't be affected by any Special Conditions."* "Can't be affected"
fits the read-time discipline every Stadium so far has used
(ADR 0042, ADR 0048): `inflict` checks `immune_under_festival_grounds`
and refuses to add a condition. "Recovers" is different in kind — it is
a one-time event at the moment a Pokémon newly qualifies, not a fact
read continuously. `has_condition` returning `false` for an immune
Pokémon while `.conditions` still held a stale entry would work by
accident for every reader that goes through `has_condition`, but
`fill_checkup` reads `.conditions` directly, and a stale entry there
would still queue checkup damage for a condition the Pokémon is
supposed to no longer have.

## Decision

`clear_conditions` runs at both moments a Pokémon can newly qualify:
`resolve_trainer`'s arm for `EnergizedPokemonImmuneToSpecialConditions`
(unlike every other Stadium static, this one is *not* a no-op at play
time — it sweeps every Pokémon in play, both sides, clearing any that
already carries Energy) and `AttachEnergy`'s own handler (clearing the
one Pokémon that action just energized, if the Stadium is already in
play). `inflict` still refuses new conditions the read-time way, since
that check is naturally idempotent and needs no sweep.

Not covered: a Pokémon newly energized by `MoveEnergy` or
`MoveEnergyToActive` rather than a fresh attach. Neither move is common
into an already-conditioned, previously-bare Pokémon in the decks this
milestone scoped against; if one surfaces, it extends the same two-call
pattern this ADR sets, not a new design question.

## Consequences

Not every Stadium effect is "derive at read time" — a card whose text
says a Pokémon *recovers*, not merely *cannot be affected*, needs an
active sweep at each moment it can become newly true, the same
discipline `resolve_trainer`'s no-op arms don't otherwise need. A
future Stadium with the same "recovers and can't be affected" shape,
for some other status, follows this one's structure: a read-time guard
for the "can't be affected" half, an active sweep for "recovers."
