# The attach mechanism

Type: task
Status: resolved

Nothing built attaches a Tool: `PlayTrainer` sent every
`TrainerKind::Tool` straight to discard, the same as an Item.

- [x] `Action::PlayTool { card, target }`, generated the same place
      `AttachEnergy` is, immediate, no phase
- [x] A Pokémon carries at most one Tool
- [x] No once-per-turn limit, unlike Energy
- [x] `Action::HealMegaEx` moves only Energy off a healed Pokémon, not a
      Tool sharing the same `attached` list — the one unsafe site the
      milestone's own `.attached` audit found

## Resolution

New `Action::PlayTool`, generated alongside `AttachEnergy` rather than
through `PlayTrainer`. Recorded in [ADR 0041](../../../docs/adr/0041-a-tool-attaches-like-energy-not-like-an-item.md).

The `.attached` audit from the spec confirmed every other reader
(`DiscardingForRetreat`, `DiscardingOpponentEnergy`,
`ActiveHasAtLeastEnergy`, `MoveAttachedEnergy`,
`MoveEnergyFromBenchToActive`, `attached_energy_types`/`pays_cost`,
`knock_out`, `view.rs`) already filters by `is_energy()` or discards
the whole list regardless of kind — both correct once a Tool can be in
`attached`. `Action::HealMegaEx` did not filter, fixed here with a
regression test (`wallys_compassion_leaves_an_attached_tool_in_place`)
since nothing built a Tool yet when that card first shipped.

No card is admitted yet — this ticket built only the mechanism a real
Tool needs to attach at all.
