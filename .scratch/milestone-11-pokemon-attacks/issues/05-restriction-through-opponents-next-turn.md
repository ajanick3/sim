# A restriction through the opponent's next turn

Type: task
Status: resolved

*"During your opponent's next turn, the Defending Pokémon can't
retreat."* — `Yveltal`, `Wellspring Mask Ogerpon ex`.

- [x] `AttackEffect::DefenderCannotRetreatNextTurn`
- [x] New `GameState.opponent_next_turn_restriction`, a lifetime that
      survives the boundary into the target's own next turn and clears
      at the one after that — distinct from every "this turn" fact
      built so far, all cleared at every `begin_turn` unconditionally

Recorded in [ADR 0058](../../../docs/adr/0058-a-restriction-through-the-opponents-next-turn-outlives-begin-turn.md).

## Resolution

One new field, one conditional clear in `begin_turn`, one new
`AttackEffect` variant, one new check in the Retreat-offering site.

`Yveltal`'s `me01-088` print completes (its other attack has no
printed text at all). `Wellspring Mask Ogerpon ex` is matched but not
fully admitted — its other attack (an Energy shuffle for a Bench hit)
awaits ticket 07/08.

Coverage: `admitted` 527 -> 528 (1 print); `AttackHasText` (refused)
1646 -> 1645.
