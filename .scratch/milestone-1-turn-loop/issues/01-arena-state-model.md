# Arena state model

Type: task
Status: resolved

Hold the game state so Rust can mutate a graph of cross-referencing objects.

- [x] Every card and every Pokémon in play lives in one `Vec` for the game
- [x] Zones hold typed indices, not references
- [x] The index types keep cards, card definitions, and Pokémon apart

## Answer

Resolved 2026-09-05 on branch `feat/milestone-1-turn-loop`, squashed to
`42cbd65` on `main` as pull request #2.

Arenas in `src/state.rs`, index types in `src/ids.rs`. A knocked-out Pokémon stays in its arena, marked, so the arena is history as well as state. Recorded as ADR 0001.
