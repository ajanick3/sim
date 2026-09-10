# Map: every Standard Trainer and Special Energy, beyond the field

## Destination

`coverage` shows 0 refused Trainers and 0 refused Special Energy.

## Notes

`known_trainer` in `src/import.rs` matches a card by name to a
`(Requirement, TrainerEffect)`. `known_trainer_by_print` overrides by
print id where two prints of one name differ. `resolve_trainer` in
`src/engine.rs` runs the effect; `legal_actions` gates the card. Adding a
plain card is a `known_trainer` line plus, where the effect is new, a
`TrainerEffect` variant and its `resolve_trainer` arm.

The refused Supporters split roughly into: plain draw, shuffle-and-draw
variants, deck searches, discard-pile retrieval, heals, switches, and
opponent-hand disruption — most map onto an effect the engine already
has, in a new shape.

## Tickets

- 01 — plain-draw Supporters (`Cheren`, `Friends in Paldea`, `Urbain`):
  a `TrainerEffect::Draw(u32)`.

## Decisions so far

Ticket 01 resolved 2026-09-10: `TrainerEffect::Draw(u32)` admits the plain-draw
Supporters (`Cheren`, `Friends in Paldea`, `Urbain`), and `progress_table` now
prints a Standard-coverage summary; details under [the ticket's Answer](issues/01-plain-draw-supporters.md).
