# The web front end draws the game as one board, not a choice of two.

**Status:** Accepted — 2026-09-10

## Context

The web front end carried two renderings of the same game. The first laid
the game out as labelled lists of Pokémon and grouped action buttons. The
second — added later behind the `/live` route and a `variant` prop — laid
it out as a felt mat with card art, a hand strip, drag-and-drop, and every
action driven from the card it belongs to. Both read the same wasm view
and drove the same `apply`; only the presentation differed.

Keeping both meant every board change was made twice or made once and left
the other to rot, and the shared pieces drifted: card sizes, corner radii,
the card shadow, and the Energy palette were each spelled out several
times, and the mat board depended on modules that belonged to the list
board. The mat board was the one people used.

The live alternatives were: keep both and invest in a shared component
layer that serves each; keep only the list board; keep only the mat board.
The mat board won — it is what the project wants the game to look like, it
already covers every phase the list board did, and one board is half the
surface to maintain and the only way the shared card look gets a single
home.

## Decision

The mat board is the board. It renders at `/`; `/live` redirects there.
Its components live under `app/board`, with the card tokens in
`globals.css` and a named size scale, so the card frame has one
definition. The engine wiring — loading wasm, replaying the recipe the URL
carries, applying moves, keeping the address bar in step — is
`app/game-shell.tsx`, which knows nothing about layout.

## Consequences

The list board's components and its `variant` prop are gone, and with them
the pass-the-device reveal gate, which only existed to differ between the
two. A board component is not finished without a Storybook story covering
its states; `web/README.md` carries that rule.
