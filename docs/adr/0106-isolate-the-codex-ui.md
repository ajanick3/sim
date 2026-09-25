# Keep the Codex UI inside its own route

**Status:** Accepted — 2026-09-25

The production board at `web/app/play` must stay stable. A new UI design
needed a place to grow without touching that route's shared components.
The team chose to add the new UI as a self-contained tree at
`web/app/codex`, built with React and CSS Modules, that imports the
existing wire types, wasm engine loader, and session and deck helpers
read-only. It defines no MUI dependency; the MUI-based packages this
repository once carried (`packages/ui`, `packages/chatgpt`) are gone as
of PR #415, so the new UI could not depend on them even by accident.

## Context

Two live alternatives existed:

- Extend the shared board components under `web/app/board` to carry the
  new visual design. This risks changing behavior on the production
  `/play` route, since both routes would read the same components.
- Add a new, isolated route tree that reuses only the read-only wire
  contract (`web/app/view.ts`) and the existing engine, session, and
  deck modules, but owns its own components, styles, and tests.

## Decision

The new UI lives at `web/app/codex`. Every directory prefixed with `_`
(`_assets`, `_components`, `_fixtures`, `_game`, `_styles`) holds a
module, not a Next.js route. The two route trees are `play/`, which
loads the wasm engine through `_game/CodexGameShell.tsx` and plays a
live or replayed game, and `preview/`, which renders five static
review pages (`atoms`, `components`, `board`, `deck-search`, `actions`)
against fixture data with no engine dependency. `_game/adapter.ts`
converts a `WireView` (and its `WireCard`, `WirePokemon`,
`WireActionMeta` fields) from `web/app/view.ts` into this UI's own
props; it does not alter those wire types.

## Consequences

- The `/play` route and its components are unchanged; `/codex/play`
  is additive.
- Card display markup exists twice, once under `web/app/board` and
  once under `web/app/codex/_components`, until one design replaces
  the other. A future ADR should record that replacement when it
  happens.
- The five `/codex/preview/*` routes ship as static production pages
  alongside `/codex/play`. They exist for visual review of the
  components in isolation and are not linked from the production
  navigation.
