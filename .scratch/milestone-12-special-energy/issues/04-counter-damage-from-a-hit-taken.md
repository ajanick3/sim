# Counter-damage from a hit taken

Type: task
Status: resolved

`Spiky Energy`: *"As long as this card is attached to a Pokémon, it
provides {C} Energy. If the Pokémon this card is attached to is in
the Active Spot and is damaged by an attack from your opponent's
Pokémon (even if this Pokémon is Knocked Out), put 2 damage counters
on the Attacking Pokémon."* 12 slots.

Nothing today reflects damage back at an attacker as a standing
consequence of a hit landing — `damage_dealt_with` computes damage
taken, but never turns around and adds damage counters to the
attacker itself. This ticket adds that read, gated on the carrier
being both Active and the one actually damaged.

- [x] The attacker takes 2 damage counters when its attack damages
      the carrier while the carrier is Active
- [x] Still fires even when the hit knocks the carrier out
- [x] Does not fire for a hit that deals zero damage (a miss, an
      invulnerability), or one that lands on the carrier while
      Benched
- [x] `Spiky Energy` plays

Blocked by: 01

## Resolution

`EnergyEffect::CountersAttackerOnDamageTakenWhileActive(u32)`, read
right where `attack()`'s own base damage lands on the primary
defender — the only site that damage ever lands, since a
direct-Bench-targeting `AttackEffect` bypasses this function
entirely. "While Active" needed no separate check: `defender` in
this function is, by construction, always the opponent's Active — no
attack in this single-Active-format engine ever targets a Benched
Pokémon through this path. Gated on `damage > 0`, the same guard
`trigger_defenders_tool` already reads by, so a miss or a prevented
hit does not counter-damage the attacker either.

Admits `Spiky Energy` (two prints). Coverage moves from 679 to 681.
