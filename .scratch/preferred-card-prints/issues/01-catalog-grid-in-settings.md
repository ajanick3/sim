# 01 — Settings shows the catalog as a grouped, searchable grid

Status: claimed

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

- [ ] A new pure function groups and orders the full card list by
      `CATALOG_ORDER` then name; covered by direct unit tests (given a
      card list, this order out), no DOM involved.
- [ ] A new pure function collects every Print sharing one card name
      from the loaded catalog; covered by direct unit tests.
- [ ] The Settings page renders one tile per distinct card name, in
      that order, each showing its deterministic-fallback art via
      `artUrl`.
- [ ] A card name with exactly one known Print renders its tile
      disabled (no tap affordance).
- [ ] A search box filters the visible tiles by name substring.
- [ ] `npm run test`, `npm run lint`, `npm run typecheck`,
      `npm run build` all pass.
