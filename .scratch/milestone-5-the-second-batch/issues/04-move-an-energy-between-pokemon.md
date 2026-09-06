# Move an energy between pokemon

Type: task
Status: resolved

`Energy Switch` moves a Basic Energy from one of your Pokémon to another.
45 slots.

Every move built so far is between zones, or from a zone onto a Pokémon. This
is neither: both ends are a Pokémon's attachments, which are indexed by
`PokemonId` and are not a `Zone`. `Phase::DiscardingOpponentEnergy` already
reaches into attachments to remove one; this needs to put it somewhere.

- [x] An attached Energy can be moved to another Pokémon its owner controls
- [x] `Energy Switch` plays
- [ ] ~~and cannot move an Energy that is not Basic~~ — see below

## Both ends chosen at once

`Phase::MovingEnergy` names only the player. The action carries both the
Energy and where it goes, so no half-made move is ever recorded. A phase for
each end would hold a state — "this Energy is chosen, and nowhere yet" — that
no rule can read and no card can produce. At most five Energy and five
Pokémon is a small enough list to offer whole.

## "Basic Energy" cannot be checked, and does not need to be

The card reads *Basic* Energy, and the engine has no special Energy to tell
it from: every special Energy is refused at import
(`Refusal::IsASpecialEnergy`), because each carries rules text. So an
attached Energy in this engine is always basic, and the filter is
`is_energy()`. The acceptance criterion was written for a distinction the
pool does not offer yet. It becomes real on the day a special Energy is
admitted, and the filter is where it will land.

## Resolution

Coverage went 380 → 382 (both prints), and the field went 1151 → 1196
playable slots of 3660 — 32.7%.
