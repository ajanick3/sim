# Setup and mulligans

Type: task
Status: resolved

Deal a legal opening board.

- [x] Shuffle, draw 7, and mulligan while the hand holds no Basic
- [x] Each mulligan the opponent took gives one extra card
- [x] Place an Active and a Bench, then set 6 Prizes aside

## Answer

Resolved 2026-09-05 on branch `feat/milestone-1-turn-loop`, squashed to
`42cbd65` on `main` as pull request #2.

`GameState::new` in `src/state.rs`. Two shortcuts stand: setup places the first Basic as Active without asking, and the coin flip for who goes first is skipped. Milestone 2 ticket 01 removes both.
