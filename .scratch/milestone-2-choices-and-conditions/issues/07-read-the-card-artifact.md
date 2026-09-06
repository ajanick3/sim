# Read the card artifact

Type: task
Status: resolved
Blocked by: 06

Turn `data/cards.json` into cards the engine can play. The artifact exists; the
engine still runs on the literals in `src/cards.rs`.

The work is not the reading. It is deciding what to do with the card text an
engine cannot execute: 734 attacks whose damage is a string such as `30+` or
`60×`, 2587 attacks with an English effect line, and 589 abilities. A card
whose text the engine cannot run must be refused, not half-loaded.

- [x] A decision on how an unplayable card is refused, and where that is visible
- [x] The engine plays a game with cards read from the artifact
- [x] A count of how many Standard cards the engine can express today

## Answer

Resolved 2026-09-06 on branch `feat/read-the-card-artifact`.

The decision is [ADR 0008](../../../docs/adr/0008-the-engine-refuses-a-card-it-cannot-run.md):
the engine admits a card only when it can run all of it, and refuses the rest
by name and by reason. `cargo run --bin coverage` prints the count.

**The engine plays 274 of 3051 Standard cards, 9.0%.** Refused: 1614 are not a
Basic Pokémon, 929 carry an attack's effect line, 234 carry an ability. No card
was refused for its damage alone, because an attack with damage such as `30+`
carries an effect line too, and that is checked first.

`src/import.rs` takes the JSON as a string, so the crate keeps no I/O and a
test hands it a literal. `serde_json` is the one dependency the crate now
carries.

Basic Energy is not printed in the Standard sets, so the artifact holds none
and an imported deck could not pay for an attack. `Import::basic_energy`
supplies it, which the ADR records.

Written test first, nine tests. The last one deals a deck of an imported card
and plays it to a winner.
