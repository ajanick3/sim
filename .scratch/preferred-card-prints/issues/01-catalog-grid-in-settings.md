# 01 — Settings shows the catalog as a grouped, searchable grid

Status: resolved

**What to build:** The Settings page grows a new section: every distinct
card name from `public/cards.json` as a tile, grouped by category
(pokémon, supporter, item, tool, stadium, special-energy, energy) via a
new `CATALOG_ORDER` constant — its own symbol, not a reuse of
`HAND_ORDER`, allowed to drift from it — then alphabetically by name
within each category. A text box filters the grid by name as the
player types. A tile shows the card's currently-resolved art: with no
preference stored yet (that lands in ticket 02), this is the
deterministic fallback — the first Print by print id for that name,
via `artUrl`. A card with only one known Print renders as a disabled
tile.

**Blocked by:** None — can start immediately

- [x] A new pure function groups and orders the full card list by
      `CATALOG_ORDER` then name; covered by direct unit tests (given a
      card list, this order out), no DOM involved.
- [x] A new pure function collects every Print sharing one card name
      from the loaded catalog; covered by direct unit tests.
- [x] The Settings page renders one tile per distinct card name, in
      that order, each showing its deterministic-fallback art via
      `artUrl`.
- [x] A card name with exactly one known Print renders its tile
      disabled (no tap affordance).
- [x] A search box filters the visible tiles by name substring.
- [x] `npm run test`, `npm run lint`, `npm run typecheck`,
      `npm run build` all pass.

## Answer

Resolved 2026-09-11, branch `feat/print-catalog-grid`, commit `f69a935`.

`web/app/prints.ts` carries `CATALOG_ORDER`, `bucketOf`,
`catalogEntries`, `printsFor`, and `defaultPrint`, all covered in
`web/app/prints.test.ts` (9 tests) and checked against the real
`public/cards.json` shape (`Pokemon`/`Trainer`/`Energy` categories,
`Supporter`/`Item`/`Tool`/`Stadium` trainer types — no surprises
against the fixtures). The Settings page fetches `/cards.json` and the
art index, and renders a searchable `flex-wrap` grid of `picker`-sized
`PlayingCard` tiles in catalog order; a single-print name renders
`disabled`/`dimmed`. `npm run test` (83 passing), `lint`, `tsc
--noEmit`, and `next build` all pass.

Beyond this ticket's AC, follow-up commits on the same branch also
made the grid span full width outside the narrow settings column,
added a row of category toggle buttons (with a pressed/dim visual
state and a filtered/total count) narrowing the grid alongside the
search box, and gave a tappable tile a hover/tap scale-and-lift. None
of this was asked for by the AC above; it's UI polish requested
alongside review of this ticket, kept on this branch since it's the
same PR (#320) rather than opening a separate one.
