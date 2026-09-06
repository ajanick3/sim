# Map: keep the raw card record

## Destination

Every card in the artifact can be read by a caller, whether or not the engine
can play it, without the engine ever playing one it cannot run.

## Notes

The artifact is 2.0 MB for 3051 cards. Keeping every record costs roughly that
much memory once, at load, which is the number the ticket has to weigh.

## Decisions so far

Ticket 01 resolved 2026-09-06: the raw record is `serde_json::Value`, not a typed struct, and the memory cost is measured at +32,408 KiB resident for all 3051 cards; details under [the ticket's Answer](issues/01-keep-the-record-of-every-card.md).

## Fog

- No caller of `raw` exists yet. A deck builder or a card browser is what
  would spend it; neither is built. The effort's only ticket is resolved, so
  it waits on the operator to close, same as a finished milestone.
