# Draws

Type: task
Status: resolved

*"Draw 2 cards."* — `Mega Sharpedo ex`'s `Greedy Fang`.

- [x] `AttackEffect::DrawCards(u32)`

Its sibling attack needed a second, small primitive to admit the same
print:

*"If this Pokémon has any damage counters on it, this attack does 150
more damage."* — `Mega Sharpedo ex`'s `Hungry Jaws`.

- [x] `AttackEffect::BonusDamageIfOwnDamaged(u32)`, read once before
      `damage_dealt_with` runs, the same slot `CoinFlipBonusDamage`
      already occupies

Recorded in [ADR 0065](../../../docs/adr/0065-a-boolean-board-fact-bonus-is-its-own-shape-not-a-count.md).

## Resolution

Two new `AttackEffect` variants, no new `Phase` or `Action` — neither
needs a player choice.

`Mega Sharpedo ex` is admitted (3 prints); both its attacks needed a
new shape, since a card admits only once every attack it prints reads.

Coverage: `admitted` 543 -> 546 (3 prints).
