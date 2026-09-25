# Map: a text-based front end

## Destination

A game plays end to end from a terminal, with no browser: board state
and legal actions printed each turn, a number typed to act.

## Notes

The wasm seam (`crates/sim-wasm/src/lib.rs`) exposes `legal_actions()`,
`action_meta()`, `view()`, `log()`, and `apply(index)`, all as JSON
strings, already exercised host-side in `crates/sim-wasm/tests/seam.rs`.
A terminal client reads this same seam directly (as a Rust crate calling
`sim-wasm`, or `sim` itself) — it does not need the wasm build or a
browser.

Because a menu lists every legal action as its own numbered row (rather
than resolving a tap against board state), the two UI problems the web
front end solved this session — the Attack auto-popup and the retreat
tap-target ambiguity — do not carry over. Retreat with more than one
legal Bench target would just be more than one numbered row
(`Retreat → Eevee`, `Retreat → Pidgey`).

## Tickets

- 01 — build the terminal client.

## Decisions so far

None yet. Set down for now at the user's request, 2026-09-25 — no active
work.
