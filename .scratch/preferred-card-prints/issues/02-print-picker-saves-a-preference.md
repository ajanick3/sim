# 02 — Tapping a card opens a print picker that saves a preference

Status: resolved

**What to build:** Tapping an enabled tile in the catalog grid (ticket
01) opens a modal listing every known Print of that card name, with
nothing pre-highlighted — the modal never implies a choice was already
made. Tapping a Print in the modal saves it as that player's preferred
Print for that name, in this browser (`localStorage`, alongside the
existing `sim.recent` key), and the grid tile behind the modal
immediately updates to show that Print's art.

**Blocked by:** 01

- [x] A new `localStorage`-backed adapter (load / save) reads and
      writes a card-name → print-id preference map, wrapped in
      try/catch and degrading silently on storage failure, matching
      how `loadRecent`/`clearRecent` are handled today.
- [x] A new pure function resolves a card's print in three tiers:
      stored preference, then a passed-in context print id, then the
      deterministic first-by-print-id fallback from ticket 01 —
      covered by direct unit tests for all three tiers.
- [x] Tapping an enabled tile opens a modal of every Print sharing
      that name, none highlighted on open.
- [x] Tapping a Print in the modal persists it as the preference and
      the underlying grid tile's art updates to match, without a
      page reload.
- [x] A card with a stored preference keeps showing that Print's art
      on return visits to Settings (tier 1 wins over tier 3).
- [x] `npm run test`, `npm run lint`, `npm run typecheck`,
      `npm run build` all pass.

## Answer

Resolved 2026-09-11, branch `feat/print-picker-preference`, commit
`7dddc8a`.

`web/app/printPrefs.ts` carries `loadPrintPrefs`/`savePrintPref`
against `sim.printPrefs`, untested in detail per the spec's testing
decision (matching `loadRecent`/`clearRecent`, which are also
untested). `resolvePrint` lands in `web/app/prints.ts` with four new
tests in `prints.test.ts` covering all three tiers plus a stale
preference (naming a print no longer in the catalog) falling through
rather than rendering broken — not called for by the AC, but a real
edge case the function's shape made easy to guard. The Settings page
grew a modal (mirroring the log modal's fixed-overlay style already in
`LiveBoard.tsx`) opened by tapping an enabled tile; picking a Print
saves it and closes the modal, and the grid's own resolution now runs
through `resolvePrint` with a null context print (tier 2 is exercised
in ticket 03). `npm run test` (87 passing), `lint`, `tsc --noEmit`,
and `next build` all pass.
