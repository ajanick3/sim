# Codex UI

This directory holds a second, isolated UI for the game board. It lives
at `/codex/play` and does not change the production `/play` route or
its components. See [ADR 0106](../../../docs/adr/0106-isolate-the-codex-ui.md)
for why the UI is isolated this way.

## Routes

- `/codex/play` runs a live or replayed game against the real wasm
  engine, the same way `/play` does.
- `/codex/play?g=<recipe>` opens a saved replay.
- `/codex/preview/atoms` shows the base card parts and controls.
- `/codex/preview/components` shows Pokémon cards, hand cards, piles,
  prizes, damage, targets, and a two-row hand.
- `/codex/preview/board` shows the full board at mobile, tablet, and
  desktop widths.
- `/codex/preview/deck-search` shows the responsive deck-search sheet.
- `/codex/preview/actions` shows the compact legal-action dialog.

Every `preview/*` route renders against bundled fixture data. It does
not load the game engine, so it never needs a running game to review.

## Directories

Any directory named with a leading underscore holds a module, not a
route; Next.js skips it when it builds the route tree.

| Directory              | Holds                                                              |
| ---------------------- | ------------------------------------------------------------------ |
| `_components/atoms/`   | Base components, stories, styles, and tests                        |
| `_components/cards/`   | Pokémon cards, hand cards, and deck/discard piles                  |
| `_components/regions/` | Battlefield, benches, hand, piles, active cards, and prize zones   |
| `_components/dialogs/` | Dialog frame, deck search, and legal-action choices                |
| `_game/`                | The wire adapter and the live wasm game shell                     |
| `_assets/`              | Dreepy artwork for Pokémon examples; Crushing Hammer for Trainer examples |
| `_fixtures/`            | Fixed image references used by the preview routes                 |
| `_styles/`              | CSS variables scoped to this UI                                    |
| `preview/`              | The five static preview routes listed above                        |
| `play/`                 | The live game route                                                 |

## The wire boundary

`_game/adapter.ts` is a pure function. It takes a `WireView` (with its
`WireCard`, `WirePokemon`, and `WireActionMeta` fields), all defined in
`web/app/view.ts`, and returns this UI's own props. It reads that file;
it does not change it. `_game/CodexGameShell.tsx` is the one file in
this tree that touches the running game: it imports the existing
`loadSim` and `Game` loader from `web/app/wasm`, and the existing
session, recipe, art, decks, and print-preference helpers, unchanged.

## Components

`CardFrame` owns the `245 / 337` card ratio; its parent sets the width.
`CardArt` requires an explicit image source, draws the full image with
`object-fit: contain`, and handles a failed load. `CardBack` draws the
card back. `CardSlot` draws an empty slot and can expose a placement
button. `PrizeMarker` shows a remaining or a taken prize. `CountBadge`,
`HealthBadge`, and `DamageCounter` each show one indicator; a health
badge shows only the number, and its accessible label keeps the full
HP description. `IconButton` supplies a named 44px control.

Wrap these components in the `theme` class from
`_styles/theme.module.css`. Place them below a Client Component
boundary when they use callbacks or `CardArt`; each `preview/*` page
and `play/page.tsx` already supplies that boundary.

## Checks

Run `pnpm test` from `web` for the full suite, including this
directory's tests. Run `pnpm typecheck` for TypeScript checks. The
repository's Storybook glob picks up the new stories under
`Codex/Atoms` and the other component groups without a config change.
