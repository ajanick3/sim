# Turn loop and win conditions

Type: task
Status: resolved

Run turns until the game ends.

- [x] A turn draws, and a player who cannot draw loses
- [x] An attack ends the turn, and a knockout it caused settles first
- [x] All three win conditions end the game

## Answer

Resolved 2026-09-05 on branch `feat/milestone-1-turn-loop`, squashed to
`42cbd65` on `main` as pull request #2.

`settle` and `end_turn` in `src/engine.rs`. An attack ends the turn but its knockout resolves first, so the state carries a `pending_end_turn` flag and `settle` loops until the game waits on a player again. This is the part of the milestone most worth redesigning.
