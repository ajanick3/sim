# Transformation Tome

Type: task
Status: resolved

*"You must play 2 Transformation Tome cards at once. (This effect
works one time for 2 cards.) Choose a Basic Pokémon in your discard
pile and switch it with 1 of your Basic Pokémon in play. Any attached
cards, damage counters, Special Conditions, turns in play, and any
other effects remain on the new Pokémon."*

Two new primitives: a `Requirement` paid in a second physical copy of
the same card, with no choice involved (ADR 0039), and an identity swap
that replaces which card a `PokemonInPlay` is, keeping the same board
slot, damage, attachments, and turn (ADR 0040).

- [x] Cannot be played holding only one copy
- [x] The second copy is consumed automatically, no phase asked
- [x] The swap keeps damage, attachments, and played-on-turn on the
      same Pokémon

## Resolution

One new `Requirement`, one new `TrainerEffect`, one new `Phase`, two new
`Action`s. Recorded in [ADR 0039](../../../docs/adr/0039-a-cost-paid-in-a-second-copy-of-itself.md)
and [ADR 0040](../../../docs/adr/0040-an-identity-swap-keeps-the-same-pokemoninplay.md).

Coverage: `admitted` 482 -> 483 (1 print); `trainers` (refused)
309 -> 308.

This is the last ticket in Milestone 8: every Item in the field is now
either built or refused with a recorded decision.
