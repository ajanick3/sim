# An attack's effect is its own vocabulary, not a reused `TrainerEffect`

**Status:** Accepted — 2026-09-08

## Context

Every Trainer-kind milestone read a card's own effect through
`TrainerEffect` and `resolve_trainer`, matched by name through
`known_trainer` (ADR 0020's print-override shape sitting in front of
it). `read_attack` has refused any attack with printed text
unconditionally since the record effort — nothing has ever read what
that text says. The sample (`.scratch/milestone-11-.../spec.md`) shows
attack effects are a different shape from Trainer effects: read from
`attack()`, not `resolve_trainer`; scoped to an attacker and a
defender, not a player and a played card; several (recoil, a
Bench hit, ignoring the defender's own effects) have no Trainer
counterpart to reuse at all.

Two shapes were live. First: extend `TrainerEffect` with attack-shaped
variants, read by a new arm in `resolve_trainer` given a synthetic
"card" for the attack. Second: a parallel enum, `AttackEffect`, read by
a new function, `resolve_attack_effect`, called from `attack()` itself.

## Decision

`AttackEffect`, parallel to `TrainerEffect`. `Attack` gains an `effect:
Option<AttackEffect>` field, read once `attack()`'s own damage and
`inflicts` have landed — the same position `resolve_trainer`'s call
sits in `PlayTrainer`'s handler, but inside a different function. A new
`known_attack(pokemon_name, attack_name)` mirrors `known_trainer`
exactly: checked in `read_attack` before the unconditional refusal,
keyed on the *pair* — a Pokémon's own name is not unique the way it
would need to be alone (many prints share an attack name across
different Pokémon), so the lookup key is one dimension richer than
`known_trainer`'s single name. `known_trainer_by_print`'s print-override
shape is not mirrored yet — no name-pair collision has surfaced; the
day one does, it extends the same way ADR 0020 already set the
precedent for.

Where a shape genuinely matches a `TrainerEffect` already built
(`SwitchOwnActive`, `SwitchOpponentActive`, a `Decide`-to-Bench search,
a coin flip), the plan is to dispatch `AttackEffect` variants into the
*same* phase machinery those already use, rather than duplicate it —
not yet exercised; the first ticket needing one of those shapes settles
exactly how.

## Consequences

Two effect vocabularies now coexist, matched by different keys
(`TrainerEffect` by a bare name; `AttackEffect` by a name pair) and
dispatched from different call sites. A card that is both a Trainer and
prints an attack never exists, so there is no ambiguity about which
vocabulary a given piece of text belongs to. `AttackEffect` starts with
one variant (`Recoil`); every following ticket in this milestone adds
more, the same way `TrainerEffect` grew one card at a time.
