# Healing is arithmetic, not a new mechanism

**Status:** Accepted — 2026-09-07

`Jumbo Ice Cream` heals 80 damage from the player's Active. Every card
built before it only added damage or moved it between Pokémon; none had
ever taken it away. `PokemonInPlay::damage` is a plain `u32` in points,
the same representation a knockout already reads
(`remaining_hp`), so healing needed no new state and no new phase:
`TrainerEffect::HealActive(u32)` runs
`damage = damage.saturating_sub(amount)` against the player's own Active,
floored at zero the same way `remaining_hp` already floors at zero rather
than going negative. There was no live alternative to weigh — a damage
counter as a subtractable number was already the whole representation.

What the card needed a decision for was its requirement: "your Active
Pokémon that has 3 or more Energy attached" reads a specific Pokémon's own
attachments, not the player's hand or the board in general, which is what
`Requirement`'s other three variants read. `Requirement::ActiveHasAtLeastEnergy(u32)`
is that fourth shape, checked the same way the others are: read once in
`legal_actions`, satisfied before the card is even offered.

## Consequences

None beyond the two additions themselves. No test needed a new fixture
shape either — attaching several Energy directly to a Pokémon in play,
bypassing the once-a-turn attach action, was already how earlier tickets
built a board to test against.
