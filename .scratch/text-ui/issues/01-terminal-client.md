Type: task
Status: needs-triage

# Build a terminal client for the engine

## Body

Add a small binary that plays a game from a terminal, driving the
existing `sim-wasm` seam (or `sim` directly) instead of a browser.

Each turn, print:

- The board: both sides' Active and Bench Pokémon with HP, energy, and
  damage counters, from `view()`.
- Recent history, from `log()`.
- Every legal action as a numbered row, from `legal_actions()` and
  `action_meta()` — one row per action, including one row per legal
  Retreat target (a menu has no board tap, so it lists targets directly
  rather than needing a two-step arm/tap flow).

Read a number from stdin and call `apply(index)`. Repeat until the game
ends.

## Acceptance criteria

- [ ] A new binary (for example `crates/sim-tui`) plays a full synthetic
      game to completion from a terminal, with no browser or `wasm-pack`
      build involved.
- [ ] Every legal action `action_meta()` reports appears as its own
      numbered row each turn, including every legal Retreat target as a
      separate row.
- [ ] An out-of-range input is rejected without crashing the client.
- [ ] `cargo test` and any client-local tests pass.
