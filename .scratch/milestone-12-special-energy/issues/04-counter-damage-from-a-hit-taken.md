# Counter-damage from a hit taken

Type: task
Status: open

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

- [ ] The attacker takes 2 damage counters when its attack damages
      the carrier while the carrier is Active
- [ ] Still fires even when the hit knocks the carrier out
- [ ] Does not fire for a hit that deals zero damage (a miss, an
      invulnerability), or one that lands on the carrier while
      Benched
- [ ] `Spiky Energy` plays

Blocked by: 01
