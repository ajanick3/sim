# Healing

Type: task
Status: resolved

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

- [x] Damage on a Pokémon in play can be healed, floored at zero
- [x] A requirement can count the Energy attached to the player's own
      Active
- [x] `Jumbo Ice Cream` plays, and cannot be played while the Active
      carries fewer than 3 Energy

## Resolution

`TrainerEffect::HealActive(u32)` runs `damage.saturating_sub(amount)` —
the same floor `remaining_hp` already used, no new state or phase needed.
`Requirement::ActiveHasAtLeastEnergy(u32)` is the fourth shape a
requirement reads: a specific Pokémon's own attachments, where the other
three read the hand, the opponent's Prizes, or history.
[ADR 0026](../../../docs/adr/0026-healing-is-arithmetic-not-a-new-mechanism.md)
records why healing needed no new mechanism at all — `damage` was already
a plain, subtractable number.

Coverage went 411 → 413 (2 prints), and the field went 1563 → 1589
playable slots of 3660 — 43.4%.

This closes Milestone 6. All nine cards it targeted, across seven
tickets, play: Team Rocket's Petrel, N's PP Up, Wondrous Patch, Pokégear
3.0, Bug Catching Set, Ciphermaniac's Codebreaking, Unfair Stamp, Switch,
and Jumbo Ice Cream.
