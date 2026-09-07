# N's Plan

Type: task
Status: resolved

*"Move up to 2 Energy from your Benched Pokémon to your Active
Pokémon."* 3 slots.

`MoveAttachedEnergy` (`Energy Switch`) already moves one Energy between
two Pokémon a player controls, chosen freely. This card narrows the
direction — Bench to Active only, never Active to Bench, never Bench to
Bench — and repeats up to twice. Check whether that is a variant of the
existing effect with the source and destination fixed, or a genuinely
different shape once "up to 2" is accounted for.

- [x] Energy can move from a Benched Pokémon to the Active specifically,
      up to a limit, rather than between any two Pokémon once
- [x] `N's Plan` plays, and the decks are measured after

## Resolution

`TrainerEffect::MoveEnergyFromBenchToActive { limit }` opens
`Phase::MovingEnergyFromBenchToActive`, a sibling to `MovingEnergy` rather
than a generalization of it: the target is fixed (the Active), so the
action needs only a card, and `FinishMovingEnergyToActive` keeps ADR
0012's rule that the choice to stop is the player's, even at the limit.

Coverage went 418 → 421 (3 prints), and the field went 1598 → 1601
playable slots of 3660 — 43.7%.
