# The engine compiles to WebAssembly behind an opaque handle, and the web app that drives it lives in this repository

**Status:** Accepted — 2026-09-10

A person could play the engine only through the terminal binary. The next step was a browser: a page that shows the board and takes a click. Three boundaries were live. Compile the engine to WebAssembly and run it in the browser tab. Wrap the engine in an HTTP server and call it over REST. Serve a stateless HTTP endpoint that replays `{seed, deck, chosen actions}` on every request. WebAssembly won because the engine is already pure — no I/O, no async, no clock, and `SeededRng` is plain xorshift — so nothing in it needs a host. The game runs entirely on the client, with no server to deploy or keep alive. The web app is a Next.js static export in `web/`, in this repository, so one repository holds the whole system.

## Context

The engine is one crate with one dependency. A `wasm-bindgen` build target and an npm toolchain are both new directions. The card artifact `data/cards.json` is 2 MB. `GameState` and the `view.rs` types derive `Clone` but not `Serialize`. `import::load` leaks card strings to reach `'static`, which is free for a process that runs once and not free for a tab where a person starts many games.

## Decision

The root becomes a Cargo workspace. The engine stays at the root and keeps its dependency set; `default-members` names the engine alone, so a bare `cargo build` or `cargo test` never compiles the WebAssembly crate. `crates/sim-wasm` holds the browser boundary and joins `members`.

`crates/sim-wasm` exposes `Game`, a `#[wasm_bindgen]` type that owns a live `GameState`. JavaScript holds the handle for the life of the game and calls methods on it. No game state crosses the boundary as data. `Game::apply` takes an index into the list `Game::legal_actions` returned, the same contract `main.rs` uses, so `Action` is never serialized.

Card data is parsed once. `CardData::new(json)` reads and leaks the 2 MB artifact a single time. `Game::new(&mut card_data, deck_a, deck_b, seed)` clones the `CardDb` into each game. The JSON reaches the page as a static asset and is passed in, which keeps the seam of [ADR 0002](0002-pure-engine-with-a-json-seam.md) and keeps the `.wasm` small.

The board crosses the boundary as JSON. Deriving `Serialize` on the `view.rs` structs was rejected: they carry `Condition` from `card.rs`, `Phase` from `state.rs`, and the id types from `ids.rs`, so the derive would spread `serde` annotations across engine modules that [ADR 0001](0001-arena-and-index-state.md) owns, to serve a browser UI. Instead `crates/sim-wasm` defines its own wire structs that mirror `PlayerView` and maps into them. The UI reads a contract that does not shift when an internal view type changes. This does not widen what a player may see; [ADR 0095](0095-a-strategy-reads-a-masked-view-not-the-state.md) keeps the view masked and it stays masked.

Every boundary method returns `String`, holding JSON, not `JsValue`. The seam runs under an ordinary `cargo test` on the host target, with no browser.

The browser drives both seats. One person plays against themselves, with a screen between turns.

## Consequences

- New build tools: `wasm-bindgen` and `wasm-pack` for `crates/sim-wasm`, and an npm toolchain under `web/`. A separate repository for the web app was the alternative; one repository was worth the toolchain in the tree.
- `serde` is already present through `serde_json`. The wire structs add `serde_derive` as a direct dependency of `crates/sim-wasm` only. The engine gains no dependency, no derive, and no field.
- The Standard data is only partly playable; the engine refuses a card it cannot run ([ADR 0008](0008-the-engine-refuses-a-card-it-cannot-run.md)). The app ships one curated pair of fully-playable decklists. A picker over every playable deck is later work.
- A stale board is impossible to send, because no board is sent; the handle is the single copy.
