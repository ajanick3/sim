# Map: read and check a decklist

## Destination

An operator hands the simulator the decklist they already have, and learns
whether it is legal and how much of it the engine can play.

## Notes

The official client exports one card per line: a count, a name, a set
abbreviation, and a number. Section headers and a total line appear too.

## Decisions so far

Ticket 01 resolved 2026-09-06: the export format parses and a list is checked against the three checkable deck construction rules; details under [the ticket's Answer](issues/01-read-and-check-a-decklist.md).

## Fog

- Basic Energy is matched by name alone, so a list naming a set the artifact
  does not hold still passes. That is right for Energy and would be wrong for
  anything else.
