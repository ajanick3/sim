# Forest of Vitality

Type: task
Status: resolved

*"Each player's {G} Pokémon can evolve into {G} Pokémon during the turn
they play those Pokémon, except during their first turn."*

Follows the established shape: a static effect read alongside an
existing check, this time rules 18-20's "in play since the start of
the turn." No new ADR — the same discipline `Academy at Night`'s ADR
0049 already set for a read-site extension, applied to a third site
(evolution eligibility) rather than a new one.

- [x] A Grass Pokémon may evolve into a Grass Pokémon the turn it was
      played, with this Stadium in play
- [x] The usual timing rule still applies without it
- [x] Applies to `Evolve` and `EvolveSkippingOneStage` (Rare Candy) alike

## Resolution

One new `TrainerEffect`, one new `GameState` method, two read sites
extended.

Coverage: `admitted` 510 -> 513 (3 prints); `trainers` (refused)
281 -> 278.

One ticket left in Milestone 10: `Festival Grounds`.
