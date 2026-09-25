# Spec: a text-based front end

The engine's wasm seam (`crates/sim-wasm`) already returns plain JSON from
`legal_actions()`, `action_meta()`, `view()`, and `log()`, and takes a
single `apply(index)` call back. A terminal client can drive a game
through this same seam without a browser.

## In scope

- A numbered-menu terminal client: one line per legal action, drawn from
  `action_meta()`, with board state from `view()` and history from
  `log()`.
- Reading input as a menu index and calling `apply(index)` directly — no
  board-tap gesture, so no dialog-auto-popup problem to solve.

## Out of scope for now

- Any UI reuse from `web/` (React, `CodexGameShell.tsx`). A terminal
  client is a new, small binary, not a port.
- Any change to `sim` or `sim-wasm`. The seam already carries what a menu
  needs.

## Constraints

- The engine stays pure (ADR 0004). The client is a new consumer of the
  existing seam, not a reason to grow it.
