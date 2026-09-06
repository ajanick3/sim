# The action log

Type: task
Status: resolved

Record every action the engine applies, so a game can be replayed from its
seed and its log.

ADR 0002 (now 0007) promised that a replay and a test are the same thing. The
engine is pure enough for it and keeps no actions, so the promise has never
been collectable.

- [x] The state holds the actions applied to it, in order
- [x] A game replays from its seed and its log to the same state
- [x] A refused action is not recorded, since it changed nothing
- [x] A recorded decision on what the log is for, and what it is not

## Answer

Resolved 2026-09-06 on branch `feat/the-action-log`.

`GameState.history` holds every `Action` that `apply` accepted, pushed at the
end of `apply` — after the arms that refuse part-way through have already
returned, so a refusal records nothing. `engine::replay(db, decklists, rng,
history)` rebuilds a game from the cards, a fresh generator, and the log, and
refuses an action that does not fit what it rebuilt.

The decision is [ADR 0013](../../../docs/adr/0013-the-log-records-what-was-applied.md).

Written test first, `tests/replay.rs`, six tests. The one that earns the
ticket plays a seeded game to its end, replays the log, and compares — a test
of every rule at once, naming none of them.

`GameState` cannot derive `PartialEq` (it holds a generator behind a trait
object), so the test spells out what "the same state" means: turn, current
player, phase, outcome, every zone, every Pokémon in play, and the prose log.
The ADR records that if a second caller needs that comparison, the test is
where to lift it from.
