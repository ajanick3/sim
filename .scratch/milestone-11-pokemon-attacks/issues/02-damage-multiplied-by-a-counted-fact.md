# Damage multiplied by a counted board fact

Type: task
Status: resolved

*"This attack does N damage for each X"* — five cards, one shape, five
counted facts.

- [x] `AttackEffect::DamagePerCount(Count, u32)`, computed before
      `damage_dealt` runs, so Weakness/Resistance/Tool/Stadium bonuses
      still apply to the total
- [x] Five `Count` variants: own damage counters, opponent's Basic
      Energy in discard, opponent's Pokémon ex in play, own Basic
      Pokémon in play, own damaged Pokémon sharing a name prefix

Recorded in [ADR 0055](../../../docs/adr/0055-a-per-count-attack-computes-its-base-before-damage-dealt.md).

## Resolution

`N's Darmanitan` and `Dudunsparce ex` are matched by name for their
counted attack, but neither card is fully admitted yet — each prints a
second attack (a Bench hit; ignoring the defender's own effects) this
ticket does not build, and `read_attack` requires every attack on a
print to be readable. Both become playable once their own tickets
(07, 03) land.

Built and fully admitted: `N's Reshiram` (3 prints, single-attack),
`Passimian` (1 print, single-attack), and `Paldean Tauros`'s
`me02-048` print (both its attacks — `Double-Edge`'s recoil from
ticket 01, and `Raging Charge`'s count here).

Coverage: `admitted` 519 -> 524 (5 prints); `AttackHasText` (refused)
1654 -> 1649.
