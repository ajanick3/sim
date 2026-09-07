# Morty's Conviction

Type: task
Status: resolved

*"You can use this card only if you discard another card from your hand.
Draw a card for each of your opponent's Benched Pokémon."* 1 slot.

The cost is `Ultra Ball`'s shape exactly: discard a chosen card from hand
to pay, `Phase::Paying` already built. The draw count is new only in
where it is read from — the size of the *opponent's* Bench, not a fixed
number.

- [x] A draw effect can count the opponent's Benched Pokémon, rather than
      naming a fixed number
- [x] `Morty's Conviction` plays, and cannot be played holding nothing
      else to discard

## A real bug, found building this ticket

`Action::PayWithCard`'s final payment ran the paid-for effect without
first clearing `Phase::Paying`. Every card that reached this path before
`Morty's Conviction` — `Ultra Ball`, `N's Zoroark ex` — has an effect that
opens a phase of its own (`Deciding`, an Ability's own phase), which
overwrites `Paying` in the same step and hid the gap. `DrawPerOpponentBenched`
opens no phase at all: paying the cost drew the cards correctly, then left
`Phase::Paying` sitting there, stale, forever — the game was stuck.

Fixed by clearing the phase to `Main` before the effect runs, the same
place `PlayTrainer` already leaves `Main` behind it and
`Action::ChooseOption` (ticket 08) already does the same for a chosen
branch. `TrainerEffect::DrawPerOpponentBenched` is the first effect that
needed the fix to be visible at all.
[ADR 0030](../../../docs/adr/0030-a-paid-cost-clears-its-own-phase-before-the-effect-runs.md)
records it.

Coverage went 450 → 453 (3 prints), and the field went 1632 → 1633
playable slots of 3660 — 44.6%.
