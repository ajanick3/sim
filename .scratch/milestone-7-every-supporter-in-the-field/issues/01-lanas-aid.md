# Lana's Aid

Type: task
Status: resolved

*"Put up to 3 in any combination of Pokémon that don't have a Rule Box
and Basic Energy cards from your discard pile into your hand."* 6 slots.

Almost everything this needs is built: a search from the discard pile to
the hand is the same shape `Night Stretcher` already plays, and
`CardFilter::PokemonWithoutRuleBox` already reads the Rule Box test —
`Poké Pad` uses it. `PokemonOrBasicEnergy` is close but not quite this
card: it admits *any* Pokémon, where `Lana's Aid` excludes a Rule Box one.
Check whether that is one new filter variant or a flag on the existing
one before building either.

- [x] `CardFilter` admits a Pokémon without a Rule Box, or a Basic Energy,
      as one filter
- [x] `Lana's Aid` plays, and the decks are measured after

## Resolution

`CardFilter::PokemonWithoutRuleBoxOrBasicEnergy` — a new variant, not a
flag added to `PokemonOrBasicEnergy`, the same way
`PokemonOfTypeOrBasicEnergyOfType` narrowed by type rather than reusing
`PokemonOrBasicEnergy` with a parameter. `Lana's Aid` plays, from the
discard pile.

Coverage went 413 → 416 (3 prints), and the field went 1589 → 1595
playable slots of 3660 — 43.6%. `tests/supporters.rs` is this milestone's
own fixture file, alongside `tests/second_batch.rs` from milestones 5
and 6.
