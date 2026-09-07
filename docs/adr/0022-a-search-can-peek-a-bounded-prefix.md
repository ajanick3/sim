# A search can peek a bounded prefix of a zone

**Status:** Accepted — 2026-09-07

`Pokégear 3.0` and `Bug Catching Set` both read *"Look at the top 7 cards
of your deck"* before offering anything — not the whole Library, the way
every search built before them did. `Slot` gained `peek: Option<u32>`
rather than a new `Zone` variant or a second phase: `None` is every search
before these two, reading the whole zone; `Some(n)` reads only the `n`
cards nearest to being drawn, which are the last `n` entries of the
`Vec<CardId>` since `draw` pops from the end.

The alternative was to make the bound part of `from` itself — a
`Zone::TopOfLibrary(u32)` alongside `Zone::Library`. That was rejected:
`Zone` names *where a card is*, a fact about the card, and every zone
already answers `state.zone(player, zone)` with the whole pile it names.
A peek limit is a fact about the *search*, not the pile — the same 7 cards
are there whether this Trainer or a different one asks — so it belongs on
`Slot` beside `limit`, the other number a search carries about itself.

What the two cards do with whatever they do not take needed no new rule at
all. Both say "shuffle the other cards back into your deck," and since
nothing below the peeked 7 was ever inspected, a plain shuffle of the
whole Library — the same one `FinishDeciding` already runs whenever a
search reads the Library — lands on an indistinguishable distribution.
`Slot::peek` changes what a search can see; it changes nothing about how
a search ends.

## Consequences

`Phase::Deciding` carries `peek` alongside `filter` and `to`, the same
"held inline because every choice reads it" reasoning the phase's own
comment already gives for those two. Every slot before `Pokégear 3.0` sets
it to `None` and never notices it exists.
