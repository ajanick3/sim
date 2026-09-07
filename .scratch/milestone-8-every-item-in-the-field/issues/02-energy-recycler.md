# Energy Recycler

Type: task
Status: resolved

*"Shuffle up to 5 Basic Energy cards from your discard pile into your
deck."*

`Destination::Zone(Zone::Library)` already exists, and the deck is
already shuffled once a search that touched the Library ends — no new
primitive.

- [x] Up to 5 Basic Energy move from the Discard into the Library

## Resolution

No new primitive. Coverage: `admitted` 471 -> 473 (2 prints); `trainers`
(refused) 320 -> 318.

This ticket's commit also carries the README `## Card progress` table
update that ticket 01 missed.
