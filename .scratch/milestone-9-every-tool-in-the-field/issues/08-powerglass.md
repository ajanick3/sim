# Powerglass

Type: task
Status: resolved

*"At the end of your turn (after your attack), if the Pokémon this
card is attached to is in the Active Spot, you may attach a Basic
Energy card from your discard pile to it."*

New: a turn-end trigger, distinct from every "damaged by an attack"
trigger this milestone built so far. `settle` checks for it before
queuing the checkup; a new `end_the_turn` helper does the checkup-and-
next-turn work either path ends up needing. Recorded in
[ADR 0047](../../../docs/adr/0047-a-turn-end-trigger-runs-before-the-checkup.md).

- [x] Offers a Basic Energy from discard when the turn ends
- [x] May be declined
- [x] The turn still ends and passes to the opponent either way
- [x] No phase opens for a Pokémon without Powerglass

## Resolution

One new `TrainerEffect`, one new `Phase`, two new `Action`s, one new
helper (`end_the_turn`) factored out of `settle`.

Coverage: `admitted` 496 -> 498 (2 prints); `trainers` (refused)
295 -> 293.

This is the last ticket in Milestone 9: every Tool in the field is now
built.
