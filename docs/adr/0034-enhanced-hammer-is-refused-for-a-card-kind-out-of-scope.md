# Enhanced Hammer is refused: Special Energy is structurally absent

**Status:** Superseded by [0080](0080-a-special-energy-carries-its-own-effect.md) — 2026-09-08

## Context

`Enhanced Hammer` reads: *"Discard a Special Energy from 1 of your
opponent's Pokémon."* `import.rs` already refuses every printed Energy
card at import (`Refusal::IsASpecialEnergy`), so `CardDef::Energy` never
represents one; every Energy the engine ever instantiates, in every
game it plays, is a Basic Energy it supplies itself from a decklist's
Basic Energy lines, never from the artifact. `Energy`'s own definition
(`card.rs`) carries no field that could mark one as Special even if a
print reached it.

Two shapes were live. First: admit the card with a filter meant to name
Special Energy, knowing it can never match anything the engine actually
plays — the target exists on paper, never in a real game. Second:
refuse it, the way `Briar` is refused for a concept (Tera) the data
cannot back.

## Decision

Refuse `Enhanced Hammer`. Its target is not merely hard to check, the
way Tera is — it cannot exist at all in this engine's model, since
Special Energy is refused before a `CardDef` for one is ever built. A
filter that could never match anything is not a faithful build of the
card; it is a card that always does nothing, dressed as a working one.
ADR 0008's rule applies the same way it did to `Briar`: refuse what the
engine cannot run in full.

## Consequences

Coverage does not move. `Enhanced Hammer` stays refused for as long as
Special Energy stays out of scope; the day that scope changes, this
record is the one to revisit.
