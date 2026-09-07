# Festival Grounds

Type: task
Status: resolved

*"Each Pokémon that has any Energy attached (both yours and your
opponent's) recovers from all Special Conditions and can't be affected
by any Special Conditions."*

Two parts, two shapes: "can't be affected" is a read-time guard in
`inflict`, the discipline every Stadium static has used; "recovers" is
an active sweep at the moments a Pokémon can newly qualify — playing
the Stadium, and attaching Energy while it's already in play — since a
one-time event cannot be derived purely at read time (`fill_checkup`
reads `.conditions` directly, not through a guarded accessor). Recorded
in [ADR 0053](../../../docs/adr/0053-recovery-is-swept-at-the-moment-it-becomes-true.md).

- [x] Blocks a new Special Condition on an energized Pokémon
- [x] Clears an existing condition the moment the Stadium enters play
- [x] Does not protect a Pokémon without Energy attached

## Resolution

One new `TrainerEffect` (the milestone's only non-no-op Stadium at play
time), one new `GameState` method, two sweep sites, one guard in
`inflict`.

Coverage: `admitted` 513 -> 515 (2 prints); `trainers` (refused)
278 -> 276.

This is the last ticket in Milestone 10: every Stadium in the field is
now either built or refused with a recorded decision.
