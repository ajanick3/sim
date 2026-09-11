# 02 — Tapping a card opens a print picker that saves a preference

**What to build:** Tapping an enabled tile in the catalog grid (ticket
01) opens a modal listing every known Print of that card name, with
nothing pre-highlighted — the modal never implies a choice was already
made. Tapping a Print in the modal saves it as that player's preferred
Print for that name, in this browser (`localStorage`, alongside the
existing `sim.recent` key), and the grid tile behind the modal
immediately updates to show that Print's art.

**Blocked by:** 01

- [ ] A new `localStorage`-backed adapter (load / save) reads and
      writes a card-name → print-id preference map, wrapped in
      try/catch and degrading silently on storage failure, matching
      how `loadRecent`/`clearRecent` are handled today.
- [ ] A new pure function resolves a card's print in three tiers:
      stored preference, then a passed-in context print id, then the
      deterministic first-by-print-id fallback from ticket 01 —
      covered by direct unit tests for all three tiers.
- [ ] Tapping an enabled tile opens a modal of every Print sharing
      that name, none highlighted on open.
- [ ] Tapping a Print in the modal persists it as the preference and
      the underlying grid tile's art updates to match, without a
      page reload.
- [ ] A card with a stored preference keeps showing that Print's art
      on return visits to Settings (tier 1 wins over tier 3).
- [ ] `npm run test`, `npm run lint`, `npm run typecheck`,
      `npm run build` all pass.
