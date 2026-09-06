# Map: keep the raw card record

## Destination

Every card in the artifact can be read by a caller, whether or not the engine
can play it, without the engine ever playing one it cannot run.

## Notes

The artifact is 2.0 MB for 3051 cards. Keeping every record costs roughly that
much memory once, at load, which is the number the ticket has to weigh.

## Decisions so far

Nothing resolved yet.

## Fog

- Whether a card browser or a deck builder is actually wanted. If neither is,
  this effort is memory spent on nothing, and the ticket should say so and
  close.
