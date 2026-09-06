# Undo replays the log without its last entry, rather than reversing an action

**Status:** Accepted — 2026-09-06

Two ways to take back an action were live. An action could know its own inverse and be walked backwards in place, which is quick and touches only what the action touched. Or the game could be rebuilt from its cards, its seed, and its log without the last entry, which costs a full replay.

Reversing in place cannot be made to work here. A shuffle has no inverse the engine can compute: the generator has already moved on, and the order it produced is written down nowhere. Neither is a shuffle rare — a mulligan, a deck search, and several Supporters each shuffle, so the cases an in-place undo could not handle are ordinary rather than exotic. Every action would also have to carry the state it destroyed, which is a second record beside [the log](0013-the-log-records-what-was-applied.md) and a new way for the two to disagree.

So `undo` replays. It is `replay` over the log minus its last entry, and it is correct for every action for the same reason `replay` is: the engine is a pure function of its cards, its seed, and its actions.

## Consequences

An undo costs a replay, which is linear in the length of the game. That is paid by a person clicking undo, not by a bot in a self-play loop, and a whole game replays in well under the time it takes to see a button react.

Undo needs what `replay` needs — the cards, the decklists, and a fresh generator — because a state keeps a generator it has already advanced, not the seed it started from. A caller that wants undo therefore has to have kept the deal, which is the same thing it must keep to save or share a game at all.

Undoing with an empty log rebuilds the deal, since the position before any action is the deal itself. Nothing refuses it: there is no error to report, only a game that has not moved.
