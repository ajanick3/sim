# Air Balloon

Type: task
Status: resolved

*"The Retreat Cost of the Pokémon this card is attached to is {C}{C}
less."*

The first static effect: nothing runs at play time, only a question
asked later. New `GameState::effective_retreat_cost`, derived from the
printed value and whatever `ReducesRetreatCost` Tools are attached,
rather than stored and mutated on attach.

- [x] Reduces the Retreat Cost of the Pokémon it is attached to, by 2
- [x] Floored at zero, not negative
- [x] Does nothing for a Pokémon it is not attached to

## Resolution

One new `TrainerEffect` (read, never dispatched), one new `GameState`
method, two read sites switched from the printed value to the derived
one. Recorded in [ADR 0042](../../../docs/adr/0042-a-static-effect-is-read-not-dispatched.md) —
the decision the rest of this milestone's static-effect Tools build on.

Coverage: `admitted` 483 -> 486 (3 prints); `trainers` (refused)
308 -> 305.
