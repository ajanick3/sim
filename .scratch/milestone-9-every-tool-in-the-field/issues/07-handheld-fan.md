# Handheld Fan

Type: task
Status: resolved

*"If the {C} Pokémon this card is attached to is in the Active Spot and
is damaged by an attack from your opponent's Pokémon (even if this
Pokémon is Knocked Out), move an Energy from the Attacking Pokémon to
1 of your opponent's Benched Pokémon."*

The same defender-triggered shape as `Punk Helmet`/`Lucky Helmet` (ADR
0045), but with a choice — which needed `Action::Attack` to be able to
defer `settle` for a phase opened mid-attack. Recorded in
[ADR 0046](../../../docs/adr/0046-an-attack-can-defer-settle-for-its-own-phase.md).

- [x] Opens a choice of Energy and destination when both exist
- [x] Does nothing with no Bench to receive it
- [x] The deferred attack resolution (knockout, and so on) still runs
      once the choice is made

## Resolution

One new `Phase`, one new `Action`, one control-flow change in
`Action::Attack`'s own handler.

Coverage: `admitted` 495 -> 496 (1 print); `trainers` (refused)
296 -> 295.
