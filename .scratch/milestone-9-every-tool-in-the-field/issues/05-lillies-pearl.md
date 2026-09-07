# Lillie's Pearl

Type: task
Status: resolved

*"If the Lillie's Pokémon this card is attached to is Knocked Out by
damage from an attack from your opponent's Pokémon, that player takes
1 fewer Prize card."*

The card ADR 0010 named as its example when it decided Prize value is
read at knockout time. Needed a way to tell an attack-caused knockout
from a checkup-caused one, which nothing built distinguished yet — a
new transient field, `attacking_defender`, consumed once per `settle`
call. Recorded in [ADR 0044](../../../docs/adr/0044-a-knockouts-cause-is-a-consumed-flag.md).

- [x] Takes 1 fewer Prize when its Lillie's Pokémon is Knocked Out by
      an attack
- [x] Does not fire for a checkup-caused knockout
- [x] Floored at zero, not negative Prizes

## Resolution

One new field, one new `TrainerEffect`, one adjustment site in
`knock_out_the_dead`.

Coverage: `admitted` 491 -> 492 (1 print); `trainers` (refused)
300 -> 299.
