# @sim/engine-client

The shared adapter between the wasm-compiled engine and any UI. Framework-agnostic — no React, no DOM assumption beyond `import()` and `fetch` being available at runtime. See [ADR 0104](../../docs/adr/0104-engine-client-splits-wire-access-from-the-selection-flow.md) and the "Wire" / "Wire client" / "Selection" / "Selection flow" entries in [the domain glossary](../../docs/architecture/glossary.md).

## Two subpaths, on purpose

- **`@sim/engine-client/wire`** — loads the wasm module, exposes the typed Wire data (`WireView`, `WireCard`, `WirePokemon`, `WireActionMeta`), applies a move by its index into `legal_actions`. Every UI needs exactly this, unmodified, with zero opinion about how a player interacts.
- **`@sim/engine-client/selection-flow`** — the reference tap-to-choose interaction model, built on `wire` but never required by it: tracks what a player has tapped (a `Selection`), narrows legal moves to it, groups the rest for a menu, and decides when a forced single choice advances on its own.

There is no root export. A consumer that only wants engine access never sees Selection-flow code in its bundle; one that wants the reference interaction model imports both explicitly. `wire` imports nothing from `selection-flow` — that's the invariant that keeps the split real rather than nominal.

## What's deliberately not here

- **Card art.** Resolving a print id to an image URL is a display concern each UI owns differently (quality tiers, an installed-PWA distinction, none at all) — never part of what the engine is doing.
- **Anything privacy-gate or same-screen-multiplayer specific**, like a "pass the device" reveal gate. `AutoAdvanceInput.revealed` exists so a UI *with* such a gate can wire it in, but a UI with no such gate just always passes `true`.
- **Migrating existing consumers.** `web/app/{wasm,view,session}.ts` and `packages/chatgpt/src/live/{wasm,view,session,adapt}.ts` predate this package and are not migrated by its existence — each adopts it on its own schedule.

## Checks

```sh
pnpm install
pnpm run typecheck
pnpm run test
```
