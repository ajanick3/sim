# Rule 32: does "stop at 0" run before or after the step-32 additions?

Type: research
Status: needs-triage

The senior review (Spec axis, fix-first 2) found the damage sequence
disagrees with the rulebook on ordering.

## What the code does

`engine.rs:4413-4416`, in `damage_dealt_with`:

```rust
// Step 32: effects on the attacking player's Pokemon. Stop at 0.
if damage == 0 {
    return 0;
}
```

At this point `damage` holds `base` only. After the check,
`engine.rs:4417-4502` adds `turn_bonus`, the attacker's Tools
(`BonusDamageWithoutRuleBoxVsEx`), Cobalt Command, and Lose Cool.

## What the rulebook says

`docs/architecture/rules.md:70-71`:

> Apply effects on **your** Pokémon (before Weakness/Resistance). **Stop if
> damage is 0.**

The stop follows the your-Pokemon effects, not precedes them.

## The consequence

A 0-base attack that carries `BonusDamageWithoutRuleBoxVsEx` on an attacker
Tool yields 0. It never reaches the Tool loop. If the same attack had one
point of base damage, the Tool bonus would apply.

## The decision

Two live alternatives:

1. **Code order is right.** An attack with 0 base damage is a "no damage"
   attack and the step-32 additions never fire on it. Amend `rules.md` and
   record why.
2. **Rulebook order is right.** Move the `if damage == 0` check below the
   step-32 additions. Add a test for the 0-base plus Tool-bonus case.

Pick one against the rulebook and the real card behaviour. Do not leave
both readings in play.

## Acceptance criteria

- [ ] An ADR records which order holds and why.
- [ ] `rules.md` and `engine.rs` agree with the ADR.
- [ ] If the check moves, a test covers a 0-base attack with a step-32
      damage bonus.
