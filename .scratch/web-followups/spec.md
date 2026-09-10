# Spec: web front-end follow-ups

The browser front end (`web/`, ADR 0096) shipped as a local hotseat: one
tab, one machine, one game that lives only in memory. This effort grows it
toward a game people can leave and come back to, and can report problems
from.

## In scope

- Save a game and resume it later.
- Share one game's state across clients, held by a server.
- Report a bug from inside the UI, with enough context for an agent to act.

## Out of scope for now

- Real-time two-player netplay (both players acting live). Resuming a shared
  game is enough.
- Accounts and login. A game is reached by an unguessable id, not a user.

## Constraints

- The engine stays pure (ADR 0004). Anything a server needs it does by
  driving `sim` or `sim-wasm`, never by changing the engine to serve HTTP.
- `GameState` is not `Serialize` and ADR 0096 chose not to make it so. The
  live alternative is replay from `{seed, deck, action indices}`, the shape
  `tests/replay.rs` already exercises.
