# Is an effect-enum match a tripwire or a classifier?

Type: task
Status: needs-triage

The senior review (Standards axis, finding 8) found two effect-enum
matches that make opposite choices, each defended by its own comment.

## The two sites

`state.rs:1035`, in `effective_hp` — every `EnergyEffect` variant spelled
out, so a new variant fails to compile until this site handles it:

```rust
.map(|e| match e.effect {
    Some(EnergyEffect::IncreasesCarrierHp(amount)) => amount,
    Some(EnergyEffect::DrawCardsOnAttachFromHand(_) | /* … every other */ ) | None => 0,
})
```

`strategy.rs:178` — a catch-all, on purpose:

```rust
_ => EffectCategory::Other,
```

Both are deliberate. `effective_hp` wants the compiler to list every site
when a card is added. `strategy.rs` wants to tolerate a new effect without
a rebuild of the heuristic.

## Why it needs a record

A reader adding the next effect-enum match cannot tell which pattern the
repo wants. The choice has a real trade-off: an exhaustive match turns
"add a card, the compiler shows every site to update" into a guarantee; a
catch-all gives that up for churn resistance.

## The decision

State the rule once. The likely shape: a match that drives a rules outcome
is exhaustive; a match that only classifies for a heuristic or for display
may use a catch-all. Record it where a contributor will find it —
`docs/architecture/effects.md` or a short ADR.

## Acceptance criteria

- [ ] The rule is written in one place under `docs/architecture/`.
- [ ] The comments at `state.rs:1035` and `strategy.rs:178` point to it
      instead of each arguing the case alone.
