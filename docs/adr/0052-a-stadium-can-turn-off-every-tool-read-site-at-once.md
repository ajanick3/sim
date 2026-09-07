# A Stadium can turn off every Tool read site at once

**Status:** Accepted — 2026-09-08

## Context

`Jamming Tower` reads: *"Pokémon Tools attached to each Pokémon (both
yours and your opponent's) have no effect."* Unlike every other Stadium
built this milestone, which extends one or two specific read sites,
this one needs to reach every place a Tool's own effect is read at all:
`effective_hp`, `effective_retreat_cost`, `damage_dealt`'s attacker-side
Tool loop, `trigger_defenders_tool`, and `powerglass_owner` — five
sites, built across five separate tickets in Milestone 9.

## Decision

`GameState::tools_disabled()` reads `stadium_effect() ==
Some(TrainerEffect::ToolsHaveNoEffect)` once; every one of the five
sites checks it before reading anything from `attached`, short-
circuiting to the same answer as if no Tool were attached at all — not
by removing the Tool or clearing its attachment, which stays exactly
where it is. A Tool can still be *played* under `Jamming Tower`
(`PlayTool`'s own offering is untouched): the card text disables what a
Tool *does*, not whether one may be attached.

## Consequences

Any future Tool effect, at any new read site, must check
`tools_disabled()` the same way — this is now a standing obligation on
every Tool read site, not only the five that existed when `Jamming
Tower` was built. A Stadium disabling a narrower slice of Tools (by
type, by kind of effect) would need its own, more specific check;
nothing built needs that yet.
