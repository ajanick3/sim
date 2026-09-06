# The log records what was applied, and a replay rebuilds from it

**Status:** Accepted — 2026-09-06

The engine was already a pure function of its cards, its seed, and its actions, and it kept none of the actions. [ADR 0007](0007-card-data-comes-from-tcgdex-into-this-repository.md) says that purity "makes a replay and a test the same thing"; nothing could collect on it, because a finished game held no account of how it got there. `GameState.history` is that account: every action `apply` accepted, in order.

Three things were decided with it. The log records what was **applied**, never what was intended: an action `apply` refused changed nothing, so it has nothing to replay, and it is not recorded. The log holds `Action` values rather than prose, so a replay reads the same thing the engine executes — `GameState.log` stays what it always was, prose for a person, which nothing reads back. And the log does not hold the seed or the cards: a state keeps a generator it has already advanced, not the seed it started from, so `replay` asks the caller for the cards, the decklists, and a fresh generator, and refuses an action that does not fit what it rebuilt.

## Consequences

A replay is a test of every rule at once, naming none of them: play a seeded game to its end, rebuild it from the log, and compare. `tests/replay.rs` does exactly that, and it will fail for any change that makes the engine's result depend on something outside the state, the seed, and the actions — which is the property ADR 0007 asserts and nothing had been checking.

Comparing two games needs a definition of "the same state". `GameState` cannot derive `PartialEq`, since it holds a generator behind a trait object, so the test spells the comparison out. If a second caller ever needs it, that spelling-out is what to move into the engine.

Undo is now a question about this log rather than about the state, which is what [the undo ticket](../../.scratch/milestone-4-the-record/issues/02-undo-one-action.md) weighs: replaying the log without its last entry is correct for any action, where reversing an action in place is quicker and impossible for a shuffle.
