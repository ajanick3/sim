# Setup as a phase

Type: task
Status: resolved

Give setup its own phase, so the player makes the choices Milestone 1 made
for them.

- [x] The opening coin flip happens, and the winner chooses who goes first
- [x] The player places their own Active and Bench from the Basics in hand
- [x] The extra card per opponent mulligan is a choice, not a gift

## Answer

Resolved 2026-09-05 on branch `feat/setup-as-a-phase`, commits `15076ec` and
`c669097`.

Setup is now four phases — `ChoosingWhoGoesFirst`, `TakingBonusDraws`,
`PlacingActive`, `PlacingBench` — and `legal_actions` drives every one.
Shuffling, drawing, and mulliganing stay in `GameState::new`, because none of
them is a choice.

Three things fell out of the work:

- The `PlayerId` variants named a turn order that the coin flip can now invert,
  so they became `One` and `Two`, which name a seat.
- The rulebook has both players set up at once. The engine asks one player for
  all of it before the other, because nothing at setup is visible to the
  opponent.
- The turn loop never drew for the player going first, which rule 15 requires.
  `tests/setup.rs` now covers it.
