# A Tool attaches like Energy, not through PlayTrainer

**Status:** Accepted — 2026-09-08

## Context

Nothing built attached a Tool: `Action::PlayTrainer`'s `TrainerKind::Item
| TrainerKind::Tool` arm sent both straight to discard, and
`legal_actions` offered a Tool the same "no target" way it offers an
Item. A Tool needs a target at play time — it stays attached, the way
Energy does — which `PlayTrainer` has no field for.

Two shapes were live. First: open a phase (`Phase::AttachingTool`) the
way a Trainer's own effect usually does, naming the card and waiting for
`Action::ChooseToolTarget`. Second: skip `PlayTrainer` altogether and
generate `Action::PlayTool { card, target }` directly in `legal_actions`,
the same place `Action::AttachEnergy` is already generated — one action
per legal target, no phase, resolved in one step.

## Decision

`PlayTool` follows `AttachEnergy`'s shape exactly: generated in the same
loop over hand cards, immediate, no phase. `legal_actions`' Trainer loop
now filters `TrainerKind::Tool` out before considering `PlayTrainer` at
all — a Tool is never offered that way — and `Action::PlayTrainer`'s
`TrainerKind::Tool` arm in `engine.rs` is `unreachable!()`, matching how
`Destination::Attach` was already `unreachable!()` for a `TakeCard`
without a target.

One Tool per Pokémon is enforced the same place: the target loop skips
any Pokémon already carrying a Tool, so illegality is structural —
nothing separate checks a limit. Unlike `Limit::EnergyAttached`, no
once-per-turn limit applies; a Tool is unlimited the way an Item is,
just with a target Energy also needs.

## Consequences

What a Tool actually does once attached is still open — `PlayTool`
moves the card, nothing more. That reads as a static or triggered
effect once one exists, not through the same dispatch a one-shot
Trainer effect runs through `resolve_trainer` at play time. This ADR
only settles how the card enters play. `Action::PlayTrainer` no longer
needs to know Tools exist at all beyond the `unreachable!()` guard.
