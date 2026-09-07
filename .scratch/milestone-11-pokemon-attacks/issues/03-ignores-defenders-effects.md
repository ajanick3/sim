# Damage that ignores the defender's own effects

Type: task
Status: resolved

*"This attack's damage isn't affected by any effects on your
opponent's Active Pokémon."* — `N's Zekrom`, `Mega Lopunny ex`,
`Dudunsparce ex`.

- [x] `AttackEffect::IgnoresDefendersEffects`, skipping Weakness and
      Resistance (both read as "effects on the defender") without
      touching the public `damage_dealt` every existing test calls

Recorded in [ADR 0056](../../../docs/adr/0056-ignoring-defenders-effects-skips-weakness-too.md).

## Resolution

`N's Zekrom` and `Mega Lopunny ex` are matched but not yet fully
admitted — each prints a second attack (a next-turn self-lock; a
"moved from Bench this turn" conditional bonus) this ticket does not
build. `Dudunsparce ex` completes: its other attack (`Tenacious Tail`)
was already matched in ticket 02.

Coverage: `admitted` 524 -> 526 (2 prints); `AttackHasText` (refused)
1649 -> 1647.
