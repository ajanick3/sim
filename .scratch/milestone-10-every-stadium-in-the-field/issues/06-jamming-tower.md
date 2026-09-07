# Jamming Tower

Type: task
Status: resolved

*"Pokémon Tools attached to each Pokémon (both yours and your
opponent's) have no effect."*

New: `GameState::tools_disabled()`, checked at all five Tool read sites
Milestone 9 built. A Tool can still be attached; it simply does
nothing while this Stadium is in play. Recorded in
[ADR 0052](../../../docs/adr/0052-a-stadium-can-turn-off-every-tool-read-site-at-once.md).

- [x] Turns off every Tool's effect, both sides, while attached and
      unremoved

## Resolution

One new `GameState` method, five call sites updated.

Coverage: `admitted` 506 -> 509 (3 prints); `trainers` (refused)
285 -> 282.
