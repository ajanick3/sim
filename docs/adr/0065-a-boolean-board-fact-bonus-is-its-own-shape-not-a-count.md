# A threshold bonus reads its own flag, not `DamagePerCount` at a cap

**Status:** Accepted — 2026-09-07

`Mega Sharpedo ex`'s `Hungry Jaws` reads "If this Pokémon has any
damage counters on it, this attack does 150 more damage" — a
yes/no board fact, not a scaling one. `AttackEffect::DamagePerCount`
already reads `Count::OwnDamageCounters` and multiplies; capping that
product at a single unit (`per_unit` applied only if the count is
nonzero, discarding how many past one) would repurpose a primitive
built for scaling to answer a threshold question it was never shaped
for, and every future card that prints a real threshold — "if
Poisoned," "if this Pokémon evolved this turn" — would need the same
workaround again. `AttackEffect::BonusDamageIfOwnDamaged(u32)` reads
the same `damage > 0` fact `Count::OwnDamageCounters` already exposes
through `count_for_attack`, but as a flag: read once in `attack`'s
`base` computation, the same pre-`damage_dealt_with` slot
`CoinFlipBonusDamage` already occupies, so `resolve_attack_effect`'s
own arm is a no-op, matching that variant's own comment.
`Greedy Fang`'s `Draw 2 cards` (`AttackEffect::DrawCards`, this
ticket's other half) needed no such choice — it runs unconditionally
in `resolve_attack_effect`, the same shape a Trainer's own draw
already takes.
