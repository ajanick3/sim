# Masked observation views

Type: research
Status: resolved

Decide how a player sees only their own hidden zones, before any bot exists.

A bot handed the whole state can read the opponent's hand and the Prizes and
cheat without meaning to. Retrofitting the mask after bots exist is painful.

- [x] A recorded decision on how a view is built and what it hides
- [x] The cost of the mask, measured against a headless self-play loop

## Answer

Resolved 2026-09-05 on branch `feat/masked-observation-views`.

The decision is [ADR 0006](../../../docs/adr/0006-a-player-sees-the-game-through-a-masked-view.md).
`PlayerView::of(state, player)` builds an owned value carrying only what that
player may see. It hides the opponent's hand, both libraries and their order,
and both Prize piles — a player cannot see their own Prizes either. Every
hidden zone keeps its count, because a count is public.

A bot never calls `legal_actions`, which takes the state. The engine computes
the list and hands it over with the view, so a bot is
`(view, legal_actions) -> Action`.

**The measurement.** `src/bin/selfplay.rs` plays headless games and reports the
cost per decision. Over 5000 games and 74333 decisions, release build, three
runs each: 1.02 microseconds per decision without views, 1.26 with. So 26400
games a second becomes 21500 — 23% of the decision, and it is paid.

The work also turned up a duplicate `Cinderpup` definition in `src/cards.rs`,
added in `42cbd65` and shadowed ever since. It is deleted.
