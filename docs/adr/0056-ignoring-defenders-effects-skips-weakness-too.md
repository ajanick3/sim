# "Isn't affected by any effects on the defender" skips Weakness and Resistance too

**Status:** Accepted — 2026-09-08

## Context

`N's Zekrom`, `Mega Lopunny ex`, and `Dudunsparce ex` each print an
attack reading "This attack's damage isn't affected by any effects on
your opponent's Active Pokémon." The official ruling for this exact
phrasing (distinct from the older "unaffected by Abilities" wording)
turns off Weakness and Resistance along with any Tool or Stadium bonus
on the defender's side — both are, mechanically, effects the
defender's own card and board carry. `damage_dealt`'s step 33 already
applies both unconditionally; nothing before this attack needed to
turn either off.

`damage_dealt` is `pub` and several tests (`tests/milestone1.rs`,
`tests/tools.rs`, `tests/supporters.rs`) call it directly with the
ordinary four arguments — changing its signature would touch every one
of them for a flag only `attack()` itself needs to set.

## Decision

`damage_dealt` becomes a thin wrapper over a new private
`damage_dealt_with`, which takes a fifth argument,
`ignore_defenders_effects: bool`, and skips step 33 entirely when it is
set. `damage_dealt`'s own signature — and every existing caller — is
unchanged; only `attack()` calls `damage_dealt_with` directly, passing
`true` when `AttackEffect::IgnoresDefendersEffects` is the attack's own
effect. `resolve_attack_effect`'s arm for this variant is a no-op, the
same shape `DamagePerCount`'s is (ADR 0055) — the read already happened
before `attack()` called into the damage order at all.

## Consequences

`damage_dealt_with` is now where a future "skip some part of the
damage order" attack effect would add its own boolean or enum
parameter, rather than growing more wrapper functions. `damage_dealt`
stays the stable public order every existing caller depends on.
