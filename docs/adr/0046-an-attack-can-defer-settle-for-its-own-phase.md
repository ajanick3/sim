# An attack can defer `settle` for a phase it opens mid-resolution

**Status:** Accepted — 2026-09-08

## Context

`Handheld Fan` reads the same trigger `Punk Helmet` and `Lucky Helmet`
do (ADR 0045), but its move — "move an Energy from the Attacking
Pokémon to 1 of your opponent's Benched Pokémon" — needs a choice: which
Energy, onto which Benched Pokémon. `Action::Attack`'s handler calls
`attack(state, index)` then `settle(state)` unconditionally; a choice
mid-attack has nowhere to pause that flow before this card.

## Decision

`trigger_defenders_tool` opens `Phase::MovingEnergyForHandheldFan`
instead of resolving outright, when the attacker has both an Energy to
move and a Bench to move it onto — the same "no legal targets, so
nothing happens" pattern `has_a_target` already applies to a played
Trainer. `Action::Attack`'s handler checks `state.phase == Phase::Main`
before calling `settle`, skipping it when a phase was opened.
`Action::MoveEnergyForHandheldFan`'s own handler calls `settle` once the
choice is made — the deferred knockout decision the interrupted attack
was owed runs there instead, not lost or duplicated.

This composes with `attacking_defender` (ADR 0044) and the "even if
Knocked Out" ordering (ADR 0045) without new interaction: the flag is
set before `trigger_defenders_tool` runs and read only once
`knock_out_the_dead` finally executes, whichever `settle` call that
turns out to be.

## Consequences

An attack's resolution can now pause for a player's choice partway
through, the same way playing a Trainer already can. A future attack-
triggered effect needing a choice reuses this shape: open a phase from
inside `attack()` or `trigger_defenders_tool`, and let
`Action::Attack`'s `Phase::Main` check skip `settle` until that phase's
own action resolves it.
