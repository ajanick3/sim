# Healing

Type: task
Status: ready-for-agent

`Jumbo Ice Cream`: *"Heal 80 damage from your Active Pokémon that has 3 or
more Energy attached."* 26 slots.

Every card built so far adds damage or moves it; none has ever taken it
away. `PokemonInPlay::damage` is a plain `u32` in points, so healing is
arithmetic once it exists at all — `damage = damage.saturating_sub(80)`,
never below zero, the same floor a knockout already reads. There is no
target to choose: the card always names the Active, so what is new is a
`Requirement` that reads a count of Energy attached to a specific Pokémon
— the player's own Active — rather than the player's hand or the board in
general, which is all `Requirement` has ever read.

- [ ] Damage on a Pokémon in play can be healed, floored at zero
- [ ] A requirement can count the Energy attached to the player's own
      Active
- [ ] `Jumbo Ice Cream` plays, and cannot be played while the Active
      carries fewer than 3 Energy
