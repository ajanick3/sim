# Energy types in a cost

Type: task
Status: resolved

Make an attack cost read Energy types instead of counting cards.

- [x] A cost names types, and Colorless accepts any Energy
- [x] `legal_actions` offers an attack only when the attached Energy pays it
- [x] Retreat pays its cost the same way, and the player chooses what to discard

## Answer

Resolved 2026-09-05 on branch `feat/energy-types-in-a-cost`, commits `836d221`,
`3f16f0a`, `400712c`, and `ad18d53`.

An attack cost is now one entry per Energy. A `Colorless` entry takes any
Energy; every other entry takes its own type. The named types are matched
first, because a Colorless entry would otherwise eat the one Energy a named
entry needed — `tests/energy_costs.rs` covers that case directly.

Retreat opens a `DiscardingForRetreat` phase and asks once per Energy the cost
owes, which follows ADR 0003: a mid-action choice is an ordinary state, not a
suspended function. A Retreat Cost of zero asks nothing.

Two things the tests caught:

- A test hunted for a "Colorless Energy" card. Colorless in a cost means any
  Energy; there is no such card.
- Two attack tests passed for the wrong reason, because rule 17 skips the first
  attack step and the fixture never left turn one. The fixture now ends that
  turn first.

The zero-cost retreat test was green when written, because the branch already
existed from the previous slice. It is a characterization test, not a red-green
cycle.
