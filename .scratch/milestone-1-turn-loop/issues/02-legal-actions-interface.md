# Legal actions interface

Type: task
Status: resolved

Give the engine one interface that a person and a bot both read.

- [x] `legal_actions(state)` lists what the player to act may do
- [x] `apply` refuses any action outside that list
- [x] The engine says who acts, which is not always the player whose turn it is

## Answer

Resolved 2026-09-05 on branch `feat/milestone-1-turn-loop`, squashed to
`42cbd65` on `main` as pull request #2.

`src/action.rs`. A knockout hands the choice to the player who lost the Pokémon, through `Phase::Promoting`. Recorded as ADR 0003.
