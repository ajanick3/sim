# A Tool's damage bonus reads every attack, not "this turn"

**Status:** Accepted — 2026-09-08

## Context

`Brave Bangle` and `Binding Mochi` both add damage against the
opponent's Active, conditioned on the attacker's own state (no Rule
Box; Poisoned) rather than on anything a Supporter granted "this turn."
`damage_dealt`'s existing bonus, `state.turn_bonus`, is read once per
turn and cleared in `begin_turn` (ADR 0028) — the wrong lifetime here.
A Tool's bonus applies to *every* attack the Pokémon it is attached to
makes, for as long as it stays attached and its own condition holds,
turn after turn.

## Decision

`damage_dealt` gains a step (32b, between the existing `turn_bonus`
step and Weakness/Resistance) that loops over the attacker's own
`attached` list, matches any Trainer whose effect is
`BonusDamageWithoutRuleBoxVsEx` or `BonusDamageIfPoisonedVsActive`, and
adds the bonus when that effect's own condition holds — read fresh on
every call, nothing stored or cleared between turns. Both restrict to
the opponent's Active implicitly: the single-Active format makes the
defender always the opponent's Active, so neither variant needed a
target field the way `TurnBonusTarget` does for a Supporter's bonus,
which can outlive the Active it was granted against.

This is the same fork ADR 0042 named for HP and Retreat Cost, applied
to damage instead: derive at read time from whatever is attached now,
rather than mutate a stored value. `turn_bonus` stays exactly as it
was — a *third* lifetime alongside "read every attack" (this) and
"read whenever a stat is asked for" (ADR 0042), not a shape either of
those needed to change.

## Consequences

A future Tool whose bonus is conditioned on the *defender's* state
(not only the attacker's own) fits the same step; one still restricted
to "this turn" would need `turn_bonus`, unchanged. Three bonus
lifetimes now coexist in `damage_dealt`, each reading a different kind
of state — a fourth would be worth naming the pattern rather than
adding a fourth ad hoc step.
