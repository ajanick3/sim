# 03 — The preference applies everywhere art renders, overriding a decklist's pin

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

- [ ] Every render site that currently passes a print id to `artUrl`
      is audited and, where that print id names a real card in play
      or in a decklist (not synthetic/placeholder art), passes it
      through the three-tier resolver first.
- [ ] A card whose Decklist line pins one Print, with a different
      Print preferred in Settings, renders the preferred Print's art
      on the board, in the deck builder, and in the log.
- [ ] A card with no stored preference renders identically to before
      this ticket, in every one of those contexts.
- [ ] `npm run test`, `npm run lint`, `npm run typecheck`,
      `npm run build` all pass.
