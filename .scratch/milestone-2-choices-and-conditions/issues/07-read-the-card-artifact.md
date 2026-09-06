# Read the card artifact

Type: task
Status: needs-triage
Blocked by: 06

Turn `data/cards.json` into cards the engine can play. The artifact exists; the
engine still runs on the literals in `src/cards.rs`.

The work is not the reading. It is deciding what to do with the card text an
engine cannot execute: 734 attacks whose damage is a string such as `30+` or
`60×`, 2587 attacks with an English effect line, and 589 abilities. A card
whose text the engine cannot run must be refused, not half-loaded.

- [ ] A decision on how an unplayable card is refused, and where that is visible
- [ ] The engine plays a game with cards read from the artifact
- [ ] A count of how many Standard cards the engine can express today
