# A search names a destination, and the Bench is one of them

**Status:** Accepted — 2026-09-06

`Buddy-Buddy Poffin` searches the deck and puts what it finds onto the Bench,
and until now a search moved a card from one `Zone` to another. The Bench is
not a `Zone`: a `Zone` holds loose cards, and the Bench holds Pokémon in
play, which have damage, attachments, and a turn they came into play on. Two
alternatives were live. The first added `Zone::Bench`, which is the smaller
change to write and the larger change to mean: every place that reads a zone
as a list of cards would then have a case that is not one, and `move_card`
would have to know how to build a Pokémon. The second gave the effect a
destination of its own. The destination won. `TrainerEffect::Decide` and
`Phase::Deciding` now carry `Destination`, which is either a `Zone` or the
Bench, and `Zone` keeps its one meaning: a pile of cards.

## Consequences

A card sent to the Bench takes the same path a Basic played from hand takes —
`GameState::put_into_play`, which stamps `played_on_turn` — so a Pokémon that
arrived by search is not a second kind of Pokémon. Rule 14 caps the Bench at
five however a Pokémon got there, so `legal_actions` offers nothing to take
once the Bench is full, and the choice ends there rather than being refused
part-way. The shuffle that follows a card put back into the deck now reads
`Destination::Zone(Zone::Library)` rather than a bare zone.

A destination the pool needs later — attaching straight from a search, which
`Crispin` does — is a further variant of this value rather than another
mechanism.
