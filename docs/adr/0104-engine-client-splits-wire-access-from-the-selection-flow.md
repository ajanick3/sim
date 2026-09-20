# `packages/engine-client` splits pure wire access from the reference Selection flow

**Status:** Accepted — 2026-09-20

Three UIs now drive the engine — `web/`, `packages/ui`'s Storybook, and `packages/chatgpt`'s live demo — and two of them (`web/` and `packages/chatgpt`) hand-duplicate the same wasm-loading, Wire-typing, and tap-interaction code, confirmed identical by diff. A new shared package was needed before a fourth UI made that duplication worse. The question was what it should contain: everything a UI currently duplicates, flat and bundled, or something narrower.

## Context

The duplicated code is two different kinds of thing. `loadSim` (wasm loading), the `WireView`/`WireCard`/`WireActionMeta` mirrors, and `apply(index)` are pure engine access — any UI needs exactly this, with zero opinion about how a player interacts. `Selection`, `movesForSelection`, `targetsForHandCard`, `groupActions`, and `shouldAutoAdvance` are a specific interaction model: tap once to select, tap again to confirm, group the rest for a menu, skip a forced single choice automatically. Every UI built so far is pointer/tap-based, so all three converged on this model independently — but a future non-tap consumer (a CLI menu, a voice interface, an automated driver sitting where a UI does) has no use for it.

Bundling both into one flat package would have made the "universal" half only accidentally universal — trustworthy today because nothing has tested the boundary, not because the boundary is real.

## Decision

`packages/engine-client` ships as two modules:

- `engine-client/wire` — `loadSim`, the Wire types, `apply`. No React, no interaction opinion.
- `engine-client/selection-flow` — the reference Selection state machine, built on `wire` but never required by it.

The package itself is framework-agnostic (no React dependency in either module) and isolated the way `packages/ui` and `packages/chatgpt` already are — its own `package.json`, no shared `node_modules` with any consumer, source-only (no npm publish).

Existing duplication in `web/app/{wasm,view,session}.ts` and `packages/chatgpt/src/live/{wasm,view,session,adapt}.ts` is not migrated by this decision. Both keep working as they are; a consumer adopts `engine-client` when it next touches that code, not on a deadline.

## Consequences

- A UI that only wants Wire access (a CLI, a bot, a minimal viewer) depends on `engine-client/wire` alone and never sees Selection-flow code in its bundle.
- The Selection flow's rules (auto-advance, action grouping) stay swappable independently of Wire access — a UI can reuse `wire` and write its own interaction model without forking anything.
- Two modules to keep straight instead of one, and a real risk of the split blurring over time if a Selection-flow concern (e.g. a display label) leaks into `wire`. `wire` accepting no import from `selection-flow` is the invariant that keeps the split honest.
- `web/` and `packages/chatgpt` carry known-duplicate code against this new package until each is migrated on its own schedule — an accepted, temporary cost, not an oversight.
