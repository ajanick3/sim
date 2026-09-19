# Two board mockups compare live behind a scoped, temporary variant.

**Status:** Accepted — 2026-09-19

## Context

PRs #341 and #342 each mock up a board style direction — rounded court
with a glow ring and pill END TURN, and full-portrait Active cards — as
real code in `app/board`, viewable on their own preview deployment. The
operator wanted both routed on the same live site, at `/341` and `/342`,
so the two directions and `/play` sit side by side on one phone instead
of three separate URLs.

[ADR 0100](0100-the-web-front-end-has-one-board.md) tore out the board's
last `variant` prop for exactly the failure this risks repeating: two
paths through the same components drift, and the shared look — card
sizes, corner radii, the card shadow — gets spelled out more than once.
The live alternatives here were the same as before: keep the mockups on
separate preview deployments only (0100's answer), or reintroduce a
variant the shipped board branches on.

This case differs from 0100's in what the variant is for and how long it
lives. 0100's two boards were both permanent, user-facing paths through
the same product; a change to one and not the other was a real bug. Here
`/play` is the only route a player ever reaches — `/341` and `/342` are
comparison scaffolding for one decision, named for the PRs they came
from, and every branch they add is commented as such. A scoped variant
merged onto `main` lets the operator compare on their phone without
juggling three deployments; the alternative cost of not merging it is a
worse comparison, not a maintenance saving, since the code re-merges once
a direction is picked either way.

## Decision

`app/board/variant.ts` exports a `BoardVariant` context
(`classic` | `rounded-court` | `card-forward`), defaulting to `classic`.
`LiveBoard`, `SideRail`, `StadiumSlot`, and `LiveMon` read it and branch
only where the two mockups actually differ from the shipped board:
corner radius, the centre-lane dividers and Active glow, the END TURN
shape, the Stadium empty-state label, and the Active card's size and
crop. `/341` and `/342` each wrap `GameShell` in the context provider
with their variant; `/play` never sets it, so `classic` is the only path
it can reach.

## Consequences

This is scoped and dated to expire, not a standing exception to 0100:
once a direction is picked (or neither is), `/341`, `/342`,
`app/board/variant.ts`, and every branch on it come out in the same
change, and the board is back to the one path 0100 decided on. Until
then, the classic path is the only one `/play` exercises, so a
regression in either mockup's branch cannot reach it — but a change to a
touched component still needs a glance at the branch it added, the cost
0100 named.
