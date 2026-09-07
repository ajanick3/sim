# Lana's Aid

Type: task
Status: ready-for-agent

*"Put up to 3 in any combination of Pokémon that don't have a Rule Box
and Basic Energy cards from your discard pile into your hand."* 6 slots.

Almost everything this needs is built: a search from the discard pile to
the hand is the same shape `Night Stretcher` already plays, and
`CardFilter::PokemonWithoutRuleBox` already reads the Rule Box test —
`Poké Pad` uses it. `PokemonOrBasicEnergy` is close but not quite this
card: it admits *any* Pokémon, where `Lana's Aid` excludes a Rule Box one.
Check whether that is one new filter variant or a flag on the existing
one before building either.

- [ ] `CardFilter` admits a Pokémon without a Rule Box, or a Basic Energy,
      as one filter
- [ ] `Lana's Aid` plays, and the decks are measured after
