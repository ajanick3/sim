# Strange Timepiece

Type: task
Status: resolved

*"Devolve 1 of your evolved Pokémon by putting any number of Evolution
cards on it into your hand. (That Pokémon can't evolve this turn.)"*

New: `Phase::Devolving` removes cards from a Pokémon's own stack, one
at a time, into hand; `PokemonInPlay.cannot_evolve_this_turn` blocks
`Evolve` and `EvolveSkippingOneStage` for the rest of the turn, cleared
in `begin_turn` the same way `turn_bonus` already is.

- [x] Devolving removes evolution cards, "any number," into hand
- [x] The Pokémon cannot evolve again this turn

## Resolution

One new `Phase`, three new `Action`s, one new `PokemonInPlay` field.
Recorded in [ADR 0038](../../../docs/adr/0038-devolving-is-a-turn-flag-not-a-timer.md).

Coverage: `admitted` 481 -> 482 (1 print); `trainers` (refused)
310 -> 309.
