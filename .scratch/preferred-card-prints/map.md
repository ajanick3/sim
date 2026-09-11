# Map: preferred card prints

## Destination

A player can pick which Print's art renders for a card, in this
browser, from a Settings page grid — overriding even a Decklist's own
pinned Print. See [the spec](spec.md) for the full decision set.

## Notes

This effort went straight from `/grill-with-docs` to `/to-spec` to
`/to-tickets` — every open question was settled in the grilling
session, so there is no separate Fog section here.

## Decisions so far

Ticket 01 resolved 2026-09-11: the Settings page shows the full card
catalog as a searchable, category-grouped grid, each tile's art
resolved by a new deterministic fallback; details under
[the ticket's Answer](issues/01-catalog-grid-in-settings.md).

Ticket 02 resolved 2026-09-11: tapping a catalog tile opens a print
picker that saves a preference to localStorage and updates the tile;
details under
[the ticket's Answer](issues/02-print-picker-saves-a-preference.md).

Ticket 03 resolved 2026-09-11: a stored preference now overrides art
everywhere the app renders it, by fixing the one `art()` function
every render site already shared; details under
[the ticket's Answer](issues/03-preference-overrides-art-everywhere.md).
This closes the effort — all three tickets are resolved.
