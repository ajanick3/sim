# Save, resume, and share a game across clients

Type: task
Status: needs-triage

The operator asked for "a way to export board state to resume later" and
"some sort of server persistent state across clients".

A game today lives only in one tab's memory. This ticket makes a game
outlive the tab and be reachable from another client.

## Shape

Follow ADR 0096: a game is not a serialized `GameState`, it is the recipe
to rebuild one — `{ mode, seed, deck_a, deck_b, actions: [index, …] }` —
replayed from the start. `sim-wasm` gains a constructor that takes this
recipe and applies each index in order.

- **Export / resume, no server.** The page can hand the player the recipe
  (a link with the recipe encoded, or a downloaded file) and rebuild the
  game from one it is given. Pure client, no new infrastructure.
- **Shared, server-held.** A small service stores a recipe under an
  unguessable id and appends an action index as each move is applied. Any
  client that opens the id replays to the current point. Last-writer-wins
  is acceptable; live turn-by-turn sync is out of scope (see spec).

## Open questions for triage

- Where the service runs. Vercel already hosts `web/` — a serverless
  route plus a KV store is the least new infrastructure. Confirm that is
  the direction before building.
- Whether the recipe is validated server-side (replay it through
  `sim-wasm` on write) or trusted from the client and only replayed on
  read.
- How a resumed game handles an engine change that alters a replay — pin
  the engine version into the recipe, or accept that old links may break.

## Acceptance criteria

- [ ] `sim-wasm` rebuilds a game from a recipe and its list of action
      indices, matching the game those moves produced live.
- [ ] The page exports the current game as a recipe and resumes one from a
      recipe it is given, with no server.
- [ ] A server stores a recipe by id, accepts appended action indices, and
      serves the recipe to any client that has the id.
- [ ] Two browsers opening the same id see the same board, one client's
      moves visible to the other on reload.
- [ ] The recipe records which engine version produced it.
