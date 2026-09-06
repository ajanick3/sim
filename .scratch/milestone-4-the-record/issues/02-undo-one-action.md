# Undo one action

Type: task
Status: resolved
Blocked by: 01

Take back the last action.

The fog names the choice: replay the log from the start without its last
entry, or reverse the action in place. A shuffle cannot be reversed without
recording what it did, which points at the first, and the first is only as
slow as a replay.

- [x] The last action can be taken back
- [x] The state after an undo equals the state before the action
- [x] A decision on which of the two ways, and why

## Answer

Resolved 2026-09-06 on branch `feat/undo-one-action`.

`engine::undo(db, decklists, rng, history)` is `replay` over the log minus its
last entry. The decision is
[ADR 0014](../../../docs/adr/0014-undo-replays-rather-than-reverses.md).

The fog asked which of the two ways. Reversing an action in place cannot be
made to work: a shuffle has no inverse the engine can compute, since the
generator has moved on and the order it produced is written down nowhere —
and a shuffle is ordinary, not exotic. A mulligan, a deck search, and several
Supporters each shuffle. Reversing would also need every action to carry the
state it destroyed, which is a second record beside the log and a new way for
the two to disagree.

Three tests in `tests/replay.rs`, beside the replay ones since they share the
same definition of "the same state": an undo gives back the position before
the last action, it can be taken more than once, and undoing an empty log
gives the deal.
