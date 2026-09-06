# Attach Energy as the effect itself

Type: task
Status: ready-for-agent

`Teal Mask Ogerpon ex`'s Ability, "Teal Dance": *"Once during your turn, you
may attach a Basic Grass Energy card from your hand to this Pokémon. If you
attached Energy to a Pokémon in this way, draw a card."* 38 slots.

`Action::AttachEnergy` already moves a card from hand onto a Pokémon in
play, once a turn, as an ordinary action a player takes. This Ability offers
the same move, but as an Ability's effect, gated by its own once-a-turn
limit rather than `Limit::EnergyAttached`, and followed by a draw only when
the attach actually happened — the same "conditional on what just moved" the
milestone's `Then` already models for a Trainer's search, read for a single
attach instead of a search's count.

- [ ] The engine attaches an Energy as an Ability's effect, distinct from
      the once-a-turn attach every player already has from hand
- [ ] A draw that only happens when the attach did
- [ ] `Teal Mask Ogerpon ex` plays, and does not compete with the player's
      own once-a-turn Energy attachment
