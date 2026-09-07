# Lumiose City

Type: task
Status: resolved

*"Once during each player's turn, that player may search their deck for
a Basic Pokémon and put it onto their Bench. Then, that player shuffles
their deck. If a player searches their deck in this way, their turn
ends."*

Reuses `enter_slot`, the same function a played card's own `Decide`
calls, entered directly from `Action::UseLumioseCity` rather than
through `resolve_trainer` — a Stadium's own effect stays a no-op at
play time (ADR 0048). New `Then::EndTurnIfMoved`. Recorded in
[ADR 0050](../../../docs/adr/0050-a-stadiums-own-search-reuses-enter-slot-not-resolve-trainer.md).

- [x] The search offers only a Basic Pokémon, to the Bench
- [x] Taking one ends the turn
- [x] Declining does not end the turn
- [x] Once a turn either way

## Resolution

One new `TrainerEffect` (no-op at play), one new `Then` variant, one new
`Action`, one new match arm in `Trainer::slots()`.

Coverage: `admitted` 504 -> 506 (2 prints); `trainers` (refused)
287 -> 285.
