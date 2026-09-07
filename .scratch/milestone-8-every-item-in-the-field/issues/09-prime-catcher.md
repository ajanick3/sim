# Prime Catcher

Type: task
Status: resolved

*"Switch in 1 of your opponent's Benched Pokémon to the Active Spot. If
you do, switch your Active Pokémon with 1 of your Benched Pokémon."*

New: `TrainerEffect::SwitchOpponentActiveThenOwn`, which is
`SwitchOpponentActive` with a new `PromoteFollowUp::AlsoSwitchOwnActive`
chained onto it — the first switch reuses `Boss's Orders`'s shape
outright.

- [x] The opponent's switch happens first, chosen by the player
- [x] The player's own switch follows, and is skipped with an empty Bench
- [x] Neither switch loses the displaced Active

## Resolution

One new `TrainerEffect`, one new `PromoteFollowUp` variant. Found and
fixed a real bug along the way: `Action::Promote`'s unconditional
`settle(state)` call reset the new phase this follow-up opened straight
back to `Main`, the same class of bug ADR 0030 already named for a paid
cost. Fixed by returning early when the follow-up actually opens a new
phase. Recorded in [ADR 0037](../../../docs/adr/0037-a-follow-up-phase-must-skip-settle.md).

Coverage: `admitted` 479 -> 481 (2 prints); `trainers` (refused)
312 -> 310.
