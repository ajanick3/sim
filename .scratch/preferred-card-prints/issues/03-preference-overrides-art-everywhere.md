# 03 — The preference applies everywhere art renders, overriding a decklist's pin

Status: resolved

**What to build:** Every other place the app resolves a card's art by
print id — the board, the deck builder, the log, anywhere `artUrl` is
called with a print id sourced from a live game or a parsed Decklist —
threads the three-tier resolution from ticket 02 in front of that
call. A player's stored preference for a card name now wins over
whatever Print a Decklist line pinned (`4 Mega Venusaur ex MEG 3`
still renders the player's preferred art, not the `MEG 3` print), and
over whatever Print the engine's own name-only lookup produced. A
card with no stored preference keeps rendering exactly what it does
today — this ticket changes nothing for a name the player hasn't
opted into.

**Blocked by:** 02

- [x] Every render site that currently passes a print id to `artUrl`
      is audited and, where that print id names a real card in play
      or in a decklist (not synthetic/placeholder art), passes it
      through the three-tier resolver first.
- [x] A card whose Decklist line pins one Print, with a different
      Print preferred in Settings, renders the preferred Print's art
      on the board, in the deck builder, and in the log.
- [x] A card with no stored preference renders identically to before
      this ticket, in every one of those contexts.
- [x] `npm run test`, `npm run lint`, `npm run typecheck`,
      `npm run build` all pass.

## Answer

Resolved 2026-09-11, branch `feat/print-preference-everywhere`, commit
`3f9b74c`.

Every art render site in the app (board, hand, bench, drag ghost,
in-game library/discard pickers) already reached art through one
`art(printId)` function created in `game-shell.tsx` and threaded down
as a single prop through `LiveBoard` — an audit (`grep -rn "art(\|artUrl("`)
confirmed no render site calls `artUrl` independently except Settings
(already correct from ticket 02) and `art.test.ts`'s own unit tests.
Fixing the one function fixed every call site: `art` now resolves
`printId` through the new `resolveCardPrint` (given a `PrintIndex`
built once from the loaded catalog, next to the existing
`CardData.new` call, and preferences loaded on mount) before calling
`artUrl`.

Two AC mentions don't correspond to a real render site today: there is
no separate deck-builder page (card search/pick happens in-game via
`DecisionBar`, which is on the same `art` seam as everything else),
and the log renders text only, no card art. Both are covered by the
same fix trivially, since there's nothing separate to wire.

`buildPrintIndex`/`resolveCardPrint` land in `prints.ts` with 5 new
tests (18 total in the file), including one proving a preference keyed
by bare "Abra" never leaks onto either same-named Abra print. Checked
against real `public/cards.json` data (a Mega Venusaur ex preference
overriding its context print). Fixing `game-shell.test.tsx`'s
`fetch("/cards.json")` stub (it returns `""`) needed a try/catch
around the new `JSON.parse` — an unparsable artifact now just skips
print preferences rather than failing the whole game load, matching
the "preferences are a nicety" framing used elsewhere. `npm run test`
(92 passing), `lint`, `tsc --noEmit`, and `next build` all pass.
