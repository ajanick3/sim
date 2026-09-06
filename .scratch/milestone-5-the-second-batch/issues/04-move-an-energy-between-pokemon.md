# Move an energy between pokemon

Type: task
Status: ready-for-agent

`Energy Switch` moves a Basic Energy from one of your Pokémon to another.
45 slots.

Every move built so far is between zones, or from a zone onto a Pokémon. This
is neither: both ends are a Pokémon's attachments, which are indexed by
`PokemonId` and are not a `Zone`. `Phase::DiscardingOpponentEnergy` already
reaches into attachments to remove one; this needs to put it somewhere.

- [ ] An attached Energy can be moved to another Pokémon its owner controls
- [ ] `Energy Switch` plays, and cannot move an Energy that is not Basic
