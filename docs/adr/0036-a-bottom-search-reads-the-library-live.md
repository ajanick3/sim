# A bottom-of-library search reads the Library live, not through Slot

**Status:** Accepted — 2026-09-07

## Context

`Dusk Ball` reads: *"Look at the bottom 7 cards of your deck. You may
reveal a Pokémon you find there and put it into your hand. Shuffle the
other cards back into your deck."* Every peeked search built so far
(`Bug Catching Set`, `Pokégear 3.0`) reads `Slot`'s `peek: Option<u32>`
against the *top* of a zone — the end `draw` pops from. `Dusk Ball`
wants the opposite end.

Two shapes were live. First: extend `Slot`/`peek` to read either end,
threading a new field through the ~32 sites that construct a `Slot`.
Second: a bespoke `TrainerEffect` and phase pair, reading the Library
directly rather than through `Decide`.

## Decision

`TrainerEffect::LookAtBottomOfLibrary { count }` opens
`Phase::LookingAtBottomOfLibrary { player, count }`. `legal_actions`
reads `library.iter().take(count)` — the front of the `Vec`, opposite
`draw`'s `pop` — matched against `CardFilter::AnyPokemon` directly,
the same way a peeked `Decide` slot already reads its zone live rather
than storing a copy in `Phase`. Nothing is stored in the phase beyond
`player` and `count`; the Library itself is the source of truth.

Threading a second peek direction through `Slot` was rejected: `Dusk
Ball` is the only card so far that reads from the bottom, and `Slot`
already carries five fields every one of its ~32 call sites sets by
hand. A sixth, mostly `None` or defaulted, would cost more than a
bespoke effect this narrow.

## Consequences

A card that peeks from a novel position in a zone gets its own
`TrainerEffect` and phase, the same call `Janine's Secret Art` and
`Dusk Ball` both made, rather than growing `Slot` for a shape only one
card needs. `Slot` stays as it is.
