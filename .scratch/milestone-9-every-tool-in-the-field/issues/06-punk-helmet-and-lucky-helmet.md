# Punk Helmet & Lucky Helmet

Type: task
Status: resolved

*"If the [Type] Pokémon this card is attached to is in the Active Spot
and is damaged by an attack from your opponent's Pokémon (even if this
Pokémon is Knocked Out), place 4 damage counters on the Attacking
Pokémon."* (`Punk Helmet`) and *"...draw 2 cards."* (`Lucky Helmet`).

New: `attack()` calls a new `trigger_defenders_tool` right after
applying damage, before `settle` decides a knockout — "even if Knocked
Out" means the trigger cannot wait. Recorded in
[ADR 0045](../../../docs/adr/0045-a-defenders-tool-can-trigger-mid-attack.md).

- [x] Punk Helmet: 40 damage counters onto the attacker
- [x] Lucky Helmet: the Tool's owner draws 2
- [x] Neither fires on a zero-damage hit

## Resolution

Two new `TrainerEffect` variants, one new function called from
`attack()`.

Coverage: `admitted` 492 -> 495 (2 prints of Punk Helmet, 1 of Lucky
Helmet); `trainers` (refused) 299 -> 296.
