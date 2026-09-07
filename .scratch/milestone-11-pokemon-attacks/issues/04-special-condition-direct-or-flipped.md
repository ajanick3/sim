# A Special Condition, direct or coin-flipped

Type: task
Status: resolved

*"Your opponent's Active Pokémon is now Poisoned."* (`Brute Bonnet`),
*"Flip a coin. If heads, your opponent's Active Pokémon is now
Paralyzed."* (`Zeraora`, `Dedenne`), and *"Flip a coin. If heads, this
attack does 20 more damage."* (`Applin`).

- [x] `AttackEffect::InflictsCondition(Condition)` — direct
- [x] `AttackEffect::CoinFlipInflicts(Condition)` — flipped
- [x] `AttackEffect::CoinFlipBonusDamage(u32)` — flipped, computed
      before `damage_dealt` the same way `DamagePerCount` is

Found and fixed a real gap along the way: `read_attack`'s damage
parsing only accepted a `×`-suffixed printed damage (ticket 02); a
`+`-suffixed one (`Applin`'s `"10+"`) refused on `DamageIsNotANumber`
even after `known_attack` matched it. Recorded in
[ADR 0057](../../../docs/adr/0057-a-conditional-damage-attack-parses-its-plus-suffix.md).

## Resolution

Three new `AttackEffect` variants. `Brute Bonnet`, `Zeraora`, and
`Dedenne` are matched but not fully admitted — each prints a second
attack this ticket does not build (a count read from the opponent's
Active specifically; a cost paid in the attacker's own Energy; a
search whose limit is itself a board-read count). `Applin`'s
single-attack print (`sv06-017`) completes.

Coverage: `admitted` 526 -> 527 (1 print); `AttackHasText` (refused)
1647 -> 1646.
