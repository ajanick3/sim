# A restriction on the attacker's own next turn

Type: task
Status: resolved

*"During your next turn, this Pokémon can't use attacks."* —
`N's Zekrom`'s `Rampaging Thunder`.

- [x] `AttackEffect::AttackerCannotAttackNextTurn`
- [x] New `GameState.own_next_turn_restriction`, the mirror of ticket
      05's field but needing a third, "armed" field to tell the
      granting turn apart from the restricted one — both are the same
      Pokémon's own turn, unlike the opponent-restriction case

Recorded in [ADR 0059](../../../docs/adr/0059-a-restriction-on-the-attackers-own-next-turn-needs-arming.md).

## Resolution

One new field, one new `AttackEffect` variant, one new check in the
Attack-offering site.

`N's Zekrom` completes (2 prints): its other attack (`Shred`) was
already read in ticket 03.

Coverage: `admitted` 528 -> 530 (2 prints); `AttackHasText` (refused)
1645 -> 1643.
