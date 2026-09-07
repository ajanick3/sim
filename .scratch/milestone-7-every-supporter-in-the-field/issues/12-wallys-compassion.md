# Wally's Compassion

Type: task
Status: resolved

*"Heal all damage from 1 of your Mega Evolution Pokémon ex. If you
healed any damage in this way, put all Energy attached to that Pokémon
into your hand."* 3 slots.

Healing a chosen target is ticket 04's primitive, narrowed to a target
filtered to a Mega Evolution ex and healing to full rather than a fixed
amount. New beyond that: a bulk move of every attachment on one Pokémon
to hand at once, conditioned on whether the heal actually removed any
damage.

- [x] A heal can remove all damage from its target, not only a fixed
      amount
- [x] Every card attached to one Pokémon can move to hand at once,
      conditioned on the heal it followed
- [x] `Wally's Compassion` plays, offered only for a Mega Evolution ex

## Resolution

`Phase::HealingMegaEx` and `TrainerEffect::HealMegaExAndTakeEnergyIfHealed`
are dedicated to this one card rather than a parameter on `HealChosen` —
nothing else needs "heal to full" or a Mega-ex-only target, so a second
generic phase would be built ahead of a second card that asks for it. A
Mega Evolution ex is read the same way `TurnBonusTarget::OpponentActiveEx`
already reads a plain ex: the prize value, 3 for a Mega ex per ADR 0010.

Coverage went 461 → 464 (3 prints), and the field went 1666 → 1669
playable slots of 3660 — 45.6%.
