# Pokémon Center Lady

Type: task
Status: ready-for-agent

*"Heal 60 damage from 1 of your Pokémon, and it recovers from all
Special Conditions."* 3 slots.

`HealActive` (`Jumbo Ice Cream`) heals a fixed target, the Active, with no
choice in it. This card heals a *chosen* Pokémon — Active or Benched —
which needs a target choice in front of the same arithmetic, the same way
`Destination::Attach` or `MoveEnergy` already choose a target alongside a
card. Clearing every Special Condition already exists:
`clear_conditions`, which evolution already calls.

- [ ] A heal can target a Pokémon the player chooses, not only the fixed
      Active
- [ ] `Pokémon Center Lady` plays, clearing every Special Condition on the
      Pokémon it heals
