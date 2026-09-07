# Enhanced Hammer

Type: task
Status: resolved

*"Discard a Special Energy from 1 of your opponent's Pokémon."*

- [x] A recorded decision on whether this card can be run in full, or
      must be refused

## Resolution

Refused. Special Energy is refused outright at import
(`Refusal::IsASpecialEnergy`), so `CardDef::Energy` never represents
one — every Energy in play, in every game the engine plays, is a Basic
Energy the engine supplies itself. A filter naming Special Energy
would never match anything real, which is not a faithful build of the
card. Recorded in [ADR 0034](../../../docs/adr/0034-enhanced-hammer-is-refused-for-a-card-kind-out-of-scope.md).

No `known_trainer` entry was added. Coverage does not move.
