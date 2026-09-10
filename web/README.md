# web

A browser front end for the engine. One person plays both seats, hotseat
style, with a screen between turns.

The engine runs as WebAssembly. It compiles from `crates/sim-wasm` and holds
the whole game in the tab; nothing but a card artifact and two deck files
crosses from the page. See `docs/adr/0096-the-engine-compiles-to-wasm-behind-an-opaque-handle.md`.

The current game is encoded into the `?g=` query string — its seed, decks,
and every move — so a link resumes the exact position (`app/recipe.ts`,
replayed by `Game.replay_standard`). "Copy link" shares it.

## Run it

```sh
npm install
npm run wasm     # build public/pkg from crates/sim-wasm (needs the Rust toolchain)
npm run dev      # http://localhost:3000
```

`npm run dev` and `npm run build` first run `npm run assets`, which copies
`data/cards.json` and the two curated deck files into `public/`.

## Checks

```sh
npm run check        # lint + format check + typecheck + tests, in one
npm run test:watch   # vitest, watching
npm run fmt          # oxfmt, writing
```

`oxlint` lints, `oxfmt` formats, `vitest` runs the tests in
`app/*.test.ts`. Pure session logic lives in `app/session.ts` so it can be
tested without a DOM or the engine. CI runs `npm run check` and
`npm run build` on every push.

## Rebuild the engine

`public/pkg` is committed, so a deploy needs no Rust. After any engine
change, run `npm run wasm` and commit the result.

Prerequisites for `npm run wasm`:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

## Deploy

Vercel, project root `web/`, domain `sim.nickaja.rocks`. The build command is
the default `next build`; no Rust runs on Vercel because `public/pkg` is in
the repository.

## Layout

| Path                | Holds                                             |
| ------------------- | ------------------------------------------------ |
| `app/table.tsx`     | The client component: board, actions, log, gate  |
| `app/wasm.ts`       | Loads and initialises the WebAssembly module     |
| `app/view.ts`       | The JSON shape `Game.view()` returns             |
| `public/pkg/`       | The built WebAssembly module and its glue        |
