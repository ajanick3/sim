# A defender's Tool can trigger mid-attack, before knockout is decided

**Status:** Accepted — 2026-09-08

## Context

`Punk Helmet` and `Lucky Helmet` both read: *"If the [Type] Pokémon
this card is attached to is in the Active Spot and is damaged by an
attack from your opponent's Pokémon (even if this Pokémon is Knocked
Out), [effect]."* Every effect built so far runs either at play time
(`resolve_trainer`) or is derived at read time (ADR 0042, ADR 0043,
both on the *attacker's* own attached Tools). Nothing yet reacts to the
*defender* taking damage, and "even if Knocked Out" means the trigger
cannot wait for `settle`'s knockout decision — it must run inside
`attack()` itself, while `defender.attached` still holds the Tool
`knock_out` would otherwise clear.

## Decision

`attack()` calls a new `trigger_defenders_tool(state, attacker,
defender)` right after applying damage, guarded by `damage > 0` (no
trigger from a blocked or zero-damage hit). It collects the defender's
attached Tools first, then matches each effect —
`DamagesAttackerWhenDefenderIsHit` (`Punk Helmet`) and
`DrawsWhenDefenderIsHit` (`Lucky Helmet`) so far — before `settle` ever
runs. "In the Active Spot" needed no separate check: this engine's
single-Active format makes the defender of an attack always the Active
being attacked.

Both built so far need no player choice, so neither opens a phase —
`attack()` applies them directly, the same way it already applies the
attack's own damage and inflicted condition. A Tool whose trigger *does*
need a choice (a future `Handheld Fan`) would need `attack()` to open a
phase instead of finishing outright, deferring `settle` until that
choice resolves — not yet needed here.

## Consequences

`attack()` is no longer only about the attacker's own damage output —
it is where a defending Tool's reaction lives too, mid-resolution,
before the knockout question `settle` asks. A third kind of Tool bonus
now exists in the codebase, alongside the attacker-side one (ADR 0043)
and the derived-stat one (ADR 0042): defender-side, triggered once per
hit rather than read continuously.
